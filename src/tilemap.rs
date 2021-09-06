use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use std::{
  collections::{HashMap, HashSet},
  fmt::Debug,
  hash::Hash,
  marker::PhantomData,
  time::{Duration, Instant},
};

#[derive(Default)]
pub struct TilemapPlugin<'a, TTile, TLayout, TLayer> {
  phantom: PhantomData<&'a (TTile, TLayout, TLayer)>,
}

impl<TTile, TLayout, TLayer> Plugin for TilemapPlugin<'static, TTile, TLayout, TLayer>
where
  TTile: 'static + Send + Sync,
  TLayer: 'static + Default + Send + Sync,
  TLayout: 'static + Layout<TSpaceCoords = Vec3>,
  <TLayout as Layout>::TChunkCoords: Default + Copy,
{
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<TileManager<TLayout::TChunkCoords, TLayer>>()
      .add_system(Self::chunk_spawner.system())
      .add_system(Self::chunk_loader.system())
      .add_system(Self::chunk_mesher.system())
      .add_system(Self::chunk_despawner.system())
      .add_system(Self::debug_gui.system());
  }
}

impl<TTile, TLayout, TLayer> TilemapPlugin<'static, TTile, TLayout, TLayer>
where
  TTile: 'static + Send + Sync,
  TLayer: 'static + Default + Send + Sync,
  TLayout: 'static + Layout<TSpaceCoords = Vec3>,
  <TLayout as Layout>::TChunkCoords: Default + Copy,
{
  fn chunk_spawner(
    mut commands: Commands,
    time: Res<Time>,
    layout: Res<TLayout>,
    mut tile_manager: ResMut<TileManager<'static, TLayout::TChunkCoords, TLayer>>,
    layer_qry: Query<&LayerComponent<TLayer>>,
    mut query: Query<(&Transform, &mut ChunkSiteComponent<TLayout::TChunkCoords>)>,
  ) {
    // load chunks around ChunkSites
    for (transform, mut site) in query.iter_mut() {
      // don't do anything if site didn't move
      if !site.fresh {
        continue;
      }
      // find which chunk we're currently on
      let current_chunk = layout.space_to_chunk(&transform.translation);

      // find neighboring chunks
      // TODO: parameterize loading distance
      let mut neighbors = layout.get_chunk_neighbors(&current_chunk, 2);
      neighbors.push(current_chunk.clone());

      for layer in layer_qry.iter() {
        // spawn chunks
        for chunk in neighbors.iter() {
          let _ = tile_manager.try_spawn_chunk(&mut commands, &layer.layer, chunk);
        }
      }

      site.fresh = false;
      site.last_loaded_chunk = Some(current_chunk);
    }
  }

  fn chunk_loader(layer_qry: Query<&LayerComponent<TLayer>>) {}

  fn chunk_mesher(layer_qry: Query<&LayerComponent<TLayer>>) {}

  fn chunk_despawner(layer_qry: Query<&LayerComponent<TLayer>>) {}

  fn debug_gui(egui_context: ResMut<EguiContext>, layer_qry: Query<&LayerComponent<TLayer>>) {
    egui::Window::new("Tilemap").show(egui_context.ctx(), |ui| {
      egui::Grid::new("debug")
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {});
    });
  }
}

#[derive(Default, Debug)]
pub struct ChunkSiteComponent<TChunkCoords> {
  pub last_loaded_chunk: Option<TChunkCoords>,
  pub fresh: bool,
}

pub trait Layout: Sync + Send {
  type TSpaceCoords: Sync + Send ;
  type TChunkCoords: Sync + Send ;
  type TTileCoords: Sync + Send ;

  fn tile_to_chunk(&self, tile: &Self::TTileCoords) -> Self::TChunkCoords;
  fn tile_to_space(&self, tile: &Self::TTileCoords) -> Self::TSpaceCoords;
  fn space_to_tile(&self, space: &Self::TSpaceCoords) -> Self::TTileCoords;
  fn space_to_chunk(&self, space: &Self::TSpaceCoords) -> Self::TChunkCoords {
    self.tile_to_chunk(&self.space_to_tile(space))
  }

  fn get_chunk_neighbors(
    &self,
    chunk: &Self::TChunkCoords,
    distance: u32,
  ) -> Vec<Self::TChunkCoords>;
  fn get_chunk_distance(&self, a: &Self::TChunkCoords, b: &Self::TChunkCoords) -> u32;
}

#[derive(Default)]
pub struct TileManager<'a, TChunkCoords, TLayer> {
  phantom: PhantomData<&'a (TChunkCoords, TLayer)>,
}
impl<'a, TChunkCoords, TLayer> TileManager<'a, TChunkCoords, TLayer> {
  pub fn try_spawn_chunk(
    &mut self,
    commands: &mut Commands,
    layer: &TLayer,
    chunk: &TChunkCoords,
  ) -> Option<Entity> {
    unimplemented!()
  }
}

pub struct LayerComponent<TLayer> {
  layer: TLayer,
}
