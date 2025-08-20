use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::damage::damage_handling::*;
use crate::map::map::*;
use crate::bloon::bloon::*;
use crate::damage::projectile::*;

pub fn keybind_spawn_bloon(mut cmd: Commands, keyboard_input: Res<ButtonInput<KeyCode>>, map: Res<Map>) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
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
            let damage = 2;
            let pierce = 5;
            // cmd.spawn((
            //     SimpleProjectile { vx: vx, vy: vy, bounce: 0, collide: false, lifetime: 40 },
            //     DamageDealer { damage: 10, pierce: 10, damage_type: DamageType::Sharp, hitbox_radius: 5., ..default() },
            //     Transform::from_translation(vec3(0.,0.,1.)),
            //     get_projectile_sprite(),
            // ));
            for _ in 0..1 {
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