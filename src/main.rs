mod main_menu;
mod game;
mod state;

use bevy::prelude::*;
use state::GameState;

fn main() {
  App::new()
    .insert_resource(Msaa { samples: 4 })
    // Set WindowDescriptor Resource to change title and size
    .insert_resource(WindowDescriptor {
        title: "Biology !".to_string(),
        width: 800.,
        height: 600.,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .add_state(GameState::MainMenu)
    .add_plugins(DefaultPlugins)
    .add_plugin(main_menu::MainMenuPlugin::default())
    .add_plugin(game::GamePlugin::default())
    .run();
}
