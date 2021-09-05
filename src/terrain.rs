use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_egui::{egui, EguiContext};
use noise::*;

#[derive(Default)]
pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<Terrain>()
      .add_startup_system(startup.system())
      .add_system(load_terrain.system())
      .add_system(terrain_gui.system());
  }
}

#[derive(Debug, PartialEq)]
enum NoiseType {
  Perlin,
  Ridged,
  Fbm,
  Worley,
  Blended,
}

struct Terrain {
  loaded: bool,
  noise_type: NoiseType,
  scale_bias: Point2<f64>,
  scale_point: Point4<f64>,
  perlin: Perlin,
  fbm: Fbm,
  ridged: RidgedMulti,
  worley: Worley,
}
impl Default for Terrain {
  fn default() -> Self {
    Terrain {
      loaded: false,
      noise_type: NoiseType::Blended,
      scale_bias: [5.0, 2.5],
      scale_point: [0.04, 0.04, 1.0, 1.0],
      perlin: Perlin::default(),
      fbm: Fbm::default(),
      ridged: RidgedMulti::default(),
      worley: Worley::default(),
    }
  }
}

impl Terrain {
  fn get_scaled(&self, src: &dyn NoiseFn<Point2<f64>>, coords: Point2<f64>) -> f64 {
    let scale_bias = ScaleBias::new(src)
      .set_scale(self.scale_bias[0])
      .set_bias(self.scale_bias[1]);
    let scale_point = ScalePoint::new(&scale_bias).set_all_scales(
      self.scale_point[0],
      self.scale_point[1],
      self.scale_point[2],
      self.scale_point[3],
    );
    scale_point.get(coords)
  }

  pub fn get(&self, coords: Point2<f64>) -> f64 {
    match &self.noise_type {
      NoiseType::Perlin => self.get_scaled(&self.perlin, coords),
      NoiseType::Ridged => self.get_scaled(&self.ridged, coords),
      NoiseType::Fbm => self.get_scaled(&self.fbm, coords),
      NoiseType::Worley => self.get_scaled(&self.worley, coords),
      NoiseType::Blended => {
        let blend = Blend::new(&self.fbm, &self.ridged, &self.perlin);
        return self.get_scaled(&blend, coords);
      }
    }
  }
}

fn load_terrain(mut commands: Commands, mut terrain: ResMut<Terrain>, mut map_query: MapQuery) {
  if terrain.loaded {
    return;
  }

  for x in 0..128u16 {
    for y in 0..128u16 {
      // ignore error?
      let position = UVec2::new(x.into(), y.into());
      let mut color = terrain.get([x.into(), y.into()]) as u16;
      if color > 3 {
        color = 3u16
      }

      let _ = map_query.set_tile(
        &mut commands,
        position,
        Tile {
          texture_index: color,
          ..Default::default()
        },
        0u16,
        0u16,
      );

      map_query.notify_chunk_for_tile(position, 0u16, 0u16);
    }
  }

  terrain.loaded = true;
}

fn terrain_gui(egui_context: ResMut<EguiContext>, mut terrain_state: ResMut<Terrain>) {
  egui::Window::new("Terrain").show(egui_context.ctx(), |ui| {
    egui::Grid::new("properties")
      .spacing([40.0, 4.0])
      .striped(true)
      .show(ui, |ui| {
        ui.label("Noise Type");
        egui::ComboBox::from_label("Noise Type")
          .selected_text(format!("{:?}", terrain_state.noise_type))
          .show_ui(ui, |ui| {
            terrain_state.loaded = terrain_state.loaded
              && !ui
                .selectable_value(&mut terrain_state.noise_type, NoiseType::Blended, "Blended")
                .changed();
            terrain_state.loaded = terrain_state.loaded
              && !ui
                .selectable_value(&mut terrain_state.noise_type, NoiseType::Perlin, "Perlin")
                .changed();
            terrain_state.loaded = terrain_state.loaded
              && !ui
                .selectable_value(&mut terrain_state.noise_type, NoiseType::Ridged, "Ridged")
                .changed();
            terrain_state.loaded = terrain_state.loaded
              && !ui
                .selectable_value(&mut terrain_state.noise_type, NoiseType::Fbm, "Fbm")
                .changed();
            terrain_state.loaded = terrain_state.loaded
              && !ui
                .selectable_value(&mut terrain_state.noise_type, NoiseType::Worley, "Worley")
                .changed();
          });
        ui.end_row();
        ui.label("Value scale");
        terrain_state.loaded = terrain_state.loaded
          && !ui
            .add(egui::Slider::new(
              &mut terrain_state.scale_bias[0],
              0.0..=10.0,
            ))
            .changed();
        ui.end_row();
        ui.label("Value bias");
        terrain_state.loaded = terrain_state.loaded
          && !ui
            .add(egui::Slider::new(
              &mut terrain_state.scale_bias[1],
              0.0..=10.0,
            ))
            .changed();
        ui.end_row();

        ui.label("Scale X");
        terrain_state.loaded = terrain_state.loaded
          && !ui
            .add(egui::Slider::new(
              &mut terrain_state.scale_point[0],
              0.0..=0.1,
            ))
            .changed();
        ui.end_row();
        ui.label("Scale Y");
        terrain_state.loaded = terrain_state.loaded
          && !ui
            .add(egui::Slider::new(
              &mut terrain_state.scale_point[1],
              0.0..=0.10,
            ))
            .changed();
        ui.end_row();
        ui.label("Scale Z");
        terrain_state.loaded = terrain_state.loaded
          && !ui
            .add(egui::Slider::new(
              &mut terrain_state.scale_point[2],
              0.0..=0.10,
            ))
            .changed();
        ui.end_row();
        ui.label("Scale W");
        terrain_state.loaded = terrain_state.loaded
          && !ui
            .add(egui::Slider::new(
              &mut terrain_state.scale_point[3],
              0.0..=0.10,
            ))
            .changed();
        ui.end_row();

        ui.label("Terrain");
        ui.checkbox(&mut terrain_state.loaded, "Loaded");
        ui.end_row();
      });
  });
}

fn startup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut map_query: MapQuery,
) {
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());

  let texture_handle = asset_server.load("tiles4.png");
  let material_handle = materials.add(ColorMaterial::texture(texture_handle));

  // Create map entity and component:
  let map_entity = commands.spawn().id();
  let mut map = Map::new(0u16, map_entity);

  let mut map_settings = LayerSettings::new(
    UVec2::new(4, 4),
    UVec2::new(32, 32),
    Vec2::new(100.0, 50.0),
    Vec2::new(400.0, 66.0),
  );
  map_settings.mesh_type = TilemapMeshType::Isometric(IsoType::Diamond);

  // terrain layer
  let (mut layer_terrain, layer_terrain_entity) =
    LayerBuilder::<TileBundle>::new(&mut commands, map_settings.clone(), 0u16, 0u16);
  map.add_layer(&mut commands, 0u16, layer_terrain_entity);

  map_query.build_layer(&mut commands, layer_terrain, material_handle.clone());

  // Spawn Map
  // Required in order to use map_query to retrieve layers/tiles.
  commands
    .entity(map_entity)
    .insert(map)
    .insert(Transform::from_xyz(0.0, 1024.0, 0.0))
    .insert(GlobalTransform::default());
}
