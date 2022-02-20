use bevy::prelude::*;
use heron::prelude::*;

use super::components::{ColliderBundle, Enemy, ProjectileBundle};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_shoot);
    }
}

fn on_shoot(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    query: Query<&Transform, With<Enemy>>,
) {
    if !keyboard_input.just_released(KeyCode::Return) {
        return;
    }

    let rotation_constraints = RotationConstraints::lock();
    for transform in query.iter() {
        commands
            .spawn_bundle(ProjectileBundle {
                sprite_bundle: SpriteBundle {
                    texture: asset_server.load("energy-star.png"),
                    transform: transform.clone(),
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
            })
            .insert(Velocity::from_linear(
                Vec3::new(1., 1., 0.).normalize_or_zero() * 100.,
            ));
    }
}
