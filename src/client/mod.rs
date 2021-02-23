use bevy::{
    audio::AudioPlugin, pbr::PbrPlugin, prelude::*, render::RenderPlugin, sprite::SpritePlugin,
    text::TextPlugin, ui::UiPlugin, wgpu::WgpuPlugin, window::WindowPlugin, winit::WinitPlugin,
};

use bevy_egui::{EguiContext, EguiPlugin, egui::Window};

mod steam;
use steam::SteamPlugin;

mod udp_client;

pub fn init(app_builder: &mut AppBuilder) {
    app_builder.add_plugins(ClientPlugins);
    app_builder.add_plugin(EguiPlugin);
    app_builder.add_system(ui_example.system());
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    let ctx = &mut egui_context.ctx;

    Window::new("Hello").show(ctx, |ui| {
        ui.label("world");
    });
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
    }
}
