mod debug;
mod helpers;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;

use rand::{thread_rng, Rng};

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
    .add_plugin(TilemapPlugin)
    .add_plugin(debug::DebugPlugin)
    .add_system(exit_on_esc_system.system())
    .add_startup_system(startup.system())
    .add_system(helpers::camera::movement.system())
    .add_system(helpers::texture::set_texture_filters_to_nearest.system())
    .run();
}

fn startup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut map_query: MapQuery,
) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());

  let texture_handle = asset_server.load("iso_color.png");
  let material_handle = materials.add(ColorMaterial::texture(texture_handle));

  // Create map entity and component:
  let map_entity = commands.spawn().id();
  let mut map = Map::new(0u16, map_entity);

  let mut map_settings = LayerSettings::new(
      UVec2::new(2, 2),
      UVec2::new(32, 32),
      Vec2::new(64.0, 32.0),
      Vec2::new(384.0, 32.0),
  );
  map_settings.mesh_type = TilemapMeshType::Isometric(IsoType::Diamond);

  // Layer 0
  let (mut layer_0, layer_0_entity) =
      LayerBuilder::<TileBundle>::new(&mut commands, map_settings.clone(), 0u16, 0u16);
  map.add_layer(&mut commands, 0u16, layer_0_entity);

  layer_0.fill(
      UVec2::new(0, 0),
      UVec2::new(32, 32),
      Tile {
          texture_index: 0,
          ..Default::default()
      }
      .into(),
  );
  layer_0.fill(
      UVec2::new(32, 0),
      UVec2::new(64, 32),
      Tile {
          texture_index: 1,
          ..Default::default()
      }
      .into(),
  );
  layer_0.fill(
      UVec2::new(0, 32),
      UVec2::new(32, 64),
      Tile {
          texture_index: 2,
          ..Default::default()
      }
      .into(),
  );
  layer_0.fill(
      UVec2::new(32, 32),
      UVec2::new(64, 64),
      Tile {
          texture_index: 3,
          ..Default::default()
      }
      .into(),
  );

  map_query.build_layer(&mut commands, layer_0, material_handle.clone());

  // Make 2 layers on "top" of the base map.
  for z in 0..5 {
      let mut new_settings = map_settings.clone();
      new_settings.layer_id = z + 1;
      let (mut layer_builder, layer_entity) = LayerBuilder::new(
          &mut commands,
          new_settings.clone(),
          0u16,
          new_settings.layer_id,
      );
      map.add_layer(&mut commands, new_settings.layer_id, layer_entity);

      let mut random = thread_rng();

      for _ in 0..1000 {
          let position = UVec2::new(random.gen_range(0..128), random.gen_range(0..128));
          // Ignore errors for demo sake.
          let _ = layer_builder.set_tile(
              position,
              TileBundle {
                  tile: Tile {
                      texture_index: 0 + z + 1,
                      ..Default::default()
                  },
                  ..Default::default()
              },
          );
      }

      map_query.build_layer(&mut commands, layer_builder, material_handle.clone());
  }

  // Spawn Map
  // Required in order to use map_query to retrieve layers/tiles.
  commands
      .entity(map_entity)
      .insert(map)
      .insert(Transform::from_xyz(0.0, 1024.0, 0.0))
      .insert(GlobalTransform::default());
}