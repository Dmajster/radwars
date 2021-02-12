use bevy::{
    audio::AudioPlugin, pbr::PbrPlugin, prelude::*, render::RenderPlugin, sprite::SpritePlugin,
    text::TextPlugin, ui::UiPlugin, wgpu::WgpuPlugin, window::WindowPlugin, winit::WinitPlugin,
};

mod udp_client;
mod steam;
mod in_game;
use in_game::InGamePlugin;

#[cfg(feature = "steam")]
use steamworks::{AppId, Client, FriendFlags, PersonaStateChange};

#[cfg(feature = "steam")]
fn init_steam() {
    let (client, single) = Client::init().unwrap();
}

#[cfg(not(feature = "steam"))]
fn init_steam() {}

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

        group.add(InGamePlugin::default());

        if cfg!(feature = "steam") {
            init_steam();
        }
    }
}
