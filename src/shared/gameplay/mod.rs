use core::panic;
use std::{convert::TryInto, error::Error};

use bevy::{prelude::*, scene::InstanceId};

mod player_movement;
use heron::{Body, Gravity, PhysicsPlugin, Velocity};
use player_movement::player_movement;

mod player_input;
use player_input::player_local_input;

use crate::shared::gameplay::player_input::PlayerInput;

#[derive(Default)]
pub struct GameplayPlugin {}

impl Plugin for GameplayPlugin {
    fn build(&self, app_builder: &mut bevy::prelude::AppBuilder) {
        app_builder
            .add_plugin(PhysicsPlugin::default()) // Add the plugin
            .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0))) // Optionally define gravity
            .add_startup_system(setup.system())
            .insert_resource(SceneInstance::default())
            .add_system(player_local_input.system())
            .add_system(player_movement.system())
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
    let test_scene_id = scene_spawner.spawn(asset_server.load("models/test_map.gltf#Scene0"));
    scene_instance.0 = Some(test_scene_id);

    // add entities to the world
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..Default::default()
        })
        .with(Body::Capsule {
            half_segment: 1.0,
            radius: 0.5,
        })
        .with(Transform::from_xyz(0.0, 8.0, 0.0))
        .with(Velocity::default())
        .with(PlayerInput::default())
        .with_children(|parent| {
            parent.spawn(PerspectiveCameraBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            });
        });

    // cube
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        })
        .with(Transform::from_xyz(0.0, 100.0, 0.0))
        .with(Body::Cuboid {
            half_extends: Vec3::new(0.5, 0.5, 0.5),
        })
        .with(Velocity::default());

    // light
    commands.spawn(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    asset_server.watch_for_changes().unwrap();
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
                    if let Ok(mesh_handle) = mesh_query.get_component::<Handle<Mesh>>(entity) {
                        if let Some(mesh) = meshes.get(mesh_handle) {
                            if let Ok((positions, indices)) = mesh_extract_positions_and_indices(mesh) {
                                commands.insert_one(entity, Body::TriMesh { positions, indices });
                            }
                        }
                    }
                });
                *done = true;
            }
        }
    }
}

fn mesh_extract_positions_and_indices(
    mesh: &Mesh,
) -> Result<(Vec<Vec3>, Vec<[u32; 3]>), Box<dyn Error>> {
    let bytes = mesh.get_vertex_buffer_data();
    let format = mesh.get_vertex_buffer_layout();
    let stride_count = format.stride as usize;

    if let Some(position_attribute) = format
        .attributes
        .iter()
        .find(|attribute| attribute.name == "Vertex_Position")
    {
        let position_offset = position_attribute.offset as usize;

        let position_byte_chunks: Vec<[u8; 12]> = bytes
            // Split bytes into byte slices the size of individual vertex
            .chunks_exact(stride_count)
            // Take 12 bytes from the offset where position starts (12 => Vec3(f32,f32,f32))
            .map(|vertex_bytes| {
                let position_bytes_slice: [u8; 12] = vertex_bytes
                    [position_offset..position_offset + 12]
                    .try_into()
                    .unwrap();

                position_bytes_slice
            })
            .collect();

        let position_bytes = position_byte_chunks.concat();

        let position_floats: Vec<f32> = position_bytes
            // Create slices of &[u8]
            .chunks_exact(4)
            // Try map &[u8] to &[u8; 4]
            .map(|c| c.try_into().unwrap())
            // Map &[u8; 4] to f32
            .map(|bytes| f32::from_le_bytes(bytes))
            .collect();

        let positions: Vec<Vec3> = position_floats
            // Create slices of &[f32]
            .chunks_exact(3)
            // Map &[f32] to Vec3
            .map(|floats| Vec3::new(floats[0], floats[1], floats[2]))
            .collect();

        let indices: Vec<[u32; 3]> = match mesh.indices().unwrap() {
            bevy::render::mesh::Indices::U16(vec) => {
                let short_indices: Vec<u32> = vec
                    .iter()
                    // Map u16 to u32
                    .map(|u| *u as u32)
                    .collect();

                short_indices
                    // Create slices of &[u32]
                    .chunks_exact(3)
                    // Try map &[u32] to &[u32; 3]
                    .map(|c| c.try_into().unwrap())
                    .collect()
            }
            bevy::render::mesh::Indices::U32(vec) => {
                vec
                    // Create slices of &[u32]
                    .chunks_exact(3)
                    // Try map &[u32] to &[u32; 3]
                    .map(|c| c.try_into().unwrap())
                    .collect()
            }
        };

        return Ok((positions, indices));
    }

    panic!("Mesh doesn't have a Vertex_Position attribute")
}
