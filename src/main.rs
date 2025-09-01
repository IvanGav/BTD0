use bevy::prelude::*;

mod map;
use map::*;

mod bloon;
use bloon::*;

mod graphics;
use graphics::*;

mod damage;
use damage::*;

mod tower;
use tower::*;

mod core;
use core::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GraphicsPlugin, BTD0CorePlugin))
        .run();
}