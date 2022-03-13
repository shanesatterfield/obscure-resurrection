use crate::texture::systems::{change_direction, flip_assets, set_texture_filters_to_nearest};
use bevy::prelude::*;

pub struct TexturePlugin;

impl Plugin for TexturePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(change_direction)
            .add_system(set_texture_filters_to_nearest)
            .add_system(flip_assets);
    }
}
