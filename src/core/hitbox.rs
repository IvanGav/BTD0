use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct HitboxSimple {
    pub radius: f32,
}

#[derive(Component, Clone)]
pub struct HitboxComposite {
    pub radii: Vec<f32>,
    pub offsets: Vec<Vec2>,
}