use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

pub mod ui;
pub mod user_input;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, ui::init_camera)
        .add_systems(Startup, ui::init_text)
        .add_systems(Update, ui::display_stats);

        app.add_systems(Update, (
            user_input::keybind_spawn_bloon, 
            user_input::keybind_global_damage, 
            // user_input::keybind_spawn_projectile,
            user_input::keybind_spawn_projectile_number,
        ));
        app.add_systems(FixedUpdate, (
            // user_input::keybind_spawn_bloon, 
            // user_input::keybind_global_damage, 
            user_input::keybind_spawn_projectile,
            // user_input::keybind_spawn_projectile_number,
        ));
    }
}