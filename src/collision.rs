use crate::game_objects::{Bullet, Explosion, Player, PowerUp, Wall};
use crate::game_utils::{BulletType, Collider, DirectionHelper, UpdateUIEvent, HitCooldownTimer, TimerType};
use bevy::{prelude::*, sprite::collide_aabb::collide, sprite::collide_aabb::Collision, sprite::MaterialMesh2dBundle, utils::Duration};
use rand::prelude::*;

pub fn collision_explosion(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform, &mut Player)>,
    mut collider_query: Query<(Entity, &Transform, &Explosion), With<Collider>>,
    mut event_writer: EventWriter<UpdateUIEvent>,
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
                event_writer.send_default();
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

pub fn collision_powerup(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Player)>,
    mut collider_query: Query<(Entity, &Transform, &PowerUp), With<Collider>>,
    mut event_writer: EventWriter<UpdateUIEvent>,
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
                event_writer.send_default();
                let mut rng = rand::thread_rng();
                let bullet_random = rng.gen_range(0..=2);
                player.powerup = true;
                player.power_up_type = BulletType::convert_int(bullet_random);
                commands.entity(collider_entity).despawn();
            }
        }
    }
}

pub fn collision_player(
    collider_query: Query<&Transform, (With<Collider>, With<Wall>)>,
    mut player_query: Query<(&Transform, &mut Player)>,
) {
    for (player_transform, mut player) in &mut player_query {
        let mut b_was_collision_up = false;
        let mut b_was_collision_down = false;
        let mut b_was_collision_right = false;
        let mut b_was_collision_left = false;
        for transform in collider_query.iter() {
            let collision = collide(
                transform.translation,
                transform.scale.truncate(),
                player_transform.translation,
                player_transform.scale.truncate() + 1.15,
            );
            if collision.is_some() {
                let direction = collision.unwrap();
                match direction {
                    Collision::Right => {
                        player.direction_block.right = true;
                        b_was_collision_right = true;
                    }
                    Collision::Left => {
                        player.direction_block.left = true;
                        b_was_collision_left = true;
                    }
                    Collision::Top => {
                        player.direction_block.up = true;
                        b_was_collision_up = true;
                    }
                    Collision::Bottom => {
                        player.direction_block.down = true;
                        b_was_collision_down = true;
                    }
                    Collision::Inside => (),
                }
            }
        }
        if !b_was_collision_up {
            player.direction_block.up = false;
        }
        if !b_was_collision_down {
            player.direction_block.down = false;
        }
        if !b_was_collision_right {
            player.direction_block.right = false;
        }
        if !b_was_collision_left {
            player.direction_block.left = false;
        }
    }
}

pub fn collision_bullet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut bullet_query: Query<(Entity, &Transform, &mut Bullet)>,
    mut collider_query: Query<(Entity, &Transform, Option<&mut Player>), With<Collider>>,
    mut event_writer: EventWriter<UpdateUIEvent>,
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
                                event_writer.send_default();
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
                        let direction_collision = collision.unwrap();
                        bullet.direction = match direction_collision {
                            Collision::Left | Collision::Right => DirectionHelper {
                                direction_x: bullet.direction.direction_x.opposite(),
                                direction_y: bullet.direction.direction_y.clone(),
                            },
                            _ => DirectionHelper {
                                direction_x: bullet.direction.direction_x.clone(),
                                direction_y: bullet.direction.direction_y.opposite(),
                            },
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
                                event_writer.send_default();
                            }
                        }
                    }
                }
            }
        }
    }
}
