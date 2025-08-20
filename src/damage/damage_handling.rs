use std::cmp::min;

use bevy::{math::{ops::hypot, vec2}, prelude::*, utils::HashSet};

use crate::bloon::bloon::*;

/*
    Components
*/

#[derive(Component, Default)]
/// A struct that allows an entity to deal damage to bloons
pub struct DamageDealer {
    pub damage: i32,
    pub pierce: u32,
    pub damage_type: DamageType,
    pub hitbox_radius: f32, // all damage dealers have a circular hitbox, for now
    pub hit_bloons: Vec<BloonHitComparator>,
}

/*
    Non-components
*/

#[derive(PartialEq, Clone, Copy, Default)]
pub enum DamageType {
    #[default]
    Normal, // can damage all
    Shatter, // cannot damage SharpImmune (lead)
    Explosion, // cannot damage ExplosionImmune (black)
    Frigid, // cannot damage ColdImmune (white)
    Magic, // cannot damage MagicImmune (purple)
    Energy, // cannot damage SharpImmune (lead) and MagicImmune (purple)
    Sharp, // cannot damage SharpImmune (lead) and Frozen (frozen by monkeys)
    Cold, // cannot damage SharpImmune (lead) and ColdImmune (white)
}

/// Purely used to test for bloon hits
pub struct BloonHitComparator {
    pub family: u32,
    pub layer: u8,
    pub tree: u32,
}

/*
    Impl
*/

impl DamageDealer {
    pub fn has_hit(&self, bloon: &BloonHitComparator)->bool {
        for i in &self.hit_bloons {
            if i.same_subtree_as(bloon) { return true; }
        }
        return false;
    }
}

impl BloonHitComparator {
    pub fn same_subtree_as(&self, other: &Self) -> bool {
        if self.family != other.family { return false; }
        let min_layer = min(self.layer, other.layer);
        let mask = create_first_n_bits_mask(min_layer);
        // if self.tree != other.tree {
        //     println!("{} and {} are {} (min layer {})", self.tree, other.tree, (self.tree & mask) == (other.tree & mask), min_layer);
        // }
        return (self.tree & mask) == (other.tree & mask);
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

/*
    Systems
*/

/// Apply queued damage to bloons.
pub fn apply_bloon_damage(mut damage_er: EventReader<BloonDamageEvent>, mut bloons: Query<&mut Bloon>) {
    for ev in damage_er.read() {
        if let Ok(mut bloon) = bloons.get_mut(ev.bloon) {
            bloon.hp -= ev.damage;
            if let Some(effect) = &ev.status_effect {
                bloon.apply_effect(*effect);
            }
        }
    }
}

/// Test if projectiles collide with bloons. If yes, send a damage taken event.
// pub fn damage_bloons(mut cmd: Commands, mut damage_ew: EventWriter<BloonDamageEvent>, bloons: Query<(Entity, &Bloon, &Transform)>, mut p: Query<(Entity, &mut DamageDealer, &Transform)>) {
//     let mut damage_events = vec![];
//     for (pe, mut p, ppos) in &mut p {
//         if p.pierce == 0 { continue; }
//         for (be, bloon, bpos) in &bloons {
//             if p.has_hit(&BloonHitComparator {family: bloon.family_id, layer: bloon.child_layer, tree: bloon.child_tree}) { continue; }
//             // and here just check for actual collision
//             if hypot(ppos.translation.x - bpos.translation.x, ppos.translation.y - bpos.translation.y) < bloon.bloon_tier.get_base_hitbox_radius() + p.hitbox_radius {
//                 damage_events.push(BloonDamageEvent { damage: p.damage, status_effect: None, bloon: be });
//                 p.hit_bloons.push(BloonHitComparator { family: bloon.family_id, layer: bloon.child_layer, tree: bloon.child_tree });
//                 p.pierce -= 1;
//                 if p.pierce == 0 { cmd.entity(pe).despawn(); break; }
//             }
//         }
//     }
//     damage_ew.send_batch(damage_events);
// }

/// Test if projectiles collide with bloons. If yes, send a damage taken event.
pub fn damage_bloons(mut cmd: Commands, mut bloons: Query<(&mut Bloon, &Transform)>, mut p: Query<(Entity, &mut DamageDealer, &Transform)>) {
    for (pe, mut p, ppos) in &mut p {
        if p.pierce == 0 { continue; }
        for (mut bloon, bpos) in &mut bloons {
            if p.has_hit(&BloonHitComparator {family: bloon.family_id, layer: bloon.child_layer, tree: bloon.child_tree}) { continue; }
            if hypot(ppos.translation.x - bpos.translation.x, ppos.translation.y - bpos.translation.y) < bloon.bloon_tier.get_base_hitbox_radius() + p.hitbox_radius {
                bloon.hp -= p.damage;
                p.hit_bloons.push(BloonHitComparator { family: bloon.family_id, layer: bloon.child_layer, tree: bloon.child_tree });
                p.pierce -= 1;
                if p.pierce == 0 { cmd.entity(pe).despawn(); break; }
            }
        }
    }
}

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

/*
    Helper functions
*/

/// Create a bit mask
/// Just don't give n > 32
fn create_first_n_bits_mask(n: u8) -> u32 {
    (1u32 << n) - 1
}