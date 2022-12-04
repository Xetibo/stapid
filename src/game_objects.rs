use bevy::{prelude::*, sprite::collide_aabb::collide, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};

use crate::game_utils::{
    Bindings, BulletType, Collider, Direction, HitCooldownTimer, Name, TimerType,
};

#[derive(Component, Inspectable)]
pub struct Player {
    pub size: i32,
    pub lifes: i32,
    pub invulnerable: bool,
    pub stunned: bool,
    pub speed: f32,
    pub direction: Direction,
    pub name: String,
    pub bindings: Bindings,
}

#[derive(Component, Inspectable)]
pub struct Bullet {
    pub bullet_type: BulletType,
    pub speed: f32,
    pub area_of_effect: f32,
    pub stuns: bool,
    pub bounces: bool,
    pub direction: Direction,
    pub color: Color,
}

#[derive(Bundle)]
pub struct Wall {
    pub direction: Direction,
    pub sprite_bundle: SpriteBundle,
    pub collider: Collider,
}
