use crate::game::events::*;
use crate::game::game::GameWorldState;
use crate::levels::IncrementLevel;
use bevy::prelude::*;
use heron::CollisionEvent;

use super::layer_filters::*;

pub fn player_attack_collision(
    mut commands: Commands,
    collision_events: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<PlayerDamaged>,
    game_world_state: Res<GameWorldState>,
) {
    if game_world_state.is_borking {
        return;
    }

    filter_events(
        collision_events,
        is_enemy_attack,
        is_player,
        move |enemy_attack_entity, _| {
            event_writer.send(PlayerDamaged::default());
            commands.entity(enemy_attack_entity).despawn();
        },
    );
}

pub fn player_item_collision(
    mut commands: Commands,
    collision_events: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<PickupItem>,
) {
    filter_events(
        collision_events,
        is_item,
        is_player,
        move |item_entity, _| {
            commands.entity(item_entity).despawn_recursive();
            event_writer.send(PickupItem::default());
        },
    );
}

pub fn player_coin_collision(
    mut commands: Commands,
    collision_events: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<PickupCoin>,
) {
    filter_events(
        collision_events,
        is_coin,
        is_player,
        move |coin_entity, _| {
            commands.entity(coin_entity).despawn_recursive();
            event_writer.send(PickupCoin::default());
        },
    );
}

pub fn player_stairs_collision(
    mut commands: Commands,
    collision_events: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<IncrementLevel>,
) {
    filter_events(
        collision_events,
        is_stairs,
        is_player,
        move |stairs_entity, _| {
            commands.entity(stairs_entity).despawn_recursive();
            event_writer.send(IncrementLevel::default());
        },
    );
}

pub fn player_attack_enemy_attack_collision(
    collision_events: EventReader<CollisionEvent>,
    mut event_writer: EventWriter<EnemyAttackBlocked>,
) {
    filter_events(
        collision_events,
        is_enemy_attack,
        is_player_attack,
        move |_, _| {
            event_writer.send(EnemyAttackBlocked::default());
        },
    );
}

pub fn filter_events<F>(
    mut collision_events: EventReader<CollisionEvent>,
    expected_filter: CollisionLayerFilter,
    collided_with: CollisionLayerFilter,
    mut callback: F,
) -> ()
where
    F: FnMut(Entity, Entity),
{
    collision_events
        .iter()
        .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();

            if expected_filter(layers_1) && collided_with(layers_2) {
                Some((entity_1, entity_2))
            } else if collided_with(layers_1) && expected_filter(layers_2) {
                Some((entity_2, entity_1))
            } else {
                None
            }
        })
        .for_each(|(entity1, entity2)| {
            callback(entity1, entity2);
        });
}
