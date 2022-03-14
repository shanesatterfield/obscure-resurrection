use bevy::prelude::*;

use crate::{
    camera::WINDOW_SCALE,
    types::{GameState, ImageAssets},
};

use super::{
    events::PlayerDamaged,
    game::{GameWorldState, PLAYER_MAX_HEALTH},
};

const MAX_UI_DIGITS: usize = 4;

#[derive(Component, Default, Clone, Debug)]
pub struct GameUi;

#[derive(Component, Default, Clone, Debug)]
pub struct UiElementIndex(usize);

#[derive(Component, Default, Clone, Debug)]
pub struct HealthContainerImage;

#[derive(Component, Default, Clone, Debug)]
pub struct BorkPointNumber;

#[derive(Component, Default, Clone, Debug)]
pub struct CoinNumber;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(spawn_ui))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(update_health_containers.after("damage_calculation"))
                    .with_system(update_potion_counter)
                    .with_system(update_coin_counter),
            )
            .add_system_set(SystemSet::on_exit(GameState::InGame).with_system(cleanup));
    }
}

fn spawn_ui(mut commands: Commands, image_assets: Res<ImageAssets>) {
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
                    // Hearth Points Counter
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
                                image: image_assets.heart.clone().into(),
                                ..Default::default()
                            })
                            .insert(UiElementIndex(index as usize))
                            .insert(HealthContainerImage::default());
                    }

                    // Bork Points Counter
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(8. * WINDOW_SCALE), Val::Px(8. * WINDOW_SCALE)),
                            ..Default::default()
                        },
                        image: image_assets.potion.clone().into(),
                        ..Default::default()
                    });
                    for index in 1..=MAX_UI_DIGITS {
                        parent
                            .spawn_bundle(ImageBundle {
                                style: Style {
                                    size: Size::new(
                                        Val::Px(8. * WINDOW_SCALE),
                                        Val::Px(8. * WINDOW_SCALE),
                                    ),
                                    ..Default::default()
                                },
                                image: image_assets.text0.clone().into(),
                                ..Default::default()
                            })
                            .insert(UiElementIndex(MAX_UI_DIGITS + 1 - index))
                            .insert(BorkPointNumber::default());
                    }

                    // Coin Counter
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(8. * WINDOW_SCALE), Val::Px(8. * WINDOW_SCALE)),
                            ..Default::default()
                        },
                        image: image_assets.coin.clone().into(),
                        ..Default::default()
                    });
                    for index in 1..=MAX_UI_DIGITS {
                        parent
                            .spawn_bundle(ImageBundle {
                                style: Style {
                                    size: Size::new(
                                        Val::Px(8. * WINDOW_SCALE),
                                        Val::Px(8. * WINDOW_SCALE),
                                    ),
                                    ..Default::default()
                                },
                                image: image_assets.text0.clone().into(),
                                ..Default::default()
                            })
                            .insert(UiElementIndex(MAX_UI_DIGITS + 1 - index))
                            .insert(CoinNumber::default());
                    }
                });
        });
}

fn update_health_containers(
    game_world_state: Res<GameWorldState>,
    image_assets: Res<ImageAssets>,
    mut query: Query<(&mut UiImage, &UiElementIndex), With<HealthContainerImage>>,
    mut event_reader: EventReader<PlayerDamaged>,
) {
    for _ in event_reader.iter() {
        for (mut image, element_index) in query.iter_mut() {
            if element_index.0 as u32 > game_world_state.player_health {
                *image = image_assets.empty_heart.clone().into();
            } else {
                *image = image_assets.heart.clone().into();
            }
        }
    }
}

fn update_potion_counter(
    game_world_state: Res<GameWorldState>,
    image_assets: Res<ImageAssets>,
    mut query: Query<(&mut UiImage, &UiElementIndex), With<BorkPointNumber>>,
) {
    let digits: Vec<u32> = game_world_state
        .bork_points
        .to_string()
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    for (mut image, element_index) in query.iter_mut() {
        let index = element_index.0;
        if index >= 1 && index <= MAX_UI_DIGITS {
            *image = number_to_image(image_assets.clone(), digits.get(index - 1)).into();
        }
    }
}

fn update_coin_counter(
    game_world_state: Res<GameWorldState>,
    image_assets: Res<ImageAssets>,
    mut query: Query<(&mut UiImage, &UiElementIndex), With<CoinNumber>>,
) {
    let digits: Vec<u32> = game_world_state
        .coins
        .to_string()
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    for (mut image, element_index) in query.iter_mut() {
        let index = element_index.0;
        if index >= 1 && index <= MAX_UI_DIGITS {
            *image = number_to_image(image_assets.clone(), digits.get(index - 1)).into();
        }
    }
}

fn number_to_image(image_assets: ImageAssets, num: Option<&u32>) -> Handle<Image> {
    let default: u32 = 0;
    let num = num.unwrap_or(&default);
    match num {
        0 => image_assets.text0.clone(),
        1 => image_assets.text1.clone(),
        2 => image_assets.text2.clone(),
        3 => image_assets.text3.clone(),
        4 => image_assets.text4.clone(),
        5 => image_assets.text5.clone(),
        6 => image_assets.text6.clone(),
        7 => image_assets.text7.clone(),
        8 => image_assets.text8.clone(),
        9 => image_assets.text9.clone(),
        _ => image_assets.text0.clone(),
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<GameUi>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
