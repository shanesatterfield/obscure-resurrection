use crate::camera::CameraFollowing;
use crate::game::components::*;
use crate::game::enemy::components::Aggroable;
use crate::texture::components::FacingDirection;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use heron::prelude::*;

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

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct CoinBundle {
    pub coin: Coin,

    #[sprite_sheet_bundle]
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,

    #[from_entity_instance]
    #[bundle]
    pub collider_bundle: ColliderBundle,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct StairsBundle {
    pub stairs: Stairs,

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

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: CollisionShape,
    pub rigid_body: RigidBody,
    pub collision_layers: CollisionLayers,
    pub velocity: Velocity,
    pub rotation_constraints: RotationConstraints,
}
