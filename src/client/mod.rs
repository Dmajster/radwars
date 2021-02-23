use bevy::{
    audio::AudioPlugin, pbr::PbrPlugin, prelude::*, render::RenderPlugin, sprite::SpritePlugin,
    text::TextPlugin, ui::UiPlugin, wgpu::WgpuPlugin, window::WindowPlugin, winit::WinitPlugin,
};

mod steam;
use steam::SteamPlugin;

mod udp_client;

mod developer;
use developer::DeveloperPlugin;

pub fn init(app_builder: &mut AppBuilder) {
    app_builder.add_plugins(ClientPlugins);
}

pub struct ClientPlugins;
impl PluginGroup for ClientPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(WindowPlugin::default());
        group.add(RenderPlugin::default());
        group.add(SpritePlugin::default());
        group.add(PbrPlugin::default());
        group.add(UiPlugin::default());
        group.add(TextPlugin::default());
        group.add(AudioPlugin::default());
        group.add(WinitPlugin::default());
        group.add(WgpuPlugin::default());

        //group.add(InGamePlugin::default()); TODO: Add this networking to gameplay plugin

        if cfg!(feature = "steam") {
            group.add(SteamPlugin::default());
        }

        // if cfg!(debug_assertions) {
        //     group.add(DeveloperPlugin::default());
        // }

        group.add(DeveloperPlugin::default());
    }
}
