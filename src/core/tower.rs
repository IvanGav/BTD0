use bevy::prelude::*;

use crate::core::attack_fn::*;

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

// impl Tower {
//     pub fn named(name: &str)->Self {
//         return match name {
//             "dart000" => Tower {
//                 effects: vec![],
//                 attacks: vec![
//                     Attack::Single { range: 100., cooldown: 0, attack_rate: 64, target_angle: None, attack_fn: attack_dart000 }
//                 ],
//                 targeting_modes: vec![TargetingMode::FirstBloon,TargetingMode::StrongBloon,TargetingMode::LastBloon,TargetingMode::CloseBloon],
//                 cur_targeting_mode: 0,
//             },
//             _ => Tower {
//                 effects: vec![],
//                 attacks: vec![],
//                 targeting_modes: vec![TargetingMode::FirstBloon],
//                 cur_targeting_mode: 0,
//             },
//         };
//     }
//     pub fn get_targeting_mode(&self)->TargetingMode {
//         return self.targeting_modes[self.cur_targeting_mode];
//     }
// }

/*
    Systems
*/

// pub fn towers_tick(mut towers: Query<(&mut Tower, &Transform)>) {
//     for (mut tower, pos) in &mut towers {
//         let targeting_mode = tower.get_targeting_mode();
//         for mut attack in &mut tower.attacks {
//             match attack {
//                 Attack::Single { range, cooldown, attack_rate, target_angle, attack_fn } => {
//                     *cooldown -= 1;
//                     if *cooldown <= 0 {
//                         target_angle
//                     }
//                 },
//                 _ => {}
//             }
//         }
//     }
// }