use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub direction: Vec3,
    pub speed: f32,
}

impl Velocity {
    pub fn new(direction: Vec3, speed: f32) -> Velocity {
        Velocity {
            direction: direction.normalize_or_zero(),
            speed,
        }
    }

    pub fn velocity(&self) -> Vec3 {
        self.direction * self.speed
    }
}
