use bevy::prelude::*;

use crate::core::{attack_fn::*, bloon::BloonModifier, upgradable::Upgradable};

#[derive(Clone, Copy)]
pub enum TowerType {
    //primary
    DartMonkey,BoomerangMonkey,TackShooter,BombShooter,IceMonkey,GlueMonkey,
    //military
    SniperMonkey,SubMonkey,BoatMonkey,AceMonkey,HeliMonkey,MortarMonkey,DartlingMonkey,
    //magic
    WizardMonkey,SuperMonkey,NinjaMonkey,AlchMonkey,DruidMonkey,MerMonkey,
    //support
    BananaFarm,SpikeShooter,Village,EngineerMonkey,WaterShooter,
    //other
    CaveMonkey,HeliMarineMonkey,
    //heroes
    Quincy,Gwendolin,StrikerJones,Obyn,Sauda,Adora,Maxwell,
    //subtowers
    AceSubtower,HeliSubtower,BoatPlaneSubtower,HeliSupportSubtower,PhoenixSubtower,SunAvatarSubtower,EngineerTurretSubtower,
}

#[derive(Clone, Copy)]
pub enum TargetingMode {
    FirstBloon, StrongBloon, LastBloon, CloseBloon,
    CloseRoad, FarRoad, SmartRoad,
    Always, InRange,
}

impl TargetingMode {
    pub fn vec_entity_modes()->Vec<Self> {
        return vec![TargetingMode::FirstBloon,TargetingMode::StrongBloon,TargetingMode::LastBloon,TargetingMode::CloseBloon];
    }
}

#[derive(Clone)]
pub enum TowerEffect {
    Range { duration: Option<i32>, strength: f32 }, 
    Damage { duration: Option<i32>, strength: i32 }, 
    AttackRate { duration: Option<i32>, strength: f32 }, 
    DetectionBuff { duration: Option<i32>, added: BloonModifier },
    DamageTypeBuff { duration: Option<i32>, added: BloonModifier },
}

#[derive(Clone)]
pub enum Attack {
    TargetEntity {
        range: f32,
        attack_rate: i32,
        target_angle: Option<f32>,
        target_entity: Option<Entity>,
        attack_fn: fn(&mut Commands, f32, Entity, &mut Vec<TowerEffect>, Vec3),
        attack_at: usize, // fixed update tick number at which to shoot next time
    },
    TargetRoad {
        range: f32,
        attack_rate: i32,
        target_waypoint: Option<Vec2>,
        attack_fn: fn(&mut Commands, Vec2, &mut Vec<TowerEffect>, Vec3),
        attack_at: usize, // fixed update tick number at which to shoot next time
    },
}

#[derive(Component)]
pub struct Tower {
    // prototype
    tower_type: TowerType,
    upgrades: Upgradable,
    attacks: Vec<Attack>, // although is technically modifiable (jerry fire)
    targeting_modes: Vec<TargetingMode>,
    // state
    cur_targeting_mode: usize,
    effects: Vec<TowerEffect>,
}

impl Tower {
    /// Dummy function because i didn't feel like doing default (it really shouldn't have a default, but I do need a dummy tower)
    fn zero()->Self {
        Tower { attacks: vec![], tower_type: TowerType::DartMonkey, upgrades: Upgradable::None, targeting_modes: vec![TargetingMode::FirstBloon], cur_targeting_mode: 0, effects: vec![] }
    }
    pub fn from(tower_type: TowerType, upgrades: Upgradable)->Self {
        match tower_type {
            TowerType::DartMonkey => {
                if let Upgradable::Crosspath520(p1, p2, p3) = upgrades {
                    // DART MONKEY
                    match (p1,p2,p3) {
                        (0,0,0) => return Tower { attacks: vec![ Attack::TargetEntity { range: 100.0, attack_rate: 60, attack_fn: attack_dart000, attack_at: 0, target_angle: None, target_entity: None } ], tower_type, upgrades, targeting_modes: TargetingMode::vec_entity_modes(), cur_targeting_mode: 0, effects: vec![] },
                        _ => {}
                    }
                }
            },
            _ => {},
        };
        return Tower::zero();
    }
    pub fn get_targeting_mode(&self)->TargetingMode {
        return self.targeting_modes[self.cur_targeting_mode];
    }
}

/*
    Systems
*/

pub fn towers_tick(mut towers: Query<(&mut Tower, &Transform, &GlobalTransform)>) {
    for (mut tower, pos, gpos) in &mut towers {
        let targeting_mode = tower.get_targeting_mode();
        for mut attack in &mut tower.attacks {
            gpos.translation();
            // match attack {
            //     Attack::Single { range, cooldown, attack_rate, target_angle, attack_fn } => {
            //         *cooldown -= 1;
            //         if *cooldown <= 0 {
            //             target_angle
            //         }
            //     },
            //     _ => {}
            // }
        }
    }
}