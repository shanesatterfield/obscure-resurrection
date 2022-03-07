use bevy::prelude::*;
use heron::{CollisionEvent, CollisionLayers};

use crate::{levels::IncrementLevel, types::GameState};

use super::{
    components::GameCollisionLayers,
    events::{EnemyAttackBlocked, PickupCoin, PickupItem, PlayerDamaged},
};

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

fn player_attack_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<PlayerDamaged>,
) {
    collision_events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();

            if is_player(layers_1) && is_enemy_attack(layers_2) {
                Some(entity_2)
            } else if is_enemy_attack(layers_1) && is_player(layers_2) {
                Some(entity_1)
            } else {
                None
            }
        })
        .for_each(|enemy_attack_entity| {
            event_writer.send(PlayerDamaged::default());
            commands.entity(enemy_attack_entity).despawn();
        });
}

fn player_item_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<PickupItem>,
) {
    collision_events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();

            if is_player(layers_1) && is_item(layers_2) {
                Some(entity_2)
            } else if is_item(layers_1) && is_player(layers_2) {
                Some(entity_1)
            } else {
                None
            }
        })
        .for_each(|item_entity| {
            commands.entity(item_entity).despawn_recursive();
            event_writer.send(PickupItem::default());
        });
}

fn player_coin_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<PickupCoin>,
) {
    collision_events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();

            if is_player(layers_1) && is_coin(layers_2) {
                Some(entity_2)
            } else if is_coin(layers_1) && is_player(layers_2) {
                Some(entity_1)
            } else {
                None
            }
        })
        .for_each(|coin_entity| {
            commands.entity(coin_entity).despawn_recursive();
            event_writer.send(PickupCoin::default());
        });
}

fn player_stairs_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<IncrementLevel>,
) {
    collision_events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();

            if is_player(layers_1) && is_stairs(layers_2) {
                Some(entity_2)
            } else if is_stairs(layers_1) && is_player(layers_2) {
                Some(entity_1)
            } else {
                None
            }
        })
        .for_each(|stairs_entity| {
            commands.entity(stairs_entity).despawn_recursive();
            event_writer.send(IncrementLevel::default());
        });
}

fn player_attack_enemy_attack_collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<EnemyAttackBlocked>,
) {
    collision_events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();

            if is_player_attack(layers_1) && is_enemy_attack(layers_2) {
                Some(entity_2)
            } else if is_enemy_attack(layers_1) && is_player_attack(layers_2) {
                Some(entity_1)
            } else {
                None
            }
        })
        .for_each(|_| {
            event_writer.send(EnemyAttackBlocked::default());
        });
}

fn is_player(layers: CollisionLayers) -> bool {
    layers.contains_group(GameCollisionLayers::Player)
        && !layers.contains_group(GameCollisionLayers::Enemy)
        && !layers.contains_group(GameCollisionLayers::EnemyAttack)
}

fn is_enemy_attack(layers: CollisionLayers) -> bool {
    !layers.contains_group(GameCollisionLayers::Player)
        && layers.contains_group(GameCollisionLayers::EnemyAttack)
}

fn is_item(layers: CollisionLayers) -> bool {
    !layers.contains_group(GameCollisionLayers::Player)
        && layers.contains_group(GameCollisionLayers::Item)
}

fn is_coin(layers: CollisionLayers) -> bool {
    !layers.contains_group(GameCollisionLayers::Player)
        && layers.contains_group(GameCollisionLayers::Coin)
}

fn is_stairs(layers: CollisionLayers) -> bool {
    !layers.contains_group(GameCollisionLayers::Player)
        && layers.contains_group(GameCollisionLayers::Stairs)
}

fn is_player_attack(layers: CollisionLayers) -> bool {
    !layers.contains_group(GameCollisionLayers::EnemyAttack)
        && layers.contains_group(GameCollisionLayers::PlayerAttack)
}
