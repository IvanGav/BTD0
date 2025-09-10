use std::sync::Mutex;

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
/// Ok this parallel shit rocks. Like, it went from turning my game into a slideshow to tanking to stable 20 fps... with 2x bloons on screen. Crazy how much difference parallel makes.
pub fn damage_bloons(cmd: ParallelCommands, damage_ew: EventWriter<BloonDamageEvent>, bloons: Query<(Entity, &Bloon, &HitboxSimple, &Transform)>, mut p: Query<(Entity, &mut DamageDealer, &HitboxSimple, &Transform)>) {
    let mutex = Mutex::from(damage_ew);
    p.par_iter_mut().for_each(|(pe, mut p, phb, ppos)| {
        let mut damage_events = vec![];
        for (be, bloon, bhb, bpos) in &bloons {
            let critical_dist = phb.radius + bhb.radius;
            // if AABB intersect, and actually intersect, and hasn't hit before
            if (bpos.translation.x - ppos.translation.x).abs() < critical_dist && (bpos.translation.y - ppos.translation.y).abs() < critical_dist &&
            hypot(ppos.translation.x - bpos.translation.x, ppos.translation.y - bpos.translation.y) < critical_dist &&
            !p.has_hit(&bloon.bid) {
                // damage the bloon
                damage_events.push(BloonDamageEvent { damage: p.damage, status_effect: None, bloon: be });
                p.hit_bloons.push(bloon.bid.clone());
                p.pierce -= 1;
                if p.pierce == 0 { cmd.command_scope(|mut cmd| { cmd.entity(pe).despawn(); }); break; }
            }
        }
        // this is fine, since it's a lock at the end, only once; although the `if` seemingly made it worse for a little while??? and then just not??
        if damage_events.len() > 0 {
            let mut val = mutex.lock().unwrap();
            val.send_batch(damage_events);
        }
    });
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