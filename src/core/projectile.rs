use bevy::{math::{ops::hypot, vec2}, prelude::*};

use crate::{core::{bloon::{Bloon, BloonID, BloonModifier}, event::BloonDamageEvent, hitbox::HitboxSimple}};

/*
    Lifetime
*/

/// A component which will decrement own lifetime every tick; despawn this entity once lifetime ran out
#[derive(Component, Clone)]
pub struct LifetimeTick {
    pub lifetime: i32,
}

/// A component which will decrement own lifetime every round; despawn this entity once lifetime ran out
#[derive(Component, Clone)]
pub struct LifetimeRound {
    pub lifetime_rounds: i32,
}

/*
    Damage dealing
*/

/// A component that allows an entity to deal damage to bloons
#[derive(Component, Clone)]
pub struct DamageDealer {
    pub damage: i32,
    pub pierce: u32,
    pub hit_bloons: Vec<BloonID>,
    pub cannot_pop_modifiers: BloonModifier,
    pub cannot_target_modifiers: BloonModifier,
}

impl DamageDealer {
    pub fn has_hit(&self, bloon: &BloonID)->bool {
        for i in &self.hit_bloons {
            if i.same_subtree_as(bloon) { return true; }
        }
        return false;
    }
}

/*
    Systems
*/

/// Test if projectiles collide with bloons. If yes, send a damage taken event.
pub fn damage_bloons(mut cmd: Commands, mut damage_ew: EventWriter<BloonDamageEvent>, bloons: Query<(Entity, &Bloon, &HitboxSimple, &Transform)>, mut p: Query<(Entity, &mut DamageDealer, &HitboxSimple, &Transform)>) {
    let mut damage_events = vec![];
    for (pe, mut p, phb, ppos) in &mut p {
        if p.pierce == 0 { continue; }
        for (be, bloon, bhb, bpos) in &bloons {
            let critical_dist = phb.radius + bhb.radius;
            // if AABB intersect, and actually intersect, and hasn't hit before
            if (bpos.translation.x - ppos.translation.x).abs() < critical_dist && (bpos.translation.y - ppos.translation.y).abs() < critical_dist &&
            hypot(ppos.translation.x - bpos.translation.x, ppos.translation.y - bpos.translation.y) < critical_dist &&
            !p.has_hit(&bloon.bid) {
                damage_events.push(BloonDamageEvent { damage: p.damage, status_effect: None, bloon: be });
                p.hit_bloons.push(bloon.bid.clone());
                p.pierce -= 1;
                if p.pierce == 0 { cmd.entity(pe).despawn(); break; }
            }
        }
    }
    damage_ew.send_batch(damage_events);
}

pub fn lifetime_tick(mut cmd: Commands, mut lifetimes: Query<(Entity, &mut LifetimeTick)>) {
    for (e, mut lifetime) in &mut lifetimes {
        lifetime.lifetime -= 1;
        if lifetime.lifetime <= 0 {
            cmd.entity(e).despawn();
        }
    }
}

/*
    Misc
*/

/// should not be here TODO
pub fn get_projectile_sprite()->Sprite {
    Sprite::from_color(Color::srgb(1.,0.,0.), vec2(10., 10.))
}