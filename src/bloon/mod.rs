use bevy::prelude::*;

pub mod bloon;

pub struct BloonPlugin;

impl Plugin for BloonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<bloon::OverkillLookupTable>()
            .add_systems(Startup, bloon::generate_lookup_overkill_bloon)
            .add_systems(FixedUpdate, (bloon::move_bloons, bloon::pop_bloons, bloon::despawn_exited_bloons).chain()
        );
    }
}