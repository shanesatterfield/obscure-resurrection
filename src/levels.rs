use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::types::GameState;

#[derive(Default, Clone, Debug)]
pub struct ResetLevel;

#[derive(Default, Clone, Debug)]
pub struct IncrementLevel;

pub struct LevelState {
    pub current_level: usize,
    pub max_levels: usize,
}

pub struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::Index(0))
            .insert_resource(LevelState {
                current_level: 0,
                max_levels: 3,
            })
            .add_event::<IncrementLevel>()
            .add_event::<ResetLevel>()
            .add_plugin(LdtkPlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::InGame)
                    .with_system(setup.label("level_setup"))
                    .with_system(reset_level.after("level_setup")),
            )
            .add_system_set(SystemSet::on_update(GameState::InGame))
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cleanup))
            .add_system(reset_level)
            .add_system(change_level);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("levels.ldtk"),
        ..Default::default()
    });
}

fn reset_level(
    mut level_selection: ResMut<LevelSelection>,
    mut level_state: ResMut<LevelState>,
    mut increment_level_event: EventReader<ResetLevel>,
) {
    for _ in increment_level_event.iter() {
        let next_level = 0;
        level_state.current_level = next_level;
        *level_selection = LevelSelection::Index(next_level);
    }
}

fn change_level(
    mut level_selection: ResMut<LevelSelection>,
    mut level_state: ResMut<LevelState>,
    mut increment_level_event: EventReader<IncrementLevel>,
) {
    for _ in increment_level_event.iter() {
        let next_level = (level_state.current_level + 1) % level_state.max_levels;
        level_state.current_level = next_level;
        *level_selection = LevelSelection::Index(next_level);
    }
}

fn cleanup(mut commands: Commands, world_query: Query<Entity, With<LevelSet>>) {
    if let Ok(ldtk_world_entity) = world_query.get_single() {
        commands.entity(ldtk_world_entity).despawn_recursive();
    }
}
