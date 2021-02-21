use bevy::{input::mouse::MouseMotion, prelude::*, scene::InstanceId};
use bevy_rapier3d::{
    na::{Quaternion, UnitQuaternion},
    physics::{RapierPhysicsPlugin, RigidBodyHandleComponent},
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
    },
    render::RapierRenderPlugin,
};

#[derive(Default)]
pub struct GameplayPlugin {}

impl Plugin for GameplayPlugin {
    fn build(&self, app_builder: &mut bevy::prelude::AppBuilder) {
        app_builder
            .add_startup_system(setup.system())
            .insert_resource(SceneInstance::default())
            .add_plugin(RapierPhysicsPlugin)
            .add_plugin(RapierRenderPlugin)
            .add_system(local_player_input.system())
            .add_system(move_player.system())
            .add_system(scene_update.system());
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut scene_instance: ResMut<SceneInstance>,
) {
    // let map_handle = asset_server.load("models/test_map.gltf#Mesh0/Primitive0");

    // //Map
    // commands.spawn(PbrBundle {
    //     mesh: map_handle,
    //     material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
    //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    //     ..Default::default()
    // });

    let test_scene_id = scene_spawner.spawn(asset_server.load("models/test_map.gltf#Scene0"));
    scene_instance.0 = Some(test_scene_id);

    // add entities to the world
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..Default::default()
        })
        .with(
            RigidBodyBuilder::new_kinematic()
                .restrict_rotations(false, true, false)
                .translation(0.0, 2.0, 0.0),
        )
        .with(ColliderBuilder::capsule_y(1.0, 1.0))
        .with(PlayerInput::default())
        .with_children(|parent| {
            parent.spawn(PerspectiveCameraBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            });
        });

    // // plane
    // commands
    //     .spawn(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
    //         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //         ..Default::default()
    //     })
    //     .with(RigidBodyBuilder::new_static())
    //     .with(ColliderBuilder::cuboid(10.0, 0.001, 10.0));

    // cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_dynamic().translation(0.0, 100.0, 0.0))
        .with(ColliderBuilder::cuboid(0.5, 0.5, 0.5));

    // light
    commands.spawn(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    asset_server.watch_for_changes().unwrap();
}

#[derive(Debug, Default)]
struct PlayerInput {
    move_left: bool,
    move_right: bool,
    move_forward: bool,
    move_back: bool,

    mouse_horizontal: f32,
    mouse_vertical: f32,
}

fn local_player_input(
    mut query: Query<&mut PlayerInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut mouse_motion_vector = Vec2::default();

    for event in mouse_motion_events.iter() {
        mouse_motion_vector -= event.delta;
    }

    for mut player_input in query.iter_mut() {
        player_input.move_forward = keyboard_input.pressed(KeyCode::W);
        player_input.move_back = keyboard_input.pressed(KeyCode::S);
        player_input.move_left = keyboard_input.pressed(KeyCode::A);
        player_input.move_right = keyboard_input.pressed(KeyCode::D);

        player_input.mouse_horizontal = mouse_motion_vector.x;
        player_input.mouse_vertical = mouse_motion_vector.y;
    }
}

fn move_player(
    time: Res<Time>,
    mut rigidbody_set: ResMut<RigidBodySet>,
    mut transform_query: Query<&mut Transform>,
    mut player_query: Query<(Entity, &Children, &RigidBodyHandleComponent, &PlayerInput)>,
) {
    for (entity, children, rigidbody_handle, player_input) in player_query.iter_mut() {
        if let Ok(transform) = transform_query.get_mut(entity) {
            // POSITION
            let mut input_vector = Vec3::default();
            let movement_speed = 5.0;
            let mouse_speed = 1.4;

            if player_input.move_right {
                input_vector.x += 1.0;
            }
            if player_input.move_left {
                input_vector.x -= 1.0;
            }
            if player_input.move_forward {
                input_vector.y -= 1.0;
            }
            if player_input.move_back {
                input_vector.y += 1.0;
            }

            if input_vector.length() > 0.0 {
                input_vector.normalize();
            }

            input_vector *= movement_speed * time.delta_seconds();

            let forward_direction = transform.rotation * Vec3::unit_z();
            let strafing_direction = transform.rotation * Vec3::unit_x();
            let movement_vector =
                forward_direction * input_vector.y + strafing_direction * input_vector.x;

            let rigidbody = rigidbody_set.get_mut(rigidbody_handle.handle()).unwrap();
            let mut rigidbody_transform = *rigidbody.position();
            rigidbody_transform.translation.vector.x += movement_vector.x;
            rigidbody_transform.translation.vector.y += movement_vector.y;
            rigidbody_transform.translation.vector.z += movement_vector.z;

            let input_rotation = Quat::from_rotation_y(
                player_input.mouse_horizontal * mouse_speed * time.delta_seconds(),
            );

            // ROTATION
            let new_rotation = UnitQuaternion::from_quaternion(Quaternion::new(
                input_rotation.w,
                input_rotation.x,
                input_rotation.y,
                input_rotation.z,
            ));

            rigidbody_transform.append_rotation_wrt_center_mut(&new_rotation);

            rigidbody.set_next_kinematic_position(rigidbody_transform);
        }

        if let Ok(mut transform) = transform_query.get_mut(children[0]) {
            transform.rotate(Quat::from_rotation_x(
                player_input.mouse_vertical * time.delta_seconds(),
            ));
        }
    }
}

#[derive(Default)]
struct SceneInstance(Option<InstanceId>);

fn scene_update(
    commands: &mut Commands,
    meshes: Res<Assets<Mesh>>,
    scene_spawner: Res<SceneSpawner>,
    scene_instance: Res<SceneInstance>,
    mut done: Local<bool>,
    mesh_query: Query<&Handle<Mesh>>,
) {
    if !*done {
        if let Some(instance_id) = scene_instance.0 {
            if let Some(entity_iter) = scene_spawner.iter_instance_entities(instance_id) {
                entity_iter.for_each(|entity| {
                    println!("entity: {:#?}", entity);

                    if let Ok(mesh_handle) = mesh_query.get_component::<Handle<Mesh>>(entity) {
                        println!("mesh handle: {:#?}", mesh_handle);

                        if let Some(mesh) = meshes.get(mesh_handle) {
                            let bytes = mesh.get_vertex_buffer_data();
                            let format = mesh.get_vertex_buffer_layout();
                            let stride_count = format.stride as usize;

                            if let Some(position_attribute) = format
                                .attributes
                                .iter()
                                .find(|attribute| attribute.name == "Vertex_Position")
                            {
                                

                                let initial_offset = position_attribute.offset as usize;
                                let position_count = 12 as usize;

                                let mut position_bytes: Vec<u8> = Vec::new();
                                let mut take_index = initial_offset;
                                while take_index < bytes.len() {
                                    position_bytes.extend_from_slice(&bytes[take_index..(take_index+position_count)]);

                                    take_index += stride_count;
                                }

                                let positions: Vec<bevy_rapier3d::na::Point<f32, bevy_rapier3d::na::U3>> =
                                    convert_using_into_raw_parts(position_bytes);

                                //println!("positions: {:?}", positions.len());

                                match mesh.indices().unwrap() {
                                    bevy::render::mesh::Indices::U16(vec) => {}
                                    bevy::render::mesh::Indices::U32(vec) => {
                                        println!("old indices: {:?}", vec);
                                        
                                        let indices: Vec<[u32; 3]> = group_vec(vec.clone());

                                        println!("indices: {:?}", indices);

                                        commands.insert_one(entity, RigidBodyBuilder::new_static());
                                        commands.insert_one(
                                            entity,
                                            ColliderBuilder::trimesh(positions, indices)
                                        );
                                    }
                                };
                            }

                            println!("format: {:#?}", format);
                        }
                    }
                });
                *done = true;
            }
        }
    }
}

fn convert_using_into_raw_parts<T, U>(v: Vec<T>) -> Vec<U> {
    let (ptr, len, cap) = v.into_raw_parts();
    unsafe { Vec::from_raw_parts(ptr as *mut U, len, cap) }
}

fn group_vec<T, const N: usize>(mut vec: Vec<T>) -> Vec<[T; N]> {
    assert_eq!(vec.len() % N, 0);
    if vec.capacity() % N != 0 {
        vec.shrink_to_fit();
    }
    let ptr = vec.as_mut_ptr();
    let len = vec.len();
    let capacity = vec.capacity();
    std::mem::forget(vec);
    unsafe { Vec::from_raw_parts(ptr.cast(), len / N, capacity / N) }
}