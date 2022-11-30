use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin};

#[derive(Component, Inspectable)]
pub struct Player {
    pub lifes: i32,
    pub speed: f32,
    pub direction: Direction,
}

#[derive(Component, Inspectable)]
pub struct Bullet {
    pub speed: f32,
    pub direction: Direction,
}

#[derive(Component, Inspectable, Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Component, Inspectable)]
struct Name(String);

#[derive(Component)]
struct Collider;

impl Player {
    fn new() -> Player {
        Player {
            lifes: 3,
            speed: 2.5,
            direction: Direction::Up,
        }
    }

    fn decrement_life(&mut self) {
        self.lifes -= 1;
    }
}

impl Bullet {
    fn new(direction_entered: Direction) -> Bullet {
        Bullet {
            speed: 10.0,
            direction: direction_entered,
        }
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
        .add_system(move_all_bullets)
        .add_system(collision_bullet)
        .run();
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player::new(),
        SpriteBundle {
            texture: asset_server.load("../assets/stick.resized.png"),
            ..default()
        },
        Collider,
        Direction::Up,
        Name(String::from("player1")),
    ));
    commands.spawn((
        Player::new(),
        SpriteBundle {
            texture: asset_server.load("../assets/stick.resized.png"),
            ..default()
        },
        Collider,
        Direction::Up,
        Name(String::from("player2")),
    ));
}

fn move_all_bullets(
    mut commands: Commands,
    mut bullets: Query<(&Bullet, &mut Transform, Entity)>,
    timer: Res<Time>,
) {
    for (bullet, mut transform, entity) in &mut bullets {
        match bullet.direction {
            Direction::Up => {
                transform.translation.y += 50. * bullet.speed * timer.delta_seconds();
                if transform.translation.y > 1000.0 {
                    commands.entity(entity).despawn();
                }
            }
            Direction::Down => {
                transform.translation.y -= 50. * bullet.speed * timer.delta_seconds();
                if transform.translation.y < -1000.0 {
                    commands.entity(entity).despawn();
                }
            }
            Direction::Right => {
                transform.translation.x += 50. * bullet.speed * timer.delta_seconds();
                if transform.translation.x > 1000.0 {
                    commands.entity(entity).despawn();
                }
            }
            Direction::Left => {
                transform.translation.x -= 50. * bullet.speed * timer.delta_seconds();
                if transform.translation.x < -1000.0 {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn collision_bullet(
    mut commands: Commands,
    mut bullet_query: Query<&Transform, With<Bullet>>,
    mut collider_query: Query<(Entity, &Transform, Option<&mut Player>), With<Collider>>,
) {
    for bullet_transform in &mut bullet_query {
        let bullet_size = bullet_transform.scale.truncate();
        for (collider_entity, transform, mut maybe_player) in &mut collider_query {
            let collision = collide(
                bullet_transform.translation,
                bullet_size,
                transform.translation,
                transform.scale.truncate(),
            );
            if collision.is_some() {
                if maybe_player.is_some() {
                    let player = &mut **maybe_player.as_mut().unwrap();
                    if player.lifes > 0 {
                        player.decrement_life();
                    } else {
                    commands.entity(collider_entity).despawn();
                    }
                }
            }
        }
    }
}

fn move_all_players(
    mut players: Query<(&mut Player, &mut Transform, &Name)>,
    timer: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    for (mut player, mut transform, name) in &mut players {
        if name.0 == String::from("player1") {
            if keys.pressed(KeyCode::Up) {
                transform.translation.y += 60. * player.speed * timer.delta_seconds();
                player.direction = Direction::Up;
            }
            if keys.pressed(KeyCode::Down) {
                transform.translation.y -= 60. * player.speed * timer.delta_seconds();
                player.direction = Direction::Down;
            }
            if keys.pressed(KeyCode::Right) {
                transform.translation.x += 60. * player.speed * timer.delta_seconds();
                player.direction = Direction::Right;
            }
            if keys.pressed(KeyCode::Left) {
                transform.translation.x -= 60. * player.speed * timer.delta_seconds();
                player.direction = Direction::Left;
            }
        }
        if name.0 == String::from("player2") {
            if keys.pressed(KeyCode::W) {
                transform.translation.y += 60. * player.speed * timer.delta_seconds();
                player.direction = Direction::Up;
            }
            if keys.pressed(KeyCode::S) {
                transform.translation.y -= 60. * player.speed * timer.delta_seconds();
                player.direction = Direction::Down;
            }
            if keys.pressed(KeyCode::D) {
                transform.translation.x += 60. * player.speed * timer.delta_seconds();
                player.direction = Direction::Right;
            }
            if keys.pressed(KeyCode::A) {
                transform.translation.x -= 60. * player.speed * timer.delta_seconds();
                player.direction = Direction::Left;
            }
        }
    }
}

fn player_shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    players: Query<(&Player, &Name, &Transform)>,
    keys: Res<Input<KeyCode>>,
) {
    for (player, name, transform) in &players {
        if name.0 == "player1" {
            if keys.just_pressed(KeyCode::LControl) {
                commands.spawn((
                    Bullet::new(player.direction.clone()),
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
                        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                        transform: *transform,
                        ..default()
                    },
                    Name(String::from("Bullet from player1")),
                ));
            }
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
