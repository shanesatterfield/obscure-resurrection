use bevy::prelude::*;
use heron::prelude::*;

#[derive(Component, Default, Clone)]
pub struct Player;

#[derive(Component, Default, Clone)]
pub struct Enemy;

#[derive(Component, Default, Clone)]
pub struct Item;

#[derive(Component, Clone, Debug, Default)]
pub struct Coin;

#[derive(Component, Clone, Debug, Default)]
pub struct Wall;

#[derive(Component, Clone, Debug, Default)]
pub struct Stairs;

#[derive(Component, Default, Clone)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct TimeToLive(pub Timer);

#[derive(PhysicsLayer)]
pub enum GameCollisionLayers {
    World,
    Player,
    PlayerAttack,
    Enemy,
    EnemyAttack,
    Item,
    Coin,
    Stairs,
}
