use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use heron::prelude::*;

use super::components::{Enemy, Item, Player};
use crate::config::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_player);
        // .add_system(pick_up_item);
    }
}

fn move_player(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    let mut direction = Vec3::default();
    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        direction.y = 1.;
    } else if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        direction.y = -1.;
    }

    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        direction.x = 1.;
    } else if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        direction.x = -1.;
    }

    for mut velocity in query.iter_mut() {
        // velocity.direction = direction.normalize_or_zero();
        *velocity = Velocity::from_linear(direction.normalize_or_zero() * 50.);
    }
}

fn pick_up_item(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    item_query: Query<(Entity, &Transform), With<Item>>,
) {
    let sprite_sizer = Vec2::new(8., 8.);
    for player_transform in player_query.iter() {
        let player = player_transform.translation;
        for (entity, transform) in item_query.iter() {
            if let Some(_) = collide(player, sprite_sizer, transform.translation, sprite_sizer) {
                commands.entity(entity).despawn();
            }
        }
    }
}
