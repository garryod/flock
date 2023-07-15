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
use sheep::{SheepBundle, SheepPlugin, SheepTag};
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
    .add_enter_system(GameState::Playing, start_round)
    .add_system(check_win.run_in_state(GameState::Playing))
    .add_startup_system(setup);
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

fn spawn_cluster(
    commands: &mut Commands,
    mesh_assets: &mut ResMut<Assets<Mesh>>,
    standard_material_assets: &mut ResMut<Assets<StandardMaterial>>,
    count: usize,
) {
    let cluster_position = Vec2::new(
        fastrand::f32() * 80_f32 - 40_f32,
        fastrand::f32() * 80_f32 - 40_f32,
    );
    (0..count).for_each(|_| {
        SheepBundle::spawn(
            commands,
            mesh_assets,
            standard_material_assets,
            cluster_position
                + Vec2::new(
                    fastrand::f32() * 10_f32 - 5_f32,
                    fastrand::f32() * 10_f32 - 5_f32,
                ),
        );
    });
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

fn setup(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut standard_material_assets: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(RoundManager::new());

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
}

fn start_round(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut standard_material_assets: ResMut<Assets<StandardMaterial>>,
    mut round_manager_query: Query<&mut RoundManager>,
) {
    let mut round_manager = round_manager_query.single_mut();
    round_manager.next_level();

    PenBundle::spawn(
        &mut commands,
        &mut mesh_assets,
        &mut standard_material_assets,
    );

    round_manager.get_cluster_sizes().iter().for_each(|&count| {
        spawn_cluster(
            &mut commands,
            &mut mesh_assets,
            &mut standard_material_assets,
            count,
        )
    });
}
