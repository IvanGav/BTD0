use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum TargetingMode {
    FirstBloon, StrongBloon, LastBloon, CloseBloon,
    CloseRoad, FarRoad, SmartRoad,
    Always, InRange,
}

#[derive(Clone)]
pub enum TowerEffect {
    Range { duration: Option<i32>, strength: f32 }, 
    Damage { duration: Option<i32>, strength: i32 }, 
    AttackRate { duration: Option<i32>, strength: f32 }, 
    CamoDetection { duration: Option<i32> }
}

#[derive(Clone)]
pub enum Attack {
    Single {
        range: f32,
        cooldown: i32,
        attack_rate: i32,
        target_angle: Option<f32>, // the calculated angle where to shoot
        attack_fn: fn(&mut Commands, f32, Vec<TowerEffect>),
    },
    Multiple {
        range: f32,
        cooldown: i32,
        attack_rate: i32,
        spread: f32, // angle
        projectile_number: u8,
        target_angle: Option<f32>, // the calculated angle where to shoot
        attack_fn: fn(&mut Commands, f32, Vec<TowerEffect>),
    },
    TargetEntity {
        range: f32,
        cooldown: i32,
        attack_rate: i32,
        target_angle: Option<Entity>, // the calculated entity which to shoot
        attack_fn: fn(&mut Commands, Entity, Vec<TowerEffect>),
    }
}

#[derive(Component)]
pub struct Tower {
    attacks: Vec<Attack>,
    effects: Vec<TowerEffect>,
    targeting_modes: Vec<TargetingMode>,
    cur_targeting_mode: usize,
}