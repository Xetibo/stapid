use bevy::{prelude::*, sprite::collide_aabb::collide, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use constants::PLAYER_SIZE;
use std::time::Duration;

pub mod game_utils;
use crate::game_utils::{BulletType, Collider, Direction, HitCooldownTimer, Name, TimerType};

pub mod game_objects;
use crate::game_objects::{Bullet, Player, Wall};

pub mod constants;
use crate::constants::{BOTTOM_BOUND, LEFT_BOUND, RIGHT_BOUND, TOP_BOUND};

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
        .register_inspectable::<Player>()
        .register_inspectable::<Bullet>()
        .add_startup_system(spawn_walls)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_system(move_all_players)
        .add_system(player_shoot)
        .add_system(move_all_bullets)
        .add_system(collision_bullet)
        .add_system(tick_timer)
        .run();
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player::new(
            String::from("player1"),
            KeyCode::LControl,
            KeyCode::LShift,
            KeyCode::W,
            KeyCode::S,
            KeyCode::D,
            KeyCode::A,
        ),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
                ..default()
            },
            texture: asset_server.load("../assets/stick.resized.png"),
            transform: Transform {
                translation: Vec3 {
                    x: 10.0,
                    y: 10.0,
                    z: 0.0,
                },
                scale: Vec3 {
                    x: PLAYER_SIZE,
                    y: PLAYER_SIZE,
                    z: 0.0,
                },
                ..default()
            },
            ..default()
        },
        Collider,
        Direction::Up,
        Name::new(String::from("player1")),
    ));
    commands.spawn((
        Player::new(
            String::from("player2"),
            KeyCode::RControl,
            KeyCode::RShift,
            KeyCode::Up,
            KeyCode::Down,
            KeyCode::Right,
            KeyCode::Left,
        ),
        SpriteBundle {
            sprite: Sprite {
                custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
                ..default()
            },
            texture: asset_server.load("../assets/stick.resized.png"),
            transform: Transform {
                translation: Vec3 {
                    x: -10.0,
                    y: -10.0,
                    z: 0.0,
                },
                scale: Vec3 {
                    x: PLAYER_SIZE,
                    y: PLAYER_SIZE,
                    z: 0.0,
                },
                ..default()
            },
            ..default()
        },
        Collider,
        Direction::Up,
        Name::new(String::from("player2")),
    ));
}

fn move_all_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, timer: Res<Time>) {
    for (bullet, mut transform) in &mut bullets {
        match bullet.direction {
            Direction::Up => {
                transform.translation.y += 50. * bullet.speed * timer.delta_seconds();
            }
            Direction::Down => {
                transform.translation.y -= 50. * bullet.speed * timer.delta_seconds();
            }
            Direction::Right => {
                transform.translation.x += 50. * bullet.speed * timer.delta_seconds();
            }
            Direction::Left => {
                transform.translation.x -= 50. * bullet.speed * timer.delta_seconds();
            }
        }
    }
}

fn collision_bullet(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &Transform, &mut Bullet)>,
    mut collider_query: Query<(Entity, &Transform, Option<&mut Player>), With<Collider>>,
) {
    for (bullet_entity, bullet_transform, mut bullet) in &mut bullet_query {
        let bullet_size = bullet_transform.scale.truncate();
        for (collider_entity, transform, mut maybe_player) in &mut collider_query {
            let collision = collide(
                bullet_transform.translation,
                bullet_size,
                transform.translation,
                transform.scale.truncate(),
            );
            if collision.is_some() {
                match bullet.bullet_type {
                    BulletType::NormalBullet => {
                        commands.entity(bullet_entity).despawn();
                        if maybe_player.is_some() {
                            let player = &mut **maybe_player.as_mut().unwrap();
                            if player.invulnerable == false {
                                if player.lifes > 1 {
                                    player.decrement_life();
                                    player.stunned = false;
                                    player.invulnerable = true;
                                    commands.spawn((HitCooldownTimer {
                                        timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
                                        associated_player: player.name.clone(),
                                        timer_type: TimerType::Invulnerable,
                                    },));
                                } else {
                                    commands.entity(collider_entity).despawn();
                                }
                            }
                        }
                    }
                    BulletType::IceBullet => {
                        commands.entity(bullet_entity).despawn();
                        if maybe_player.is_some() {
                            let player = &mut **maybe_player.as_mut().unwrap();
                            if player.invulnerable == false && player.stunned == false {
                                player.stunned = true;
                                commands.spawn((HitCooldownTimer {
                                    timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
                                    associated_player: player.name.clone(),
                                    timer_type: TimerType::Stun,
                                },));
                            }
                        }
                    }
                    BulletType::ExplosiveBullet => {
                        // will be added later
                        commands.entity(bullet_entity).despawn();
                    }
                    BulletType::BouncyBullet => {
                        bullet.direction = bullet.direction.opposite();
                        if maybe_player.is_some() {
                            let player = &mut **maybe_player.as_mut().unwrap();
                            if player.invulnerable == false {
                                if player.lifes > 1 {
                                    commands.entity(bullet_entity).despawn();
                                    player.decrement_life();
                                    player.stunned = false;
                                    player.invulnerable = true;
                                    commands.spawn((HitCooldownTimer {
                                        timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
                                        associated_player: player.name.clone(),
                                        timer_type: TimerType::Invulnerable,
                                    },));
                                } else {
                                    commands.entity(collider_entity).despawn();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn tick_timer(
    mut commands: Commands,
    mut player_query: Query<&mut Player>,
    mut timer_query: Query<(Entity, &mut HitCooldownTimer)>,
    time: Res<Time>,
) {
    for (entity, mut hit_timer) in &mut timer_query {
        hit_timer.timer.tick(time.delta());
        for mut player in &mut player_query {
            if hit_timer.timer.finished() && hit_timer.associated_player == player.name {
                match hit_timer.timer_type {
                    TimerType::Stun => {
                        player.stunned = false;
                        commands.entity(entity).despawn();
                    }
                    TimerType::Invulnerable => {
                        player.invulnerable = false;
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}

fn move_all_players(
    mut players: Query<(&mut Player, &mut Transform)>,
    timer: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    for (mut player, mut transform) in &mut players {
        if player.stunned == false {
            if keys.pressed(player.bindings.up) {
                let new_position =
                    transform.translation.y + 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position.clamp(TOP_BOUND, BOTTOM_BOUND);
                player.direction = Direction::Up;
            }
            if keys.pressed(player.bindings.down) {
                let new_position =
                    transform.translation.y - 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position.clamp(TOP_BOUND, BOTTOM_BOUND);
                player.direction = Direction::Down;
            }
            if keys.pressed(player.bindings.right) {
                let new_position =
                    transform.translation.x + 80. * player.speed * timer.delta_seconds();
                transform.translation.x = new_position.clamp(LEFT_BOUND, RIGHT_BOUND);
                player.direction = Direction::Right;
            }
            if keys.pressed(player.bindings.left) {
                let new_position =
                    transform.translation.x - 80. * player.speed * timer.delta_seconds();
                transform.translation.x = new_position.clamp(LEFT_BOUND, RIGHT_BOUND);
                player.direction = Direction::Left;
            }
        }
    }
}

fn player_shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    players: Query<(&Player, &Transform)>,
    keys: Res<Input<KeyCode>>,
) {
    for (player, transform) in &players {
        if keys.just_pressed(player.bindings.shoot) && player.stunned == false {
            let (bullet_x, bullet_y) = player.get_bullet_spawn_position();
            commands.spawn((
                Bullet::normal_bullet(player.direction.clone()),
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    transform: Transform::from_translation(
                        transform.translation
                            + Vec3 {
                                x: bullet_x,
                                y: bullet_y,
                                z: 0.0,
                            },
                    ),
                    ..default()
                },
            ));
        }
        if keys.just_pressed(player.bindings.shoot_special) && player.stunned == false {
            let (bullet_x, bullet_y) = player.get_bullet_spawn_position();
            commands.spawn((
                Bullet::ice_bullet(player.direction.clone()),
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
                    material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
                    transform: Transform::from_translation(
                        transform.translation
                            + Vec3 {
                                x: bullet_x,
                                y: bullet_y,
                                z: 0.0,
                            },
                    ),
                    ..default()
                },
            ));
        }
    }
}

fn spawn_walls(mut commands: Commands) {
    commands.spawn(Wall::new(Direction::Up));
    commands.spawn(Wall::new(Direction::Down));
    commands.spawn(Wall::new(Direction::Right));
    commands.spawn(Wall::new(Direction::Left));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
