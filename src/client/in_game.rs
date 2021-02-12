use bevy::{core::FixedTimestep, prelude::*};

use crate::shared::game_message::{ClientInputData, GameMessageType};
use crate::client::udp_client::UdpManager;

#[derive(Default)]
pub struct InGamePlugin {}
impl Plugin for InGamePlugin {
    fn build(&self, app_builder: &mut bevy::prelude::AppBuilder) {
        let client_updates_per_second = 1;
        let client_target_address = "127.0.0.1:8311";
        let client_listen_address = "127.0.0.1:8310";

        let mut server = UdpManager::new(client_listen_address, 1024, true).expect(&format!(
            "Failed to create server at: {}",
            client_listen_address
        ));
        server.connect(client_target_address).unwrap();

        app_builder.add_resource(server).add_stage_before(
            stage::UPDATE,
            "multiplayer_pre_update",
            SystemStage::serial()
                .with_run_criteria(FixedTimestep::step(1.0 / client_updates_per_second as f64))
                .with_system(client_receive.system())
                .with_system(client_send_input.system()),
        );
    }
}

fn client_receive(mut udp_manager: ResMut<UdpManager>) {
    println!("Client update");

    'data: loop {
        let received = udp_manager.receive().expect("Failed to retrieve message");

        if let Some(message) = received {
            match message.content {
                GameMessageType::ServerGameStateSnapshot(message) => {
                    println!("Received game snapshot: {:#?}", message);
                }
                message => {
                    println!("Received unexpected message: {:#?}", message);
                }
            }
        } else {
            break 'data;
        }
    }
}

fn client_send_input(udp_manager: Res<UdpManager>, keyboard_input: Res<Input<KeyCode>>) {
    udp_manager
        .send(GameMessageType::ClientInput(ClientInputData {
            move_forward: keyboard_input.pressed(KeyCode::W),
            move_left: keyboard_input.pressed(KeyCode::A),
            move_back: keyboard_input.pressed(KeyCode::S),
            move_right: keyboard_input.pressed(KeyCode::D),
        }))
        .unwrap();
}
