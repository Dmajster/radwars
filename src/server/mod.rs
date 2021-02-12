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
    fn build(&self, app_builder: &mut bevy::prelude::AppBuilder) {
        let server_updates_per_second = 1;

        let server = UdpServerBuilder::new()
            .bind_address("127.0.0.1:8311")
            .client_max_count(32)
            .message_buffer_byte_size(1500)
            .nonblocking(true)
            .build();

        app_builder.add_resource(server);
        app_builder.add_stage_before(
            stage::UPDATE,
            "multiplayer_pre_update",
            SystemStage::serial()
                .with_run_criteria(FixedTimestep::step(1.0 / server_updates_per_second as f64))
                .with_system(server_receive.system())
                .with_system(server_send.system()),
        );
    }
}

fn server_receive(mut udp_server: ResMut<UdpServer>) {
    println!("Server update");

    //TODO: check if this can be abused by spamming the server with messages?!
    'data: loop {
        let received = udp_server.receive().expect("Failed to retrieve message");

        if let Some(message) = received {
            match message.content {
                GameMessageType::ClientInput(message) => {
                    //println!("Received input: {:#?}", message)
                }
                GameMessageType::ClientConnected => {}
                GameMessageType::ClientDisconnected => {}
                GameMessageType::ClientLoading => {}
                GameMessageType::ClientEntered => {}
                unknown_message => {
                    println!("Received unexpected message: {:#?}", unknown_message);
                }
            }
        } else {
            break 'data;
        }
    }
}

fn server_send(udp_server: Res<UdpServer>) {
    udp_server.send_to_all(GameMessageType::ServerGameStateSnapshot(
        ServerGameStateSnapshotData {
            player_ids: Vec::new(),
            player_positions: Vec::new(),
            player_rotations: Vec::new(),
        },
    ));
}
