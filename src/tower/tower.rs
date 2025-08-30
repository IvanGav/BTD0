use bevy::{math::vec2, prelude::*};

use crate::{bloon::bloon::Bloon, damage::{damage_handling::{DamageDealer, DamageType}, projectile::Projectile}};

/*
    Stuff
*/

// #[derive(Clone)]
pub enum Attack {
    Single {
        range: f32,
        cooldown: i32,
        attack_rate: i32,
        projectile: Projectile,
        damage_dealer: DamageDealer,
        target_angle: Option<f32>, // the calculated angle where to shoot
    },
    Multiple {
        range: f32,
        cooldown: i32,
        attack_rate: i32,
        projectile: Projectile,
        damage_dealer: DamageDealer,
        spread: f32, // angle
        projectile_number: u8,
        target_angle: Option<f32>, // the calculated angle where to shoot
    }
}

#[derive(Component)]
pub struct Tower {
    pub attacks: Vec<Attack>, // every attack triggers every tick
    pub cur_targeting_mode: usize,
    pub allowed_targeting_modes: Vec<TargetingMode>,
    pub effects: Vec<TowerEffect>, // effects applied when added or removed
}

#[derive(Clone, Copy)]
pub enum TargetingMode {
    FirstBloon, StrongBloon, LastBloon, CloseBloon,
    CloseTrack, FarTrack, SmartTrack,
    Always, InRange
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

impl Tower {
    pub fn named(name: &str)->Self {
        match name {
            _ => Tower {
                allowed_targeting_modes: vec![TargetingMode::FirstBloon, TargetingMode::StrongBloon],
                cur_targeting_mode: 0,
                effects: vec![],
                attacks: vec![Attack::Single {
                    attack_rate: 10,
                    cooldown: 0,
                    range: 100.,
                    projectile: Projectile::Simple { velocity: vec2(10.,0.), lifetime: 40, collide: false },
                    damage_dealer: DamageDealer { damage: 1, pierce: 4, damage_type: DamageType::Sharp, hitbox_radius: 5., hit_bloons: vec![] },
                    target_angle: None
                }]
            }
        }
    }
}

/*
    Systems
*/

// TODO here
pub fn tower_attack(mut cmd: Commands, mut towers: Query<(&mut Tower, &Transform)>, bloons: Query<(&Bloon, &Transform)>) {
    for (mut tower, pos) in &mut towers {
        let targeting_mode = tower.allowed_targeting_modes[tower.cur_targeting_mode];
        for attack in &mut tower.attacks {
            match attack {
                Attack::Single {range, cooldown, attack_rate, projectile, damage_dealer, target_angle} => {
                    *cooldown -= 1;
                    if let Some(angle) = target_angle {
                        if *cooldown <= 0 {
                            *cooldown = *attack_rate;
                            // perform the attack
                            cmd.spawn((
                                projectile.clone_with_angle(*angle),
                                damage_dealer.clone()
                            ));
                        }
                    }
                },
                _ => {}
            };
        }
    }
}

/*
    Helper functions
*/

/// Only if sees the center of the hitbox
fn find_attack_angle(targeting_mode: TargetingMode, range: f32, tower_pos: &Transform, bloons: &Query<(&Bloon, &Transform)>)->f32 {
    vec2(0.,1.,).angle_to(vec2(1.,0.));
    return 0.;
}