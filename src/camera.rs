use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

#[derive(Component)]
pub struct MainCameraTag;

#[derive(Bundle)]
struct MainCameraBundle {
    tag: MainCameraTag,
    #[bundle]
    camera_bundle: Camera3dBundle,
}

impl MainCameraBundle {
    fn new(transform: Transform) -> Self {
        Self {
            tag: MainCameraTag,
            camera_bundle: Camera3dBundle {
                camera: Camera {
                    priority: 1,
                    ..default()
                },
                camera_3d: Camera3d {
                    clear_color: ClearColorConfig::Custom(Color::hsl(
                        196.0, 0.5, 0.75,
                    )),
                    ..default()
                },
                transform,
                ..default()
            },
        }
    }
}

pub struct MainCameraPlugin;

fn startup(mut commands: Commands) {
    commands.spawn_bundle(MainCameraBundle::new(
        Transform::from_xyz(-50.0, 50.0, -100_f32)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup);
    }
}
