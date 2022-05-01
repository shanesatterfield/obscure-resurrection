use bevy::prelude::*;
use big_brain::BigBrainPlugin;
use heron::prelude::*;

use bevy_asset_loader::AssetLoader;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use types::{AudioAssets, FontAssets, ImageAssets, LevelAssets};

mod camera;
mod game;
mod game_over;
mod levels;
mod main_menu;
mod texture;
mod types;

fn main() {
    let mut app = App::new();

    // Load all assets before the game starts up
    AssetLoader::new(types::GameState::Loading)
        .continue_to_state(types::GameState::MainMenu)
        .with_collection::<AudioAssets>()
        .with_collection::<ImageAssets>()
        .with_collection::<FontAssets>()
        .with_collection::<LevelAssets>()
        .build(&mut app);

    app.insert_resource(WindowDescriptor {
        title: "Obscure Resurrection".to_string(),
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.098, 0.078, 0.169)))
    .add_state(types::GameState::Loading)
    // Embed assets into the binary
    .add_plugins_with(DefaultPlugins, |group| {
        group.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
    })
    .add_plugin(PhysicsPlugin::default())
    .add_plugin(BigBrainPlugin)
    .add_plugin(texture::plugin::TexturePlugin)
    .add_plugin(camera::CameraPlugin)
    .add_plugin(main_menu::MainMenuPlugin)
    .add_plugin(game_over::GameOverPlugin)
    .add_plugin(levels::LevelsPlugin)
    .add_plugin(game::game::GamePlugin)
    .add_system_set(
        SystemSet::on_enter(types::GameState::MainMenu).with_system(play_background_audio),
    )
    .run();
}

fn play_background_audio(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play_with_settings(
        audio_assets.background_music.clone(),
        PlaybackSettings::LOOP,
    );
}
