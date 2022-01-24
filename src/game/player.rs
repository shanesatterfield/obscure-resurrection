use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use super::components::{Enemy, Item, Player};
use crate::components::Velocity;
use crate::config::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_player)
            .add_system(bounce_on_wall)
            .add_system(pick_up_item)
            .add_system(camera_follow_player.after("apply_movement"));
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
        velocity.direction = direction.normalize_or_zero();
    }
}

fn bounce_on_wall(mut query: Query<(&mut Transform, &mut Velocity), With<Enemy>>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        if transform.translation.x < -WINDOW_WIDTH || transform.translation.x > WINDOW_WIDTH {
            velocity.direction = (velocity.direction * -Vec3::X).normalize_or_zero();
            transform.translation += velocity.direction;
        }
        if transform.translation.y < -WINDOW_HEIGHT || transform.translation.y > WINDOW_HEIGHT {
            velocity.direction = (velocity.direction * -Vec3::Y).normalize_or_zero();
            transform.translation += velocity.direction;
        }
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

fn camera_follow_player(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation = player_transform.translation;
        }
    }
}
