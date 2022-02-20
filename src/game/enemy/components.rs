use bevy::prelude::*;

#[derive(Component, Default, Clone, Debug)]
pub struct Aggroable {
    pub distance: f32,
}

#[derive(Component, Default, Clone, Debug)]
pub struct Aggroed;

#[derive(Component, Default, Clone, Debug)]
pub struct AttackPlayer;

#[derive(Component, Default, Clone, Debug)]
pub struct Attacking {
    pub timer: Timer,
    pub is_attacking: bool,
}
