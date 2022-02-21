use bevy::prelude::*;
use heron::prelude::*;

use crate::{game::components::GameCollisionLayers, types::GameState};

use super::{
    super::components::{ColliderBundle, Enemy, Player, ProjectileBundle, TimeToLive},
    components::Attacking,
    shaman_ai::ShamanAi,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ShamanAi)
            .add_system_set(SystemSet::on_update(GameState::InGame).with_system(on_shoot));
    }
}

fn on_shoot(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<Player>>,
    mut query: Query<(&Transform, &mut Attacking), With<Enemy>>,
) {
    let rotation_constraints = RotationConstraints::allow();
    if let Ok(player_transform) = player_query.get_single() {
        for (transform, mut attacking) in query.iter_mut() {
            // Only shoot when the cooldown is over
            if !attacking.is_attacking || !attacking.timer.tick(time.delta()).just_finished() {
                continue;
            }

            let direction =
                (player_transform.translation - transform.translation).normalize_or_zero();

            // Make sure that the projectiles spawn outside of the body so that it doesn't collide
            let beyond_body_diff = direction * 8.;
            let mut new_transform = transform.clone();
            new_transform.translation = transform.translation + beyond_body_diff;

            commands
                .spawn_bundle(ProjectileBundle {
                    sprite_bundle: SpriteBundle {
                        texture: asset_server.load("projectiles/energy_star.png"),
                        transform: new_transform,
                        ..Default::default()
                    },

                    collider_bundle: ColliderBundle {
                        collider: CollisionShape::Cuboid {
                            half_extends: Vec3::new(4., 4., 0.),
                            border_radius: None,
                        },
                        collision_layers: CollisionLayers::none()
                            .with_group(GameCollisionLayers::EnemyAttack)
                            .with_masks(&[
                                GameCollisionLayers::World,
                                GameCollisionLayers::Player,
                                GameCollisionLayers::PlayerAttack,
                            ]),
                        rigid_body: RigidBody::Dynamic,
                        rotation_constraints,
                        ..Default::default()
                    },

                    ttl: TimeToLive(Timer::from_seconds(0.5, false)),
                })
                .insert(PhysicMaterial {
                    restitution: 0.7,
                    density: 1.,
                    friction: 0.5,
                })
                .insert(Velocity::from_linear(direction * 150.));
        }
    }
}
