use bevy::prelude::*;

use super::collision::*;
use crate::types::GameState;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(player_attack_collision)
                .with_system(player_item_collision)
                .with_system(player_coin_collision)
                .with_system(player_stairs_collision)
                .with_system(player_attack_enemy_attack_collision),
        );
    }
}
