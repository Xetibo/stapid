use bevy::prelude::*;
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
        Player { pos_x: 0.0f32, pos_y: 0.0f32, speed: 2.5, }
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
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_system(move_all_players)
        .run();

}

fn spawn_player(mut commands: Commands) {
    commands.spawn((Player::new(),Name(String::from("PingPang"))));
}

fn move_all_players(query: Query<&Name, With<Player>>) {
    for name in query.iter() {
        println!("{} moved!", name.0);
    }
}
