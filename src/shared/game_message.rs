use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameMessageType {
    //A new client connected to the server
    ClientConnected,

    //A client left the server
    ClientDisconnected,

    //client is loading the game assets
    ClientLoading,

    //client entered the game
    ClientEntered,

    //client input data
    ClientInput(ClientInputData),

    //server snapshot of the world
    ServerGameStateSnapshot(ServerGameStateSnapshotData),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameMessage {
    //consecutive number incrementing with each packet sent
    pub sequence_index: u32,

    //last received sequence index from the other socket
    pub last_received_sequence_index: u32,

    //bitmask showing which of the last 16 packets have been received
    pub acknowledge_mask: u16,

    //duration the game needed before it responded back to the other socket
    pub processing_duration: Duration,

    //enum of all possible message types you can send
    pub content: GameMessageType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientInputData {
    pub move_left: bool,
    pub move_right: bool,
    pub move_forward: bool,
    pub move_back: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerGameStateSnapshotData {
    pub player_ids: Vec<u8>,
    pub player_positions: Vec<(f32, f32, f32)>,
    pub player_rotations: Vec<(f32, f32, f32)>,
}
