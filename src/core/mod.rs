use bevy::prelude::*;

pub mod bloon;
pub mod movement;
pub mod hitbox;
pub mod projectile;
pub mod tower;
pub mod map;
pub mod event;

pub struct BTD0CorePlugin;

impl Plugin for BTD0CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(map::Map::get_map(1))
        .init_resource::<bloon::OverkillLookupTable>();

        app.add_event::<event::GlobalDamageEvent>()
        .add_event::<event::BloonDamageEvent>();

        app.add_systems(Startup, bloon::generate_lookup_overkill_bloon)
        .add_systems(FixedPreUpdate, 
            (event::global_damage_effects, projectile::damage_bloons, event::apply_bloon_damage).chain()
        )
        .add_systems(FixedUpdate, (
            (movement::move_along_road, bloon::pop_bloons, movement::despawn_exited_road_entities).chain(),
            (projectile::lifetime_tick, movement::move_simple).chain(),
        ));
    }
}