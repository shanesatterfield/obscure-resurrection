use bevy::prelude::*;

#[derive(Clone, Debug)]
pub enum HorizontalDirection {
    LEFT,
    RIGHT,
}

#[derive(Component, Clone, Debug)]
pub struct FacingDirection(pub HorizontalDirection);

impl Default for FacingDirection {
    fn default() -> Self {
        FacingDirection(HorizontalDirection::RIGHT)
    }
}
