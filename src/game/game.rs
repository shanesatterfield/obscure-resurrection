use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::levels::ResetLevel;
use crate::types::GameState;

use super::collision::plugin::CollisionPlugin;
use super::components::TimeToLive;
use super::enemy::enemy::EnemyPlugin;
use super::events::EnemyAttackBlocked;
use super::events::PickupCoin;
use super::events::PickupItem;
use super::events::PlayerBorked;
use super::events::PlayerDamaged;
use super::level::components::{
    CoinBundle, EnemyBundle, PlayerBundle, PotionBundle, StairsBundle, WallBundle,
};
use super::player::PlayerPlugin;
use super::sfx::SfxPlugin;
use super::ui::UiPlugin;

pub const PLAYER_MAX_HEALTH: u32 = 3;

pub struct GamePlugin;

#[derive(Clone, Debug)]
pub struct GameWorldState {
    pub player_health: u32,
    pub potion_inventory: u32,
    pub bork_points: u32,
    pub coins: u32,
    pub play_time: f64,
}

impl Default for GameWorldState {
    fn default() -> Self {
        Self {
            player_health: PLAYER_MAX_HEALTH,
            potion_inventory: 0,
            bork_points: 0,
            coins: 0,
            play_time: 0.,
        }
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(SfxPlugin)
            .add_plugin(CollisionPlugin)
            .insert_resource(GameWorldState::default())
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(reset_game_world))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(player_damaged.label("damage_calculation"))
                    .with_system(player_picked_up_item)
                    .with_system(player_picked_up_coin)
                    .with_system(time_to_live_system)
                    .with_system(increase_play_time),
            )
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<PotionBundle>("Potion")
            .register_ldtk_entity::<CoinBundle>("Coin")
            .register_ldtk_entity::<WallBundle>("Wall")
            .register_ldtk_entity::<StairsBundle>("Stairs")
            .register_ldtk_entity::<EnemyBundle>("Enemy")
            .add_event::<PlayerDamaged>()
            .add_event::<PickupItem>()
            .add_event::<PickupCoin>()
            .add_event::<EnemyAttackBlocked>()
            .add_event::<PlayerBorked>();
    }
}

fn reset_game_world(
    mut game_world_state: ResMut<GameWorldState>,
    mut reset_level_event: EventWriter<ResetLevel>,
) {
    *game_world_state = GameWorldState::default();
    reset_level_event.send(ResetLevel::default());
}

fn time_to_live_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TimeToLive)>,
) {
    for (entity, mut timer) in query.iter_mut() {
        if timer.0.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn player_damaged(
    mut event_reader: EventReader<PlayerDamaged>,
    mut game_world_state: ResMut<GameWorldState>,
    mut game_state: ResMut<State<GameState>>,
) {
    for _ in event_reader.iter() {
        if game_world_state.player_health > 0 {
            game_world_state.player_health -= 1;
        }

        if game_world_state.player_health == 0 {
            game_state.set(GameState::GameOver).ok();
        }
    }
}

fn player_picked_up_item(
    mut event_reader: EventReader<PickupItem>,
    mut game_world_state: ResMut<GameWorldState>,
) {
    game_world_state.bork_points += event_reader.iter().count() as u32;
}

fn player_picked_up_coin(
    mut event_reader: EventReader<PickupCoin>,
    mut game_world_state: ResMut<GameWorldState>,
) {
    game_world_state.coins += event_reader.iter().count() as u32;
}

fn increase_play_time(time: Res<Time>, mut game_world_state: ResMut<GameWorldState>) {
    game_world_state.play_time += time.delta_seconds_f64();
}
