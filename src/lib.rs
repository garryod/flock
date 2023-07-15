mod barrier;
mod camera;
mod common;
mod field;
mod pen;
mod player;
mod sheep;
mod terrain;

use bevy::prelude::*;
use camera::MainCameraPlugin;
use field::Field;
use iyes_loopless::prelude::*;
use pen::{Pen, PenBundle};
use player::{PlayerBundle, PlayerPlugin};
use sheep::{SheepPlugin, SheepTag, SpawnSheepEvent};
use terrain::TerrainPlugin;

pub const LAUNCHER_TITLE: &str = "Flock! Combine the herd.";

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum GameState {
    Playing,
    Success,
}

pub fn app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: LAUNCHER_TITLE.to_string(),
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
            ..Default::default()
        },
        ..default()
    }))
    .add_plugin(MainCameraPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(SheepPlugin)
    .add_plugin(TerrainPlugin)
    .add_loopless_state(GameState::Playing)
    .add_enter_system(GameState::Playing, start_game)
    .add_event::<SpawnClusterEvent>()
    .add_system(spawn_cluster)
    .add_system(check_win.run_in_state(GameState::Playing))
    .add_startup_system(startup);
    app
}

#[derive(Component)]
struct RoundManager(usize);

impl RoundManager {
    fn new() -> Self {
        Self(0)
    }

    fn next_level(&mut self) {
        self.0 += 1;
    }

    fn get_cluster_sizes(&self) -> Vec<usize> {
        (0..self.0)
            .map(|_| fastrand::usize(1..self.0 + 1))
            .collect()
    }
}

struct SpawnClusterEvent(usize);

fn spawn_cluster(
    mut spawn_cluster_event_reader: EventReader<SpawnClusterEvent>,
    mut spawn_sheep_event_writer: EventWriter<SpawnSheepEvent>,
) {
    spawn_cluster_event_reader
        .iter()
        .for_each(|spawn_cluster_event| {
            let cluster_position = Vec2::new(
                fastrand::f32() * 80_f32 - 40_f32,
                fastrand::f32() * 80_f32 - 40_f32,
            );
            (0..spawn_cluster_event.0).for_each(|_| {
                spawn_sheep_event_writer.send(SpawnSheepEvent::new(
                    cluster_position
                        + Vec2::new(
                            fastrand::f32() * 10_f32 - 5_f32,
                            fastrand::f32() * 10_f32 - 5_f32,
                        ),
                ));
            });
        });
}

fn startup(mut commands: Commands) {
    commands.spawn(RoundManager::new());
}

fn check_win(
    pen_query: Query<&Pen>,
    sheep_query: Query<&Transform, With<SheepTag>>,
    mut commands: Commands,
) {
    if let Ok(pen) = pen_query.get_single() {
        if sheep_query.iter().all(|transform| {
            pen.contains(Vec2::new(
                transform.translation.x,
                transform.translation.z,
            ))
        }) {
            commands.insert_resource(NextState(GameState::Success))
        }
    }
}

fn start_game(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut standard_material_assets: ResMut<Assets<StandardMaterial>>,
    mut round_manager_query: Query<&mut RoundManager>,
    mut spawn_cluster_event_writer: EventWriter<SpawnClusterEvent>,
) {
    let mut round_manager = round_manager_query.single_mut();
    round_manager.next_level();

    commands.spawn(DirectionalLightBundle {
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

    Field::spawn(
        &mut commands,
        &mut mesh_assets,
        &mut standard_material_assets,
    );

    PlayerBundle::spawn(
        &mut commands,
        &mut mesh_assets,
        &mut standard_material_assets,
        Vec2::new(0_f32, 0_f32),
    );

    PenBundle::spawn(
        &mut commands,
        &mut mesh_assets,
        &mut standard_material_assets,
    );

    round_manager
        .get_cluster_sizes()
        .iter()
        .for_each(|&cluster_size| {
            spawn_cluster_event_writer.send(SpawnClusterEvent(cluster_size))
        });
}
