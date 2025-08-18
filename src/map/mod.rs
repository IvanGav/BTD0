use bevy::prelude::*;

pub mod map;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(map::Map::get_map(1));
    }
}