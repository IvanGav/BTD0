use bevy::prelude::*;

pub mod damage_handling;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<damage_handling::GlobalDamageEvent>()
        .add_systems(FixedPreUpdate, 
            (damage_handling::global_damage_effects, damage_handling::damage_bloons, damage_handling::apply_bloon_damage).chain()
        );
    }
}