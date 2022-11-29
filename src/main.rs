use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
#[derive(Component)]
struct Player {
    pos_x: f32,
    pos_y: f32,
    speed: f32,
}

#[derive(Component)]
struct Name(String);

impl Player {
    fn new() -> Player {
        Player {
            pos_x: 0.0f32,
            pos_y: 0.0f32,
            speed: 2.5,
        }
    }

    fn move_player(&mut self, value: f32, direction: bool) {
        if direction {
            self.pos_x = value * self.speed;
        } else {
            self.pos_y = value * self.speed;
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
        .add_startup_system(spawn_player)
        .add_startup_system(display_sprite)
        // .add_system(move_all_players)
        .run();
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((Player::new(), Name(String::from("PingPang"))));
}

fn move_all_players(query: Query<&Name, With<Player>>) {
    for name in query.iter() {
        println!("{} moved!", name.0);
    }
}

fn display_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    // commands.spawn(SpriteBundle {
    //     texture: asset_server.load("../assets/stick.png"),
    //     ..default()
    // });
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.5, 0.5),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        ..default()
    }); 

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
        ..default()
    });
}
