use bevy::{math::vec2, prelude::*};

use crate::damage::{damage_handling::DamageDealer, projectile::SimpleProjectile};

/*
    Stuff
*/

// #[derive(Clone)]
pub struct Attack {
    pub range: f32,
    pub cooldown: i32, pub attack_rate: f32,
    pub projectile: Box<dyn Fn(Vec2)->(SimpleProjectile, DamageDealer) + Send + Sync>, // needs to be able to hold any projectile type (TODO); maybe have a trait `projectile`?
}

#[derive(Clone, Copy)]
pub enum TargetingMode {
    FirstBloon, StrongestBloon, LastBloon, CloseBloon,
    CloseTrack, FarTrack, SmartTrack,
    Always, InRange
}

#[derive(Component)]
pub struct Tower {
    pub attacks: Vec<Attack>, // every attack triggers every tick
    pub cur_targeting_mode: usize,
    pub allowed_targeting_modes: Vec<TargetingMode>,
    pub effects: Vec<TowerEffect>,
}

#[derive(Clone, Copy)]
pub enum TowerEffect {
    Range { duration: Option<i32>, strength: f32 }, 
    Damage { duration: Option<i32>, strength: i32 }, 
    AttackRate { duration: Option<i32>, strength: f32 }, 
    CamoDetection { duration: Option<i32> }
}

/*
    Impl
*/

// impl Tower {
//     pub fn named(name: &str)->Self {
//         match name {
//             "dart" => Tower {
//                 allowed_targeting_modes: vec![TargetingMode::FirstBloon],
//                 cur_targeting_mode: 0,
//                 effects: vec![],
//                 attacks: vec![Attack {
//                     attack_rate: 10.,
//                     cooldown: 0,
//                     range: 100.,
//                     projectile: SimpleProjectile {
//                         ..default(),
//                     }
//                 }]
//             },
//             _ => Tower {
//                 allowed_targeting_modes: vec![TargetingMode::FirstBloon],
//                 cur_targeting_mode: 0,
//                 effects: vec![],
//                 attacks: vec![Attack {
//                     attack_rate: 10.,
//                     cooldown: 0,
//                     range: 100.,
//                     projectile: SimpleProjectile {
//                         ..Default::default(),
//                     }
//                 }]
//             }
//         }
//     }
// }

/*
    Systems
*/

