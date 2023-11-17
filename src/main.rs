use bevy::window::WindowTheme;
use bevy::{prelude::*, sprite::collide_aabb::collide, utils::Duration};
use game_objects::get_direction_sprite;

pub mod game_utils;
use crate::game_utils::{
    AnimationTimer, BulletType, Collider, Direction, DirectionHelper, HitCooldownTimer,
    InvulnerableBlinkTimer, Name, PlayerDeadEvent, PlayerHitEvent, PlayerPowerUpEvent,
    ResetGameEvent, TimerType, UpdateUIEvent,
};

pub mod game_objects;
use crate::game_objects::{Bullet, Player, PowerUp, Totem, UINode, UIText, WallBundle};

pub mod constants;
use crate::constants::{BOTTOM_BOUND, LEFT_BOUND, PLAYER_SIZE, RIGHT_BOUND, TOP_BOUND};

pub mod collision;
use crate::collision::*;

pub mod level1;
use level1::spawn_level_1;

fn main() {
    App::new()
        // expand functionality
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "stapid".into(),
                resolution: (1920., 1080.).into(),
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: true,
                    ..Default::default()
                },
                // This will spawn an invisible window
                // The window will be made visible in the make_visible() system after 3 frames.
                // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                visible: false,
                ..default()
            }),
            ..default()
        }),))
        // debug functionality
        // .register_inspectable::<Player>()
        // .register_inspectable::<Bullet>()
        // .register_inspectable::<Wall>()
        // events
        .add_event::<ResetGameEvent>()
        .add_event::<UpdateUIEvent>()
        .add_event::<PlayerDeadEvent>()
        .add_event::<PlayerHitEvent>()
        .add_event::<PlayerPowerUpEvent>()
        // happens once
        .add_systems(
            Startup,
            (
                spawn_walls,
                spawn_level_1,
                spawn_camera,
                spawn_ui,
                spawn_player,
            )
                .chain(),
        )
        // happens severy frame
        .add_systems(
            FixedUpdate,
            (
                collision_player,
                collision_bullet,
                collision_powerup,
                collision_explosion,
                move_all_bullets,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                reset_clicked,
                clear_totems,
                spawn_player,
                spawn_totem,
                reset_powerup,
                spawn_ui,
                spawn_powerup,
                player_invulnerable_blink,
                move_all_players,
                player_shoot,
                tick_timer,
                animate_sprite,
                update_ui,
            )
                .chain(),
        )
        .run();
}

fn clear_totems(
    mut commands: Commands,
    totems: Query<Entity, With<Totem>>,
    mut event_reader: EventReader<ResetGameEvent>,
) {
    for _ in event_reader.read() {
        for entity in &totems {
            commands.entity(entity).despawn();
        }
    }
}

fn reset_powerup(
    mut commands: Commands,
    powerups: Query<Entity, With<PowerUp>>,
    mut event_reader: EventReader<ResetGameEvent>,
    mut event_writer: EventWriter<PlayerPowerUpEvent>,
) {
    for _ in event_reader.read() {
        for entity in &powerups {
            commands.entity(entity).despawn();
            event_writer.send_default();
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    existing_players: Query<Entity, With<Player>>,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<ResetGameEvent>,
) {
    for _ in event_reader.read() {
        for entity in &existing_players {
            commands.entity(entity).despawn();
        }
        commands.spawn((
            Player::new(
                1,
                String::from("player1"),
                KeyCode::ControlLeft,
                KeyCode::ShiftLeft,
                KeyCode::W,
                KeyCode::S,
                KeyCode::D,
                KeyCode::A,
                DirectionHelper {
                    direction_x: Direction::Right,
                    direction_y: Direction::None,
                },
            ),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
                    ..default()
                },
                texture: asset_server.load("../assets/images/player/player_right.png"),
                transform: Transform {
                    translation: Vec3 {
                        x: -700.0,
                        y: 350.0,
                        z: 2.0,
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
            Direction::Right,
            Name::new(String::from("player1")),
        ));
        commands.spawn((
            Player::new(
                2,
                String::from("player2"),
                KeyCode::ControlRight,
                KeyCode::ShiftRight,
                KeyCode::Up,
                KeyCode::Down,
                KeyCode::Right,
                KeyCode::Left,
                DirectionHelper {
                    direction_x: Direction::Left,
                    direction_y: Direction::None,
                },
            ),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
                    ..default()
                },
                texture: asset_server.load("../assets/images/player/player_left.png"),
                transform: Transform {
                    translation: Vec3 {
                        x: 700.0,
                        y: 350.0,
                        z: 2.0,
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
            Direction::Left,
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
                DirectionHelper {
                    direction_x: Direction::Right,
                    direction_y: Direction::None,
                },
            ),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
                    ..default()
                },
                texture: asset_server.load("../assets/images/player/player_right.png"),
                transform: Transform {
                    translation: Vec3 {
                        x: -700.0,
                        y: -350.0,
                        z: 2.0,
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
            Direction::Right,
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
                DirectionHelper {
                    direction_x: Direction::Left,
                    direction_y: Direction::None,
                },
            ),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
                    ..default()
                },
                texture: asset_server.load("../assets/images/player/player_left.png"),
                transform: Transform {
                    translation: Vec3 {
                        x: 700.0,
                        y: -350.0,
                        z: 2.0,
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
            Direction::Left,
            Name::new(String::from("player4")),
        ));
    }
}

fn move_all_bullets(mut bullets: Query<(&Bullet, &mut Transform)>) {
    //, timer: Res<Time>) {
    for (bullet, mut transform) in &mut bullets {
        match bullet.direction.direction_y {
            Direction::Up => {
                transform.translation.y += bullet.speed; //* timer.delta_seconds();
            }
            Direction::Down => {
                transform.translation.y -= bullet.speed; //* timer.delta_seconds();
            }
            _ => (),
        }
        match bullet.direction.direction_x {
            Direction::Right => {
                transform.translation.x += bullet.speed; //* timer.delta_seconds();
            }
            Direction::Left => {
                transform.translation.x -= bullet.speed; //* timer.delta_seconds();
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
                        *player_sprite = asset_server.load(get_direction_sprite(
                            &player.direction.direction_x,
                            &player.direction.direction_y,
                        ));
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

fn player_invulnerable_blink(
    mut commands: Commands,
    mut players: Query<(&mut Sprite, &Player)>,
    mut event_reader: EventReader<PlayerHitEvent>,
    mut timer_query: Query<(Entity, &mut InvulnerableBlinkTimer)>,
    time: Res<Time>,
) {
    for (entity, mut hit_timer) in &mut timer_query {
        hit_timer.timer.tick(time.delta());
        for (mut sprite, player) in &mut players {
            if hit_timer.timer.finished() && hit_timer.associated_player == player.name {
                if !player.invulnerable {
                    sprite.color.set_a(1.0);
                    commands.entity(entity).despawn();
                    break;
                }
                if hit_timer.color {
                    sprite.color.set_a(1.0);
                    commands.spawn((InvulnerableBlinkTimer {
                        timer: Timer::new(Duration::from_millis(200), TimerMode::Once),
                        color: false,
                        associated_player: player.name.clone(),
                    },));
                } else {
                    sprite.color.set_a(0.5);
                    commands.spawn((InvulnerableBlinkTimer {
                        timer: Timer::new(Duration::from_millis(200), TimerMode::Once),
                        color: true,
                        associated_player: player.name.clone(),
                    },));
                }
                commands.entity(entity).despawn();
            }
        }
    }
    for _ in event_reader.read() {
        for (mut sprite, player) in &mut players {
            if !player.invulnerable {
                continue;
            }
            sprite.color.set_a(0.5);
            commands.spawn((InvulnerableBlinkTimer {
                timer: Timer::new(Duration::from_millis(200), TimerMode::Once),
                color: true,
                associated_player: player.name.clone(),
            },));
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
        if !player.stunned {
            if keys.pressed(player.bindings.up)
                && keys.pressed(player.bindings.right)
                && !player.direction_block.up
                && !player.direction_block.right
            {
                *player_sprite = asset_server.load("../assets/images/player/player_right_up.png");
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
                *player_sprite = asset_server.load("../assets/images/player/player_left_up.png");
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
                *player_sprite = asset_server.load("../assets/images/player/player_right_down.png");
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
                *player_sprite = asset_server.load("../assets/images/player/player_left_down.png");
                let new_position_y =
                    transform.translation.y - 80. * player.speed * timer.delta_seconds();
                let new_position_x =
                    transform.translation.x - 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position_y.clamp(TOP_BOUND, BOTTOM_BOUND);
                transform.translation.x = new_position_x.clamp(LEFT_BOUND, RIGHT_BOUND);
                player.direction.direction_y = Direction::Down;
                player.direction.direction_x = Direction::Left;
            } else if keys.pressed(player.bindings.up) && !player.direction_block.up {
                *player_sprite = asset_server.load("../assets/images/player/player_up.png");
                let new_position =
                    transform.translation.y + 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position.clamp(TOP_BOUND, BOTTOM_BOUND);
                player.direction.direction_y = Direction::Up;
                player.direction.direction_x = Direction::None;
            } else if keys.pressed(player.bindings.down) && !player.direction_block.down {
                *player_sprite = asset_server.load("../assets/images/player/player_down.png");
                let new_position =
                    transform.translation.y - 80. * player.speed * timer.delta_seconds();
                transform.translation.y = new_position.clamp(TOP_BOUND, BOTTOM_BOUND);
                player.direction.direction_y = Direction::Down;
                player.direction.direction_x = Direction::None;
            } else if keys.pressed(player.bindings.right) && !player.direction_block.right {
                *player_sprite = asset_server.load("../assets/images/player/player_right.png");
                let new_position =
                    transform.translation.x + 80. * player.speed * timer.delta_seconds();
                transform.translation.x = new_position.clamp(LEFT_BOUND, RIGHT_BOUND);
                player.direction.direction_x = Direction::Right;
                player.direction.direction_y = Direction::None;
            } else if keys.pressed(player.bindings.left) && !player.direction_block.left {
                *player_sprite = asset_server.load("../assets/images/player/player_left.png");
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
    for _ in event_reader.read() {
        for (entity, transform, player) in players.iter() {
            if player.lifes > 0 {
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
                    texture: asset_server.load("../assets/images/dead.png"),
                    transform: Transform {
                        translation: Vec3 {
                            x: transform.translation.x,
                            y: transform.translation.y,
                            z: 1.0,
                        },
                        scale: Vec3::new(PLAYER_SIZE, PLAYER_SIZE, 0.0),
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
                    texture: asset_server.load("../assets/images/bullet.png"),
                    transform: Transform {
                        translation: transform.translation
                            + Vec3 {
                                x: bullet_x,
                                y: bullet_y,
                                z: 2.0,
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
            commands.spawn((HitCooldownTimer {
                timer: Timer::new(Duration::from_millis(200), TimerMode::Once),
                associated_player: player.name.clone(),
                timer_type: TimerType::Shoot,
            },));
            commands.spawn(AudioBundle {
                source: asset_server.load("../assets/sounds/shot.wav"),
                ..default()
            });
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
                        BulletType::IceBullet => {
                            asset_server.load("../assets/images/freezing_bullet.png")
                        }
                        BulletType::ExplosiveBullet => {
                            asset_server.load("../assets/images/granate.png")
                        }
                        _ => asset_server.load("../assets/images/bouncy_ball.png"),
                    },
                    transform: Transform {
                        translation: transform.translation
                            + Vec3 {
                                x: bullet_x,
                                y: bullet_y,
                                z: 2.0,
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
            event_writer.send(UpdateUIEvent {
                player_number: player.player_number as usize,
            });
            commands.spawn(AudioBundle {
                source: asset_server.load("../assets/sounds/shot.wav"),
                ..default()
            });
        }
    }
}

fn spawn_powerup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    collision_query: Query<&Transform, With<Collider>>,
    mut event_reader: EventReader<PlayerPowerUpEvent>,
) {
    info!("spawn powerup");
    for _ in event_reader.read() {
        let mut powerup_transform = Transform { ..default() };
        loop {
            let mut collided = false;
            powerup_transform.translation = PowerUp::generate_random_position();
            powerup_transform.scale = Vec3 {
                x: 40.0,
                y: 40.0,
                z: 0.0,
            };
            for transform in &collision_query {
                let collision = collide(
                    transform.translation,
                    transform.scale.truncate(),
                    powerup_transform.translation,
                    powerup_transform.scale.truncate(),
                );
                if collision.is_some() {
                    collided = true;
                    break;
                }
            }
            if !collided {
                break;
            }
        }
        commands.spawn((
            PowerUp {
                pickup_type: BulletType::IceBullet,
            },
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Option::Some(Vec2 { x: 1.0, y: 1.0 }),
                    ..default()
                },
                texture: asset_server.load("../assets/images/coin.png"),
                transform: powerup_transform,
                ..default()
            },
            Collider,
        ));
    }
}

fn spawn_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("spawn walls");
    commands.spawn(WallBundle::new(Direction::Up, &asset_server));
    commands.spawn(WallBundle::new(Direction::Down, &asset_server));
    commands.spawn(WallBundle::new(Direction::Right, &asset_server));
    commands.spawn(WallBundle::new(Direction::Left, &asset_server));
}

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_reader: EventReader<ResetGameEvent>,
    existing_node: Query<Entity, With<UINode>>,
) {
    info!("spawn ui");
    for _ in event_reader.read() {
        for entity in &existing_node {
            commands.entity(entity).despawn_recursive();
        }
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        // size: Size::new(Val::Px(150.0), Val::Percent(100.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        flex_direction: FlexDirection::Column,
                        flex_wrap: FlexWrap::NoWrap,
                        overflow: Overflow::DEFAULT,
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                    ..default()
                },
                UINode {},
            ))
            .with_children(|parent| {
                for n in 1..5 {
                    parent
                        .spawn(ImageBundle {
                            style: Style {
                                // size: Size::new(Val::Px(144.0), Val::Px(108.0)),
                                padding: UiRect {
                                    left: Val::Px(20.0),
                                    right: Val::Px(20.0),
                                    top: Val::Px(10.0),
                                    bottom: Val::Px(10.0),
                                },
                                ..default()
                            },
                            image: asset_server.load("../assets/images/hud_64_48.png").into(),
                            ..default()
                        })
                        .with_children(|subparent| {
                            subparent.spawn((
                                TextBundle::from_section(
                                    format!("Player {}\nLifes: 3\nSpecial:\nNone\n\n", n),
                                    TextStyle {
                                        font: asset_server.load("fonts/PixeloidSans.ttf"),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_style(Style {
                                    margin: UiRect::all(Val::Px(1.0)),
                                    ..default()
                                }),
                                UIText {},
                            ));
                        });
                }
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            // size: Size::new(Val::Px(128.0), Val::Px(64.0)),
                            margin: UiRect::all(Val::Px(8.0)),
                            padding: UiRect {
                                left: Val::Px(17.0),
                                right: Val::Px(17.0),
                                top: Val::Px(15.0),
                                bottom: Val::Px(15.0),
                            },
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        image: asset_server.load("../assets/images/hud_64_32.png").into(),
                        ..default()
                    })
                    .with_children(|subparent| {
                        subparent.spawn(TextBundle::from_section(
                            "Reset",
                            TextStyle {
                                font: asset_server.load("fonts/PixeloidSans.ttf"),
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
    mut player_dead_event_writer: EventWriter<PlayerDeadEvent>,
) {
    info!("update ui");
    for event in event_reader_hit.read() {
        let mut text_nodes = text_query.iter_mut();
        for player in &player_query {
            if player.player_number as usize != event.player_number {
                continue;
            }
            let mut maybe_node = text_nodes.nth(event.player_number - 1);
            let text_node = &mut **maybe_node.as_mut().unwrap();
            if player.lifes < 1 {
                *text_node = Text::from_section(
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/PixeloidSans.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                );
                player_dead_event_writer.send_default();
                return;
            }
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
                    font: asset_server.load("fonts/PixeloidSans.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            );
            return;
        }
    }
}

fn reset_clicked(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut event_writer: EventWriter<ResetGameEvent>,
) {
    info!("reset clicked");
    for interaction in &interaction_query {
        #[allow(clippy::single_match)]
        match *interaction {
            Interaction::Pressed => {
                event_writer.send_default();
            }
            _ => (),
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    info!("spawn camera");
    commands.spawn(Camera2dBundle::default());
}
