use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use heron::prelude::*;

use super::components::{
    BorkBundle, ColliderBundle, GameCollisionLayers, Item, Player, Speed, TimeToLive,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, move_player.label("apply_movement"))
            .add_system(pick_up_item)
            .add_system(bork);
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

fn pick_up_item(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    item_query: Query<(Entity, &Transform), With<Item>>,
) {
    let sprite_sizer = Vec2::new(8., 8.);
    for player_transform in player_query.iter() {
        let player = player_transform.translation;
        for (entity, transform) in item_query.iter() {
            if let Some(_) = collide(player, sprite_sizer, transform.translation, sprite_sizer) {
                commands.entity(entity).despawn();
            }
        }
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
                    collider: CollisionShape::Cuboid {
                        half_extends: Vec3::new(16., 16., 0.),
                        border_radius: None,
                    },
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
