extern crate rand;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use rand::thread_rng;
use rand::Rng;

use crate::game::components::Player;
use crate::levels::IncrementLevel;
use crate::types::GameState;

use super::components::EnemyBundle;
use super::components::Item;
use super::components::PlayerBundle;
use super::components::PotionBundle;
use super::components::TimeToLive;
use super::components::WallBundle;
use super::enemy::enemy::EnemyPlugin;
use super::player::PlayerPlugin;

pub struct GamePlugin;

pub struct GameWorldState {
    pub is_level_loaded: bool,
    pub potion_count: u32,
    pub potion_inventory: u32,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .insert_resource(GameWorldState {
                is_level_loaded: false,
                potion_count: 0,
                potion_inventory: 0,
            })
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(change_level)
                    .with_system(setup_item)
                    .with_system(time_to_live_system),
            )
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<PotionBundle>("Potion")
            .register_ldtk_entity::<WallBundle>("Wall")
            .register_ldtk_entity::<EnemyBundle>("Enemy")
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cleanup));
    }
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

fn cleanup(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
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
