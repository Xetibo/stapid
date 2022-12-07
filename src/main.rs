use bevy::{prelude::*, sprite::collide_aabb::collide, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use constants::PLAYER_SIZE;
use game_objects::{Explosion, PowerUp};
use rand::prelude::*;
use std::time::Duration;

pub mod game_utils;
use crate::game_utils::{
    BulletType, Collider, Direction, DirectionHelper, HitCooldownTimer, Name, TimerType,
};

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
        .add_system(spawn_powerup)
        .add_system(move_all_players)
        .add_system(player_shoot)
        .add_system(move_all_bullets)
        .add_system(collision_bullet)
        .add_system(collision_powerup)
        .add_system(collision_explosion)
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
        match bullet.direction.direction_y {
            Direction::Up => {
                transform.translation.y += 50. * bullet.speed * timer.delta_seconds();
            }
            Direction::Down => {
                transform.translation.y -= 50. * bullet.speed * timer.delta_seconds();
            }
            _ => (),
        }
        match bullet.direction.direction_x {
            Direction::Right => {
                transform.translation.x += 50. * bullet.speed * timer.delta_seconds();
            }
            Direction::Left => {
                transform.translation.x -= 50. * bullet.speed * timer.delta_seconds();
            }
            _ => (),
        }
    }
}

fn collision_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
                        commands.entity(bullet_entity).despawn();
                        commands.spawn((
                            Explosion { radius: 50.0 },
                            MaterialMesh2dBundle {
                                mesh: meshes.add(shape::Circle::new(50.0).into()).into(),
                                material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                                transform: Transform::from_translation(
                                    bullet_transform.translation,
                                ),
                                ..default()
                            },
                            Collider,
                        ));
                    }
                    BulletType::BouncyBullet => {
                        bullet.direction = DirectionHelper {
                            direction_x: bullet.direction.direction_x.opposite(),
                            direction_y: bullet.direction.direction_y.opposite(),
                        };
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

fn collision_powerup(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Player)>,
    mut collider_query: Query<(Entity, &Transform, &PowerUp), With<Collider>>,
) {
    for (player_transform, mut player) in &mut player_query {
        for (collider_entity, transform, _maybe_powerup) in &mut collider_query {
            let collision = collide(
                transform.translation,
                transform.scale.truncate(),
                player_transform.translation,
                player_transform.scale.truncate(),
            );
            if collision.is_some() {
                let mut rng = rand::thread_rng();
                let bullet_random = rng.gen_range(0..=2);
                player.powerup = true;
                player.power_up_type = BulletType::convert_int(bullet_random);
                commands.entity(collider_entity).despawn();
            }
        }
    }
}

fn collision_explosion(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &mut Player)>,
    mut collider_query: Query<(Entity, &Transform, &Explosion), With<Collider>>,
) {
    for (player_entity, player_transform, mut player) in &mut player_query {
        for (collider_entity, transform, explosion) in &mut collider_query {
            let collision = collide(
                transform.translation,
                Vec2 {
                    x: explosion.radius,
                    y: explosion.radius,
                },
                player_transform.translation,
                player_transform.scale.truncate(),
            );
            if collision.is_some() {
                if player.lifes > 2 {
                    player.lifes -= 2;
                } else {
                    commands.entity(player_entity).despawn();
                }
            }
            commands.entity(collider_entity).despawn();
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
            if keys.pressed(player.bindings.up) && keys.pressed(player.bindings.right) {
                let new_position_y =
                    transform.translation.y + 80. * player.speed * timer.delta_seconds();
                let new_position_x =
                    transform.translation.x + 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position_y.clamp(TOP_BOUND, BOTTOM_BOUND);
                transform.translation.x = new_position_x.clamp(LEFT_BOUND, RIGHT_BOUND);
                player.direction.direction_y = Direction::Up;
                player.direction.direction_x = Direction::Right;
            } else if keys.pressed(player.bindings.up) && keys.pressed(player.bindings.left) {
                let new_position_y =
                    transform.translation.y + 80. * player.speed * timer.delta_seconds();
                let new_position_x =
                    transform.translation.x - 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position_y.clamp(TOP_BOUND, BOTTOM_BOUND);
                transform.translation.x = new_position_x.clamp(LEFT_BOUND, RIGHT_BOUND);
                player.direction.direction_y = Direction::Up;
                player.direction.direction_x = Direction::Left;
            } else if keys.pressed(player.bindings.down) && keys.pressed(player.bindings.right) {
                let new_position_y =
                    transform.translation.y - 80. * player.speed * timer.delta_seconds();
                let new_position_x =
                    transform.translation.x + 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position_y.clamp(TOP_BOUND, BOTTOM_BOUND);
                transform.translation.x = new_position_x.clamp(LEFT_BOUND, RIGHT_BOUND);
                player.direction.direction_y = Direction::Down;
                player.direction.direction_x = Direction::Right;
            } else if keys.pressed(player.bindings.down) && keys.pressed(player.bindings.left) {
                let new_position_y =
                    transform.translation.y - 80. * player.speed * timer.delta_seconds();
                let new_position_x =
                    transform.translation.x - 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position_y.clamp(TOP_BOUND, BOTTOM_BOUND);
                transform.translation.x = new_position_x.clamp(LEFT_BOUND, RIGHT_BOUND);
                player.direction.direction_y = Direction::Down;
                player.direction.direction_x = Direction::Left;
            } else if keys.pressed(player.bindings.up) {
                let new_position =
                    transform.translation.y + 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position.clamp(TOP_BOUND, BOTTOM_BOUND);
                player.direction.direction_y = Direction::Up;
                player.direction.direction_x = Direction::None;
            } else if keys.pressed(player.bindings.down) {
                let new_position =
                    transform.translation.y - 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position.clamp(TOP_BOUND, BOTTOM_BOUND);
                player.direction.direction_y = Direction::Down;
                player.direction.direction_x = Direction::None;
            } else if keys.pressed(player.bindings.right) {
                let new_position =
                    transform.translation.x + 80. * player.speed * timer.delta_seconds();
                transform.translation.x = new_position.clamp(LEFT_BOUND, RIGHT_BOUND);
                player.direction.direction_x = Direction::Right;
                player.direction.direction_y = Direction::None;
            } else if keys.pressed(player.bindings.left) {
                let new_position =
                    transform.translation.x - 80. * player.speed * timer.delta_seconds();
                transform.translation.x = new_position.clamp(LEFT_BOUND, RIGHT_BOUND);
                player.direction.direction_x = Direction::Left;
                player.direction.direction_y = Direction::None;
            }
        }
    }
}

fn player_shoot(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut players: Query<(&mut Player, &Transform)>,
    keys: Res<Input<KeyCode>>,
) {
    for (mut player, transform) in &mut players {
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
        if keys.just_pressed(player.bindings.shoot_special)
            && player.stunned == false
            && player.powerup == true
        {
            let (bullet_x, bullet_y) = player.get_bullet_spawn_position();
            commands.spawn((
                Bullet::bullet_from_enum(player.power_up_type.as_ref(), &player.direction),
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
            player.powerup = false;
        }
    }
}

fn spawn_powerup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    power_up_query: Query<Entity, With<PowerUp>>,
) {
    if power_up_query.is_empty() {
        commands.spawn((
            PowerUp {
                pickup_type: BulletType::IceBullet,
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
                material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
                transform: Transform::from_translation(PowerUp::generate_random_position()),
                ..default()
            },
            Collider,
        ));
    }
}

fn spawn_walls(mut commands: Commands) {
    commands.spawn(Wall::new(Direction::Up));
    commands.spawn(Wall::new(Direction::Down));
    commands.spawn(Wall::new(Direction::Right));
    commands.spawn(Wall::new(Direction::Left));
    commands.spawn(Wall::new_random_wall());
    commands.spawn(Wall::new_random_wall());
    commands.spawn(Wall::new_random_wall());
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
