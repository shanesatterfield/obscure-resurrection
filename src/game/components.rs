use crate::camera::CameraFollowing;
use crate::texture::components::FacingDirection;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use heron::prelude::*;

#[derive(Component, Default, Clone)]
pub struct Player;

#[derive(Component, Default, Clone)]
pub struct Enemy;

#[derive(Component, Default, Clone)]
pub struct Item;

#[derive(Component, Clone, Debug, Default)]
pub struct Wall;

#[derive(Component, Default, Clone)]
pub struct Direction(pub Vec2);

#[derive(Component, Default, Clone)]
pub struct Speed(pub f32);

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub player: Player,
    pub camera_following: CameraFollowing,
    pub facing_direction: FacingDirection,
    pub direction: Direction,
    pub speed: Speed,

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
