use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::types::GameState;

use super::events::{EnemyAttackBlocked, PickupCoin, PickupItem, PlayerDamaged};

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(player_picked_up_item)
                .with_system(player_picked_up_coin_sfx)
                .with_system(player_damaged_sfx)
                .with_system(enemy_attack_blocked),
        );
    }
}

fn player_picked_up_item(
    mut event_reader: EventReader<PickupItem>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for _ in event_reader.iter() {
        audio.play(asset_server.load("audio/coin_sfx.wav"));
    }
}

fn player_picked_up_coin_sfx(
    mut event_reader: EventReader<PickupCoin>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for _ in event_reader.iter() {
        audio.play(asset_server.load("audio/coin_sfx.wav"));
    }
}

fn player_damaged_sfx(
    mut event_reader: EventReader<PlayerDamaged>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for _ in event_reader.iter() {
        audio.play(asset_server.load("audio/hit_sfx.wav"));
    }
}

fn enemy_attack_blocked(
    mut event_reader: EventReader<EnemyAttackBlocked>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for _ in event_reader.iter() {
        audio.play(asset_server.load("audio/block_sfx.wav"));
    }
}
