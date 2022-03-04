use bevy::prelude::*;

use crate::{camera::WINDOW_SCALE, types::GameState};

use super::{
    events::PlayerDamaged,
    game::{GameWorldState, PLAYER_MAX_HEALTH},
};

#[derive(Component, Default, Clone, Debug)]
pub struct GameUi;

#[derive(Component, Default, Clone, Debug)]
pub struct UiElementIndex(u32);

#[derive(Component, Default, Clone, Debug)]
pub struct HealthContainerImage;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(spawn_ui))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(update_health_containers.after("damage_calculation")),
            )
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cleanup));
    }
}

fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .insert(GameUi::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Px(10. * WINDOW_SCALE)),
                        padding: Rect {
                            left: Val::Px(1. * WINDOW_SCALE),
                            right: Val::Px(1. * WINDOW_SCALE),
                            top: Val::Px(1. * WINDOW_SCALE),
                            bottom: Val::Px(1. * WINDOW_SCALE),
                        },
                        ..Default::default()
                    },
                    color: Color::rgb(0.098, 0.078, 0.169).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    for index in 1..=PLAYER_MAX_HEALTH {
                        parent
                            .spawn_bundle(ImageBundle {
                                style: Style {
                                    size: Size::new(
                                        Val::Px(8. * WINDOW_SCALE),
                                        Val::Px(8. * WINDOW_SCALE),
                                    ),
                                    ..Default::default()
                                },
                                image: asset_server.load("individual_sprites/tile045.png").into(),
                                ..Default::default()
                            })
                            .insert(UiElementIndex(index))
                            .insert(HealthContainerImage::default());
                    }
                });
        });
}

fn update_health_containers(
    game_world_state: Res<GameWorldState>,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut UiImage, &UiElementIndex), With<HealthContainerImage>>,
    mut event_reader: EventReader<PlayerDamaged>,
) {
    for _ in event_reader.iter() {
        for (mut image, element_index) in query.iter_mut() {
            if element_index.0 > game_world_state.player_health {
                *image = asset_server.load("icons/empty_heart_container.png").into();
            } else {
                *image = asset_server.load("icons/heart.png").into();
            }
        }
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<GameUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
