use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::damage::damage_handling::*;
use crate::map::map::*;
use crate::bloon::bloon::*;
use crate::damage::projectile::*;

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
            let damage = 4;
            let pierce = 10;
            for _ in 0..100 {
                cmd.spawn_batch(vec![
                    (
                        SimpleProjectile { vx: vx, vy: vy, bounce: 0, collide: false, lifetime: 40 },
                        DamageDealer { damage: damage, pierce: pierce, damage_type: DamageType::Sharp, hitbox_radius: 5., ..default() },
                        Transform::from_translation(vec3(0.,0.,1.)),
                        get_projectile_sprite(),
                    ),(
                        SimpleProjectile { vx: vx+5., vy: vy, bounce: 0, collide: false, lifetime: 40 },
                        DamageDealer { damage: damage, pierce: pierce, damage_type: DamageType::Sharp, hitbox_radius: 5., ..default() },
                        Transform::from_translation(vec3(0.,0.,1.)),
                        get_projectile_sprite(),
                    ),(
                        SimpleProjectile { vx: vx-5., vy: vy, bounce: 0, collide: false, lifetime: 40 },
                        DamageDealer { damage: damage, pierce: pierce, damage_type: DamageType::Sharp, hitbox_radius: 5., ..default() },
                        Transform::from_translation(vec3(0.,0.,1.)),
                        get_projectile_sprite(),
                    ),(
                        SimpleProjectile { vx: vx, vy: vy+5., bounce: 0, collide: false, lifetime: 40 },
                        DamageDealer { damage: damage, pierce: pierce, damage_type: DamageType::Sharp, hitbox_radius: 5., ..default() },
                        Transform::from_translation(vec3(0.,0.,1.)),
                        get_projectile_sprite(),
                    ),(
                        SimpleProjectile { vx: vx, vy: vy-5., bounce: 0, collide: false, lifetime: 40 },
                        DamageDealer { damage: damage, pierce: pierce, damage_type: DamageType::Sharp, hitbox_radius: 5., ..default() },
                        Transform::from_translation(vec3(0.,0.,1.)),
                        get_projectile_sprite(),
                    )
                ]);
            }
        }
    }
}

pub fn keybind_spawn_projectile_number(mut cmd: Commands, keyboard_input: Res<ButtonInput<KeyCode>>, window: Single<&Window, With<PrimaryWindow>>) {
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
        cmd.spawn((
            SimpleProjectile { vx: vx, vy: vy, bounce: 0, collide: false, lifetime: 40 },
            DamageDealer { damage, pierce, damage_type: DamageType::Sharp, hitbox_radius: 5., ..default() },
            Transform::from_translation(vec3(0.,0.,1.)),
            get_projectile_sprite(),
        ));
    }
}