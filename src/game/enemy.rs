use bevy::prelude::*;
use heron::prelude::*;

use crate::types::GameState;

use super::components::{ColliderBundle, Enemy, Player, ProjectileBundle, TimeToLive};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::InGame).with_system(on_shoot));
    }
}

fn on_shoot(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<Player>>,
    query: Query<&Transform, With<Enemy>>,
) {
    if !keyboard_input.just_released(KeyCode::Return) {
        return;
    }

    let rotation_constraints = RotationConstraints::lock();
    if let Ok(player_transform) = player_query.get_single() {
        for transform in query.iter() {
            let direction =
                (player_transform.translation - transform.translation).normalize_or_zero();

            // Make sure that the projectiles spawn outside of the body so that it doesn't collide
            let beyond_body_diff = direction * 8.;
            let mut new_transform = transform.clone();
            new_transform.translation = transform.translation + beyond_body_diff;

            commands
                .spawn_bundle(ProjectileBundle {
                    sprite_bundle: SpriteBundle {
                        texture: asset_server.load("energy-star.png"),
                        transform: new_transform,
                        ..Default::default()
                    },

                    collider_bundle: ColliderBundle {
                        collider: CollisionShape::Cuboid {
                            half_extends: Vec3::new(4., 4., 0.),
                            border_radius: None,
                        },
                        rigid_body: RigidBody::KinematicVelocityBased,
                        rotation_constraints,
                        ..Default::default()
                    },

                    ttl: TimeToLive(Timer::from_seconds(0.75, false)),
                })
                .insert(Velocity::from_linear(direction * 100.));
        }
    }
}
