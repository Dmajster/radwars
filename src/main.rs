#![feature(vec_into_raw_parts)]
#![feature(min_const_generics)]
#![feature(array_chunks)] //TODO remove unsafe from collider mesh generation with this?

use bevy::prelude::App;

mod shared;
use shared::SharedPlugins;

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
use server::init;

#[cfg(not(feature = "server"))]
mod client;
#[cfg(not(feature = "server"))]
use client::init;

fn main() {
    let mut app_builder = App::build();
    app_builder.add_plugins(SharedPlugins);

    init(&mut app_builder);

    app_builder.run();
}
