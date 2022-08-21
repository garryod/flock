use std::marker::PhantomData;

use bevy::prelude::shape::UVSphere;
use bevy::prelude::*;

use crate::common::Speed;
use crate::player::PlayerTag;

#[derive(Component)]
struct MoveInfluences(Vec<Vec2>);

#[derive(Component)]
struct Avoidance<C: Component> {
    strength: f32,
    range: f32,
    component: PhantomData<C>,
}

impl<C: Component> Avoidance<C> {
    fn new(strength: f32, range: f32) -> Self {
        Self {
            strength,
            range,
            component: PhantomData,
        }
    }
}

#[derive(Component)]
struct SheepTag;

#[derive(Bundle)]
pub struct SheepBundle {
    tag: SheepTag,
    move_influences: MoveInfluences,
    #[bundle]
    material_mesh: PbrBundle,
    speed: Speed,
    player_fear: Avoidance<PlayerTag>,
}

impl SheepBundle {
    fn new(
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
        position: Vec2,
    ) -> Self {
        Self {
            tag: SheepTag,
            move_influences: MoveInfluences(Vec::new()),
            material_mesh: PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(position.x, 1_f32, position.y),
                ..default()
            },
            speed: Speed::new(5.0),
            player_fear: Avoidance::new(100_f32, 20.0),
        }
    }
}

pub struct SpawnSheepEvent {
    position: Vec2,
}

impl SpawnSheepEvent {
    pub fn new(x: f32, z: f32) -> Self {
        Self {
            position: Vec2 { x, y: z },
        }
    }
}

fn spawn_sheep(
    mut spawn_sheep_event_reader: EventReader<SpawnSheepEvent>,
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut standard_material_assets: ResMut<Assets<StandardMaterial>>,
) {
    spawn_sheep_event_reader
        .iter()
        .for_each(|spawn_sheep_event| {
            commands.spawn_bundle(SheepBundle::new(
                mesh_assets.add(Mesh::from(UVSphere {
                    radius: 0.5,
                    ..default()
                })),
                standard_material_assets
                    .add(StandardMaterial::from(Color::ANTIQUE_WHITE)),
                spawn_sheep_event.position,
            ));
        });
}

fn player_move_influence(
    mut sheep_query: Query<
        (&mut MoveInfluences, &Transform, &Avoidance<PlayerTag>),
        (With<SheepTag>, Without<PlayerTag>),
    >,
    player_query: Query<&Transform, (With<PlayerTag>, Without<SheepTag>)>,
) {
    sheep_query.iter_mut().for_each(
        |(mut move_influences, sheep_transform, avoidance)| {
            player_query.iter().for_each(|player_transform| {
                let seperation = Vec2::new(
                    sheep_transform.translation.x,
                    sheep_transform.translation.z,
                ) - Vec2::new(
                    player_transform.translation.x,
                    player_transform.translation.z,
                );
                if seperation.length() < avoidance.range {
                    let influence = avoidance.strength * seperation
                        / seperation.length().powi(3);
                    move_influences.0.push(influence);
                }
            })
        },
    )
}

#[derive(SystemLabel)]
struct MoveSheepLabel;

fn move_sheep(
    mut sheep_query: Query<
        (&mut Transform, &mut MoveInfluences, &Speed),
        With<SheepTag>,
    >,
    time: Res<Time>,
) {
    sheep_query.iter_mut().for_each(
        |(mut transform, mut move_influences, max_speed)| {
            let move_vec = (move_influences.0.iter().sum::<Vec2>()
                * time.delta_seconds())
            .clamp_length_max(max_speed.0);
            transform.translation.x += move_vec.x;
            transform.translation.z += move_vec.y;
            move_influences.0.clear();
        },
    )
}

pub struct SheepPlugin;

impl Plugin for SheepPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnSheepEvent>()
            .add_system(spawn_sheep)
            .add_system(move_sheep.label(MoveSheepLabel))
            .add_system(player_move_influence.before(MoveSheepLabel));
    }
}
