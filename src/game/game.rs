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

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::InGame)
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

fn setup_item(mut query: Query<&mut TextureAtlasSprite, Added<Item>>) {
    let mut rng = thread_rng();
    for mut sprite in query.iter_mut() {
        sprite.index = rng.gen_range(35, 37);
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
