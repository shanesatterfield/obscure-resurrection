use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use heron::prelude::*;

use crate::game::components::GameCollisionLayers;

use super::components::ColliderBundle;

impl From<EntityInstance> for ColliderBundle {
    fn from(entity_instance: EntityInstance) -> ColliderBundle {
        let rotation_constraints = RotationConstraints::lock();

        match entity_instance.identifier.as_ref() {
            "Player" => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(4., 4., 0.),
                    border_radius: None,
                },
                collision_layers: CollisionLayers::none()
                    .with_group(GameCollisionLayers::Player)
                    .with_masks(&[
                        GameCollisionLayers::World,
                        GameCollisionLayers::Enemy,
                        GameCollisionLayers::EnemyAttack,
                        GameCollisionLayers::Item,
                        GameCollisionLayers::Coin,
                        GameCollisionLayers::Stairs,
                    ]),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                ..Default::default()
            },
            "Enemy" => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(4., 4., 0.),
                    border_radius: None,
                },
                collision_layers: CollisionLayers::none()
                    .with_group(GameCollisionLayers::Enemy)
                    .with_masks(&[
                        GameCollisionLayers::World,
                        GameCollisionLayers::Player,
                        GameCollisionLayers::PlayerAttack,
                    ]),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                ..Default::default()
            },
            "Potion" => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(4., 4., 0.),
                    border_radius: None,
                },
                collision_layers: CollisionLayers::none()
                    .with_group(GameCollisionLayers::Item)
                    .with_mask(GameCollisionLayers::Player),
                rigid_body: RigidBody::Sensor,
                rotation_constraints,
                ..Default::default()
            },
            "Coin" => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(4., 4., 0.),
                    border_radius: None,
                },
                collision_layers: CollisionLayers::none()
                    .with_group(GameCollisionLayers::Coin)
                    .with_mask(GameCollisionLayers::Player),
                rigid_body: RigidBody::Sensor,
                rotation_constraints,
                ..Default::default()
            },
            "Stairs" => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(4., 4., 0.),
                    border_radius: None,
                },
                collision_layers: CollisionLayers::none()
                    .with_group(GameCollisionLayers::Stairs)
                    .with_mask(GameCollisionLayers::Player),
                rigid_body: RigidBody::Sensor,
                rotation_constraints,
                ..Default::default()
            },
            "Wall" => {
                let left = get_int_from_ldtk(&entity_instance, "left");
                let right = get_int_from_ldtk(&entity_instance, "right");
                let top = get_int_from_ldtk(&entity_instance, "top");
                let bottom = get_int_from_ldtk(&entity_instance, "bottom");

                let left: f32 = (4. + (8 * (left - 1)) as f32) * -1.;
                let right: f32 = 4. + (8 * (right - 1)) as f32;
                let top: f32 = 4. + (8 * (top - 1)) as f32;
                let bottom: f32 = (4. + (8 * (bottom - 1)) as f32) * -1.;

                let rotation_constraints = RotationConstraints::lock();
                return ColliderBundle {
                    collider: CollisionShape::ConvexHull {
                        points: vec![
                            Vec3::new(left, top, 0.),
                            Vec3::new(right, top, 0.),
                            Vec3::new(right, bottom, 0.),
                            Vec3::new(left, bottom, 0.),
                        ],
                        border_radius: None,
                    },
                    collision_layers: CollisionLayers::none()
                        .with_group(GameCollisionLayers::World)
                        .with_masks(&[
                            GameCollisionLayers::Player,
                            GameCollisionLayers::PlayerAttack,
                            GameCollisionLayers::Enemy,
                            GameCollisionLayers::EnemyAttack,
                        ]),
                    rigid_body: RigidBody::Static,
                    rotation_constraints,
                    ..Default::default()
                };
            }
            _ => ColliderBundle::default(),
        }
    }
}

pub fn get_int_from_ldtk(entity_instance: &EntityInstance, field_name: &str) -> i32 {
    let field = entity_instance
        .field_instances
        .iter()
        .find(|f| f.identifier == field_name)
        .unwrap();

    let mut result: i32 = 1;
    if let FieldValue::Int(value) = &field.value {
        result = value.unwrap();
    }
    return result;
}
