use bevy::{prelude::*, sprite::collide_aabb::collide, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
use std::time::Duration;

const WALL_THICKNESS: f32 = 10.0;
const WALL_TOP: f32 = -500.0;
const WALL_BOTTOM: f32 = 500.0;
const WALL_LEFT: f32 = -500.0;
const WALL_RIGHT: f32 = 500.0;

#[derive(Component, Inspectable)]
pub struct Player {
    pub size: i32,
    pub lifes: i32,
    pub invulnerable: bool,
    pub speed: f32,
    pub direction: Direction,
    pub name: String,
    pub bindings: Bindings,
}

#[derive(Component, Inspectable)]
pub struct Bullet {
    pub speed: f32,
    pub direction: Direction,
}

#[derive(Bundle)]
struct Wall {
    direction: Direction,
    sprite_bundle: SpriteBundle,
    collider: Collider,
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

#[derive(Component)]
struct HitCooldownTimer {
    timer: Timer,
    associated_player: String,
}

#[derive(Component, Inspectable)]
pub struct Bindings {
    #[inspectable(ignore)]
    pub shoot: KeyCode,
    #[inspectable(ignore)]
    pub up: KeyCode,
    #[inspectable(ignore)]
    pub down: KeyCode,
    #[inspectable(ignore)]
    pub right: KeyCode,
    #[inspectable(ignore)]
    pub left: KeyCode,
}

impl Player {
    fn new(
        entered_name: String,
        entered_shootbind: KeyCode,
        entered_upbind: KeyCode,
        entered_downbind: KeyCode,
        entered_rightbind: KeyCode,
        entered_leftbind: KeyCode,
    ) -> Player {
        Player {
            size: 50,
            lifes: 3,
            invulnerable: false,
            speed: 2.5,
            direction: Direction::Up,
            name: entered_name,
            bindings: Bindings {
                shoot: entered_shootbind,
                up: entered_upbind,
                down: entered_downbind,
                right: entered_rightbind,
                left: entered_leftbind,
            },
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

impl Wall {
    fn new(entered_direction: Direction) -> Wall {
        Wall {
            direction: entered_direction.clone(),
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: match entered_direction {
                        Direction::Up => Vec3 {
                            x: 0.0,
                            y: WALL_TOP,
                            z: (0.0),
                        },
                        Direction::Down => Vec3 {
                            x: 0.0,
                            y: WALL_BOTTOM,
                            z: (0.0),
                        },
                        Direction::Right => Vec3 {
                            x: WALL_RIGHT,
                            y: 0.0,
                            z: (0.0),
                        },
                        Direction::Left => Vec3 {
                            x: WALL_LEFT,
                            y: 0.0,
                            z: (0.0),
                        },
                    },
                    scale: match entered_direction {
                        Direction::Up | Direction::Down => Vec3 {
                            x: 1000.0,
                            y: WALL_THICKNESS,
                            z: (1.0),
                        },
                        Direction::Right | Direction::Left => Vec3 {
                            x: WALL_THICKNESS,
                            y: 1000.0,
                            z: (1.0),
                        },
                    },
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    ..default()
                },
                ..default()
            },
            collider: Collider {},
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
                    x: 50.0,
                    y: 50.0,
                    z: 0.0,
                },
                ..default()
            },
            ..default()
        },
        Collider,
        Direction::Up,
        Name(String::from("player1")),
    ));
    commands.spawn((
        Player::new(
            String::from("player2"),
            KeyCode::RControl,
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
                    x: 50.0,
                    y: 50.0,
                    z: 0.0,
                },
                ..default()
            },
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
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    mut collider_query: Query<(Entity, &Transform, Option<&mut Player>), With<Collider>>,
) {
    for (bullet_entity, bullet_transform) in &bullet_query {
        let bullet_size = bullet_transform.scale.truncate();
        for (collider_entity, transform, mut maybe_player) in &mut collider_query {
            let collision = collide(
                bullet_transform.translation,
                bullet_size,
                transform.translation,
                transform.scale.truncate(),
            );
            if collision.is_some() {
                commands.entity(bullet_entity).despawn();
                if maybe_player.is_some() {
                    let player = &mut **maybe_player.as_mut().unwrap();
                    if player.invulnerable == false {
                        if player.lifes > 1 {
                            player.decrement_life();
                            player.invulnerable = true;
                            commands.spawn((HitCooldownTimer {
                                timer: Timer::new(Duration::from_secs(2), TimerMode::Once),
                                associated_player: player.name.clone(),
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
                player.invulnerable = false;
                commands.entity(entity).despawn();
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
        if keys.pressed(player.bindings.up) {
            transform.translation.y += 80. * player.speed * timer.delta_seconds();
            player.direction = Direction::Up;
        }
        if keys.pressed(player.bindings.down) {
            transform.translation.y -= 80. * player.speed * timer.delta_seconds();
            player.direction = Direction::Down;
        }
        if keys.pressed(player.bindings.right) {
            transform.translation.x += 80. * player.speed * timer.delta_seconds();
            player.direction = Direction::Right;
        }
        if keys.pressed(player.bindings.left) {
            transform.translation.x -= 80. * player.speed * timer.delta_seconds();
            player.direction = Direction::Left;
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
        if keys.just_pressed(player.bindings.shoot) {
            match player.direction {
                Direction::Up => {
                    commands.spawn((
                        Bullet::new(player.direction.clone()),
                        MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
                            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                            transform: Transform::from_translation(
                                transform.translation
                                    + Vec3 {
                                        x: 0.0,
                                        y: 50.0,
                                        z: 0.0,
                                    },
                            ),
                            ..default()
                        },
                        Name(String::from("Bullet from player1")),
                    ));
                }
                Direction::Down => {
                    commands.spawn((
                        Bullet::new(player.direction.clone()),
                        MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
                            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                            transform: Transform::from_translation(
                                transform.translation
                                    + Vec3 {
                                        x: 0.0,
                                        y: -50.0,
                                        z: 0.0,
                                    },
                            ),
                            ..default()
                        },
                        Name(String::from("Bullet from player1")),
                    ));
                }
                Direction::Right => {
                    commands.spawn((
                        Bullet::new(player.direction.clone()),
                        MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
                            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                            transform: Transform::from_translation(
                                transform.translation
                                    + Vec3 {
                                        x: 50.0,
                                        y: 0.0,
                                        z: 0.0,
                                    },
                            ),
                            ..default()
                        },
                        Name(String::from("Bullet from player1")),
                    ));
                }
                Direction::Left => {
                    commands.spawn((
                        Bullet::new(player.direction.clone()),
                        MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
                            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                            transform: Transform::from_translation(
                                transform.translation
                                    + Vec3 {
                                        x: -50.0,
                                        y: 0.0,
                                        z: 0.0,
                                    },
                            ),
                            ..default()
                        },
                        Name(String::from("Bullet from player1")),
                    ));
                }
            }
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
