use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use heron::prelude::*;

use super::components::{Item, Player, Speed};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, move_player.label("apply_movement"))
            .add_system(pick_up_item);
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Velocity), With<Player>>,
) {
    let mut direction = Vec2::default();
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

    for (speed, mut velocity) in query.iter_mut() {
        *velocity = Velocity::from_linear(direction.extend(0.).normalize_or_zero() * speed.0);
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
