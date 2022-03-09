use heron::CollisionLayers;

use crate::game::components::GameCollisionLayers;

pub type CollisionLayerFilter = fn(CollisionLayers) -> bool;

pub fn is_player(layers: CollisionLayers) -> bool {
    layers.contains_group(GameCollisionLayers::Player)
        && !layers.contains_group(GameCollisionLayers::Enemy)
        && !layers.contains_group(GameCollisionLayers::EnemyAttack)
}

pub fn is_enemy_attack(layers: CollisionLayers) -> bool {
    !layers.contains_group(GameCollisionLayers::Player)
        && layers.contains_group(GameCollisionLayers::EnemyAttack)
}

pub fn is_item(layers: CollisionLayers) -> bool {
    !layers.contains_group(GameCollisionLayers::Player)
        && layers.contains_group(GameCollisionLayers::Item)
}

pub fn is_coin(layers: CollisionLayers) -> bool {
    !layers.contains_group(GameCollisionLayers::Player)
        && layers.contains_group(GameCollisionLayers::Coin)
}

pub fn is_stairs(layers: CollisionLayers) -> bool {
    !layers.contains_group(GameCollisionLayers::Player)
        && layers.contains_group(GameCollisionLayers::Stairs)
}

pub fn is_player_attack(layers: CollisionLayers) -> bool {
    !layers.contains_group(GameCollisionLayers::EnemyAttack)
        && layers.contains_group(GameCollisionLayers::PlayerAttack)
}
