use bevy::prelude::*;

#[derive(Component)]
pub struct Barrier {
    vertex_a: Vec2,
    vertex_b: Vec2,
}

impl Barrier {
    fn new(vertex_a: Vec2, vertex_b: Vec2) -> Self {
        Self { vertex_a, vertex_b }
    }

    pub fn projected_point(&self, point: Vec2) -> Vec2 {
        let point_to_a = point - self.vertex_a;
        let b_to_a = self.vertex_b - self.vertex_a;
        let linear_position = (point_to_a.dot(b_to_a)
            / b_to_a.length_squared())
        .clamp(0_f32, 1_f32);
        self.vertex_a + linear_position * b_to_a
    }
}

#[derive(Bundle)]
pub struct BarrierBundle {
    barrier: Barrier,
    #[bundle]
    mesh: PbrBundle,
}

impl BarrierBundle {
    fn new(
        vertex_a: Vec2,
        vertex_b: Vec2,
        mesh_assets: &mut Assets<Mesh>,
        standard_material_assets: &mut Assets<StandardMaterial>,
    ) -> Self {
        Self {
            barrier: Barrier::new(vertex_a, vertex_b),
            mesh: PbrBundle {
                mesh: mesh_assets.add(Mesh::from(shape::Box {
                    min_x: -0.1,
                    max_x: 0.1,
                    min_y: 0.0,
                    max_y: 1.0,
                    min_z: -(vertex_a - vertex_b).length(),
                    max_z: 0.0,
                })),
                material: standard_material_assets
                    .add(StandardMaterial::from(Color::hsl(26.0, 0.30, 0.35))),
                transform: Transform::from_xyz(vertex_a.x, 0_f32, vertex_a.y)
                    .looking_at(
                        Vec3::new(vertex_b.x, 0_f32, vertex_b.y),
                        Vec3::Y,
                    ),
                ..default()
            },
        }
    }

    pub fn spawn(
        commands: &mut Commands,
        vertex_a: Vec2,
        vertex_b: Vec2,
        mesh_assets: &mut ResMut<Assets<Mesh>>,
        standard_material_assets: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn(Self::new(
            vertex_a,
            vertex_b,
            mesh_assets,
            standard_material_assets,
        ));
    }
}
