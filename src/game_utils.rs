use bevy::prelude::*;
use std::fmt;

#[derive(Default, Event)]
pub struct ResetGameEvent {}

#[derive(Default, Event)]
pub struct PlayerHitEvent {}

#[derive(Default, Event)]
pub struct PlayerShootEvent {}

#[derive(Default, Event)]
pub struct PlayerPowerUpEvent {}

#[derive(Default, Event)]
pub struct UpdateUIEvent {
    pub player_number: usize,
}

#[derive(Default, Event)]
pub struct PlayerDeadEvent {}

#[derive(Component, Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
    None,
}
impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::None => Direction::None,
        }
    }
}

#[derive(Component, Clone)]
pub struct DirectionHelper {
    pub direction_y: Direction,
    pub direction_x: Direction,
}

#[derive(Component, Clone)]
pub struct DirectionBlock {
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub left: bool,
}

#[derive(Component, Clone)]
pub enum BulletType {
    NormalBullet,
    IceBullet,
    ExplosiveBullet,
    BouncyBullet,
}

#[derive(Component, Clone)]
pub enum TimerType {
    Invulnerable,
    Stun,
    Shoot,
}

#[derive(Component)]
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

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
    pub counter: i32,
}

#[derive(Component)]
pub struct InvulnerableBlinkTimer {
    pub timer: Timer,
    pub color: bool,
    pub associated_player: String,
}

#[derive(Component)]
pub struct Bindings {
    pub shoot: KeyCode,
    pub shoot_special: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
    pub right: KeyCode,
    pub left: KeyCode,
}

impl BulletType {
    pub fn convert_int(number: i32) -> Option<BulletType> {
        match number {
            0 => Some(BulletType::IceBullet),
            1 => Some(BulletType::ExplosiveBullet),
            2 => Some(BulletType::BouncyBullet),
            _ => None,
        }
    }
}

impl fmt::Display for BulletType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BulletType::IceBullet => write!(f, "Ice"),
            BulletType::NormalBullet => write!(f, "None"),
            BulletType::ExplosiveBullet => write!(f, "Grenade"),
            BulletType::BouncyBullet => write!(f, "Bouncy"),
        }
    }
}
