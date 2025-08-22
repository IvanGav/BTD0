use bevy::{math::vec2, prelude::*};

use crate::damage::projectile::SimpleProjectile;

pub struct Attack {
    pub range: f32,
    pub cooldown: i32, pub attack_rate: f32,
    pub projectile: SimpleProjectile, // needs to be able to hold any projectile type (TODO); maybe have a trait `projectile`?
}

pub struct Tower {
    pub center: Vec2,
    pub attacks: Vec<Attack>, // every attack triggers every tick
    pub cur_targeting_mode: usize,
    pub allowed_targeting_modes: Vec<TargetingMode>,
    pub effects: Vec<TowerEffect>,
}

pub enum TargetingMode {
    FirstBloon, StrongestBloon, LastBloon, CloseBloon,
    CloseTrack, FarTrack, SmartTrack,
    Always, InRange
}

pub enum TowerEffect {
    Range { duration: Option<i32>, strength: f32 }, 
    Damage { duration: Option<i32>, strength: i32 }, 
    AttackRate { duration: Option<i32>, strength: f32 }, 
    CamoDetection { duration: Option<i32> }
}