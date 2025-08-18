use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore,FrameTimeDiagnosticsPlugin};

/*
    Camera
*/

pub fn init_camera(mut cmd: Commands) {
    cmd.spawn(Camera2d);
}

/*
    FPS display
*/

//some more stolen code for displaying fps
pub fn display_stats(diagnostics: Res<DiagnosticsStore>, mut dtexts: Query<(&mut Text, &mut TextColor)>) {
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

pub fn init_text(mut cmd: Commands) {
    cmd.spawn((Text::default(),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }
    ));
}
