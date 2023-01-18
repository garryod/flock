use bevy::prelude::{shape::Plane, *};

pub struct TerrainPlugin;

impl TerrainPlugin {
    fn create_land(
        mut commands: Commands,
        mut mesh_assets: ResMut<Assets<Mesh>>,
        mut standard_material_assets: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn(PbrBundle {
            mesh: mesh_assets.add(Mesh::from(Plane { size: 120.0 })),
            material: standard_material_assets
                .add(StandardMaterial::from(Color::hsl(135.0, 0.5, 0.25))),
            ..default()
        });
    }
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(TerrainPlugin::create_land);
    }
}
