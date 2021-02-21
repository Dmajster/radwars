use bevy::{
    asset::AssetPlugin, core::CorePlugin, diagnostic::DiagnosticsPlugin, gltf::GltfPlugin,
    input::InputPlugin, log::LogPlugin, prelude::PluginGroup, reflect::ReflectPlugin,
    scene::ScenePlugin, transform::TransformPlugin,
};

pub mod game_message;

pub mod gameplay;
use gameplay::GameplayPlugin;

pub struct SharedPlugins;
impl PluginGroup for SharedPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(LogPlugin::default());
        group.add(ReflectPlugin::default());
        group.add(CorePlugin::default());
        group.add(TransformPlugin::default());
        group.add(DiagnosticsPlugin::default());
        group.add(InputPlugin::default());
        group.add(AssetPlugin::default());
        group.add(ScenePlugin::default());

        group.add(GltfPlugin::default());

        group.add(GameplayPlugin::default());
    }
}
