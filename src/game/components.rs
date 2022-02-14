use crate::camera::CameraFollowing;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use heron::prelude::*;

#[derive(Component, Default, Clone)]
pub struct Player;

#[derive(Component, Default, Clone)]
pub struct Enemy;

#[derive(Component, Default, Clone)]
pub struct Item;

#[derive(Clone, Debug, Default, Component)]
pub struct Wall;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    pub camera_following: CameraFollowing,

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

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: CollisionShape,
    pub rigid_body: RigidBody,
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
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                ..Default::default()
            },
            // "Enemy" => ColliderBundle {
            // collider: CollisionShape::Cuboid {
            // half_extends: Vec3::new(5., 5., 0.),
            // border_radius: None,
            // },
            // rigid_body: RigidBody::KinematicVelocityBased,
            // rotation_constraints,
            // ..Default::default()
            // },
            "Potion" => ColliderBundle {
                collider: CollisionShape::Cuboid {
                    half_extends: Vec3::new(8., 8., 0.),
                    border_radius: None,
                },
                rigid_body: RigidBody::Sensor,
                rotation_constraints,
                ..Default::default()
            },
            "Wall" => {
                let width_field = entity_instance
                    .field_instances
                    .iter()
                    .find(|f| f.identifier == "width".to_string())
                    .unwrap();

                let mut width: i32 = 1;
                if let FieldValue::Int(width_value) = &width_field.value {
                    width = width_value.unwrap();
                }

                let height_field = entity_instance
                    .field_instances
                    .iter()
                    .find(|f| f.identifier == "height".to_string())
                    .unwrap();

                let mut height: i32 = 1;
                if let FieldValue::Int(height_value) = &height_field.value {
                    height = height_value.unwrap();
                }

                let width: f32 = 4. + (8 * (width - 1)) as f32;
                let height: f32 = 4. + (8 * (height - 1)) as f32;

                let rotation_constraints = RotationConstraints::lock();
                return ColliderBundle {
                    collider: CollisionShape::Cuboid {
                        half_extends: Vec3::new(width, height, 0.),
                        border_radius: None,
                    },
                    rigid_body: RigidBody::Static,
                    rotation_constraints,
                    ..Default::default()
                };
            }
            _ => ColliderBundle::default(),
        }
    }
}
