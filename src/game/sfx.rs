use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::types::{AudioAssets, GameState};

use super::events::{EnemyAttackBlocked, PickupCoin, PickupItem, PlayerBorked, PlayerDamaged};

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(player_picked_up_item)
                .with_system(player_picked_up_coin_sfx)
                .with_system(player_damaged_sfx)
                .with_system(enemy_attack_blocked)
                .with_system(player_borked_sfx),
        );
    }
}

fn player_picked_up_item(
    mut event_reader: EventReader<PickupItem>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    for _ in event_reader.iter() {
        audio.play(audio_assets.coin.clone());
    }
}

fn player_picked_up_coin_sfx(
    mut event_reader: EventReader<PickupCoin>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    for _ in event_reader.iter() {
        audio.play(audio_assets.coin.clone());
    }
}

fn player_damaged_sfx(
    mut event_reader: EventReader<PlayerDamaged>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    for _ in event_reader.iter() {
        audio.play(audio_assets.hit.clone());
    }
}

fn enemy_attack_blocked(
    mut event_reader: EventReader<EnemyAttackBlocked>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    for _ in event_reader.iter() {
        audio.play(audio_assets.block.clone());
    }
}

fn player_borked_sfx(
    mut event_reader: EventReader<PlayerBorked>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    for _ in event_reader.iter() {
        audio.play(audio_assets.bork.clone());
    }
}
