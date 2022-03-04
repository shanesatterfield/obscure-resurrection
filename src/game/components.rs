use crate::camera::CameraFollowing;
use crate::texture::components::FacingDirection;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use heron::prelude::*;

use super::enemy::components::Aggroable;

#[derive(Component, Default, Clone)]
pub struct Player;

#[derive(Component, Default, Clone)]
pub struct Enemy;

#[derive(Component, Default, Clone)]
pub struct Item;

#[derive(Component, Clone, Debug, Default)]
pub struct Wall;

#[derive(Component, Default, Clone)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct TimeToLive(pub Timer);

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    pub camera_following: CameraFollowing,
    pub facing_direction: FacingDirection,
    pub speed: Speed,

    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite_bundle: SpriteSheetBundle,

    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub aggroable: Aggroable,
    pub facing_direction: FacingDirection,

    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite_bundle: SpriteSheetBundle,

    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
}

#[derive(Clone, Debug, Default, Bundle, LdtkEntity)]
pub struct WallBundle {
    wall: Wall,

    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PotionBundle {
    pub item: Item,

    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,

    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub ttl: TimeToLive,

    #[bundle]
    pub sprite_bundle: SpriteBundle,

    #[bundle]
    pub collider_bundle: ColliderBundle,
}

#[derive(Bundle)]
pub struct BorkBundle {
    pub ttl: TimeToLive,

    #[bundle]
    pub sprite_bundle: SpriteBundle,

    #[bundle]
    pub collider_bundle: ColliderBundle,
}

#[derive(PhysicsLayer)]
pub enum GameCollisionLayers {
    World,
    Player,
    PlayerAttack,
    Enemy,
    EnemyAttack,
    Item,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: CollisionShape,
    pub rigid_body: RigidBody,
    pub collision_layers: CollisionLayers,
    pub velocity: Velocity,
    pub rotation_constraints: RotationConstraints,
}

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
                    half_extends: Vec3::new(8., 8., 0.),
                    border_radius: None,
                },
                collision_layers: CollisionLayers::none()
                    .with_group(GameCollisionLayers::Item)
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

fn get_int_from_ldtk(entity_instance: &EntityInstance, field_name: &str) -> i32 {
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
