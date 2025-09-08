use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::core::bloon::BloonModifier;
use crate::core::bloon::{create_bloon, BloonTier};
use crate::core::event::GlobalDamageEvent;
use crate::core::hitbox::HitboxSimple;
use crate::core::map::Map;
use crate::core::movement::MoveSimple;
use crate::core::projectile::{get_projectile_sprite, DamageDealer, LifetimeTick};

pub fn keybind_spawn_bloon(mut cmd: Commands, keyboard_input: Res<ButtonInput<KeyCode>>, map: Res<Map>) {
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        cmd.spawn(create_bloon(BloonTier::Ceramic, &*map));
    } else if keyboard_input.just_pressed(KeyCode::KeyB) {
        cmd.spawn(create_bloon(BloonTier::BAD, &*map));
    }
}

pub fn keybind_global_damage(mut global_damage_ev: EventWriter<GlobalDamageEvent>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        global_damage_ev.send(GlobalDamageEvent { damage: 20000, status_effect: None });
    }
}

pub fn keybind_spawn_projectile(mut cmd: Commands, keyboard_input: Res<ButtonInput<MouseButton>>, window: Single<&Window, With<PrimaryWindow>>) {
    if keyboard_input.pressed(MouseButton::Left) {
        if let Some(pos) = window.cursor_position() {
            let vx = (pos.x-window.width()/2.)/10.;
            let vy = -(pos.y-window.height()/2.)/10.;
            let damage = 1;
            let pierce = 20;
            for i in 0..100 {
                cmd.spawn_batch(vec![
                    simple_projectile(damage, pierce, vec2(vx + (i as f32/20.),vy + (i as f32/20.))),
                    simple_projectile(damage, pierce, vec2(vx+5. + (i as f32/20.),vy + (i as f32/20.))),
                    simple_projectile(damage, pierce, vec2(vx-5. + (i as f32/20.),vy + (i as f32/20.))),
                    simple_projectile(damage, pierce, vec2(vx + (i as f32/20.),vy+5. + (i as f32/20.))),
                    simple_projectile(damage, pierce, vec2(vx + (i as f32/20.),vy-5. + (i as f32/20.))),
                ]);
            }
        }
    }
}

pub fn keybind_spawn_projectile_number(mut cmd: Commands, keyboard_input: Res<ButtonInput<KeyCode>>, window: Single<&Window, With<PrimaryWindow>>) {
    // if keyboard_input.just_pressed(KeyCode::KeyP) {
    //     cmd.insert_resource(Time::<Fixed>::from_hz(256.0));
    //     return;
    // }

    let damage = if keyboard_input.just_pressed(KeyCode::Numpad1) {
        1
    } else if keyboard_input.just_pressed(KeyCode::Numpad2) {
        2
    } else if keyboard_input.just_pressed(KeyCode::Numpad3) {
        3
    } else if keyboard_input.just_pressed(KeyCode::Numpad4) {
        4
    } else if keyboard_input.just_pressed(KeyCode::Numpad5) {
        5
    } else if keyboard_input.just_pressed(KeyCode::Numpad6) {
        6
    } else if keyboard_input.just_pressed(KeyCode::Numpad7) {
        7
    } else if keyboard_input.just_pressed(KeyCode::Numpad8) {
        8
    } else if keyboard_input.just_pressed(KeyCode::Numpad9) {
        9
    } else {
        return;
    };
    if let Some(pos) = window.cursor_position() {
        let vx = (pos.x-window.width()/2.)/10.;
        let vy = -(pos.y-window.height()/2.)/10.;
        let pierce = 10;
        cmd.spawn(simple_projectile(damage, pierce, vec2(vx,vy)));
    }
}

fn simple_projectile(damage: i32, pierce: u32, velocity: Vec2)->(MoveSimple, DamageDealer, Transform, HitboxSimple, LifetimeTick, Sprite) {
    (
        MoveSimple { velocity, bounce: 0, collide_height: None },
        DamageDealer { damage: damage, pierce: pierce, cannot_pop_modifiers: (0 as BloonModifier), cannot_target_modifiers: (0 as BloonModifier), hit_bloons: vec![] },
        Transform::from_translation(vec3(0.,0.,1.)),
        HitboxSimple { radius: 5. },
        LifetimeTick { lifetime: 40 },
        get_projectile_sprite(),
    )
}