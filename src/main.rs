use bevy::prelude::*;
use heron::prelude::*;

mod camera;
mod components;
mod config;
mod game;
mod levels;
mod movement;
mod textures;
mod types;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.098, 0.078, 0.169)))
        .add_state(types::GameState::Loading)
        .add_system_set(
            SystemSet::on_update(types::GameState::Loading).with_system(transition_to_game),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(textures::TexturePlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(levels::LevelsPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(game::game::GamePlugin)
        .run();
}

fn transition_to_game(mut game_state: ResMut<State<types::GameState>>) {
    game_state.set(types::GameState::InGame).unwrap();
}
