use std::f32::consts::PI;

use bevy::prelude::{shape::Capsule, *};
use leafwing_input_manager::{orientation::Direction, prelude::*};

use crate::{camera::MainCameraTag, common::MaxSpeed};

#[derive(Component)]
pub struct PlayerTag;

#[derive(Actionlike, Copy, Clone, Debug)]
enum PlayerMovementAction {
    Forward,
    Backward,
    Left,
    Right,
}

impl PlayerMovementAction {
    fn direction(self) -> Direction {
        match self {
            PlayerMovementAction::Forward => Direction::NORTH,
            PlayerMovementAction::Backward => Direction::SOUTH,
            PlayerMovementAction::Left => Direction::EAST,
            PlayerMovementAction::Right => Direction::WEST,
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    tag: PlayerTag,
    #[bundle]
    mesh: PbrBundle,
    #[bundle]
    input_manager: InputManagerBundle<PlayerMovementAction>,
    speed: MaxSpeed,
}

impl PlayerBundle {
    fn default_input_map() -> InputMap<PlayerMovementAction> {
        let mut input_map = InputMap::default();

        input_map.insert(KeyCode::W, PlayerMovementAction::Forward);
        input_map.insert(KeyCode::S, PlayerMovementAction::Backward);
        input_map.insert(KeyCode::A, PlayerMovementAction::Left);
        input_map.insert(KeyCode::D, PlayerMovementAction::Right);

        input_map
    }

    fn new(
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
        position: Vec2,
    ) -> Self {
        Self {
            tag: PlayerTag,
            mesh: PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(position.x, 1.0, position.y),
                ..default()
            },
            input_manager: InputManagerBundle {
                action_state: ActionState::default(),
                input_map: Self::default_input_map(),
            },
            speed: MaxSpeed::new(10.0),
        }
    }
}

pub struct SpawnPlayerEvent {
    position: Vec2,
}

impl SpawnPlayerEvent {
    pub fn new(x: f32, z: f32) -> Self {
        Self {
            position: Vec2 { x, y: z },
        }
    }
}
fn spawn_player(
    mut spawn_player_event_reader: EventReader<SpawnPlayerEvent>,
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut standard_material_assets: ResMut<Assets<StandardMaterial>>,
) {
    spawn_player_event_reader
        .iter()
        .for_each(|spawn_player_event| {
            commands.spawn_bundle(PlayerBundle::new(
                mesh_assets.add(Mesh::from(Capsule {
                    radius: 0.5,
                    depth: 1.0,
                    ..default()
                })),
                standard_material_assets
                    .add(StandardMaterial::from(Color::hsl(300.0, 0.5, 0.5))),
                spawn_player_event.position,
            ));
        })
}

fn move_player(
    mut player_query: Query<
        (
            &mut Transform,
            &ActionState<PlayerMovementAction>,
            &MaxSpeed,
        ),
        (With<PlayerTag>, Without<MainCameraTag>),
    >,
    camera_query: Query<&Transform, (With<MainCameraTag>, Without<PlayerTag>)>,
    time: Res<Time>,
) {
    let camera_angle = camera_query.single().rotation.y - PI / 2_f32;
    player_query
        .iter_mut()
        .for_each(|(mut transform, action, max_speed)| {
            let direction = action
                .get_pressed()
                .iter()
                .fold(Vec2::ZERO, |acc, action| {
                    acc + Vec2::from(action.direction())
                })
                .normalize_or_zero()
                .rotate(Vec2::from_angle(camera_angle))
                * time.delta_seconds()
                * max_speed.0;
            transform.translation.x += direction.x;
            transform.translation.z += direction.y;
        })
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<PlayerMovementAction>::default())
            .add_event::<SpawnPlayerEvent>()
            .add_system(spawn_player)
            .add_system(move_player);
    }
}
