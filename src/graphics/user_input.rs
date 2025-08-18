use bevy::prelude::*;

use crate::damage::damage_handling::GlobalDamageEvent;
use crate::map::map::Map;
use crate::bloon::bloon::BloonTier;
use crate::bloon::bloon::create_bloon;

pub fn keybind_spawn_bloon(mut cmd: Commands, keyboard_input: Res<ButtonInput<KeyCode>>, map: Res<Map>) {
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        cmd.spawn(create_bloon(BloonTier::Ceramic, &*map));
    }
}

pub fn keybind_global_damage(mut global_damage_ev: EventWriter<GlobalDamageEvent>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        global_damage_ev.send(GlobalDamageEvent { damage: 20000, status_effect: None });
    }
}