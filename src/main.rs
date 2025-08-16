use bevy::{math::vec2, prelude::*};

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

#[derive(Component, Default)]
/// A component that lets an entity to occupy a position on a road and move along the road
/// Do not apply to road items, as they don't need to move along the road
struct RoadEntity {
    target_node: usize,
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
        } / 20.;
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

fn keybind_spawn_bloon(mut cmd: Commands, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        cmd.spawn((
            Bloon::with(BloonTier::Red, vec![]),
            Sprite::from_color(Color::Srgba(Srgba { red: 1., green: 0., blue: 0., alpha: 1. }), vec2(50., 50.)),
            Transform::from_xyz(-200., -200., 1.)
        ));
        cmd.spawn((
            Bloon::with(BloonTier::Pink, vec![]), 
            Sprite::from_color(Color::Srgba(Srgba { red: 1., green: 0.5, blue: 0.5, alpha: 1. }), vec2(50., 50.)),
            Transform::from_xyz(-200., -200., 1.)
        ));
    }
}

fn keybind_global_damage(mut global_damage_ev: EventWriter<GlobalDamageEvent>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        global_damage_ev.send(GlobalDamageEvent { damage: 1, status_effect: None });
    }
}

/// Move bloons along the track
fn move_bloons(mut cmd: Commands, mut bloons: Query<(Entity, &Bloon, &mut RoadEntity, &mut Transform)>) {
    for (e, bloon, re, pos) in &mut bloons {
        move_bloon(&mut cmd, e, bloon, re, pos);
    }
}

/// Check if bloons are dead. If yes, spawn children or despawn. Should happen only after the bloons have moved this turn.
fn pop_bloons(mut cmd: Commands, bloons: Query<(Entity, &Bloon, &RoadEntity, &Transform)>) {
    for (e, bloon, re, pos) in &bloons {
        if bloon.hp <= 0 {
            let child_bloons = bloon.get_child_bloons();
            for child in child_bloons {
                cmd.spawn((
                    get_bloon_sprite(child.bloon_tier),
                    child,
                    RoadEntity { target_node: re.target_node },
                    pos.clone(),
                ));
            }
            cmd.entity(e).despawn();
        }
    }
}

fn damage_bloons(mut cmd: Commands) {

}

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

fn move_bloon(cmd: &mut Commands, e: Entity, bloon: &Bloon, mut re: Mut<'_, RoadEntity>, mut pos: Mut<'_, Transform>) {
    let dx = PATH[re.target_node].x - pos.translation.x;
    let dy = PATH[re.target_node].y - pos.translation.y;
    let total_dist = (dx*dx + dy*dy).sqrt();
    let speed = bloon.bloon_tier.get_base_speed();

    if total_dist < speed {
        // Move to the node and advance the node index
        // TODO: Should overflow to movement along the next node
        pos.translation.x += dx;
        pos.translation.y += dy;
        re.target_node += 1;
        if re.target_node >= 4 {
            cmd.entity(e).despawn();
        }
    } else {
        pos.translation.x += dx * speed / total_dist;
        pos.translation.y += dy * speed / total_dist;
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
        BloonTier::Black => Sprite::from_color(Color::Srgba(Srgba { red: 0., green: 0., blue: 0., alpha: 1. }), vec2(50., 50.)),
        BloonTier::White => Sprite::from_color(Color::Srgba(Srgba { red: 1., green: 1., blue: 1., alpha: 1. }), vec2(50., 50.)),
        BloonTier::Zebra => Sprite::from_color(Color::Srgba(Srgba { red: 0.7, green: 0.7, blue: 0.7, alpha: 1. }), vec2(50., 50.)),
        BloonTier::Lead => Sprite::from_color(Color::Srgba(Srgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 1. }), vec2(50., 50.)),
        _ => Sprite::from_color(Color::Srgba(Srgba { red: 0., green: 1., blue: 0., alpha: 1. }), vec2(100., 100.)),
    };
}

/*
    Main
*/

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<GlobalDamageEvent>()
        .add_systems(Startup, init_camera)
        .add_systems(Update, (keybind_spawn_bloon, keybind_global_damage))
        .add_systems(FixedPreUpdate, (
            damage_bloons, global_damage_effects,
        ))
        .add_systems(FixedUpdate, (
            move_bloons, pop_bloons.after(move_bloons)
        ))
        .run();
}