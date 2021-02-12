use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket},
    time::Duration,
};

use crate::shared::game_message::{GameMessage, GameMessageType};

pub struct UdpServer {
    //Is used to listen for data from other clients
    server_socket: UdpSocket,

    //Buffer that stores incoming message bytes
    message_buffer: Vec<u8>,

    //Other client sockets
    client_sockets: Vec<SocketAddr>,
}

impl UdpServer {
    pub fn send(&self, target_client_socket_index: usize, message_content: GameMessageType) {
        let message = GameMessage {
            sequence_index: 0,
            last_received_sequence_index: 0,
            processing_duration: Duration::default(),
            acknowledge_mask: 0b0000_0000,
            content: message_content,
        };

        let bytes = bincode::serialize(&message).unwrap();

        self.server_socket
            .send_to(&bytes, self.client_sockets[target_client_socket_index])
            .unwrap();
    }

    pub fn send_to_all(&self, message_content: GameMessageType) {
        let message = GameMessage {
            sequence_index: 0,
            last_received_sequence_index: 0,
            processing_duration: Duration::default(),
            acknowledge_mask: 0b0000_0000,
            content: message_content,
        };

        let bytes = bincode::serialize(&message).unwrap();
        for client_socket_addr in &self.client_sockets {
            self.server_socket
                .send_to(&bytes, client_socket_addr)
                .unwrap();
        }
    }

    pub fn receive(&mut self) -> Result<Option<GameMessage>, Box<dyn Error>> {
        match self.server_socket.recv_from(&mut self.message_buffer) {
            Ok((byte_count, client_socket_addr)) => {
                if !self.client_sockets.contains(&client_socket_addr) {
                    self.client_sockets.push(client_socket_addr);
                    println!("Client socket added: {}", client_socket_addr);
                }

                let game_message: GameMessage =
                    bincode::deserialize(&self.message_buffer[..byte_count])?;

                println!("{:#?}", game_message);

                Ok(Some(game_message))
            }

            _ => Ok(None),
        }
    }
}

pub struct UdpServerBuilder {
    bind_address: SocketAddr,
    message_buffer_byte_size: usize,
    client_max_count: usize,
    nonblocking: bool,
}

impl UdpServerBuilder {
    pub fn new() -> Self {
        Self {
            bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8311),
            message_buffer_byte_size: 1500,
            client_max_count: 32,
            nonblocking: true,
        }
    }

    pub fn bind_address<A: ToSocketAddrs>(mut self, address: A) -> Self {
        self.bind_address = address.to_socket_addrs().unwrap().next().unwrap();
        self
    }

    pub fn message_buffer_byte_size(mut self, byte_size: usize) -> Self {
        self.message_buffer_byte_size = byte_size;
        self
    }

    pub fn client_max_count(mut self, max_count: usize) -> Self {
        self.client_max_count = max_count;
        self
    }

    pub fn nonblocking(mut self, nonblocking: bool) -> Self {
        self.nonblocking = nonblocking;
        self
    }

    pub fn build(self) -> UdpServer {
        let server_socket = UdpSocket::bind(self.bind_address).unwrap();
        server_socket.set_nonblocking(self.nonblocking).unwrap();

        UdpServer {
            server_socket,
            message_buffer: vec![0; self.message_buffer_byte_size],
            client_sockets: Vec::with_capacity(self.client_max_count),
        }
    }
}
