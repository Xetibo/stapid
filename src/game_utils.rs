use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable, Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}
impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

#[derive(Component, Inspectable)]
pub enum BulletType {
    NormalBullet,
    IceBullet,
    ExplosiveBullet,
    BouncyBullet,
}

#[derive(Component, Inspectable, Clone)]
pub enum TimerType {
    Invulnerable,
    Stun,
}

#[derive(Component, Inspectable)]
pub struct Name(String);
impl Name {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct HitCooldownTimer {
    pub timer: Timer,
    pub associated_player: String,
    pub timer_type: TimerType,
}

#[derive(Component, Inspectable)]
pub struct Bindings {
    #[inspectable(ignore)]
    pub shoot: KeyCode,
    #[inspectable(ignore)]
    pub shoot_special: KeyCode,
    #[inspectable(ignore)]
    pub up: KeyCode,
    #[inspectable(ignore)]
    pub down: KeyCode,
    #[inspectable(ignore)]
    pub right: KeyCode,
    #[inspectable(ignore)]
    pub left: KeyCode,
}