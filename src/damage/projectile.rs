use bevy::{math::vec2, prelude::*};

use crate::bloon::bloon::*;
use crate::damage::damage_handling::*;

/*
    Components
*/

#[derive(Component, Clone)]
#[require(DamageDealer)]
pub enum Projectile {
    Simple { velocity: Vec2, lifetime: i32, collide: bool },
    // Bouncing { velocity: Vec2, lifetime: i32, bounce: i32 },
    Static { waypoint: Option<Vec2>, lifetime: i32 },
}

/*
    Impl
*/

/*
    Non-components
*/

/*
    Systems
*/

pub fn move_projectile(mut p: Query<(&mut Projectile, &mut Transform)>) {
    for (mut p, mut pos) in &mut p {
        match &mut *p {
            Projectile::Simple {velocity, lifetime, collide} => {
                // TODO: collision with obstacles (after I got obstacles in the first place)
                pos.translation.x += velocity.x;
                pos.translation.y += velocity.y;
                *lifetime -= 1;
            },
            Projectile::Static {waypoint, lifetime} => {
                // TODO: do static projectile logic
                *lifetime -= 1;
            }
        }
    }
}

pub fn despawn_projectile(mut cmd: Commands, p: Query<(Entity, &Projectile, &Transform)>) {
    for (e, p, pos) in &p {
        match &*p {
            Projectile::Simple {velocity: _, lifetime, collide: _} => {
                if *lifetime <= 0 || (pos.translation.x > 500. || pos.translation.x < -500. || pos.translation.y > 500. || pos.translation.y < -500.) {
                    cmd.entity(e).despawn();
                }
            },
            Projectile::Static {waypoint: _, lifetime} => {
                if *lifetime <= 0 || (pos.translation.x > 500. || pos.translation.x < -500. || pos.translation.y > 500. || pos.translation.y < -500.) {
                    cmd.entity(e).despawn();
                }
            }
        }
    }
}

// /// Despawn projectiles which have moved out of bounds - (-500,-500) to (500,500)
// pub fn despawn_stray_projectile(mut cmd: Commands, p: Query<(Entity, ), With<DamageDealer>>) {
//     for (e, p) in &p {
//         if p.lifetime <= 0 || (p.translation.x > 500. || p.translation.x < -500. || p.translation.y > 500. || p.translation.y < -500.) {
//             cmd.entity(e).despawn();
//         }
//     }
// }

/*
    Hepler functions
*/

pub fn get_projectile_sprite()->Sprite {
    Sprite::from_color(Color::srgb(1.,0.,0.), vec2(10., 10.))
}