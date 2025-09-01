use bevy::prelude::*;

mod graphics;
use graphics::*;

mod core;
use core::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GraphicsPlugin, BTD0CorePlugin))
        .run();
}