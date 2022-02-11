use crate::{camera::CameraFollowing, components::Velocity};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Component, Default)]
pub struct Item;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    pub camera_following: CameraFollowing,
    pub velocity: Velocity,

    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}
