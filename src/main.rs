use bevy::prelude::*;

mod map;
use map::*;

mod bloon;
use bloon::*;

mod graphics;
use graphics::*;

mod damage;
use damage::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MapPlugin, BloonPlugin, GraphicsPlugin, DamagePlugin))
        .run();
}