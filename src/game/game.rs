extern crate rand;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use heron::prelude::*;
use rand::thread_rng;
use rand::Rng;

use crate::game::components::Player;
use crate::types::GameState;

use super::components::Item;
use super::components::PlayerBundle;
use super::components::PotionBundle;
use super::components::WallBundle;
use super::player::PlayerPlugin;

pub struct GamePlugin;

pub struct LevelState {
    pub current_level: usize,
    pub max_levels: usize,
    pub is_level_loaded: bool,
    pub potion_count: u32,
    pub potion_inventory: u32,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .insert_resource(LevelState {
                current_level: 0,
                max_levels: 2,
                is_level_loaded: false,
                potion_count: 0,
                potion_inventory: 0,
            })
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(change_level)
                    .with_system(setup_player)
                    .with_system(setup_item),
            )
            .register_ldtk_entity::<PlayerBundle>("Player")
            .register_ldtk_entity::<PotionBundle>("Potion")
            .register_ldtk_entity::<WallBundle>("Wall")
            .add_system(log_collisions)
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cleanup));
    }
}

fn setup_player(mut query: Query<&mut TextureAtlasSprite, Added<Player>>) {
    for mut sprite in query.iter_mut() {
        sprite.index = 81;
    }
}

fn setup_item(
    mut level_state: ResMut<LevelState>,
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

fn log_collisions(mut events: EventReader<CollisionEvent>) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(a, b) => {
                println!(
                    "Entity {:?} and {:?} started to collide",
                    a.rigid_body_entity(),
                    b.rigid_body_entity(),
                )
            }
            CollisionEvent::Stopped(a, b) => {
                println!(
                    "Entity {:?} and {:?} stopped colliding",
                    a.rigid_body_entity(),
                    b.rigid_body_entity(),
                )
            }
        }
    }
}

fn change_level(
    mut level_selection: ResMut<LevelSelection>,
    mut level_state: ResMut<LevelState>,
    query: Query<Entity, With<Item>>,
) {
    let mut item_count = 0;
    for _ in query.iter() {
        item_count += 1;
    }

    if item_count == 0 && level_state.is_level_loaded {
        level_state.is_level_loaded = false;
        let next_level = (level_state.current_level + 1) % level_state.max_levels;
        level_state.current_level = next_level;
        *level_selection = LevelSelection::Index(next_level);
    }
}
