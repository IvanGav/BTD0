use bevy::{math::vec2, prelude::*};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

mod map;
use map::*;

use crate::map::map::*;

// Game space is (-1,-1) (bottom left)..(1,1) (top right)

/// Should be a resource, in the future
pub const PATH: [Vec2; 4] = [vec2(-200.,-200.), vec2(-100.,0.), vec2(100., 0.), vec2(200.,200.)];

/*
    Components
*/

#[derive(Component)]
#[require(RoadEntity, Sprite, Transform, Visibility)]
/// A component that indicates that an entity is a bloon
struct Bloon {
    bloon_tier: BloonTier,
    hp: i32,
    bloon_modifiers: Vec<BloonModifier>,
    status_effects: Vec<BloonEffect>,
}

#[derive(Component, Default, Clone)]
/// A component that lets an entity to occupy a position on a road and move along the road
/// Do not apply to road items, as they don't need to move along the road
struct RoadEntity {
    target_node: usize, // next targeted node
    track_pos: f32, // position on the track
    waypoint: Vec2, // may or may not be target node's position; after reaching, incrememnt `target_node`
}

#[derive(PartialEq, Default, Clone, Copy, Debug)]
/// The bloon tier determines base stats - speed, hp, etc
enum BloonTier {
    #[default]
    Red, Blue, Green, Yellow, Pink, Purple, Black, White, Zebra, Lead, Rainbow, Ceramic, MOAB, BFB, ZOMG, DDT, BAD
}

#[derive(PartialEq, Clone, Copy)]
/// Every bloon tier has an associated type that determines interactions with damage dealers
enum BloonType {
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
            BloonTier::White => modifiers.push(BloonModifier::FrozenImmune),
            BloonTier::Zebra => {
                modifiers.push(BloonModifier::ExplosionImmune);
                modifiers.push(BloonModifier::FrozenImmune);
            },
            BloonTier::Lead => modifiers.push(BloonModifier::SharpImmune),
            BloonTier::DDT => {
                modifiers.push(BloonModifier::SharpImmune);
                modifiers.push(BloonModifier::Camo);
            },
            _ => ()
        };
        let hp_mult = if modifiers.contains(&BloonModifier::Fortified) { tier.get_fortified_hp_mult() } else { 1 };
        return Bloon {hp: tier.get_base_hp() * hp_mult, bloon_tier: tier, bloon_modifiers: modifiers, status_effects: vec![]};
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
            BloonTier::BAD => vec![BloonTier::ZOMG, BloonTier::ZOMG, BloonTier::ZOMG, BloonTier::DDT, BloonTier::DDT],
        }
    }
    pub fn get_fortified_hp_mult(&self)->i32 {
        return match self {
            BloonTier::Lead => 4,
            _ => 2,
        }
    }
}

/*
    Events
*/

#[derive(Event)]
struct GlobalDamageEvent {
    damage: i32,
    status_effect: Option<BloonEffect>
}

/*
    Other structs
*/

#[derive(PartialEq, Clone, Copy)]
/// Effects that bloons can have. Duration in game ticks.
enum BloonEffect {
    Weakness {strength: i32, duration: i32},
    Speed {strength: f32, duration: i32}, // also serves as slow and stun
    BonusIncome {strength: i32, duration: i32},
}

#[derive(PartialEq, Clone, Copy)]
enum BloonModifier {
    Fortified, Camo, Regrow, SharpImmune, MagicImmune, FrozenImmune, ExplosionImmune
}

/*
    Systems
*/

fn init_camera(mut cmd: Commands) {
    cmd.spawn(Camera2d);
}

fn keybind_spawn_bloon(mut cmd: Commands, keyboard_input: Res<ButtonInput<KeyCode>>, map: Res<Map>) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        // cmd.spawn(create_bloon(BloonTier::Pink, &*map));
        // cmd.spawn(create_bloon(BloonTier::Blue, &*map));
        cmd.spawn(create_bloon(BloonTier::Ceramic, &*map));
    }
}

fn keybind_global_damage(mut global_damage_ev: EventWriter<GlobalDamageEvent>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        global_damage_ev.send(GlobalDamageEvent { damage: 20000, status_effect: None });
    }
}

/// Move bloons along the track
fn move_bloons(map: Res<Map>, mut bloons: Query<(&Bloon, &mut RoadEntity, &mut Transform)>) {
    for (bloon, re, pos) in &mut bloons {
        advance_road_entity(bloon.bloon_tier.get_base_speed(), &*map, re.into_inner(), pos.into_inner());
    }
}

/// Check if bloons are dead. If yes, spawn children or despawn. Should happen only after the bloons have moved this turn.
fn pop_bloons(mut cmd: Commands, map: Res<Map>, bloons: Query<(Entity, &Bloon, &RoadEntity, &Transform)>) {
    for (e, bloon, re, pos) in &bloons {
        if bloon.hp <= 0 {
            // TODO: layer skip
            let child_bloons = bloon.get_child_bloons();
            let mut i = 0;
            for child in child_bloons {
                let mut child_transform = pos.clone();
                let mut child_re = (*re).clone();
                advance_road_entity(25.0 * i as f32, &*map, &mut child_re, &mut child_transform);
                cmd.spawn((
                    get_bloon_sprite(child.bloon_tier),
                    child,
                    child_re,
                    child_transform,
                ));
                i += 1;
            }
            cmd.entity(e).despawn();
        }
    }
}

/// Despawn bloons which have exited the map (gone past the last node of the map)
fn despawn_exited_bloons(mut cmd: Commands, map: Res<Map>, bloons: Query<(Entity, &RoadEntity)>) {
    for (e, re) in &bloons {
        if re.target_node == map.path.len() {
            cmd.entity(e).despawn();
        }
    }
}

/// Apply queued damage to bloons.
fn apply_bloon_damage(mut cmd: Commands) {

}

/// Test if projectiles collide with bloons. If yes, send a damage taken event.
fn damage_bloons(mut cmd: Commands) {

}

/// A system that applies a global damage effect on all active bloons
fn global_damage_effects(mut bloons: Query<&mut Bloon>, mut global_damage_ev: EventReader<GlobalDamageEvent>) {
    for ev in global_damage_ev.read() {
        for mut bloon in &mut bloons {
            bloon.hp -= ev.damage;
            if let Some(effect) = &ev.status_effect {
                bloon.apply_effect(*effect);
            }
        }
    }
}

/*
    Hepler functions
*/

/// Create a bloon at the beginning of the given track
fn create_bloon(tier: BloonTier, map: &Map)->(Bloon, Sprite, RoadEntity, Transform) {
    return (
        Bloon::with(tier, vec![]),
        get_bloon_sprite(tier),
        RoadEntity { target_node: 0, track_pos: 0., waypoint: map.start_pos()},
        Transform::from_xyz(map.start_pos().x, map.start_pos().y, 1.),
    );
}

/// Move a given RoadEntity along the road with the given step size (that should depend on its speed)
fn advance_road_entity(step: f32, map: &Map, re: &mut RoadEntity, pos: &mut Transform) {
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

fn get_bloon_sprite(tier: BloonTier)->Sprite {
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
    FPS display
*/


//some more stolen code for displaying fps
fn display_stats(diagnostics: Res<DiagnosticsStore>, mut dtexts: Query<(&mut Text, &mut TextColor)>) {
    for (mut text, mut color) in &mut dtexts {
        // try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            // Format the number as to leave space for 4 digits, just in case,
            // right-aligned and rounded. This helps readability when the
            // number changes rapidly.
            text.0 = format!("{value:>4.0}fps");

            // Let's make it extra fancy by changing the color of the
            // text according to the FPS value:
            color.0 = if value >= 120.0 {
                // Above 120 FPS, use green color
                Color::srgb(0., 1., 0.)
            } else if value >= 60.0 {
                // Between 60-120 FPS, gradually transition from yellow to green
                Color::srgb(
                    (1.0 - (value - 60.0) / (120.0 - 60.0)) as f32,
                    1.0,
                    0.0,
                )
            } else if value >= 30.0 {
                // Between 30-60 FPS, gradually transition from red to yellow
                Color::srgb(
                    1.0,
                    ((value - 30.0) / (60.0 - 30.0)) as f32,
                    0.0,
                )
            } else {
                // Below 30 FPS, use red color
                Color::srgb(1., 0., 0.)
            }
        } else {
            // display "N/A" if we can't get a FPS measurement
            // add an extra space to preserve alignment
            text.0 = " N/A".into();
            color.0 = Color::WHITE;
        }
    }
}

fn init_text(mut cmd: Commands) {
    cmd.spawn((Text::default(),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }
    ));
}

/*
    Main
*/

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MapPlugin, FrameTimeDiagnosticsPlugin))
        .add_event::<GlobalDamageEvent>()
        .add_systems(Startup, init_camera)
        .add_systems(Update, (keybind_spawn_bloon, keybind_global_damage))
        .add_systems(FixedPreUpdate, (
            (global_damage_effects, damage_bloons, apply_bloon_damage).chain()
        ))
        .add_systems(FixedUpdate, (
            (move_bloons, pop_bloons, despawn_exited_bloons).chain()
        ))
        .add_systems(Update, display_stats)
        .add_systems(Startup, init_text)
        .run();
}