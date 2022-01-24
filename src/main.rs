use bevy::prelude::*;

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
        .add_plugin(movement::MovementPlugin)
        .add_plugin(game::game::GamePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    // Set the scale on the window right away
    // let main_window = windows.get_primary().unwrap();
    // camera.orthographic_projection.scale = config::WINDOW_WIDTH / main_window.width();
    camera.orthographic_projection.scale = 1. / 4.;

    // commands.spawn_bundle(camera).insert(ScalingCamera);
    commands.spawn_bundle(camera);
    // commands.spawn_bundle(UiCameraBundle::default());
}
