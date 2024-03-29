use crate::constants::WALL_THICKNESS;
use crate::game_objects::{Wall, WallBundle};
use crate::game_utils::{Direction, PlayerPowerUpEvent, ResetGameEvent};
use crate::Collider;
use bevy::prelude::*;

const WALL_V_LENGTH_SPAWN: f32 = 200.0;
const WALL_H_LENGTH_SPAWN: f32 = 100.0;
const WALL_H_LEFT_X: f32 = -603.0;
const WALL_H_RIGHT_X: f32 = 603.0;
const WALL_H_TOP_Y: f32 = 358.0;
const WALL_H_BOTTOM_Y: f32 = -358.0;
const WALL_H_MIDDLE_X: f32 = 0.0;
const WALL_H_MIDDLE_TOP_Y: f32 = 395.0;
const WALL_H_MIDDLE_BOTTOM_Y: f32 = -395.0;
const WALL_V_LEFT_X: f32 = -695.0;
const WALL_V_RIGHT_X: f32 = 695.0;
const WALL_V_TOP_Y: f32 = 300.0;
const WALL_V_BOTTOM_Y: f32 = -300.0;
const WALL_V_MIDDLE_RIGHT_X: f32 = 300.0;
const WALL_V_MIDDLE_LEFT_X: f32 = -300.0;
const WALL_V_MIDDLE_Y: f32 = 0.0;
const WALL_V_MIDDLE_LENGTH: f32 = 350.0;

pub fn spawn_level_1(
    mut commands: Commands,
    mut event_writer: EventWriter<ResetGameEvent>,
    mut event_writer_powerup: EventWriter<PlayerPowerUpEvent>,
    asset_server: Res<AssetServer>,
) {
    event_writer.send_default();
    event_writer_powerup.send_default();
    event_writer_powerup.send_default();
    let walls = generate_walls(&asset_server);
    for wall in walls {
        commands.spawn((wall, Wall {}));
    }
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Vec3 {
                x: 1920.0,
                y: 1080.0,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
            ..default()
        },
        texture: asset_server.load("/assets/images/floor_bricks_960_540.png"),
        ..default()
    });
}

fn generate_walls(asset_server: &Res<AssetServer>) -> Vec<WallBundle> {
    vec![
        create_spawn_wall(
            Direction::Up,
            Direction::Right,
            asset_server,
            "/assets/images/walls/bricks_8_50_rotate.png",
        ),
        create_spawn_wall(
            Direction::Up,
            Direction::Left,
            asset_server,
            "/assets/images/walls/bricks_8_50_rotate.png",
        ),
        create_spawn_wall(
            Direction::Up,
            Direction::None,
            asset_server,
            "/assets/images/walls/bricks_8_100_rotate.png",
        ),
        create_spawn_wall(
            Direction::Down,
            Direction::Right,
            asset_server,
            "/assets/images/walls/bricks_8_50_rotate.png",
        ),
        create_spawn_wall(
            Direction::Down,
            Direction::Left,
            asset_server,
            "/assets/images/walls/bricks_8_50_rotate.png",
        ),
        create_spawn_wall(
            Direction::Down,
            Direction::None,
            asset_server,
            "/assets/images/walls/bricks_8_100_rotate.png",
        ),
        create_spawn_wall(
            Direction::Right,
            Direction::Up,
            asset_server,
            "/assets/images/walls/bricks_100_8.png",
        ),
        create_spawn_wall(
            Direction::Right,
            Direction::Down,
            asset_server,
            "/assets/images/walls/bricks_100_8.png",
        ),
        create_spawn_wall(
            Direction::Right,
            Direction::None,
            asset_server,
            "/assets/images/walls/bricks_175_8.png",
        ),
        create_spawn_wall(
            Direction::Left,
            Direction::Up,
            asset_server,
            "/assets/images/walls/bricks_100_8.png",
        ),
        create_spawn_wall(
            Direction::Left,
            Direction::Down,
            asset_server,
            "/assets/images/walls/bricks_100_8.png",
        ),
        create_spawn_wall(
            Direction::Left,
            Direction::None,
            asset_server,
            "/assets/images/walls/bricks_175_8.png",
        ),
    ]
}

fn create_spawn_wall(
    direction_wall: Direction,
    direction: Direction,
    asset_server: &Res<AssetServer>,
    asset: &'static str,
) -> WallBundle {
    let (wall_x, wall_y, wall_scale_x, wall_scale_y) = get_vals(direction_wall, direction);
    WallBundle {
        direction: Direction::None,
        sprite_bundle: SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    x: wall_x,
                    y: wall_y,
                    z: (2.0),
                },
                scale: Vec3 {
                    x: wall_scale_x,
                    y: wall_scale_y,
                    z: (0.0),
                },
                ..default()
            },
            sprite: Sprite {
                custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
                ..default()
            },
            texture: asset_server.load(asset),
            ..default()
        },
        collider: Collider,
    }
}

fn get_vals(direction_wall: Direction, direction: Direction) -> (f32, f32, f32, f32) {
    match direction_wall {
        Direction::Up => match direction {
            Direction::Right => (
                WALL_H_RIGHT_X,
                WALL_H_TOP_Y,
                WALL_THICKNESS,
                WALL_H_LENGTH_SPAWN,
            ),
            Direction::Left => (
                WALL_H_LEFT_X,
                WALL_H_TOP_Y,
                WALL_THICKNESS,
                WALL_H_LENGTH_SPAWN,
            ),
            Direction::None => (
                WALL_H_MIDDLE_X,
                WALL_H_MIDDLE_TOP_Y,
                WALL_THICKNESS,
                WALL_V_LENGTH_SPAWN,
            ),
            _ => (0.0, 0.0, 0.0, 0.0),
        },
        Direction::Down => match direction {
            Direction::Right => (
                WALL_H_RIGHT_X,
                WALL_H_BOTTOM_Y,
                WALL_THICKNESS,
                WALL_H_LENGTH_SPAWN,
            ),
            Direction::Left => (
                WALL_H_LEFT_X,
                WALL_H_BOTTOM_Y,
                WALL_THICKNESS,
                WALL_H_LENGTH_SPAWN,
            ),
            Direction::None => (
                WALL_H_MIDDLE_X,
                WALL_H_MIDDLE_BOTTOM_Y,
                WALL_THICKNESS,
                WALL_V_LENGTH_SPAWN,
            ),
            _ => (0.0, 0.0, 0.0, 0.0),
        },
        Direction::Right => match direction {
            Direction::Up => (
                WALL_V_RIGHT_X,
                WALL_V_TOP_Y,
                WALL_V_LENGTH_SPAWN,
                WALL_THICKNESS,
            ),
            Direction::Down => (
                WALL_V_RIGHT_X,
                WALL_V_BOTTOM_Y,
                WALL_V_LENGTH_SPAWN,
                WALL_THICKNESS,
            ),
            Direction::None => (
                WALL_V_MIDDLE_RIGHT_X,
                WALL_V_MIDDLE_Y,
                WALL_V_MIDDLE_LENGTH,
                WALL_THICKNESS,
            ),
            _ => (0.0, 0.0, 0.0, 0.0),
        },
        Direction::Left => match direction {
            Direction::Up => (
                WALL_V_LEFT_X,
                WALL_V_TOP_Y,
                WALL_V_LENGTH_SPAWN,
                WALL_THICKNESS,
            ),
            Direction::Down => (
                WALL_V_LEFT_X,
                WALL_V_BOTTOM_Y,
                WALL_V_LENGTH_SPAWN,
                WALL_THICKNESS,
            ),
            Direction::None => (
                WALL_V_MIDDLE_LEFT_X,
                WALL_V_MIDDLE_Y,
                WALL_V_MIDDLE_LENGTH,
                WALL_THICKNESS,
            ),
            _ => (0.0, 0.0, 0.0, 0.0),
        },
        _ => (0.0, 0.0, 0.0, 0.0),
    }
}
