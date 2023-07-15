use bevy::prelude::*;

use crate::barrier::BarrierBundle;

#[derive(Component)]
pub struct Pen {
    centre: Vec2,
    width: f32,
    height: f32,
    angle: f32,
}

impl Pen {
    fn new(centre: Vec2, width: f32, height: f32, angle: f32) -> Self {
        Self {
            centre,
            width,
            height,
            angle,
        }
    }

    pub fn contains(&self, position: Vec2) -> bool {
        let distance =
            (position - self.centre).rotate(Vec2::from_angle(self.angle));
        distance.x < self.width / 2_f32 && distance.y < self.height / 2_f32
    }
}

#[derive(Bundle)]
pub struct PenBundle {
    pen: Pen,
    #[bundle]
    mesh: PbrBundle,
}

impl PenBundle {
    fn new(
        centre: Vec2,
        width: f32,
        height: f32,
        angle: f32,
        mesh_assets: &mut Assets<Mesh>,
        standard_material_assets: &mut Assets<StandardMaterial>,
    ) -> Self {
        let looking = centre + Vec2::from_angle(angle);
        Self {
            pen: Pen::new(centre, width, height, angle),
            mesh: PbrBundle {
                mesh: mesh_assets.add(Mesh::from(shape::Box {
                    min_x: -height / 2_f32,
                    max_x: height / 2_f32,
                    min_y: 0_f32,
                    max_y: 0.1_f32,
                    min_z: -width / 2_f32,
                    max_z: width / 2_f32,
                })),
                material: standard_material_assets.add(StandardMaterial {
                    base_color: Color::hsla(110.0, 0.5, 0.5, 0.2),
                    emissive: Color::hsl(110.0, 1.0, 0.1),
                    ..default()
                }),
                transform: Transform::from_xyz(centre.x, 0_f32, centre.y)
                    .looking_at(
                        Vec3::new(looking.x, 0_f32, looking.y),
                        Vec3::Y,
                    ),
                ..default()
            },
        }
    }

    pub fn spawn(
        commands: &mut Commands,
        mesh_assets: &mut ResMut<Assets<Mesh>>,
        standard_material_assets: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let centre = Vec2::new(
            fastrand::f32() * 80_f32 - 40_f32,
            fastrand::f32() * 80_f32 - 40_f32,
        );
        let width = fastrand::f32() * 10_f32 + 10_f32;
        let height = fastrand::f32() * 10_f32 + 10_f32;
        let angle = fastrand::f32() * 2_f32 * std::f32::consts::PI;

        commands.spawn(Self::new(
            centre,
            width,
            height,
            angle,
            mesh_assets,
            standard_material_assets,
        ));

        let rot_vector = Vec2::from_angle(angle);

        let rel_corners = [
            centre
                + Vec2::new(width / 2_f32, height / 2_f32).rotate(rot_vector),
            centre
                + Vec2::new(width / 2_f32, -height / 2_f32).rotate(rot_vector),
            centre
                + Vec2::new(-width / 2_f32, -height / 2_f32).rotate(rot_vector),
            centre
                + Vec2::new(-width / 2_f32, height / 2_f32).rotate(rot_vector),
        ];

        rel_corners.iter().zip(rel_corners.iter().skip(1)).for_each(
            |(vertex_a, vertex_b)| {
                BarrierBundle::spawn(
                    commands,
                    *vertex_a,
                    *vertex_b,
                    mesh_assets,
                    standard_material_assets,
                );
            },
        );
    }
}
