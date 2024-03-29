use crate::game_objects::{get_direction_sprite, Bullet, Explosion, Player, PowerUp};
use crate::game_utils::{
    AnimationTimer, BulletType, Collider, DirectionHelper, HitCooldownTimer, PlayerHitEvent,
    PlayerPowerUpEvent, TimerType, UpdateUIEvent,
};
use bevy::{
    prelude::*, sprite::collide_aabb::collide, sprite::collide_aabb::Collision, utils::Duration,
};
use rand::prelude::*;

pub fn collision_explosion(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Handle<Image>, &mut Player)>,
    mut collider_query: Query<(Entity, &Transform, &Explosion), With<Collider>>,
    mut event_writer: EventWriter<UpdateUIEvent>,
    mut event_writer_player_hit: EventWriter<PlayerHitEvent>,
    asset_server: ResMut<AssetServer>,
) {
    for (player_transform, mut player_sprite, mut player) in &mut player_query {
        if player.invulnerable {
            continue;
        }
        for (_collider_entity, transform, _explosion) in &mut collider_query {
            let collision = collide(
                transform.translation,
                transform.scale.truncate(),
                player_transform.translation,
                player_transform.scale.truncate(),
            );
            if collision.is_some() {
                *player_sprite = asset_server.load(get_direction_sprite(
                    &player.direction.direction_x,
                    &player.direction.direction_y,
                ));
                player.lifes -= 2;
                event_writer.send(UpdateUIEvent {
                    player_number: player.player_number as usize,
                });
                if player.lifes > 0 {
                    player.invulnerable = true;
                    commands.spawn((HitCooldownTimer {
                        timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
                        associated_player: player.name.clone(),
                        timer_type: TimerType::Invulnerable,
                    },));
                    event_writer_player_hit.send_default();
                }
            }
        }
    }
}

pub fn collision_powerup(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Player)>,
    mut collider_query: Query<(Entity, &Transform, &PowerUp), With<Collider>>,
    mut event_writer: EventWriter<UpdateUIEvent>,
    mut event_writer_powerup: EventWriter<PlayerPowerUpEvent>,
    asset_server: ResMut<AssetServer>,
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
                event_writer.send(UpdateUIEvent {
                    player_number: player.player_number as usize,
                });
                event_writer_powerup.send_default();

                commands.spawn(AudioBundle {
                    source: asset_server.load("/assets/sounds/powerup.wav"),
                    ..default()
                });
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn collision_player(
    collider_query: Query<
        &Transform,
        (
            With<Collider>,
            Without<Bullet>,
            Without<PowerUp>,
            Without<Explosion>,
        ),
    >,
    mut player_query: Query<(&Transform, &mut Player)>,
) {
    for (player_transform, mut player) in &mut player_query {
        let mut b_was_collision_up = false;
        let mut b_was_collision_down = false;
        let mut b_was_collision_right = false;
        let mut b_was_collision_left = false;
        for transform in collider_query.iter() {
            if let Some(direction) = collide(
                transform.translation,
                transform.scale.truncate(),
                player_transform.translation,
                player_transform.scale.truncate() + 1.15,
            ) {
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

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
pub fn collision_bullet(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &Transform, &mut Bullet)>,
    mut collider_query: Query<
        (&Transform, &mut Handle<Image>, Option<&mut Player>),
        With<Collider>,
    >,
    mut event_writer: EventWriter<UpdateUIEvent>,
    mut event_writer_player_hit: EventWriter<PlayerHitEvent>,
    asset_server: ResMut<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (bullet_entity, bullet_transform, mut bullet) in &mut bullet_query {
        let bullet_size = bullet_transform.scale.truncate();
        for (transform, mut player_sprite, mut maybe_player) in &mut collider_query {
            if let Some(collision) = collide(
                bullet_transform.translation,
                bullet_size,
                transform.translation,
                transform.scale.truncate(),
            ) {
                match bullet.bullet_type {
                    BulletType::NormalBullet => {
                        commands.entity(bullet_entity).despawn();
                        if maybe_player.is_none() {
                            commands.spawn(AudioBundle {
                                source: asset_server.load("/assets/sounds/hitwall.wav"),
                                ..default()
                            });
                            continue;
                        }
                        let player = &mut **maybe_player.as_mut().unwrap();
                        if !player.invulnerable {
                            player.decrement_life();
                            event_writer.send(UpdateUIEvent {
                                player_number: player.player_number as usize,
                            });
                            commands.spawn(AudioBundle {
                                source: asset_server.load("/assets/sounds/hit.wav"),
                                ..default()
                            });
                            if player.lifes > 0 {
                                event_writer_player_hit.send_default();
                                player.stunned = false;
                                *player_sprite = asset_server.load(get_direction_sprite(
                                    &player.direction.direction_x,
                                    &player.direction.direction_y,
                                ));
                                player.invulnerable = true;
                                commands.spawn((HitCooldownTimer {
                                    timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
                                    associated_player: player.name.clone(),
                                    timer_type: TimerType::Invulnerable,
                                },));
                            }
                        }
                    }
                    BulletType::IceBullet => {
                        commands.entity(bullet_entity).despawn();
                        if maybe_player.is_none() {
                            commands.spawn(AudioBundle {
                                source: asset_server.load("/assets/sounds/hitwall.wav"),
                                ..default()
                            });
                            continue;
                        }
                        let player = &mut **maybe_player.as_mut().unwrap();
                        if !player.invulnerable && !player.stunned {
                            player.stunned = true;
                            *player_sprite =
                                asset_server.load("/assets/images/player/player_frozen.png");
                            commands.spawn(AudioBundle {
                                source: asset_server.load("/assets/sounds/frozen.wav"),
                                ..default()
                            });
                            commands.spawn((HitCooldownTimer {
                                timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
                                associated_player: player.name.clone(),
                                timer_type: TimerType::Stun,
                            },));
                        }
                    }
                    BulletType::ExplosiveBullet => {
                        commands.entity(bullet_entity).despawn();
                        let texture_handle =
                            asset_server.load("/assets/images/explosion_anim.png");
                        let texture_atlas = TextureAtlas::from_grid(
                            texture_handle,
                            Vec2::new(32.0, 32.0),
                            3,
                            1,
                            None,
                            None,
                        );
                        let texture_atlas_handle = texture_atlases.add(texture_atlas);
                        commands.spawn(AudioBundle {
                            source: asset_server.load("/assets/sounds/explosion.wav"),
                            ..default()
                        });
                        commands.spawn((
                            SpriteSheetBundle {
                                sprite: TextureAtlasSprite {
                                    custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
                                    ..default()
                                },
                                texture_atlas: texture_atlas_handle,
                                transform: Transform {
                                    translation: bullet_transform.translation,
                                    scale: Vec3 {
                                        x: 150.0,
                                        y: 150.0,
                                        z: 1.0,
                                    },
                                    ..default()
                                },
                                ..default()
                            },
                            AnimationTimer {
                                timer: Timer::from_seconds(0.05, TimerMode::Repeating),
                                counter: 2,
                            },
                            Explosion { radius: 50.0 },
                            Collider,
                        ));
                    }
                    BulletType::BouncyBullet => {
                        bullet.direction = match collision {
                            Collision::Left | Collision::Right => DirectionHelper {
                                direction_x: bullet.direction.direction_x.opposite(),
                                direction_y: bullet.direction.direction_y.clone(),
                            },
                            _ => DirectionHelper {
                                direction_x: bullet.direction.direction_x.clone(),
                                direction_y: bullet.direction.direction_y.opposite(),
                            },
                        };
                        bullet.bounces_left -= 1;
                        if maybe_player.is_none() {
                            if bullet.bounces_left < 1 {
                                commands.entity(bullet_entity).despawn();
                                commands.spawn(AudioBundle {
                                    source: asset_server.load("/assets/sounds/hitwall.wav"),
                                    ..default()
                                });
                                continue;
                            }
                            commands.spawn(AudioBundle {
                                source: asset_server.load("/assets/sounds/bouncywall.wav"),
                                ..default()
                            });
                            continue;
                        }
                        let player = &mut **maybe_player.as_mut().unwrap();
                        *player_sprite = asset_server.load(get_direction_sprite(
                            &player.direction.direction_x,
                            &player.direction.direction_y,
                        ));
                        if player.invulnerable {
                            continue;
                        }
                        player.decrement_life();
                        event_writer.send(UpdateUIEvent {
                            player_number: player.player_number as usize,
                        });
                        commands.spawn(AudioBundle {
                            source: asset_server.load("/assets/sounds/hit.wav"),
                            ..default()
                        });
                        if player.lifes > 0 {
                            event_writer_player_hit.send_default();
                            commands.entity(bullet_entity).despawn();
                            player.stunned = false;
                            player.invulnerable = true;
                            commands.spawn((HitCooldownTimer {
                                timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
                                associated_player: player.name.clone(),
                                timer_type: TimerType::Invulnerable,
                            },));
                        }
                    }
                }
            }
        }
    }
}
