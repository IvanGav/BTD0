use bevy::prelude::*;

pub mod damage_handling;
pub mod projectile;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<damage_handling::GlobalDamageEvent>()
        .add_event::<damage_handling::BloonDamageEvent>()
        .add_systems(FixedPreUpdate, 
            (damage_handling::global_damage_effects, damage_handling::damage_bloons, damage_handling::apply_bloon_damage).chain()
        )
        .add_systems(FixedUpdate, (
            // projectile::despawn_stray_projectile, 
            projectile::despawn_projectile, 
            projectile::move_projectile
        ));
    }
}