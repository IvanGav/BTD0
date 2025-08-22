use bevy::{math::vec2, prelude::*};

use crate::bloon::bloon::*;
use crate::damage::damage_handling::*;

/*
    Components
*/

#[derive(Component, Clone)]
#[require(DamageDealer)]
pub struct SimpleProjectile {
    pub vx: f32,
    pub vy: f32,
    pub bounce: i32, // number of allowed bounces
    pub collide: bool, // if false, don't collide with obstacles
    pub lifetime: i32,
}

#[derive(Component)]
#[require(DamageDealer)]
pub struct HomingProjectile {
    pub vx: f32,
    pub vy: f32,
    pub bounce: i32, // number of allowed bounces
    pub collide: bool, // if false, don't collide with obstacles
    pub lifetime: i32,
}

#[derive(Component)]
#[require(DamageDealer)]
pub struct StaticProjectile {
    pub waypoint: Vec2,
    pub lifetime: u32,
}

/*
    Non-components
*/

/*
    Systems
*/

pub fn move_simple_projectile(mut p: Query<(&mut SimpleProjectile, &mut Transform)>) {
    for (mut p, mut pos) in &mut p {
        // TODO: collision with obstacles (after I got obstacles in the first place)
        pos.translation.x += p.vx;
        pos.translation.y += p.vy;
        p.lifetime -= 1;
    }
}

pub fn despawn_simple_projectile(mut cmd: Commands, p: Query<(Entity, &SimpleProjectile, &Transform)>) {
    for (e, p, pos) in &p {
        if p.lifetime <= 0 || (pos.translation.x > 500. || pos.translation.x < -500. || pos.translation.y > 500. || pos.translation.y < -500.) {
            cmd.entity(e).despawn();
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