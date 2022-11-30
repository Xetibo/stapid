use std::f32::consts::PI;

use bevy::{prelude::*,sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin};

#[derive(Component,Inspectable)]
pub struct Player {
    pub speed: f32,
    pub direction: Direction,
}

#[derive(Component, Inspectable)]
pub struct Bullet {
    pub speed: f32,
}

#[derive(Component, Inspectable)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Component, Inspectable)]
struct Name(String);

impl Player {
    fn new() -> Player {
        Player {
            speed: 2.5,
            direction: Direction::Up,
        }
    }
}

impl Bullet {
    fn new() -> Bullet {
        Bullet { speed: 10.0 }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: String::from("stapid"),
                resizable: true,
                decorations: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_system(move_all_players)
        .add_system(player_shoot)
        .run();
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player::new(),
        SpriteBundle {
            texture: asset_server.load("../assets/stick.resized.png"),
            ..default()
        },
        Direction::Up,
        Name(String::from("player1")),
    ));
    commands.spawn((
        Player::new(),
        SpriteBundle {
            texture: asset_server.load("../assets/stick.resized.png"),
            ..default()
        },
        Direction::Up,
        Name(String::from("player2")),
    ));
}

fn move_all_players(
    mut players: Query<(&Player, &mut Transform, &Name)>,
    timer: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    for (player, mut transform, name) in &mut players {
        if name.0 == String::from("player1") {
            if keys.pressed(KeyCode::Up) {
                transform.translation.y += 60. * player.speed * timer.delta_seconds();
            }
            if keys.pressed(KeyCode::Down) {
                transform.translation.y -= 60. * player.speed * timer.delta_seconds();
            }
            if keys.pressed(KeyCode::Right) {
                transform.translation.x += 60. * player.speed * timer.delta_seconds();
            }
            if keys.pressed(KeyCode::Left) {
                transform.translation.x -= 60. * player.speed * timer.delta_seconds();
            }
        }
        if name.0 == String::from("player2") {
            if keys.pressed(KeyCode::W) {
                transform.translation.y += 60. * player.speed * timer.delta_seconds();
            }
            if keys.pressed(KeyCode::S) {
                transform.translation.y -= 60. * player.speed * timer.delta_seconds();
            }
            if keys.pressed(KeyCode::D) {
                transform.translation.x += 60. * player.speed * timer.delta_seconds();
            }
            if keys.pressed(KeyCode::A) {
                transform.translation.x -= 60. * player.speed * timer.delta_seconds();
            }
        }
        // match player.direction {
        //     Direction::Up => transform.translation.y += 30. * player.speed * timer.delta_seconds(),
        //     Direction::Down => transform.translation.y -= 30. * player.speed * timer.delta_seconds(),
        //     Direction::Right => transform.translation.x += 30. * player.speed * timer.delta_seconds(),
        //     Direction::Left => transform.translation.x -= 30. * player.speed * timer.delta_seconds(),
        // }
    }
}

fn player_shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    players: Query<(&Player, &Name, &Transform)>,
    keys: Res<Input<KeyCode>>,
) {
    for (_player, name, transform) in &players {
        if name.0 == "player1" {
            if keys.just_pressed(KeyCode::LControl) { 
                commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
                        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                        transform: *transform,
                        ..default()
                    })
                    .insert(Name(String::from("Bullet from player1")));
            }
        }
    }
}



fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
