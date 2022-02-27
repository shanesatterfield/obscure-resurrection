use bevy::prelude::*;
use heron::{CollisionEvent, CollisionLayers};

use crate::types::GameState;

use super::components::{GameCollisionLayers, PlayerDamaged};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::InGame).with_system(player_collision));
    }
}

fn player_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_damange_event: EventWriter<PlayerDamaged>,
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
            player_damange_event.send(PlayerDamaged::default());
            commands.entity(enemy_attack_entity).despawn();
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
