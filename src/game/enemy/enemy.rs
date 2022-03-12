use bevy::prelude::*;
use big_brain::prelude::FirstToScore;
use big_brain::prelude::Thinker;
use heron::prelude::*;

use crate::game::level::components::ColliderBundle;
use crate::game::level::components::ProjectileBundle;
use crate::types::ImageAssets;
use crate::{game::components::GameCollisionLayers, types::GameState};

use super::{
    super::components::{Enemy, Player, TimeToLive},
    components::{Aggroable, Aggroed, AttackPlayer, Attacking},
    shaman_ai::ShamanAi,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ShamanAi).add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(setup_enemy)
                .with_system(on_shoot),
        );
    }
}

fn setup_enemy(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TextureAtlasSprite, &mut Aggroable), Added<Enemy>>,
) {
    for (entity, mut sprite, mut aggroable) in query.iter_mut() {
        sprite.index = 89;
        aggroable.distance = 100.;
        commands
            .entity(entity)
            .insert(Attacking {
                timer: Timer::from_seconds(1., true),
                is_attacking: false,
            })
            .insert(
                Thinker::build()
                    .picker(FirstToScore { threshold: 0.8 })
                    .when(Aggroed, AttackPlayer),
            );
    }
}

fn on_shoot(
    mut commands: Commands,
    time: Res<Time>,
    image_assets: Res<ImageAssets>,
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
                        texture: image_assets.energy_star.clone(),
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
