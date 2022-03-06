use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin};
use big_brain::BigBrainPlugin;
use heron::prelude::*;

mod camera;
mod game;
mod game_over;
mod levels;
mod main_menu;
mod texture;
mod types;

pub struct StartGameTimer(Timer);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Obscure Resurrection".to_string(),
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.098, 0.078, 0.169)))
        .add_state(types::GameState::MainMenu)
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(BigBrainPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(texture::textures::TexturePlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(game_over::GameOverPlugin)
        .add_plugin(levels::LevelsPlugin)
        .add_plugin(game::game::GamePlugin)
        .add_startup_system(play_background_audio)
        .run();
}

fn play_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(asset_server.load("audio/background_music.wav"));
}
