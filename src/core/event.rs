use bevy::prelude::*;

use crate::core::bloon::{Bloon, BloonEffect};

/// A system that applies a global damage effect on all active bloons
pub fn global_damage_effects(mut bloons: Query<&mut Bloon>, mut global_damage_ev: EventReader<GlobalDamageEvent>) {
    for ev in global_damage_ev.read() {
        for mut bloon in &mut bloons {
            // bloon.hp -= ev.damage;
            bloon.hp = 0;
            if let Some(effect) = &ev.status_effect {
                bloon.apply_effect(*effect);
            }
        }
    }
}

/// Apply queued damage to bloons.
pub fn apply_bloon_damage(mut bloons: Query<&mut Bloon>, mut damage_er: EventReader<BloonDamageEvent>) {
    for ev in damage_er.read() {
        if let Ok(mut bloon) = bloons.get_mut(ev.bloon) {
            bloon.hp -= ev.damage;
            if let Some(effect) = &ev.status_effect {
                bloon.apply_effect(*effect);
            }
        }
    }
}

/*
    Events
*/

#[derive(Event)]
/// Damage all alive bloons
pub struct GlobalDamageEvent {
    pub damage: i32,
    pub status_effect: Option<BloonEffect>,
}

#[derive(Event)]
/// Damage a specific bloon
pub struct BloonDamageEvent {
    pub damage: i32,
    pub status_effect: Option<BloonEffect>,
    pub bloon: Entity,
}