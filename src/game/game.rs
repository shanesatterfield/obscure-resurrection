extern crate rand;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use rand::thread_rng;
use rand::Rng;

use crate::levels::IncrementLevel;
use crate::levels::ResetLevel;
use crate::types::GameState;

use super::collision::CollisionPlugin;
use super::components::EnemyBundle;
use super::components::Item;
use super::components::PlayerBundle;
use super::components::PotionBundle;
use super::components::TimeToLive;
use super::components::WallBundle;
use super::enemy::enemy::EnemyPlugin;
use super::events::PlayerDamaged;
use super::player::PlayerPlugin;
use super::ui::UiPlugin;

pub const PLAYER_MAX_HEALTH: u32 = 3;

pub struct GamePlugin;

pub struct GameWorldState {
    pub is_level_loaded: bool,
    pub potion_count: u32,
    pub potion_inventory: u32,
    pub player_health: u32,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(CollisionPlugin)
            .insert_resource(GameWorldState {
                is_level_loaded: false,
                potion_count: 0,
                potion_inventory: 0,
                player_health: PLAYER_MAX_HEALTH,
            })
            .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(reset_game_world))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(player_damaged.label("damage_calculation"))
                    .with_system(change_level)
                    .with_system(setup_item)
                    .with_system(time_to_live_system),
            )
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<PotionBundle>("Potion")
            .register_ldtk_entity::<WallBundle>("Wall")
            .register_ldtk_entity::<EnemyBundle>("Enemy")
            .add_event::<PlayerDamaged>();
    }
}

fn reset_game_world(
    mut game_world_state: ResMut<GameWorldState>,
    mut reset_level_event: EventWriter<ResetLevel>,
) {
    *game_world_state = GameWorldState {
        is_level_loaded: false,
        potion_count: 0,
        potion_inventory: 0,
        player_health: PLAYER_MAX_HEALTH,
    };
    reset_level_event.send(ResetLevel::default());
}

fn setup_item(
    mut level_state: ResMut<GameWorldState>,
    mut query: Query<&mut TextureAtlasSprite, Added<Item>>,
) {
    let mut rng = thread_rng();
    for mut sprite in query.iter_mut() {
        sprite.index = rng.gen_range(35, 37);

        level_state.potion_count += 1;
        if !level_state.is_level_loaded {
            level_state.is_level_loaded = true;
        }
    }
}

fn change_level(
    mut game_world_state: ResMut<GameWorldState>,
    query: Query<Entity, With<Item>>,
    mut increment_level: EventWriter<IncrementLevel>,
) {
    if query.is_empty() && game_world_state.is_level_loaded {
        game_world_state.is_level_loaded = false;
        increment_level.send(IncrementLevel::default());
    }
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
            game_state.set(GameState::MainMenu).unwrap();
        }
    }
}
