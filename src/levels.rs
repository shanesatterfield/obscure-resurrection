use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::Index(0))
            .add_plugin(LdtkPlugin)
            .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels.ldtk"),
        ..Default::default()
    });
}
