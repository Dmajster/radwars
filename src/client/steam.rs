use bevy::prelude::*;

#[cfg(feature = "steam")]
use steamworks::{AppId, Client, FriendFlags, PersonaStateChange};

#[derive(Default)]
pub struct SteamPlugin {}

impl Plugin for SteamPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        init(app_builder);
    }
}

#[cfg(feature = "steam")]
fn init(app_builder: &mut AppBuilder) {
    let (client, single) = Client::init().unwrap();
}

#[cfg(not(feature = "steam"))]
fn init(_: &mut AppBuilder) {}
