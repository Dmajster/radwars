use std::{
    error::Error,
    net::{ToSocketAddrs, UdpSocket},
    time::Duration,
};

use crate::shared::game_message::{GameMessage, GameMessageType};

#[derive(Default)]
pub struct UdpManager {
    local_socket: Option<UdpSocket>,
    buffer: Vec<u8>,
}

impl UdpManager {
    pub fn new<A: ToSocketAddrs>(
        listen_address: A,
        buffer_size: usize,
        nonblocking: bool,
    ) -> Result<UdpManager, Box<dyn Error>> {
        let local_socket = UdpSocket::bind(listen_address)?;
        local_socket.set_nonblocking(nonblocking)?;

        Ok(UdpManager {
            local_socket: Some(local_socket),
            buffer: vec![0u8; buffer_size],
        })
    }

    pub fn connect<A: ToSocketAddrs>(&mut self, target_address: A) -> Result<(), Box<dyn Error>> {
        self.local_socket
            .as_ref()
            .unwrap()
            .connect(target_address)?;

        Ok(())
    }

    pub fn send(&self, message_content: GameMessageType) -> Result<(), Box<dyn Error>> {
        let message = GameMessage {
            sequence_index: 0,
            last_received_sequence_index: 0,
            processing_duration: Duration::default(),
            acknowledge_mask: 0b0000_0000,
            content: message_content,
        };

        let bytes = bincode::serialize(&message)?;

        self.local_socket.as_ref().unwrap().send(&bytes)?;

        Ok(())
    }

    pub fn receive(&mut self) -> Result<Option<GameMessage>, Box<dyn Error>> {
        let received = self.local_socket.as_ref().unwrap().recv(&mut self.buffer);

        match received {
            Ok(number_of_bytes) => {
                let game_message: GameMessage =
                    bincode::deserialize(&self.buffer[..number_of_bytes])?;

                println!("{:#?}", game_message);

                return Ok(Some(game_message));
            }
            _ => {}
        }

        Ok(None)
    }
}
