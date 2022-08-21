use std::marker::PhantomData;

use bevy::prelude::shape::UVSphere;
use bevy::prelude::*;

use crate::common::{MaxSpeed, MinSpeed};
use crate::player::PlayerTag;

#[derive(Component)]
struct MoveInfluences(Vec<Vec2>);

#[derive(Component)]
struct Fear<C: Component>(f32, PhantomData<C>);

impl<C: Component> Fear<C> {
    fn new(fear: f32) -> Self {
        Self(fear, PhantomData)
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
    max_speed: MaxSpeed,
    min_speed: MinSpeed,
    player_fear: Fear<PlayerTag>,
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
            max_speed: MaxSpeed::new(5.0),
            min_speed: MinSpeed::new(0.01),
            player_fear: Fear::new(100_f32),
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
        (&mut MoveInfluences, &Transform, &Fear<PlayerTag>),
        (With<SheepTag>, Without<PlayerTag>),
    >,
    player_query: Query<&Transform, (With<PlayerTag>, Without<SheepTag>)>,
) {
    sheep_query.iter_mut().for_each(
        |(mut move_influences, sheep_transform, player_fear)| {
            player_query.iter().for_each(|player_transform| {
                let seperation = Vec2::new(
                    sheep_transform.translation.x,
                    sheep_transform.translation.z,
                ) - Vec2::new(
                    player_transform.translation.x,
                    player_transform.translation.z,
                );
                let influence =
                    player_fear.0 * seperation / seperation.length().powi(3);
                move_influences.0.push(influence);
            })
        },
    )
}

fn vec_minimum_or_zero(vec: Vec2, minimum: f32) -> Vec2 {
    if vec.length() < minimum {
        Vec2::ZERO
    } else {
        vec
    }
}

fn move_sheep(
    mut sheep_query: Query<
        (&mut Transform, &mut MoveInfluences, &MaxSpeed, &MinSpeed),
        With<SheepTag>,
    >,
    time: Res<Time>,
) {
    sheep_query.iter_mut().for_each(
        |(mut transform, mut move_influences, max_speed, min_speed)| {
            let move_vec = vec_minimum_or_zero(
                (move_influences.0.iter().sum::<Vec2>() * time.delta_seconds())
                    .clamp_length_max(max_speed.0),
                min_speed.0,
            );
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
            .add_system(move_sheep)
            .add_system(player_move_influence);
    }
}
