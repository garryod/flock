mod camera;
mod common;
mod player;
mod sheep;
mod terrain;

use bevy::prelude::*;

use camera::MainCameraPlugin;
use player::{PlayerPlugin, SpawnPlayerEvent};
use sheep::{SheepPlugin, SpawnSheepEvent};

use terrain::TerrainPlugin;

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
    .add_plugin(MainCameraPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(SheepPlugin)
    .add_plugin(TerrainPlugin)
    .add_startup_system(startup);
    app
}

fn startup(
    mut commands: Commands,
    mut spawn_player_event_writer: EventWriter<SpawnPlayerEvent>,
    mut spawn_sheep_event_writer: EventWriter<SpawnSheepEvent>,
) {
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

    spawn_player_event_writer.send(SpawnPlayerEvent::new(0_f32, 0_f32));

    spawn_sheep_event_writer.send(SpawnSheepEvent::new(10_f32, 10_f32));
}
