use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::constants::{WALL_BOTTOM, WALL_LEFT, WALL_RIGHT, WALL_THICKNESS, WALL_TOP};
use crate::game_utils::{Bindings, BulletType, Collider, Direction, DirectionHelper};
use rand::prelude::*;

#[derive(Component, Inspectable)]
pub struct Player {
    pub size: i32,
    pub lifes: i32,
    pub invulnerable: bool,
    pub stunned: bool,
    pub powerup: bool,
    pub speed: f32,
    pub direction: DirectionHelper,
    pub name: String,
    pub bindings: Bindings,
    pub power_up_type: Option<BulletType>,
}

#[derive(Component, Inspectable)]
pub struct Bullet {
    pub bullet_type: BulletType,
    pub speed: f32,
    pub area_of_effect: f32,
    pub stuns: bool,
    pub bounces: bool,
    pub direction: DirectionHelper,
    pub color: Color,
}

#[derive(Component, Inspectable)]
pub struct Explosion {
    pub radius: f32,
}

#[derive(Component, Inspectable)]
pub struct PowerUp {
    pub pickup_type: BulletType,
}

#[derive(Bundle)]
pub struct Wall {
    pub direction: Direction,
    pub sprite_bundle: SpriteBundle,
    pub collider: Collider,
}

impl Player {
    pub fn new(
        entered_name: String,
        entered_shootbind: KeyCode,
        entered_shoot_specialbind: KeyCode,
        entered_upbind: KeyCode,
        entered_downbind: KeyCode,
        entered_rightbind: KeyCode,
        entered_leftbind: KeyCode,
    ) -> Player {
        Player {
            size: 50,
            lifes: 3,
            invulnerable: false,
            stunned: false,
            powerup: false,
            speed: 2.5,
            direction: DirectionHelper {
                direction_y: Direction::Up,
                direction_x: Direction::Right,
            },
            name: entered_name,
            bindings: Bindings {
                shoot: entered_shootbind,
                shoot_special: entered_shoot_specialbind,
                up: entered_upbind,
                down: entered_downbind,
                right: entered_rightbind,
                left: entered_leftbind,
            },
            power_up_type: None,
        }
    }

    pub fn decrement_life(&mut self) {
        self.lifes -= 1;
    }

    pub fn get_bullet_spawn_position(&self) -> (f32, f32) {
        let bullet_x = match self.direction.direction_x {
            Direction::Right => 30.0,
            Direction::Left => -30.0,
            _ => 0.0,
        };
        let bullet_y = match self.direction.direction_y {
            Direction::Up => 30.0,
            Direction::Down => -30.0,
            _ => 0.0,
        };
        (bullet_x, bullet_y)
    }
}

impl Bullet {
    pub fn bullet_from_enum(
        entered_bullet_type: Option<&BulletType>,
        direction: &DirectionHelper,
    ) -> Bullet {
        match entered_bullet_type.unwrap() {
            bullet_type => match bullet_type {
                BulletType::NormalBullet => Self::normal_bullet(direction.clone()),
                BulletType::IceBullet => Self::ice_bullet(direction.clone()),
                BulletType::ExplosiveBullet => Self::explosive_bullet(direction.clone()),
                BulletType::BouncyBullet => Self::bouncy_bullet(direction.clone()),
            },
        }
    }

    pub fn normal_bullet(direction_entered: DirectionHelper) -> Bullet {
        Bullet {
            bullet_type: BulletType::NormalBullet,
            speed: 10.0,
            area_of_effect: 1.0,
            stuns: false,
            bounces: false,
            direction: direction_entered,
            color: Color::rgb(1.0, 0.0, 0.0),
        }
    }

    pub fn ice_bullet(direction_entered: DirectionHelper) -> Bullet {
        Bullet {
            bullet_type: BulletType::IceBullet,
            speed: 20.0,
            area_of_effect: 1.0,
            stuns: true,
            bounces: false,
            direction: direction_entered,
            color: Color::rgb(0.0, 0.0, 1.0),
        }
    }

    pub fn explosive_bullet(direction_entered: DirectionHelper) -> Bullet {
        Bullet {
            bullet_type: BulletType::ExplosiveBullet,
            speed: 6.0,
            area_of_effect: 5.0,
            stuns: false,
            bounces: false,
            direction: direction_entered,
            color: Color::rgb(1.0, 1.0, 0.0),
        }
    }

    pub fn bouncy_bullet(direction_entered: DirectionHelper) -> Bullet {
        Bullet {
            bullet_type: BulletType::BouncyBullet,
            speed: 15.0,
            area_of_effect: 1.0,
            stuns: false,
            bounces: true,
            direction: direction_entered,
            color: Color::rgb(0.0, 1.0, 0.0),
        }
    }
}

impl PowerUp {
    pub fn generate_random_position() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range((WALL_LEFT + 15.0)..=(WALL_RIGHT - 15.0)),
            y: rng.gen_range((WALL_TOP + 15.0)..=(WALL_BOTTOM - 15.0)),
            z: 0.0,
        }
    }
}

impl Wall {
    pub fn new(entered_direction: Direction) -> Wall {
        Wall {
            direction: entered_direction.clone(),
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: match entered_direction {
                        Direction::Up => Vec3 {
                            x: 0.0,
                            y: WALL_TOP,
                            z: (0.0),
                        },
                        Direction::Down => Vec3 {
                            x: 0.0,
                            y: WALL_BOTTOM,
                            z: (0.0),
                        },
                        Direction::Right => Vec3 {
                            x: WALL_RIGHT,
                            y: 0.0,
                            z: (0.0),
                        },
                        Direction::Left => Vec3 {
                            x: WALL_LEFT,
                            y: 0.0,
                            z: (0.0),
                        },
                        Direction::None => Vec3 {x:0.0, y:0.0, z:0.0,},
                    },
                    scale: match entered_direction {
                        Direction::Up | Direction::Down => Vec3 {
                            x: 1610.0,
                            y: WALL_THICKNESS,
                            z: (1.0),
                        },
                        Direction::Right | Direction::Left => Vec3 {
                            x: WALL_THICKNESS,
                            y: 1010.0,
                            z: (1.0),
                        },
                        Direction::None => Vec3 {x:0.0, y:0.0, z:0.0,},
                    },
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    ..default()
                },
                ..default()
            },
            collider: Collider {},
        }
    }

    pub fn new_random_wall() -> Wall {
        let mut rng = rand::thread_rng();
        let direction = Self::convert_int(rng.gen_range(0..=1));
        match direction.unwrap() {
            Direction::Up => Wall {
                direction: Direction::Up,
                sprite_bundle: SpriteBundle {
                    transform: Transform {
                        translation: Vec3 {
                            x: rng.gen_range(-500..=500) as f32,
                            y: rng.gen_range(-500..=500) as f32,
                            z: (0.0),
                        },
                        scale: Vec3 {
                            x: 500.0,
                            y: WALL_THICKNESS,
                            z: (1.0),
                        },
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(1.0, 0.0, 0.0),
                        ..default()
                    },
                    ..default()
                },
                collider: Collider {},
            },
            _ => Wall {
                direction: Direction::Right,
                sprite_bundle: SpriteBundle {
                    transform: Transform {
                        translation: Vec3 {
                            x: rng.gen_range(0..=500) as f32,
                            y: rng.gen_range(0..=500) as f32,
                            z: (0.0),
                        },
                        scale: Vec3 {
                            x: WALL_THICKNESS,
                            y: 500.0,
                            z: (1.0),
                        },
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(1.0, 0.0, 0.0),
                        ..default()
                    },
                    ..default()
                },
                collider: Collider {},
            },
        }
    }

    pub fn convert_int(number: i32) -> Option<Direction> {
        match number {
            0 => Some(Direction::Up),
            1 => Some(Direction::Right),
            _ => None,
        }
    }
}
