use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::{
    na::Quaternion,
    physics::{RapierPhysicsPlugin, RigidBodyHandleComponent},
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::ColliderBuilder,
    },
};
use bevy_rapier3d::{na::UnitQuaternion, render::RapierRenderPlugin};

#[derive(Default)]
pub struct GameplayPlugin {}

impl Plugin for GameplayPlugin {
    fn build(&self, app_builder: &mut bevy::prelude::AppBuilder) {
        app_builder
            .add_startup_system(setup.system())
            .add_plugin(RapierPhysicsPlugin)
            .add_plugin(RapierRenderPlugin)
            .add_system(local_player_input.system())
            .add_system(move_player.system());
    }
}

fn setup(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
            parent.spawn(Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..Default::default()
            });
        });

    // plane
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_static())
        .with(ColliderBuilder::cuboid(10.0, 0.001, 10.0));

    // cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        })
        .with(RigidBodyBuilder::new_dynamic().translation(0.0, 10.0, 0.0))
        .with(ColliderBuilder::cuboid(0.5, 0.5, 0.5));

    // light
    commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(-2.0, 2.5, 5.0))
            .looking_at(Vec3::default(), Vec3::unit_y()),
        ..Default::default()
    });
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

#[derive(Default)]
struct State {
    mouse_motion_event_reader: EventReader<MouseMotion>,
}

fn local_player_input(
    mut state: Local<State>,
    mut query: Query<&mut PlayerInput>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut mouse_motion_vector = Vec2::default();

    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
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
            let movement_speed = 2.0;
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
