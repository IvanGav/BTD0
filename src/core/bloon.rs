use bevy::{math::{ops::log2, vec2}, prelude::*, utils::HashMap};
use rand::RngCore;
use std::cmp::min;

use crate::core::{hitbox::HitboxSimple, map::Map, movement::{advance_move_along_road, MoveAlongRoad}};

/*
    Helper Functions
*/

/// Create a bit mask
/// Just don't give n > 32
fn first_n_bits_mask(n: u8) -> BIDTree {
    ((1 as BIDTree) << n) - 1
}

/// Create a bloon at the beginning of the given track
pub fn create_bloon(tier: BloonTier, map: &Map)->(Bloon, Sprite, MoveAlongRoad, Transform, HitboxSimple) {
    return (
        Bloon::with(tier, BloonModifier::default()),
        get_bloon_sprite(tier),
        MoveAlongRoad { target_node: 0, road_pos: 0., waypoint: map.start_pos(), velocity: tier.get_base_speed() },
        Transform::from_xyz(map.start_pos().x, map.start_pos().y, 1.),
        HitboxSimple { radius: tier.get_base_hitbox_radius() },
    );
}

pub fn bloon_entity_with(tier: BloonTier, modifiers: BloonModifier)->(Bloon, MoveAlongRoad, HitboxSimple, Option<BloonPropertyRegrow>) {
    todo!()
}

/*
    Overkill system
*/

#[derive(Resource, Deref, DerefMut, Default)]
pub struct OverkillLookupTable(pub HashMap<(BloonTier, i32),Vec<BloonTier>>);

/// A system that generates an overkill table; it's very jank, redo later (TODO)
pub fn generate_lookup_overkill_bloon(mut map: ResMut<OverkillLookupTable>) {
    // this is a terrible function and I don't care to make it good - assume that all bloons have 1 hp
    // well, except bloons listed in `starters`, they are already overkilled, so I don't care if they have more than 1 base hp
    let starters = vec![
        BloonTier::Ceramic, BloonTier::Purple, BloonTier::Lead,
        BloonTier::Rainbow, BloonTier::Zebra, BloonTier::Black, BloonTier::White,
        BloonTier::Pink, BloonTier::Yellow, BloonTier::Green, BloonTier::Blue, BloonTier::Red
    ];
    for tier in &starters {
        let mut cur_ch = tier.get_base_child_bloons();
        let mut i = 0;
        while cur_ch.len() > 0 {
            map.0.insert((*tier, i), cur_ch.to_vec());
            let mut temp = vec![];
            cur_ch.iter().for_each(|ch| temp.append(&mut ch.get_base_child_bloons()));
            cur_ch = temp;
            i -= 1;
        }
    }
}

/// Assume bloon.hp to be 0 or <0. Return all children that it would spawn, taking overkill (layer skip) into account
pub fn calculate_overkill(bloon: &Bloon, overkill_bloon_map: &HashMap<(BloonTier, i32),Vec<BloonTier>>)->Vec<Bloon> {
    // if bloon, search a lookup table
    if bloon.tier.get_type() == BloonType::Bloon { return lookup_overkill_bloon(bloon, overkill_bloon_map); }
    // blimp
    let mut children = bloon.get_child_bloons();
    if bloon.hp < 0 {
        let mut new_children = vec![];
        for mut ch in children {
            ch.hp += bloon.hp;
            new_children.append(&mut calculate_overkill(&ch, overkill_bloon_map));
        }
        children = new_children;
    }
    // children.sort();
    return children;
}

pub fn lookup_overkill_bloon(bloon: &Bloon, overkill_bloon_map: &HashMap<(BloonTier, i32),Vec<BloonTier>>)->Vec<Bloon> {
    if let Some(children) = overkill_bloon_map.get(&(bloon.tier, bloon.hp)) {
        let mut out = vec![];
        for (i, child) in children.iter().enumerate() {
            out.push(Bloon::with(*child, bloon.modifiers).child_of(bloon, i, children.len()));
        }
        return out;
    }
    return vec![];
}

/*
    Bloon ID
*/

type BIDFamily = u32;
type BIDTree = u32;

/// Uniquily identify bloons
#[derive(Clone)]
pub struct BloonID {
    pub family: BIDFamily,
    pub layer: u8,
    pub tree: BIDTree,
}

impl BloonID {
    /// Return true iff self is parent or a child of `other`. That means, if self and `other` are in the same subtree.
    pub fn same_subtree_as(&self, other: &Self) -> bool {
        if self.family != other.family { return false; }
        let min_layer = min(self.layer, other.layer);
        let mask = first_n_bits_mask(min_layer);
        return (self.tree & mask) == (other.tree & mask);
    }
    pub fn new()->Self {
        return Self { family: rand::rng().next_u32(), layer: 0, tree: 0 };
    }
}

/*
    Bloon data
*/

/// Basically damage types and immunities to damage types
pub type BloonModifier = u16;

enum BloonModifierData {
    Lead = 0b1,
    Purple = 0b10,
    Black = 0b100,
    White = 0b1000,
    Frozen = 0b10000,
    Camo = 0b100000,
    Fortified = 0b1000000,
}

pub enum DamageType {
    Normal = 0, // can damage all
    Shatter = BloonModifierData::Lead as isize,
    Explosion = BloonModifierData::Black as isize,
    Frigid = BloonModifierData::White as isize,
    Magic = BloonModifierData::Purple as isize,
    Energy = BloonModifierData::Lead as isize | BloonModifierData::Purple as isize,
    Sharp = BloonModifierData::Lead as isize | BloonModifierData::Frozen as isize,
    Cold = BloonModifierData::Lead as isize | BloonModifierData::White as isize,
}

/// Effects that bloons can have. Duration in game ticks. No duration indicates an instant effect, such as de-fortify.
#[derive(PartialEq, Clone, Copy)]
pub enum BloonEffect {
    Weakness {strength: i32, duration: i32},
    Speed {strength: f32, duration: i32}, // also serves as slow and stun
    BonusIncome {strength: i32, duration: i32},
}

/// Every bloon tier has an associated type that determines interactions with damage dealers
#[derive(PartialEq, Clone, Copy)]
pub enum BloonType {
    Bloon, Blimp, Boss,
}

/*
    Bloon component and related
*/

#[derive(Component, Clone)]
pub struct Bloon {
    pub bid: BloonID,
    pub hp: i32, // negative hp is overkill amount
    pub modifiers: BloonModifier,
    pub effects: Vec<BloonEffect>,
    pub tier: BloonTier,
}

impl Bloon {
    pub fn with(tier: BloonTier, mut modifiers: BloonModifier)->Bloon {
        modifiers = match tier {
            BloonTier::Purple => modifiers | BloonModifierData::Purple as BloonModifier,
            BloonTier::Black => modifiers | BloonModifierData::Black as BloonModifier,
            BloonTier::White => modifiers | BloonModifierData::White as BloonModifier,
            BloonTier::Zebra => modifiers | BloonModifierData::Black as BloonModifier | BloonModifierData::White as BloonModifier,
            BloonTier::Lead => modifiers | BloonModifierData::Lead as BloonModifier,
            BloonTier::DDT => modifiers | BloonModifierData::Lead as BloonModifier | BloonModifierData::Camo as BloonModifier,
            _ => modifiers,
        };
        let hp_mult = if (modifiers & BloonModifierData::Fortified as BloonModifier) != 0 { tier.get_fortified_hp_mult() } else { 1 };
        return Bloon {
            hp: tier.get_base_hp() * hp_mult,
            tier: tier,
            modifiers: modifiers,
            bid: BloonID::new(), // make a new and random family id
            effects: vec![],
        };
    }
    pub fn apply_effect(&mut self, effect: BloonEffect) {
        self.effects.push(effect);
    }
    pub fn get_child_bloons(&self)->Vec<Bloon> {
        let base_children = self.tier.get_base_child_bloons();
        let mut actual_children = vec![];
        let mut i = 0;
        let child_num = base_children.len();
        for ch in base_children {
            // TODO: Right now fortified will propagate through the red bloon; should drop at ceram level
            let mut child = Bloon::with(ch, self.modifiers);
            child.bid.family = self.bid.family;
            child.bid.tree = self.bid.tree;
            child.bid.layer = self.bid.layer;
            child.update_child_parent_tree(i, child_num);
            actual_children.push(child);
            i += 1;
        }
        return actual_children;
    }
    // this one is jank, but i'm just desperate rn
    pub fn child_of(mut self, parent: &Bloon, child_i: usize, total_children: usize)->Self {
        self.bid.family = parent.bid.family;
        self.bid.tree = parent.bid.tree;
        self.bid.layer = parent.bid.layer;
        self.update_child_parent_tree(child_i, total_children);
        return self;
    }
    /// Given a `bloon` which is a child of a just-popped bloon, how many children were spawned `child_num` and `bloon`'s index in those child bloons `i`,
    /// update `bloon`'s `child_layer` and `child_tree` appropriately
    pub fn update_child_parent_tree(&mut self, i: usize, child_num: usize) {
        // let add_layer = log2(child_num as f32).floor() as u8;
        // let rem_layer = child_num - (2 << add_layer as usize);
        let add_layer = log2(child_num as f32).ceil() as u8; // suboptimal; BADs will use up extra space; doesn't matter for now (TODO)
        self.bid.tree |= (i as BIDTree) << self.bid.layer;
        self.bid.layer += add_layer;
    }
}

/*
impl fmt::Debug for Bloon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bloon")
         .field("tier", &self.bloon_tier)
         .field("hp", &self.hp)
         .finish()
    }
}

impl Ord for Bloon {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.bloon_tier.cmp(&other.bloon_tier);
    }
}

impl PartialOrd for Bloon {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Bloon {
    fn eq(&self, other: &Self) -> bool {
        return self.bloon_tier == other.bloon_tier;
    }
}

impl Eq for Bloon {}
*/

/// The bloon tier determines base stats (speed, hp, etc) as well as children and type (bloon, blimp, boss)
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy, Hash)]
pub enum BloonTier {
    #[default]
    Red, Blue, Green, Yellow, Pink, Purple, Black, White, Zebra, Lead, Rainbow, Ceramic, MOAB, BFB, ZOMG, DDT, BAD,
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
        } / 135.;
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
        // for any new bloons: here's a tip
        // my other code is optimized so it best handles it if "weaker" bloons (bloons with less total children) are put in last in this list
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
    Bloon properties (it's sad and lonely here)
*/

#[derive(Component)]
pub struct BloonPropertyRegrow {
    cooldown_total: i32,
    cooldown_left: i32,
    max_tier: BloonTier,
}

/*
    Misc systems
*/

/// Check if bloons are dead. If yes, spawn children or despawn. Should happen only after the bloons have moved this turn.
/// Big and ugly, sorry, can't do much about that.
pub fn pop_bloons(mut cmd: Commands, map: Res<Map>, bloons: Query<(Entity, &Bloon, &MoveAlongRoad, &Transform)>, overkill_map: Res<OverkillLookupTable>) {
    let mut new_bloons: Vec<(Bloon, MoveAlongRoad, HitboxSimple, Transform, Sprite)> = vec![];
    for (e, bloon, re, pos) in &bloons {
        if bloon.hp > 0 { continue; }
        // Decide whether layer skip is necessary or not
        let child_bloons = if bloon.hp == 0 { bloon.get_child_bloons() } else { calculate_overkill(bloon, &**overkill_map) };
        match child_bloons.len() {
            0 => { cmd.entity(e).despawn(); },
            1 => {
                // let mut child = child_bloons.remove(0); // requires to make `child_bloons` mut and mut is stinky
                let child = child_bloons.into_iter().next().unwrap();
                let child_re = re.clone_with_velocity(child.tier.get_base_speed());
                let child_hb = HitboxSimple { radius: child.tier.get_base_hitbox_radius() };
                let child_sprite = get_bloon_sprite(child.tier); // TODO: sprites should not be here
                cmd.entity(e).insert((child,child_re,child_hb,child_sprite));
            },
            _ => {
                let mut i = 0;
                for child in child_bloons {
                    let mut child_re = re.clone_with_velocity(child.tier.get_base_speed());
                    let child_hb = HitboxSimple { radius: child.tier.get_base_hitbox_radius() };
                    let child_sprite = get_bloon_sprite(child.tier); // TODO: sprites should not be here
                    if i == 0 { 
                        // replace self; no need to spawn an extra bloon
                        cmd.entity(e).insert((child,child_re,child_hb,child_sprite));
                    } else {
                        let mut child_transform = pos.clone();
                        advance_move_along_road(25.0 * i as f32, &*map, &mut child_re, &mut child_transform);
                        new_bloons.push((
                            child,
                            child_re,
                            child_hb,
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

// TODO: remove from here
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