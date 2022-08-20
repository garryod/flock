use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

pub const LAUNCHER_TITLE: &str = "Flock! Combine the herd.";

pub fn app() -> App {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(startup);
    app
}

fn startup(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        camera: Camera {
            priority: 1,
            ..default()
        },
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::hsl(196.0, 0.5, 0.75)),
            ..default()
        },
        transform: Transform::from_xyz(-50.0, 50.0, -100_f32)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 50000_f32,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-100_f32, 50_f32, -50_f32)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
