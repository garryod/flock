mod barrier;
mod camera;
mod common;
mod player;
mod sheep;
mod terrain;

use barrier::{BarrierPlugin, SpawnBarrierEvent};
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
    .add_plugin(BarrierPlugin)
    .add_plugin(TerrainPlugin)
    .add_startup_system(startup);
    app
}

fn startup(
    mut commands: Commands,
    mut spawn_player_event_writer: EventWriter<SpawnPlayerEvent>,
    mut spawn_sheep_event_writer: EventWriter<SpawnSheepEvent>,
    mut spawn_barrier_event_writer: EventWriter<SpawnBarrierEvent>,
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

    (0..10).for_each(|_| {
        spawn_sheep_event_writer.send(SpawnSheepEvent::new(
            fastrand::f32() * 20.0 - 10.0,
            fastrand::f32() * 20.0 - 10.0,
        ))
    });

    let field_corners = [
        Vec2::new(
            45_f32 + fastrand::f32() * 10_f32,
            45_f32 + fastrand::f32() * 10_f32,
        ),
        Vec2::new(
            45_f32 + fastrand::f32() * 10_f32,
            -45_f32 - fastrand::f32() * 10_f32,
        ),
        Vec2::new(
            -45_f32 - fastrand::f32() * 10_f32,
            -45_f32 - fastrand::f32() * 10_f32,
        ),
        Vec2::new(
            -45_f32 - fastrand::f32() * 10_f32,
            45_f32 + fastrand::f32() * 10_f32,
        ),
    ];

    field_corners
        .iter()
        .zip(field_corners.iter().cycle().skip(1))
        .for_each(|(vertex_a, vertex_b)| {
            spawn_barrier_event_writer
                .send(SpawnBarrierEvent::new(*vertex_a, *vertex_b))
        });

    let pen_centre = Vec2::new(
        fastrand::f32() * 80_f32 - 40_f32,
        fastrand::f32() * 80_f32 - 40_f32,
    );
    let pen_corners = [
        pen_centre + Vec2::new(7.5, 7.5),
        pen_centre + Vec2::new(7.5, -7.5),
        pen_centre + Vec2::new(-7.5, -7.5),
        pen_centre + Vec2::new(-7.5, 7.5),
    ];

    pen_corners.iter().zip(pen_corners.iter().skip(1)).for_each(
        |(vertex_a, vertex_b)| {
            spawn_barrier_event_writer
                .send(SpawnBarrierEvent::new(*vertex_a, *vertex_b))
        },
    );
}
