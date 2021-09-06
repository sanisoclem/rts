mod debug;
mod helpers;
mod terrain;
mod tilemap;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_egui::EguiPlugin;

fn main() {
  App::build()
    .insert_resource(Msaa { samples: 4 })
    .insert_resource(WindowDescriptor {
      title: "Biology!".to_string(),
      width: 800.,
      height: 600.,
      ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .add_plugins(DefaultPlugins)
    .add_plugin(EguiPlugin)
    .add_plugin(debug::DebugPlugin)
    .add_plugin(terrain::TerrainPlugin)
    .add_system(exit_on_esc_system.system())
    .run();
}
