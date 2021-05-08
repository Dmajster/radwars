use bevy::{
    core::FixedTimestep,
    ecs::{ResMut, SystemStage},
    prelude::*,
};

use crate::shared::game_message::{GameMessageType, ServerGameStateSnapshotData};

mod udp_server;
use crate::server::udp_server::{UdpServer, UdpServerBuilder};

pub fn init(app_builder: &mut AppBuilder) {
    app_builder.add_plugins(ServerPlugins);
    app_builder.set_runner(move |app| server_runner(app));
}

fn server_runner(mut app: App) {
    loop {
        app.update();
    }
}

pub struct ServerPlugins;
impl PluginGroup for ServerPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(ServerPlugin::default());
    }
}

#[derive(Debug, Default)]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, _: &mut bevy::prelude::AppBuilder) {}
}
