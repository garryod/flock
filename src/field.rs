use bevy::prelude::{Assets, Commands, Mesh, ResMut, StandardMaterial, Vec2};

use crate::barrier::BarrierBundle;

pub struct Field {
    bounds: [Vec2; 4],
}

impl Default for Field {
    fn default() -> Self {
        Self {
            bounds: [
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
            ],
        }
    }
}

impl Field {
    pub fn spawn(
        commands: &mut Commands,
        mesh_assets: &mut ResMut<Assets<Mesh>>,
        standard_material_assets: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let field = Self::default();
        field
            .bounds
            .iter()
            .zip(field.bounds.iter().cycle().skip(1))
            .for_each(|(&vertex_a, &vertex_b)| {
                BarrierBundle::spawn(
                    commands,
                    vertex_a,
                    vertex_b,
                    mesh_assets,
                    standard_material_assets,
                )
            });
    }
}
