use bevy::{math::vec2, prelude::*};

use crate::core::{movement::MoveSimple, tower::TowerEffect};

pub fn attack_dart000(cmd: &mut Commands, angle: f32, entity: Entity, cur_effects: &mut Vec<TowerEffect>, tower_pos: Vec3) {
    cmd.spawn((MoveSimple { velocity: vec2(1.,1.), bounce: 0, collide_height: None}, Transform::from_translation(tower_pos)));
}