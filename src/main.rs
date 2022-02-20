use bevy::prelude::*;
use big_brain::BigBrainPlugin;
use heron::prelude::*;

mod camera;
mod game;
mod levels;
mod texture;
mod types;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Obscure Resurrection".to_string(),
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.098, 0.078, 0.169)))
        .add_state(types::GameState::Loading)
        .add_system_set(
            SystemSet::on_update(types::GameState::Loading).with_system(transition_to_game),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(BigBrainPlugin)
        .add_plugin(texture::textures::TexturePlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(levels::LevelsPlugin)
        .add_plugin(game::game::GamePlugin)
        .run();
}

fn transition_to_game(mut game_state: ResMut<State<types::GameState>>) {
    game_state.set(types::GameState::InGame).unwrap();
}
