use bevy::prelude::*;
use heron::prelude::*;

use crate::types::GameState;

use super::components::{
    BorkBundle, ColliderBundle, GameCollisionLayers, Player, Speed, TimeToLive,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, move_player.label("apply_movement"))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(setup_player)
                    .with_system(bork),
            );
    }
}

fn setup_player(mut query: Query<&mut Speed, Added<Player>>) {
    for mut speed in query.iter_mut() {
        speed.0 = 50.;
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
        *velocity = Velocity::from_linear(direction.extend(0.).normalize_or_zero() * speed.0);
    }
}

fn bork(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<Player>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    for entity in query.iter() {
        let child = commands
            .spawn_bundle(BorkBundle {
                ttl: TimeToLive(Timer::from_seconds(1., false)),

                sprite_bundle: SpriteBundle {
                    texture: asset_server.load("projectiles/bork_3.png"),
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
    }
}
