use bevy::prelude::*;
use heron::prelude::*;

use crate::types::{GameState, ImageAssets};

use super::{
    components::{Bork, GameCollisionLayers, Player, Speed, TimeToLive},
    events::PlayerBorked,
    game::GameWorldState,
    level::components::{BorkBundle, ColliderBundle},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(
                    move_player
                        .before(PhysicsSystem::TransformUpdate)
                        .before(PhysicsSystem::VelocityUpdate),
                )
                .with_system(setup_player)
                .with_system(bork)
                .with_system(is_borking),
        );
    }
}

fn setup_player(mut query: Query<&mut Speed, Added<Player>>) {
    for mut speed in query.iter_mut() {
        speed.0 = 100.;
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Speed, &mut Velocity), With<Player>>,
) {
    let mut direction = Vec2::default();
    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up) {
        direction.y = 1.;
    } else if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
        direction.y = -1.;
    }

    if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
        direction.x = 1.;
    } else if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
        direction.x = -1.;
    }

    for (speed, mut velocity) in query.iter_mut() {
        let new_velocity =
            Velocity::from_linear(direction.extend(0.).normalize_or_zero() * speed.0);
        if velocity.linear != new_velocity.linear {
            *velocity = new_velocity;
        }
    }
}

fn bork(
    mut commands: Commands,
    mut game_world_state: ResMut<GameWorldState>,
    keyboard_input: Res<Input<KeyCode>>,
    image_assets: Res<ImageAssets>,
    query: Query<Entity, With<Player>>,
    mut event_writer: EventWriter<PlayerBorked>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    // Not enough bork points to bork!
    if game_world_state.bork_points == 0 {
        return;
    }

    for entity in query.iter() {
        // Check again to make sure that it's still possible to bork
        if game_world_state.bork_points == 0 {
            return;
        }

        // Use up a bork point
        game_world_state.bork_points -= 1;

        // Spawn the bork as a child of the player
        let child = commands
            .spawn_bundle(BorkBundle {
                bork: Bork::default(),
                ttl: TimeToLive(Timer::from_seconds(3., false)),

                sprite_bundle: SpriteBundle {
                    texture: image_assets.bork.clone(),
                    ..Default::default()
                },

                collider_bundle: ColliderBundle {
                    collider: CollisionShape::Sphere { radius: 16. },
                    collision_layers: CollisionLayers::none()
                        .with_group(GameCollisionLayers::PlayerAttack)
                        .with_masks(&[
                            GameCollisionLayers::Enemy,
                            GameCollisionLayers::EnemyAttack,
                        ]),
                    rigid_body: RigidBody::KinematicPositionBased,
                    rotation_constraints: RotationConstraints::lock(),
                    ..Default::default()
                },
            })
            .insert(PhysicMaterial {
                restitution: 0.7,
                density: 1.,
                ..Default::default()
            })
            .id();

        commands.entity(entity).push_children(&[child]);

        event_writer.send(PlayerBorked::default());
    }
}

fn is_borking(mut game_world_state: ResMut<GameWorldState>, query: Query<Entity, With<Bork>>) {
    game_world_state.is_borking = query.iter().count() > 0;
}
