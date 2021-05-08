#[derive(Default)]
pub struct SceneSetupPlugin {}

impl Plugin for GameplayPlugin {
    fn build(&self, app_builder: &mut bevy::prelude::AppBuilder) {
        app_builder
            .add_plugin(RapierPhysicsPlugin)
            .add_plugin(RapierRenderPlugin)
            .add_startup_system(setup.system())
            .insert_resource(SceneInstance::default())
            .add_system(player_local_input.system())
            .add_system(player_movement.system())
            .add_system(scene_update.system());
    }
}



use std::{fs::File, io::Write};

use crate::shared::gameplay::SceneState;
use bevy::{prelude::*, reflect::TypeRegistry};

pub fn scene_load(
    mut scene_state: ResMut<SceneState>,
    mut scene_spawner: ResMut<SceneSpawner>,
    asset_server: Res<AssetServer>,
) {
    if scene_state.loading {
        return;
    }

    scene_state.loading = true;

    println!("Loading scene!");

    // Scenes are loaded just like any other asset.
    let scene_handle: Handle<DynamicScene> = asset_server.load("./scenes/test_scene.scn.ron");

    // SceneSpawner can "spawn" scenes. "Spawning" a scene creates a new instance of the scene in the World with new entity ids.
    // This guarantees that it will not overwrite existing entities.
    scene_spawner.spawn_dynamic(scene_handle);
}

pub fn scene_save(world: &mut World, resources: &mut Resources) {
    let type_registry = resources.get::<TypeRegistry>().unwrap();
    let scene = DynamicScene::from_world(&world, &type_registry);

    let scene_ron = scene.serialize_ron(&type_registry).unwrap();

    let mut file = File::create("./assets/scenes/test_scene.scn.ron").unwrap();
    file.write(scene_ron.as_bytes()).unwrap();

    // if let Ok(scene_json) = scene.serialize_ron(&type_registry) {
    //     println!("scene_json: {:#?}", scene_json);

    //     if let Ok(mut file) = File::create("./scenes/test_scene.ron") {
    //         println!("is ok");

    //         file.write(scene_json.as_bytes()).unwrap();
    //     }
    // }
}
