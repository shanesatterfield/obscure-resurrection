use bevy::prelude::*;
use heron::prelude::Velocity;

use crate::types::GameState;

use super::components::{FacingDirection, HorizontalDirection};

pub struct TexturePlugin;

#[derive(Default)]
pub struct Textures {
    pub texture_atlas_handle: Handle<TextureAtlas>,
}

impl Plugin for TexturePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Textures::default())
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_assets))
            .add_system(change_direction)
            .add_system(flip_assets);
    }
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Textures>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("oracle_1bit_assets/8x8.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8., 8.), 8, 12);
    textures.texture_atlas_handle = texture_atlases.add(texture_atlas);
}

fn change_direction(mut query: Query<(&Velocity, &mut FacingDirection), Changed<Velocity>>) {
    for (velocity, mut facing_direction) in query.iter_mut() {
        if velocity.linear.x < 0. {
            facing_direction.0 = HorizontalDirection::LEFT;
        } else if velocity.linear.x > 0. {
            facing_direction.0 = HorizontalDirection::RIGHT;
        }
    }
}

fn flip_assets(mut query: Query<(&mut Sprite, &FacingDirection), Changed<FacingDirection>>) {
    for (mut sprite, facing_direction) in query.iter_mut() {
        match facing_direction.0 {
            HorizontalDirection::LEFT => {
                sprite.flip_x = true;
            }
            HorizontalDirection::RIGHT => {
                sprite.flip_x = false;
            }
        }
    }
}
