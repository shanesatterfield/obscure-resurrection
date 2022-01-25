use bevy::prelude::*;

mod camera;
mod components;
mod config;
mod game;
mod movement;
mod textures;
mod types;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.098, 0.078, 0.169)))
        .add_state(types::GameState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(textures::TexturePlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(game::game::GamePlugin)
        .run();
}
