use bevy::prelude::*;

#[derive(Component)]
pub struct Speed(pub f32);

impl Speed {
    pub fn new(speed: f32) -> Self {
        Self(speed)
    }
}
