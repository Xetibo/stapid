use bevy::{prelude::*, utils::Duration};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use game_objects::Totem;
use game_utils::PlayerDeadEvent;

pub mod game_utils;
use crate::game_utils::{
    AnimationTimer, BulletType, Collider, Direction, HitCooldownTimer, Name, ResetGameEvent,
    TimerType, UpdateUIEvent,
};

pub mod game_objects;
use crate::game_objects::{Bullet, Player, PowerUp, UINode, UIText, Wall, WallBundle};

pub mod constants;
use crate::constants::{BOTTOM_BOUND, LEFT_BOUND, PLAYER_SIZE, RIGHT_BOUND, TOP_BOUND};

pub mod collision;
use crate::collision::*;

pub mod level1;
use level1::spawn_level_1;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: String::from("stapid"),
                        resizable: true,
                        decorations: false,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Player>()
        .register_inspectable::<Bullet>()
        .register_inspectable::<Wall>()
        .add_event::<ResetGameEvent>()
        .add_event::<UpdateUIEvent>()
        .add_event::<PlayerDeadEvent>()
        .add_startup_system(spawn_walls)
        .add_startup_system(spawn_level_1)
        .add_startup_system(spawn_camera)
        .add_system(reset_clicked)
        .add_system(clear_totems)
        .add_system(spawn_player)
        .add_system(spawn_totem)
        .add_system(spawn_ui)
        .add_system(spawn_powerup)
        .add_system(move_all_players)
        .add_system(player_shoot)
        .add_system(move_all_bullets)
        .add_system(collision_player)
        .add_system(collision_bullet)
        .add_system(collision_powerup)
        .add_system(collision_explosion)
        .add_system(tick_timer)
        .add_system(animate_sprite)
        .add_system(update_ui)
        .run();
}

fn clear_totems(
    mut commands: Commands,
    totems: Query<Entity, With<Totem>>,
    mut event_reader: EventReader<ResetGameEvent>,
) {
    for _ in event_reader.iter() {
        for entity in &totems {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    existing_players: Query<Entity, With<Player>>,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<ResetGameEvent>,
) {
    for _ in event_reader.iter() {
        for entity in &existing_players {
            commands.entity(entity).despawn();
        }
        commands.spawn((
            Player::new(
                1,
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
                texture: asset_server.load("../assets/player_right.png"),
                transform: Transform {
                    translation: Vec3 {
                        x: -700.0,
                        y: 350.0,
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
                2,
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
                texture: asset_server.load("../assets/player_left.png"),
                transform: Transform {
                    translation: Vec3 {
                        x: 700.0,
                        y: 350.0,
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
        commands.spawn((
            Player::new(
                3,
                String::from("player3"),
                KeyCode::Y,
                KeyCode::U,
                KeyCode::T,
                KeyCode::G,
                KeyCode::H,
                KeyCode::F,
            ),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
                    ..default()
                },
                texture: asset_server.load("../assets/player_right.png"),
                transform: Transform {
                    translation: Vec3 {
                        x: -700.0,
                        y: -350.0,
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
            Name::new(String::from("player3")),
        ));
        commands.spawn((
            Player::new(
                4,
                String::from("player4"),
                KeyCode::O,
                KeyCode::P,
                KeyCode::I,
                KeyCode::K,
                KeyCode::L,
                KeyCode::J,
            ),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
                    ..default()
                },
                texture: asset_server.load("../assets/player_left.png"),
                transform: Transform {
                    translation: Vec3 {
                        x: 700.0,
                        y: -350.0,
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
            Name::new(String::from("player4")),
        ));
    }
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

fn tick_timer(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut Handle<Image>)>,
    mut timer_query: Query<(Entity, &mut HitCooldownTimer)>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for (entity, mut hit_timer) in &mut timer_query {
        hit_timer.timer.tick(time.delta());
        for (mut player, mut player_sprite) in &mut player_query {
            if hit_timer.timer.finished() && hit_timer.associated_player == player.name {
                match hit_timer.timer_type {
                    TimerType::Stun => {
                        player.stunned = false;
                        *player_sprite = asset_server.load(player.get_direction_sprite());
                        commands.entity(entity).despawn();
                    }
                    TimerType::Invulnerable => {
                        player.invulnerable = false;
                        commands.entity(entity).despawn();
                    }
                    TimerType::Shoot => {
                        player.shoot = true;
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}

fn animate_sprite(
    mut commands: Commands,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (entity, mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            if timer.counter < 1 {
                commands.entity(entity).despawn();
                return;
            }

            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            timer.counter -= 1;
        }
    }
}

fn move_all_players(
    mut players: Query<(&mut Player, &mut Transform, &mut Handle<Image>)>,
    timer: Res<Time>,
    keys: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    for (mut player, mut transform, mut player_sprite) in &mut players {
        if player.stunned == false {
            if keys.pressed(player.bindings.up)
                && keys.pressed(player.bindings.right)
                && !player.direction_block.up
                && !player.direction_block.right
            {
                *player_sprite = asset_server.load("../assets/player_right_up.png");
                let new_position_y =
                    transform.translation.y + 80. * player.speed * timer.delta_seconds();
                let new_position_x =
                    transform.translation.x + 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position_y.clamp(TOP_BOUND, BOTTOM_BOUND);
                transform.translation.x = new_position_x.clamp(LEFT_BOUND, RIGHT_BOUND);
                player.direction.direction_y = Direction::Up;
                player.direction.direction_x = Direction::Right;
            } else if keys.pressed(player.bindings.up)
                && keys.pressed(player.bindings.left)
                && !player.direction_block.up
                && !player.direction_block.left
            {
                *player_sprite = asset_server.load("../assets/player_left_up.png");
                let new_position_y =
                    transform.translation.y + 80. * player.speed * timer.delta_seconds();
                let new_position_x =
                    transform.translation.x - 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position_y.clamp(TOP_BOUND, BOTTOM_BOUND);
                transform.translation.x = new_position_x.clamp(LEFT_BOUND, RIGHT_BOUND);
                player.direction.direction_y = Direction::Up;
                player.direction.direction_x = Direction::Left;
            } else if keys.pressed(player.bindings.down)
                && keys.pressed(player.bindings.right)
                && !player.direction_block.down
                && !player.direction_block.right
            {
                *player_sprite = asset_server.load("../assets/player_right_down.png");
                let new_position_y =
                    transform.translation.y - 80. * player.speed * timer.delta_seconds();
                let new_position_x =
                    transform.translation.x + 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position_y.clamp(TOP_BOUND, BOTTOM_BOUND);
                transform.translation.x = new_position_x.clamp(LEFT_BOUND, RIGHT_BOUND);
                player.direction.direction_y = Direction::Down;
                player.direction.direction_x = Direction::Right;
            } else if keys.pressed(player.bindings.down)
                && keys.pressed(player.bindings.left)
                && !player.direction_block.down
                && !player.direction_block.left
            {
                *player_sprite = asset_server.load("../assets/player_left_down.png");
                let new_position_y =
                    transform.translation.y - 80. * player.speed * timer.delta_seconds();
                let new_position_x =
                    transform.translation.x - 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position_y.clamp(TOP_BOUND, BOTTOM_BOUND);
                transform.translation.x = new_position_x.clamp(LEFT_BOUND, RIGHT_BOUND);
                player.direction.direction_y = Direction::Down;
                player.direction.direction_x = Direction::Left;
            } else if keys.pressed(player.bindings.up) && !player.direction_block.up {
                *player_sprite = asset_server.load("../assets/player_up.png");
                let new_position =
                    transform.translation.y + 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position.clamp(TOP_BOUND, BOTTOM_BOUND);
                player.direction.direction_y = Direction::Up;
                player.direction.direction_x = Direction::None;
            } else if keys.pressed(player.bindings.down) && !player.direction_block.down {
                *player_sprite = asset_server.load("../assets/player_down.png");
                let new_position =
                    transform.translation.y - 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position.clamp(TOP_BOUND, BOTTOM_BOUND);
                player.direction.direction_y = Direction::Down;
                player.direction.direction_x = Direction::None;
            } else if keys.pressed(player.bindings.right) && !player.direction_block.right {
                *player_sprite = asset_server.load("../assets/player_right.png");
                let new_position =
                    transform.translation.x + 80. * player.speed * timer.delta_seconds();
                transform.translation.x = new_position.clamp(LEFT_BOUND, RIGHT_BOUND);
                player.direction.direction_x = Direction::Right;
                player.direction.direction_y = Direction::None;
            } else if keys.pressed(player.bindings.left) && !player.direction_block.left {
                *player_sprite = asset_server.load("../assets/player_left.png");
                let new_position =
                    transform.translation.x - 80. * player.speed * timer.delta_seconds();
                transform.translation.x = new_position.clamp(LEFT_BOUND, RIGHT_BOUND);
                player.direction.direction_x = Direction::Left;
                player.direction.direction_y = Direction::None;
            }
        }
    }
}

fn spawn_totem(
    mut commands: Commands,
    players: Query<(Entity, &Transform, &Player)>,
    asset_server: ResMut<AssetServer>,
    mut event_reader: EventReader<PlayerDeadEvent>,
) {
    for _ in event_reader.iter() {
        for (entity, transform, player) in players.iter() {
            if player.lifes > 1 {
                continue;
            }
            commands.entity(entity).despawn();
            commands.spawn((
                Totem {},
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
                        ..default()
                    },
                    texture: asset_server.load("../assets/dead.png"),
                    transform: Transform {
                        translation: transform.translation,
                        scale: Vec3::new(PLAYER_SIZE, PLAYER_SIZE, 1.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        }
    }
}

fn player_shoot(
    mut commands: Commands,
    mut players: Query<(&mut Player, &Transform)>,
    mut event_writer: EventWriter<UpdateUIEvent>,
    asset_server: ResMut<AssetServer>,
    keys: Res<Input<KeyCode>>,
) {
    for (mut player, transform) in &mut players {
        if keys.just_pressed(player.bindings.shoot) && !player.stunned && player.shoot {
            player.shoot = false;
            let (bullet_x, bullet_y) = player.get_bullet_spawn_position();
            commands.spawn((
                Bullet::normal_bullet(player.direction.clone()),
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
                        ..default()
                    },
                    texture: asset_server.load("../assets/bullet.png"),
                    transform: Transform {
                        translation: transform.translation
                            + Vec3 {
                                x: bullet_x,
                                y: bullet_y,
                                z: 0.0,
                            },
                        scale: Vec3 {
                            x: 30.0,
                            y: 30.0,
                            z: 1.0,
                        },
                        ..default()
                    },
                    ..default()
                },
            ));
            commands.spawn((HitCooldownTimer {
                timer: Timer::new(Duration::from_millis(200), TimerMode::Once),
                associated_player: player.name.clone(),
                timer_type: TimerType::Shoot,
            },));
        }
        if keys.just_pressed(player.bindings.shoot_special) && !player.stunned && player.powerup {
            let (bullet_x, bullet_y) = player.get_bullet_spawn_position();
            commands.spawn((
                Bullet::bullet_from_enum(player.power_up_type.as_ref(), &player.direction),
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
                        ..default()
                    },
                    texture: match player.power_up_type.clone().unwrap() {
                        BulletType::IceBullet => asset_server.load("../assets/freezing_bullet.png"),
                        BulletType::ExplosiveBullet => asset_server.load("../assets/granate.png"),
                        _ => asset_server.load("../assets/bouncy_ball.png"),
                    },
                    transform: Transform {
                        translation: transform.translation
                            + Vec3 {
                                x: bullet_x,
                                y: bullet_y,
                                z: 0.0,
                            },
                        scale: Vec3 {
                            x: 30.0,
                            y: 30.0,
                            z: 0.0,
                        },
                        ..default()
                    },
                    ..default()
                },
            ));
            player.powerup = false;
            player.power_up_type = None;
            event_writer.send_default();
        }
    }
}

fn spawn_powerup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    power_up_query: Query<Entity, With<PowerUp>>,
) {
    let mut count = 0;
    for _entity in &power_up_query {
        count += 1;
    }
    if count < 2 {
        commands.spawn((
            PowerUp {
                pickup_type: BulletType::IceBullet,
            },
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
                    ..default()
                },
                texture: asset_server.load("../assets/coin.png"),
                transform: Transform {
                    translation: PowerUp::generate_random_position(),
                    scale: Vec3 {
                        x: 40.0,
                        y: 40.0,
                        z: 0.0,
                    },
                    ..default()
                },
                ..default()
            },
            Collider,
        ));
    }
}

fn spawn_walls(mut commands: Commands) {
    commands.spawn(WallBundle::new(Direction::Up));
    commands.spawn(WallBundle::new(Direction::Down));
    commands.spawn(WallBundle::new(Direction::Right));
    commands.spawn(WallBundle::new(Direction::Left));
}

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<ResetGameEvent>,
    existing_node: Query<Entity, With<UINode>>,
) {
    for _ in event_reader.iter() {
        for entity in &existing_node {
            commands.entity(entity).despawn_recursive();
        }
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Percent(100.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        flex_direction: FlexDirection::Column,
                        flex_wrap: FlexWrap::NoWrap,
                        overflow: Overflow::Visible,
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                    ..default()
                },
                UINode {},
            ))
            .with_children(|parent| {
                for n in 1..5 {
                    parent.spawn((
                        TextBundle::from_section(
                            format!("Player {}\nLifes: 3\nSpecial:\nNone\n\n", n),
                            TextStyle {
                                font: asset_server.load("fonts/font.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(1.0)),
                            ..default()
                        }),
                        UIText {},
                    ));
                }
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(100.0), Val::Px(60.0)),
                            margin: UiRect::all(Val::Px(8.0)),
                            ..default()
                        },
                        background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                        ..default()
                    })
                    .with_children(|subparent| {
                        subparent.spawn(TextBundle::from_section(
                            "Reset",
                            TextStyle {
                                font: asset_server.load("fonts/font.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        ));
                    });
            });
    }
}

fn update_ui(
    mut text_query: Query<&mut Text, With<UIText>>,
    player_query: Query<&Player>,
    asset_server: Res<AssetServer>,
    mut event_reader_hit: EventReader<UpdateUIEvent>,
) {
    for _ in event_reader_hit.iter() {
        let mut player_iter = player_query.iter();
        for mut text_node in &mut text_query {
            let maybe_player = player_iter.next();
            if !maybe_player.is_some() {
                *text_node = Text::from_section(
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/font.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                );
                continue;
            }
            let player = maybe_player.unwrap();
            let mut powerup = BulletType::NormalBullet;
            if player.power_up_type.is_some() {
                powerup = player.power_up_type.clone().unwrap();
            }
            *text_node = Text::from_section(
                format!(
                    "Player {}\nLifes: {}\nSpecial:\n{}\n\n",
                    player.player_number, player.lifes, powerup
                ),
                TextStyle {
                    font: asset_server.load("fonts/font.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            );
        }
    }
}

fn reset_clicked(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut event_writer: EventWriter<ResetGameEvent>,
) {
    for interaction in &interaction_query {
        match *interaction {
            Interaction::Clicked => {
                event_writer.send_default();
            }
            _ => (),
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
