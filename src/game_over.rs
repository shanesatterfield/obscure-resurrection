use bevy::prelude::*;

use crate::{game::game::GameWorldState, types::GameState};

#[derive(Component, Default, Clone, Debug)]
pub struct OnlyInGameOver;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(spawn_ui))
            .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(load_game))
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(cleanup));
    }
}

fn spawn_ui(
    mut commands: Commands,
    game_world_state: Res<GameWorldState>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(OnlyInGameOver)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    ..Default::default()
                },
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Game Over\n".to_string(),
                            style: TextStyle {
                                font: asset_server.load("ConsolaMono.ttf"),
                                font_size: 120.,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: format!("You collected {} coins\n", game_world_state.coins,)
                                .to_string(),
                            style: TextStyle {
                                font: asset_server.load("ConsolaMono.ttf"),
                                font_size: 80.,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: format!("In {} seconds\n", game_world_state.play_time as u32,)
                                .to_string(),
                            style: TextStyle {
                                font: asset_server.load("ConsolaMono.ttf"),
                                font_size: 80.,
                                color: Color::WHITE,
                            },
                        },
                        TextSection {
                            value: "Press Enter to Restart".to_string(),
                            style: TextStyle {
                                font: asset_server.load("ConsolaMono.ttf"),
                                font_size: 80.,
                                color: Color::WHITE,
                            },
                        },
                    ],
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn load_game(keyboard_input: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if keyboard_input.just_released(KeyCode::Space) || keyboard_input.just_released(KeyCode::Return)
    {
        game_state.set(GameState::InGame).unwrap();
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<OnlyInGameOver>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
