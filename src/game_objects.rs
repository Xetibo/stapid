use crate::Collider;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::constants::{WALL_BOTTOM, WALL_LEFT, WALL_RIGHT, WALL_THICKNESS, WALL_TOP};
use crate::game_utils::{Bindings, BulletType, Direction, DirectionBlock, DirectionHelper};
use rand::prelude::*;

#[derive(Component, Inspectable)]
pub struct Player {
    pub player_number: i32,
    pub size: i32,
    pub lifes: i32,
    pub invulnerable: bool,
    pub stunned: bool,
    pub powerup: bool,
    pub shoot: bool,
    pub speed: f32,
    pub direction: DirectionHelper,
    pub direction_block: DirectionBlock,
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
    pub bounces_left: i32,
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
pub struct WallBundle {
    pub direction: Direction,
    pub sprite_bundle: SpriteBundle,
    pub collider: Collider,
}

// create totem pls
#[derive(Component)]
pub struct Totem {}

#[derive(Component, Inspectable)]
pub struct Wall {}

#[derive(Component)]
pub struct UINode {}

#[derive(Component)]
pub struct UIText {}

impl Player {
    pub fn new(
        number: i32,
        entered_name: String,
        entered_shootbind: KeyCode,
        entered_shoot_specialbind: KeyCode,
        entered_upbind: KeyCode,
        entered_downbind: KeyCode,
        entered_rightbind: KeyCode,
        entered_leftbind: KeyCode,
        entered_direction: DirectionHelper,
    ) -> Player {
        Player {
            player_number: number,
            size: 50,
            lifes: 3,
            invulnerable: false,
            stunned: false,
            powerup: false,
            shoot: true,
            speed: 2.5,
            direction: entered_direction,
            direction_block: DirectionBlock {
                up: false,
                down: false,
                right: false,
                left: false,
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
            Direction::Right => 41.0,
            Direction::Left => -41.0,
            _ => 0.0,
        };
        let bullet_y = match self.direction.direction_y {
            Direction::Up => 41.0,
            Direction::Down => -41.0,
            _ => 0.0,
        };
        (bullet_x, bullet_y)
    }

    pub fn get_direction_sprite(&self) -> &str {
        match &self.direction.direction_x {
            Direction::Right => match &self.direction.direction_y {
                Direction::Up => "../assets/player_right_up.png",
                Direction::Down => "../assets/player_right_down.png",
                _ => "../assets/player_right.png",
            },
            Direction::Left => match &self.direction.direction_y {
                Direction::Up => "../assets/player_left_up.png",
                Direction::Down => "../assets/player_left_down.png",
                _ => "../assets/player_left.png",
            },
            _ => match &self.direction.direction_y {
                Direction::Up => "../assets/player_up.png",
                Direction::Down => "../assets/player_down.png",
                _ => "",
            },
        }
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
            bounces_left: 0,
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
            bounces_left: 0,
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
            bounces_left: 0,
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
            bounces_left: 4,
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
            z: 2.0,
        }
    }
}

impl WallBundle {
    pub fn new(entered_direction: Direction, asset_server: &Res<AssetServer>) -> WallBundle {
        WallBundle {
            direction: entered_direction.clone(),
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: match entered_direction {
                        Direction::Up => Vec3 {
                            x: 0.0,
                            y: WALL_TOP,
                            z: (2.0),
                        },
                        Direction::Down => Vec3 {
                            x: 0.0,
                            y: WALL_BOTTOM,
                            z: (2.0),
                        },
                        Direction::Right => Vec3 {
                            x: WALL_RIGHT,
                            y: 0.0,
                            z: (2.0),
                        },
                        Direction::Left => Vec3 {
                            x: WALL_LEFT,
                            y: 0.0,
                            z: (2.0),
                        },
                        Direction::None => Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 2.0,
                        },
                    },
                    scale: match entered_direction {
                        Direction::Up | Direction::Down => Vec3 {
                            x: 1610.0,
                            y: WALL_THICKNESS,
                            z: (0.0),
                        },
                        Direction::Right | Direction::Left => Vec3 {
                            x: WALL_THICKNESS,
                            y: 1010.0,
                            z: (0.0),
                        },
                        Direction::None => Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    },
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
                    ..default()
                },
                texture: match entered_direction {
                    Direction::Up | Direction::Down => asset_server.load("../assets/bricks.png"),
                    _ => asset_server.load("../assets/bricks.png"),
                },
                ..default()
            },
            collider: Collider,
        }
    }

    pub fn new_random_wall() -> WallBundle {
        let mut rng = rand::thread_rng();
        let direction = Self::convert_int(rng.gen_range(0..=1));
        match direction.unwrap() {
            Direction::Up => WallBundle {
                direction: Direction::Up,
                sprite_bundle: SpriteBundle {
                    transform: Transform {
                        translation: Vec3 {
                            x: rng.gen_range(-500..=500) as f32,
                            y: rng.gen_range(-400..=400) as f32,
                            z: (2.0),
                        },
                        scale: Vec3 {
                            x: 500.0,
                            y: WALL_THICKNESS,
                            z: (0.0),
                        },
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(1.0, 0.0, 0.0),
                        ..default()
                    },
                    ..default()
                },
                collider: Collider,
            },
            _ => WallBundle {
                direction: Direction::Right,
                sprite_bundle: SpriteBundle {
                    transform: Transform {
                        translation: Vec3 {
                            x: rng.gen_range(-500..=500) as f32,
                            y: rng.gen_range(-250..=250) as f32,
                            z: (2.0),
                        },
                        scale: Vec3 {
                            x: WALL_THICKNESS,
                            y: 500.0,
                            z: (0.0),
                        },
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(1.0, 0.0, 0.0),
                        ..default()
                    },
                    ..default()
                },
                collider: Collider,
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
