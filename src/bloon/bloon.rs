use bevy::{math::vec2, prelude::*, utils::HashSet};

use crate::map::map::*;

/*
    Components
*/

#[derive(Component, Default)]
#[require(RoadEntity, Transform)]
/// A component that indicates that an entity is a bloon
pub struct Bloon {
    pub bloon_tier: BloonTier,
    pub hp: i32,
    pub bloon_modifiers: Vec<BloonModifier>,
    pub status_effects: Vec<BloonEffect>,
    pub parents: HashSet<Entity>,
    pub id: u32, // TODO: replace all Entity things with this id, because of a bug that idk how to fix otherwise, described in README
}

#[derive(Component, Default, Clone)]
/// A component that lets an entity to occupy a position on a road and move along the road
/// Do not apply to road items, as they don't need to move along the road
pub struct RoadEntity {
    pub target_node: usize, // next targeted node
    pub track_pos: f32, // position on the track
    pub waypoint: Vec2, // may or may not be target node's position; after reaching, incrememnt `target_node`
}

#[derive(PartialEq, Default, Clone, Copy, Debug)]
/// The bloon tier determines base stats - speed, hp, etc
pub enum BloonTier {
    #[default]
    Red, Blue, Green, Yellow, Pink, Purple, Black, White, Zebra, Lead, Rainbow, Ceramic, MOAB, BFB, ZOMG, DDT, BAD
}

#[derive(PartialEq, Clone, Copy)]
/// Every bloon tier has an associated type that determines interactions with damage dealers
pub enum BloonType {
    Bloon, Blimp, Boss
}

/*
    Components (impl)
*/

impl Bloon {
    pub fn with(tier: BloonTier, mut modifiers: Vec<BloonModifier>)->Bloon {
        match tier {
            BloonTier::Purple => modifiers.push(BloonModifier::MagicImmune),
            BloonTier::Black => modifiers.push(BloonModifier::ExplosionImmune),
            BloonTier::White => modifiers.push(BloonModifier::ColdImmune),
            BloonTier::Zebra => {
                modifiers.push(BloonModifier::ExplosionImmune);
                modifiers.push(BloonModifier::ColdImmune);
            },
            BloonTier::Lead => modifiers.push(BloonModifier::SharpImmune),
            BloonTier::DDT => {
                modifiers.push(BloonModifier::SharpImmune);
                modifiers.push(BloonModifier::Camo);
            },
            _ => ()
        };
        let hp_mult = if modifiers.contains(&BloonModifier::Fortified) { tier.get_fortified_hp_mult() } else { 1 };
        return Bloon {hp: tier.get_base_hp() * hp_mult, bloon_tier: tier, bloon_modifiers: modifiers, ..Default::default()};
    }
    pub fn apply_effect(&mut self, effect: BloonEffect) {
        self.status_effects.push(effect);
    }
    pub fn get_child_bloons(&self)->Vec<Bloon> {
        let base_children = self.bloon_tier.get_base_child_bloons();
        let mut actual_children = vec![];
        for ch in base_children {
            // TODO: Right now fortified will propagate through the red bloon; should drop at ceram level
            actual_children.push(Bloon::with(ch, self.bloon_modifiers.to_vec()));
        }
        return actual_children;
    }
}

impl BloonTier {
    pub fn get_type(&self)->BloonType {
        match self {
            BloonTier::MOAB | BloonTier::BFB | BloonTier::ZOMG | BloonTier::DDT | BloonTier::BAD => BloonType::Blimp,
            _ => BloonType::Bloon
        }
    }
    pub fn get_base_speed(&self)->f32 {
        return match self {
            BloonTier::Red => 25., 
            BloonTier::Blue => 35.,
            BloonTier::Green => 45.,
            BloonTier::Yellow => 80.,
            BloonTier::Pink => 87.5,
            BloonTier::Purple => 75.,
            BloonTier::Black => 45.,
            BloonTier::White => 50.,
            BloonTier::Zebra => 45.,
            BloonTier::Lead => 25.,
            BloonTier::Rainbow => 55.,
            BloonTier::Ceramic => 62.5,
            BloonTier::MOAB => 25.,
            BloonTier::BFB => 6.25,
            BloonTier::ZOMG => 4.5,
            BloonTier::DDT => 66.,
            BloonTier::BAD => 4.5,
        } / 35.;
    }
    pub fn get_base_hp(&self)->i32 {
        return match self {
            BloonTier::Red => 1,
            BloonTier::Blue => 1,
            BloonTier::Green => 1,
            BloonTier::Yellow => 1,
            BloonTier::Pink => 1,
            BloonTier::Purple => 1,
            BloonTier::Black => 1,
            BloonTier::White => 1,
            BloonTier::Zebra => 1,
            BloonTier::Lead => 1,
            BloonTier::Rainbow => 1,
            BloonTier::Ceramic => 10,
            BloonTier::MOAB => 200,
            BloonTier::BFB => 700,
            BloonTier::ZOMG => 4000,
            BloonTier::DDT => 400,
            BloonTier::BAD => 20000,
        };
    }
    pub fn get_base_child_bloons(&self)->Vec<BloonTier> {
        match self {
            BloonTier::Red => vec![],
            BloonTier::Blue => vec![BloonTier::Red],
            BloonTier::Green => vec![BloonTier::Blue],
            BloonTier::Yellow => vec![BloonTier::Green],
            BloonTier::Pink => vec![BloonTier::Yellow],
            BloonTier::Purple => vec![BloonTier::Pink, BloonTier::Pink],
            BloonTier::Black => vec![BloonTier::Pink, BloonTier::Pink],
            BloonTier::White => vec![BloonTier::Pink, BloonTier::Pink],
            BloonTier::Zebra => vec![BloonTier::Black, BloonTier::White],
            BloonTier::Lead => vec![BloonTier::Black, BloonTier::Black],
            BloonTier::Rainbow => vec![BloonTier::Zebra, BloonTier::Zebra],
            BloonTier::Ceramic => vec![BloonTier::Rainbow, BloonTier::Rainbow],
            BloonTier::MOAB => vec![BloonTier::Ceramic, BloonTier::Ceramic, BloonTier::Ceramic, BloonTier::Ceramic],
            BloonTier::BFB => vec![BloonTier::MOAB, BloonTier::MOAB, BloonTier::MOAB, BloonTier::MOAB],
            BloonTier::ZOMG => vec![BloonTier::BFB, BloonTier::BFB, BloonTier::BFB, BloonTier::BFB],
            BloonTier::DDT => vec![BloonTier::Ceramic, BloonTier::Ceramic, BloonTier::Ceramic, BloonTier::Ceramic],
            BloonTier::BAD => vec![BloonTier::ZOMG, BloonTier::ZOMG, BloonTier::DDT, BloonTier::DDT, BloonTier::DDT],
        }
    }
    pub fn get_fortified_hp_mult(&self)->i32 {
        return match self {
            BloonTier::Lead => 4,
            _ => 2,
        }
    }
    pub fn get_base_hitbox_radius(&self)->f32 {
        return match self {
            BloonTier::Red => 25.,
            BloonTier::Blue => 25.,
            BloonTier::Green => 25.,
            BloonTier::Yellow => 25.,
            BloonTier::Pink => 25.,
            BloonTier::Purple => 25.,
            BloonTier::Black => 25.,
            BloonTier::White => 25.,
            BloonTier::Zebra => 25.,
            BloonTier::Lead => 25.,
            BloonTier::Rainbow => 25.,
            BloonTier::Ceramic => 25.,
            BloonTier::MOAB => 50.,
            BloonTier::BFB => 75.,
            BloonTier::ZOMG => 100.,
            BloonTier::DDT => 50.,
            BloonTier::BAD => 150.,
        };
    }
}

/*
    Non-components
*/

#[derive(PartialEq, Clone, Copy)]
/// Effects that bloons can have. Duration in game ticks. No duration indicates an instant effect, such as de-fortify.
pub enum BloonEffect {
    Weakness {strength: i32, duration: i32},
    Speed {strength: f32, duration: i32}, // also serves as slow and stun
    BonusIncome {strength: i32, duration: i32},
}

#[derive(PartialEq, Clone, Copy)]
pub enum BloonModifier {
    Fortified, Camo, Regrow, Frozen, SharpImmune, MagicImmune, ColdImmune, ExplosionImmune,
}


/*
    Hepler functions
*/

/// Create a bloon at the beginning of the given track
pub fn create_bloon(tier: BloonTier, map: &Map)->(Bloon, Sprite, RoadEntity, Transform) {
    return (
        Bloon::with(tier, vec![]),
        get_bloon_sprite(tier),
        RoadEntity { target_node: 0, track_pos: 0., waypoint: map.start_pos()},
        Transform::from_xyz(map.start_pos().x, map.start_pos().y, 1.),
    );
}

/// Move a given RoadEntity along the road with the given step size (that should depend on its speed)
pub fn advance_road_entity(step: f32, map: &Map, re: &mut RoadEntity, pos: &mut Transform) {
    let dx = re.waypoint.x - pos.translation.x; // x difference between a waypoint and a current position
    let dy = re.waypoint.y - pos.translation.y; // y difference between a waypoint and a current position
    let total_dist = (dx*dx + dy*dy).sqrt();

    if total_dist < step {
        // Move to the node and advance the node index
        // TODO: Should overflow to movement along the next node
        pos.translation.x = re.waypoint.x;
        pos.translation.y = re.waypoint.y;
        re.target_node += 1;
        if re.target_node < map.path.len() {
            re.waypoint = map.path[re.target_node];
            re.track_pos = map.cumulative_dist[re.target_node];
        } else {
            // maybe do something else; essentially make it do something for a tick until it's despawned
            re.waypoint = vec2(0.,0.);
            re.track_pos = 0.;
        }
    } else {
        pos.translation.x += dx * step / total_dist;
        pos.translation.y += dy * step / total_dist;
        re.track_pos += step;
    }
}

// TODO: move to graphics and later replace entirely
pub fn get_bloon_sprite(tier: BloonTier)->Sprite {
    return match tier {
        BloonTier::Red => Sprite::from_color(Color::Srgba(Srgba { red: 1., green: 0., blue: 0., alpha: 1. }), vec2(50., 50.)),
        BloonTier::Blue => Sprite::from_color(Color::Srgba(Srgba { red: 0., green: 0., blue: 1., alpha: 1. }), vec2(50., 50.)),
        BloonTier::Green => Sprite::from_color(Color::Srgba(Srgba { red: 0., green: 1., blue: 0., alpha: 1. }), vec2(50., 50.)),
        BloonTier::Yellow => Sprite::from_color(Color::Srgba(Srgba { red: 1., green: 1., blue: 0., alpha: 1. }), vec2(50., 50.)),
        BloonTier::Pink => Sprite::from_color(Color::Srgba(Srgba { red: 1., green: 0.5, blue: 0.5, alpha: 1. }), vec2(50., 50.)),
        BloonTier::Purple => Sprite::from_color(Color::Srgba(Srgba { red: 1., green: 0., blue: 1., alpha: 1. }), vec2(50., 50.)),
        BloonTier::Black => Sprite::from_color(Color::Srgba(Srgba { red: 0., green: 0., blue: 0., alpha: 1. }), vec2(25., 25.)),
        BloonTier::White => Sprite::from_color(Color::Srgba(Srgba { red: 1., green: 1., blue: 1., alpha: 1. }), vec2(25., 25.)),
        BloonTier::Zebra => Sprite::from_color(Color::Srgba(Srgba { red: 0.7, green: 0.7, blue: 0.7, alpha: 1. }), vec2(50., 50.)),
        BloonTier::Lead => Sprite::from_color(Color::Srgba(Srgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 1. }), vec2(50., 50.)),
        BloonTier::Rainbow => Sprite::from_color(Color::Srgba(Srgba { red: 0.2, green: 0.8, blue: 0.2, alpha: 1. }), vec2(50., 50.)),
        BloonTier::Ceramic => Sprite::from_color(Color::Srgba(Srgba { red: 0.59, green: 0.29, blue: 0.0, alpha: 1. }), vec2(50., 50.)),
        BloonTier::MOAB => Sprite::from_color(Color::Srgba(Srgba { red: 0., green: 0., blue: 0.8, alpha: 1. }), vec2(100., 100.)),
        BloonTier::BFB => Sprite::from_color(Color::Srgba(Srgba { red: 0.8, green: 0., blue: 0., alpha: 1. }), vec2(120., 120.)),
        BloonTier::ZOMG => Sprite::from_color(Color::Srgba(Srgba { red: 0., green: 0.7, blue: 0., alpha: 1. }), vec2(150., 150.)),
        BloonTier::DDT => Sprite::from_color(Color::Srgba(Srgba { red: 0.1, green: 0.1, blue: 0.1, alpha: 1. }), vec2(120., 120.)),
        BloonTier::BAD => Sprite::from_color(Color::Srgba(Srgba { red: 0.9, green: 0.3, blue: 0.4, alpha: 1. }), vec2(200., 200.)),
    };
}

/*
    Systems
*/

/// Move bloons along the track
pub fn move_bloons(map: Res<Map>, mut bloons: Query<(&Bloon, &mut RoadEntity, &mut Transform)>) {
    for (bloon, re, pos) in &mut bloons {
        advance_road_entity(bloon.bloon_tier.get_base_speed(), &*map, re.into_inner(), pos.into_inner());
    }
}

/// Check if bloons are dead. If yes, spawn children or despawn. Should happen only after the bloons have moved this turn.
/// Big and ugly, sorry, can't do much about that.
pub fn pop_bloons(mut cmd: Commands, map: Res<Map>, bloons: Query<(Entity, &Bloon, &RoadEntity, &Transform)>) {
    let mut new_bloons: Vec<(Bloon, RoadEntity, Transform, Sprite)> = vec![];
    for (e, bloon, re, pos) in &bloons {
        if bloon.hp > 0 { continue; }
        // Decide whether layer skip is necessary or not
        let child_bloons = if bloon.hp == 0 { bloon.get_child_bloons() } else { calculate_overkill(bloon) };
        match child_bloons.len() {
            0 => { cmd.entity(e).despawn(); },
            1 => {
                // let mut child = child_bloons.remove(0); // requires to make `child_bloons` mut and mut is stinky
                let child = child_bloons.into_iter().next().unwrap();
                let child_sprite = get_bloon_sprite(child.bloon_tier);
                cmd.entity(e).insert((child,child_sprite));
            },
            _ => {
                let mut i = 0;
                for mut child in child_bloons {
                    let child_sprite = get_bloon_sprite(child.bloon_tier);
                    if i == 0 { 
                        // replace self; no need to spawn an extra bloon
                        cmd.entity(e).insert((child,child_sprite));
                    } else {
                        child.parents.insert(e);
                        let mut child_re = re.clone();
                        let mut child_transform = pos.clone();
                        advance_road_entity(25.0 * i as f32, &*map, &mut child_re, &mut child_transform);
                        new_bloons.push((
                            child,
                            child_re,
                            child_transform,
                            child_sprite,
                        ));
                    }
                    i += 1;
                }
            },
        }
    }
    cmd.spawn_batch(new_bloons);
}

/// Despawn bloons which have exited the map (gone past the last node of the map)
pub fn despawn_exited_bloons(mut cmd: Commands, map: Res<Map>, bloons: Query<(Entity, &RoadEntity)>) {
    for (e, re) in &bloons {
        if re.target_node == map.path.len() {
            cmd.entity(e).despawn();
        }
    }
}

/*
    Helper functions
*/

/// Assume bloon.hp to be 0 or <0. Return all children that it would spawn, taking overkill (layer skip) into account
pub fn calculate_overkill(bloon: &Bloon)->Vec<Bloon> {
    let mut children = bloon.get_child_bloons();
    if bloon.hp < 0 {
        let mut new_children = vec![];
        for mut ch in children {
            ch.hp += bloon.hp;
            new_children.append(&mut calculate_overkill(&ch));
        }
        children = new_children;
    }
    return children;
}