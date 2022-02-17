use bevy::prelude::*;

use crate::types::GameState;

pub struct TexturePlugin;

#[derive(Default)]
pub struct Textures {
    pub texture_atlas_handle: Handle<TextureAtlas>,
}

impl Plugin for TexturePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Textures::default())
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(load_assets));
    }
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Textures>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("oracle_1bit_assets/8x8.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 8, 12);
    textures.texture_atlas_handle = texture_atlases.add(texture_atlas);
}
