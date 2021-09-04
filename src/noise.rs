use bevy::prelude::*;
use noise::*;
use std::{collections::HashMap, marker::PhantomData};


pub trait TerrainGenerator: Sync + Send {
  type TVoxelId: VoxelId;

  fn scale(&self) -> Vec3;
  fn set_scale(&mut self, scale: Vec3);
  fn bias(&self) -> f32;
  fn set_bias(&mut self, scale: f32);

  //fn get_voxel_value(&self, voxel: &Self::TVoxelId) -> f32;
  fn generate_voxel_data(&self, buffer: &mut HashMap<Self::TVoxelId, VoxelData>);
}


pub struct DefaultNoiseGenerator<TVoxelId: VoxelId> {
    seed: u32,
    bias: f64,
    scale: Point3<f64>,
    phantom: PhantomData<TVoxelId>,
}

impl<TVoxelId: VoxelId> TerrainGenerator for DefaultNoiseGenerator<TVoxelId> {
    type TVoxelId = TVoxelId;

    #[inline]
    fn scale(&self) -> Vec3 {
        Vec3::new(
            self.scale[0] as f32,
            self.scale[1] as f32,
            self.scale[2] as f32,
        )
    }

    #[inline]
    fn set_scale(&mut self, scale: Vec3) {
        self.scale = [scale.x() as f64, scale.y() as f64, scale.z() as f64];
    }

    #[inline]
    fn bias(&self) -> f32 {
        self.bias as f32
    }

    #[inline]
    fn set_bias(&mut self, bias: f32) {
        self.bias = bias as f64;
    }

    fn generate_voxel_data(&self, buffer: &mut HashMap<Self::TVoxelId, VoxelData>) {
        let perlin = Perlin::new();
        let ridged = RidgedMulti::new();
        let fbm = Fbm::new();
        let blend: Blend<Point2<f64>> = Blend::new(&perlin, &ridged, &fbm);
        let scale_bias = ScaleBias::new(&blend).set_bias(self.bias as f64);
        let generator = ScalePoint::new(&scale_bias).set_all_scales(
            self.scale[0],
            self.scale[1],
            self.scale[2],
            1.0,
        );
        for (voxel, mut data) in buffer {
            data.value = generator.get([voxel.u() as f64, voxel.v() as f64]) as f32;
        }
    }
}
impl<TVoxelId: VoxelId> Default for DefaultNoiseGenerator<TVoxelId> {
    fn default() -> Self {
        DefaultNoiseGenerator {
            seed: 0,
            bias: 0.0,
            scale: [1.0f64; 3],
            phantom: PhantomData,
        }
    }
}
