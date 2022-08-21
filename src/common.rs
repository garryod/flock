use bevy::prelude::*;

#[derive(Component)]
pub struct MaxSpeed(pub f32);

impl MaxSpeed {
    pub fn new(speed: f32) -> Self {
        Self(speed)
    }
}

#[derive(Component)]
pub struct MinSpeed(pub f32);

impl MinSpeed {
    pub fn new(speed: f32) -> Self {
        Self(speed)
    }
}
