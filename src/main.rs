mod debug;
mod helpers;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::EguiPlugin;
use noise::*;

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
    //.add_system(helpers::texture::set_texture_filters_to_nearest.system())
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
      UVec2::new(6, 6),
      UVec2::new(32, 32),
      Vec2::new(64.0, 32.0),
      Vec2::new(384.0, 32.0),
  );
  map_settings.mesh_type = TilemapMeshType::Isometric(IsoType::Diamond);

  // terrain layer
  let (mut layer_terrain, layer_terrain_entity) =
      LayerBuilder::<TileBundle>::new(&mut commands, map_settings.clone(), 0u16, 0u16);
  map.add_layer(&mut commands, 0u16, layer_terrain_entity);

  let perlin = Perlin::new();
  let ridged = RidgedMulti::new();
  let fbm = Fbm::new();
  let blend: Blend<Point2<f64>> = Blend::new(&perlin, &ridged, &fbm);
  let scale_bias = ScaleBias::new(&blend).set_bias(2.0).set_scale(4.0);
  let generator = ScalePoint::new(&scale_bias).set_all_scales(
      0.05,
      0.05,
      1.0,
      1.0,
  );
  for x in 0..1024u16 {
    for y in 0..1024u16 {
      // ignore error?
      let _ = layer_terrain.set_tile(
        UVec2::new(x.into(), y.into()),
        TileBundle {
            tile: Tile {
                texture_index: (generator.get([x.into(), y.into()]) as u16) % 4u16,
                ..Default::default()
            },
            ..Default::default()
        },
      );
    }
  }


  map_query.build_layer(&mut commands, layer_terrain, material_handle.clone());

  // Spawn Map
  // Required in order to use map_query to retrieve layers/tiles.
  commands
      .entity(map_entity)
      .insert(map)
      .insert(Transform::from_xyz(0.0, 1024.0, 0.0))
      .insert(GlobalTransform::default());
}