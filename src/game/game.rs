extern crate rand;

use bevy::prelude::*;

use rand::thread_rng;
use rand::Rng;

use crate::camera::CameraFollowing;
use crate::components::Velocity;
use crate::config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::game::components::Player;
use crate::textures::Textures;
use crate::types::GameState;

use super::components::Enemy;
use super::components::Item;
use super::player::PlayerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::InGame)
                    .with_system(spawn_player)
                    .with_system(spawn_items),
                // .with_system(spawn_enemies),
            )
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cleanup));
    }
}

fn spawn_player(mut commands: Commands, textures: Res<Textures>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(81),
            ..Default::default()
        })
        .insert(Player::default())
        .insert(CameraFollowing::default())
        .insert(Velocity::new(Vec3::new(0., 0., 0.), 50.));
}

fn spawn_enemies(mut commands: Commands, textures: Res<Textures>) {
    let mut rng = thread_rng();
    for _ in 0..100 {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: textures.texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(rng.gen_range(1, 95)),
                transform: Transform::from_xyz(
                    rng.gen_range(-WINDOW_WIDTH, WINDOW_WIDTH),
                    rng.gen_range(-WINDOW_HEIGHT, WINDOW_HEIGHT),
                    0.,
                ),
                ..Default::default()
            })
            .insert(Enemy::default())
            .insert(Velocity::new(
                Vec3::new(
                    rng.gen_range(-WINDOW_WIDTH / 2., WINDOW_WIDTH / 2.),
                    rng.gen_range(-WINDOW_HEIGHT / 2., WINDOW_HEIGHT / 2.),
                    0.,
                ),
                rng.gen_range(10., 200.),
            ));
    }
}

fn spawn_items(mut commands: Commands, textures: Res<Textures>) {
    let mut rng = thread_rng();
    for _ in 0..100 {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: textures.texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(rng.gen_range(35, 37)),
                transform: Transform::from_xyz(
                    rng.gen_range(-WINDOW_WIDTH, WINDOW_WIDTH),
                    rng.gen_range(-WINDOW_HEIGHT, WINDOW_HEIGHT),
                    0.,
                ),
                ..Default::default()
            })
            .insert(Item::default());
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
