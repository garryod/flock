use bevy::prelude::shape::UVSphere;
use bevy::prelude::*;
#[derive(Component)]
struct SheepTag;

#[derive(Bundle)]
pub struct SheepBundle {
    tag: SheepTag,
    #[bundle]
    material_mesh: PbrBundle,
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

pub struct SheepPlugin;

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
                    radius: 1_f32,
                    sectors: 16,
                    stacks: 16,
                })),
                standard_material_assets
                    .add(StandardMaterial::from(Color::ANTIQUE_WHITE)),
                spawn_sheep_event.position,
            ));
        });
}

fn move_sheep(mut sheep_query: Query<&mut Transform, With<SheepTag>>) {
    sheep_query
        .iter_mut()
        .for_each(|mut sheep_transform| sheep_transform.translation.x += 0.5)
}

impl Plugin for SheepPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnSheepEvent>()
            .add_system(spawn_sheep)
            .add_system(move_sheep);
    }
}
