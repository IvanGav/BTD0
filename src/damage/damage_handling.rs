use bevy::{math::vec2, prelude::*};

use crate::bloon::bloon::*;

/*
    Events
*/

#[derive(Event)]
pub struct GlobalDamageEvent {
    pub damage: i32,
    pub status_effect: Option<BloonEffect>
}

/*
    Systems
*/

/// Apply queued damage to bloons.
pub fn apply_bloon_damage(mut cmd: Commands) {

}

/// Test if projectiles collide with bloons. If yes, send a damage taken event.
pub fn damage_bloons(mut cmd: Commands) {

}

/// A system that applies a global damage effect on all active bloons
pub fn global_damage_effects(mut bloons: Query<&mut Bloon>, mut global_damage_ev: EventReader<GlobalDamageEvent>) {
    for ev in global_damage_ev.read() {
        for mut bloon in &mut bloons {
            bloon.hp -= ev.damage;
            if let Some(effect) = &ev.status_effect {
                bloon.apply_effect(*effect);
            }
        }
    }
}