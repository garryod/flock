use std::marker::PhantomData;

use bevy::prelude::shape;
use bevy::prelude::*;

use crate::barrier::Barrier;
use crate::common::MaxSpeed;
use crate::player::PlayerTag;

#[derive(Component)]
struct Speed(Vec2);

impl Speed {
    fn new() -> Self {
        Self(Vec2::ZERO)
    }
}

#[derive(Component)]
struct Avoidance<C: Component> {
    strength: f32,
    range: f32,
    influences: Vec<Vec2>,
    component: PhantomData<C>,
}

impl<C: Component> Avoidance<C> {
    fn new(strength: f32, range: f32) -> Self {
        Self {
            strength,
            range,
            influences: Vec::new(),
            component: PhantomData,
        }
    }
}

#[derive(Component)]
struct Coalescence<C: Component> {
    strength: f32,
    range: f32,
    influences: Vec<Vec2>,
    component: PhantomData<C>,
}

impl<C: Component> Coalescence<C> {
    fn new(strength: f32, range: f32) -> Self {
        Self {
            strength,
            range,
            influences: Vec::new(),
            component: PhantomData,
        }
    }
}

#[derive(Component)]
struct Alignment<C: Component> {
    strength: f32,
    range: f32,
    influences: Vec<Vec2>,
    component: PhantomData<C>,
}

impl<C: Component> Alignment<C> {
    fn new(strength: f32, range: f32) -> Self {
        Self {
            strength,
            range,
            influences: Vec::new(),
            component: PhantomData,
        }
    }
}

#[derive(Component)]
pub struct SheepTag;

#[derive(Bundle)]
pub struct SheepBundle {
    tag: SheepTag,
    #[bundle]
    material_mesh: PbrBundle,
    speed: MaxSpeed,
    momentum: Speed,
    player_avoidance: Avoidance<PlayerTag>,
    barrier_avoidance: Avoidance<Barrier>,
    sheep_avoidance: Avoidance<SheepTag>,
    sheep_coalescence: Coalescence<SheepTag>,
    sheep_alignment: Alignment<SheepTag>,
}

impl SheepBundle {
    fn new(
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
        position: Vec2,
    ) -> Self {
        Self {
            tag: SheepTag,
            material_mesh: PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(position.x, 0_f32, position.y),
                ..default()
            },
            speed: MaxSpeed::new(5.0),
            momentum: Speed::new(),
            player_avoidance: Avoidance::new(100.0, 10_f32),
            barrier_avoidance: Avoidance::new(100.0, 5_f32),
            sheep_avoidance: Avoidance::new(10.0, 10_f32),
            sheep_coalescence: Coalescence::new(5.0, 10_f32),
            sheep_alignment: Alignment::new(1.0, 10_f32),
        }
    }

    pub fn spawn(
        commands: &mut Commands,
        mesh_assets: &mut ResMut<Assets<Mesh>>,
        standard_material_assets: &mut ResMut<Assets<StandardMaterial>>,
        position: Vec2,
    ) {
        commands.spawn(SheepBundle::new(
            mesh_assets.add(Mesh::from(shape::Box {
                min_x: -0.5,
                max_x: 0.5,
                min_y: 0.0,
                max_y: 0.5,
                min_z: -0.25,
                max_z: 0.25,
            })),
            standard_material_assets
                .add(StandardMaterial::from(Color::ANTIQUE_WHITE)),
            position,
        ));
    }
}

fn player_influence(
    mut sheep_query: Query<
        (&mut Avoidance<PlayerTag>, &Transform),
        With<SheepTag>,
    >,
    player_query: Query<&Transform, (With<PlayerTag>, Without<SheepTag>)>,
) {
    sheep_query
        .iter_mut()
        .for_each(|(mut avoidance, sheep_transform)| {
            player_query.iter().for_each(|player_transform| {
                let seperation = Vec2::new(
                    sheep_transform.translation.x,
                    sheep_transform.translation.z,
                ) - Vec2::new(
                    player_transform.translation.x,
                    player_transform.translation.z,
                );
                if seperation.length() < avoidance.range {
                    avoidance
                        .influences
                        .push(seperation / seperation.length_squared());
                }
            })
        })
}

#[allow(clippy::type_complexity)]
fn sheep_influences(
    mut sheep_query: Query<
        (
            &mut Avoidance<SheepTag>,
            &mut Coalescence<SheepTag>,
            &mut Alignment<SheepTag>,
            &Transform,
            &Speed,
        ),
        With<SheepTag>,
    >,
) {
    let mut combinations = sheep_query.iter_combinations_mut::<2>();
    while let Some(
        [(
            mut sheep_a_avoidance,
            mut sheep_a_coalescence,
            mut sheep_a_alignment,
            sheep_a_transform,
            sheep_a_speed,
        ), (
            mut sheep_b_avoidance,
            mut sheep_b_coalescence,
            mut sheep_b_alignment,
            sheep_b_transform,
            sheep_b_speed,
        )],
    ) = combinations.fetch_next()
    {
        let seperation = Vec2::new(
            sheep_a_transform.translation.x,
            sheep_a_transform.translation.z,
        ) - Vec2::new(
            sheep_b_transform.translation.x,
            sheep_b_transform.translation.z,
        );
        let seperation_length_squared = seperation.length_squared();
        let seperation_length = seperation_length_squared.sqrt();

        if seperation_length_squared < sheep_a_avoidance.range.powi(2) {
            sheep_a_avoidance
                .influences
                .push(seperation / seperation_length_squared)
        }
        if seperation_length_squared < sheep_b_avoidance.range.powi(2) {
            sheep_b_avoidance
                .influences
                .push(-seperation / seperation_length_squared)
        }
        if seperation_length_squared < sheep_a_coalescence.range.powi(2) {
            sheep_a_coalescence
                .influences
                .push(-seperation / seperation_length)
        }
        if seperation_length_squared < sheep_b_coalescence.range.powi(2) {
            sheep_b_coalescence
                .influences
                .push(seperation / seperation_length)
        }
        if seperation_length_squared < sheep_a_alignment.range.powi(2) {
            sheep_a_alignment
                .influences
                .push(sheep_b_speed.0 / seperation_length)
        }
        if seperation_length_squared < sheep_b_alignment.range.powi(2) {
            sheep_b_alignment
                .influences
                .push(sheep_a_speed.0 / seperation_length)
        }
    }
}

fn barrier_influence(
    mut sheep_query: Query<
        (&mut Avoidance<Barrier>, &Transform),
        With<SheepTag>,
    >,
    linear_barrier_query: Query<&Barrier, Without<SheepTag>>,
) {
    sheep_query
        .iter_mut()
        .for_each(|(mut avoidance, sheep_transform)| {
            linear_barrier_query.iter().for_each(|barrier| {
                let sheep_position = Vec2::new(
                    sheep_transform.translation.x,
                    sheep_transform.translation.z,
                );
                let seperation =
                    sheep_position - barrier.projected_point(sheep_position);
                if seperation.length() < avoidance.range {
                    avoidance
                        .influences
                        .push(seperation / seperation.length_squared());
                }
            })
        })
}

#[derive(SystemLabel)]
struct MoveSheepLabel;

#[allow(clippy::type_complexity)]
fn move_sheep(
    mut sheep_query: Query<
        (
            &mut Transform,
            &mut Avoidance<PlayerTag>,
            &mut Avoidance<Barrier>,
            &mut Avoidance<SheepTag>,
            &mut Coalescence<SheepTag>,
            &mut Alignment<SheepTag>,
            &mut Speed,
            &MaxSpeed,
        ),
        With<SheepTag>,
    >,
    time: Res<Time>,
) {
    sheep_query.iter_mut().for_each(
        |(
            mut transform,
            mut player_avoidance,
            mut barrier_avoidance,
            mut sheep_avoidance,
            mut sheep_coalescence,
            mut sheep_alignment,
            mut speed,
            max_speed,
        )| {
            let player_avoidance_influence =
                player_avoidance.influences.iter().sum::<Vec2>();
            player_avoidance.influences.clear();
            let barrier_avoidance_influence =
                barrier_avoidance.influences.iter().sum::<Vec2>();
            barrier_avoidance.influences.clear();
            let sheep_avoidance_influence =
                sheep_avoidance.influences.iter().sum::<Vec2>();
            sheep_avoidance.influences.clear();
            let sheep_coalescence_influence =
                sheep_coalescence.influences.iter().sum::<Vec2>();
            sheep_coalescence.influences.clear();
            let sheep_alignment_influence =
                if !sheep_alignment.influences.is_empty() {
                    sheep_alignment.influences.iter().sum::<Vec2>()
                        / sheep_alignment.influences.len() as f32
                } else {
                    Vec2::ZERO
                };
            sheep_alignment.influences.clear();
            speed.0 = (speed.0
                + player_avoidance_influence
                    * player_avoidance.strength
                    * time.delta_seconds()
                + barrier_avoidance_influence
                    * barrier_avoidance.strength
                    * time.delta_seconds()
                + sheep_avoidance_influence
                    * sheep_avoidance.strength
                    * time.delta_seconds()
                + sheep_coalescence_influence
                    * sheep_coalescence.strength
                    * time.delta_seconds()
                + sheep_alignment_influence
                    * sheep_alignment.strength
                    * time.delta_seconds()
                - 0.8 * speed.0 * time.delta_seconds())
            .clamp_length_max(max_speed.0);

            if speed.0.length_squared() > 0.01_f32.powi(2) {
                transform.translation.x += speed.0.x * time.delta_seconds();
                transform.translation.z += speed.0.y * time.delta_seconds();
                transform.rotation =
                    Quat::from_rotation_y(speed.0.angle_between(Vec2::X));
            }
        },
    )
}

pub struct SheepPlugin;

impl Plugin for SheepPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_sheep.label(MoveSheepLabel))
            .add_system(player_influence.before(MoveSheepLabel))
            .add_system(barrier_influence.before(MoveSheepLabel))
            .add_system(sheep_influences.before(MoveSheepLabel));
    }
}
