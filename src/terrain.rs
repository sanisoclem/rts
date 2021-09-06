use crate::tilemap::Layout;
use bevy::{prelude::*, math::swizzles::*};

use std::{
  collections::HashMap,
  hash::Hash,
  ops::{Add, Sub},
};

pub struct SquareTileLayout {
  chunk_size: UVec2,
  tile_size: Vec2,
}

impl Default for SquareTileLayout {
  fn default() -> Self {
    SquareTileLayout {
      chunk_size: UVec2::new(100u32, 100u32),
      tile_size: Vec2::new(10.0, 10.0),
    }
  }
}

impl Layout for SquareTileLayout {
  type TChunkCoords = IVec2;
  type TSpaceCoords = Vec3;
  type TTileCoords = IVec4;

  fn tile_to_chunk(&self, tile: &Self::TTileCoords) -> Self::TChunkCoords {
    tile.xy()
  }

  fn tile_to_space(&self, tile: &Self::TTileCoords) -> Self::TSpaceCoords {
    let xz = ((tile.xy() * self.chunk_size.as_i32()) + tile.zw()) * self.tile_size.as_i32();
    Vec3::new(xz.x as f32, 0.0, xz.y as f32)
  }

  fn space_to_tile(&self, space: &Self::TSpaceCoords) -> Self::TTileCoords {
    let abs_tile = IVec2::new(
      space.x.div_euclid(self.tile_size.x) as i32,
      space.z.div_euclid(self.tile_size.y) as i32,
    );
    let chunk = IVec2::new(
      abs_tile.x.div_euclid(self.chunk_size.x as i32),
      abs_tile.y.div_euclid(self.chunk_size.y as i32),
    );
    let relative_tile = IVec2::new(
      abs_tile.x.rem_euclid(self.chunk_size.x as i32),
      abs_tile.y.rem_euclid(self.chunk_size.y as i32),
    );
    (chunk, relative_tile).into()
  }

  fn get_chunk_neighbors(
    &self,
    chunk: &Self::TChunkCoords,
    distance: u32,
  ) -> Vec<Self::TChunkCoords> {
    (1..=(distance as i32))
      .flat_map(move |ring| {
        (0..(2 * ring)).flat_map(move |offset| {
          [
            IVec2::new(-ring + offset, -ring),
            IVec2::new(ring, -ring + offset),
            IVec2::new(ring - offset, ring),
            IVec2::new(ring, ring - offset),
          ]
        })
      })
      .map(|relative_chunks| relative_chunks + *chunk)
      .collect()
  }

  fn get_chunk_distance(&self, a: &Self::TChunkCoords, b: &Self::TChunkCoords) -> f32 {
    (*a - *b).abs().as_f32().length()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use proptest::prelude::*;

  prop_compose! {
    fn arb_IVec2()(x in -1000..1000, y in -1000..1000) -> IVec2 {
      IVec2::new(x, y)
    }
  }

  proptest! {
    #[test]
    fn chunk_should_have_appropriate_number_of_neighbors(chunk in arb_IVec2(), distance in 1u32..=100u32) {
      let layout = SquareTileLayout::default();
      let count =  layout.get_chunk_neighbors(&chunk, distance).len();
      let expected = ((distance * 2) + 1) * ((distance * 2) + 1) - 1;
      assert_eq!(expected, count as u32);
    }

    #[test]
    fn neighbor_should_have_correct_distance(chunk in arb_IVec2(), distance in 1u32..=100u32) {
      let layout = SquareTileLayout::default();
      let neighbors = layout.get_chunk_neighbors(&chunk, distance);
      let max_distance: f32 = (distance as f32 * distance as f32).sqrt();
      for neighbor in neighbors.iter() {
        let distance = layout.get_chunk_distance(&chunk, &neighbor);
        assert!(distance <= max_distance, "Max: {:?}, Actual: {:?}", max_distance, distance);
      }
    }

    #[test]
    fn neighbor_should_be_mutual(chunk in arb_IVec2(), distance in 1u32..=100u32) {
      let layout = SquareTileLayout::default();
      let neighbors = layout.get_chunk_neighbors(&chunk, distance);
      for neighbor in neighbors.iter() {
        let ns = layout.get_chunk_neighbors(neighbor, distance);
        let is_neighbors_with_original = ns.iter().any(|&n| n == chunk);
        assert!(is_neighbors_with_original, "Origin: {:?}, Distance: {:?}, neighbor: {:?}", chunk, distance, neighbor);
      }
    }

    #[test]
    fn tile_space_coordinates_should_be_reversible(chunk in arb_IVec2(), tile_x in 0..100, tile_y in 0..100) {
      let layout = SquareTileLayout::default();
      let tile = IVec4::new(chunk.x, chunk.y, tile_x, tile_y);
      let space_coords = layout.tile_to_space(&tile);
      let result = layout.space_to_tile(&space_coords);
      assert_eq!(result, tile, "Coords: {:?}", space_coords);
    }
  }
}
