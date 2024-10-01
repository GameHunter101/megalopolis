use gamezap::texture::Texture;
use nalgebra::Vector2;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::perlin_noise::PerlinNoise;

#[derive(Debug)]
pub struct ResourceMap {
    origin_points: Vec<Vector2<f32>>,
    magnitude: f32,
    spread: f32,
}

impl ResourceMap {
    pub fn new(
        splat_count: usize,
        splat_spread: f32,
        spread_between_splats: f32,
        points_per_splat: usize,
        magnitude: f32,
        spread: f32,
        texture_size: u32,
        seed: u64,
    ) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut origin_points = Vec::new();
        for i in 0..splat_count {
            let mut test = Vector2::new(
                rng.gen_range(0..texture_size) as f32,
                rng.gen_range(0..texture_size) as f32,
            );
            for mut j in 0..(i) {
                let dist_vector: Vector2<f32> = test - origin_points[j * points_per_splat];
                let dist_squared = dist_vector.dot(&dist_vector);
                /* dbg!(
                    &test,
                    &dist_squared,
                    spread_between_splats * spread_between_splats
                ); */
                if dist_squared < spread_between_splats * spread_between_splats {
                    test = Vector2::new(
                        rng.gen_range(0..texture_size) as f32,
                        rng.gen_range(0..texture_size) as f32,
                    );

                    j = 0;
                }
            }
            origin_points.push(test);
            for _ in 1..=points_per_splat {
                let splat_origin = &origin_points[i * points_per_splat];
                origin_points.push(Vector2::new(
                    rng.gen_range((splat_origin.x - splat_spread)..(splat_origin.x + splat_spread)),
                    rng.gen_range((splat_origin.y - splat_spread)..(splat_origin.y + splat_spread)),
                ));
            }
        }

        ResourceMap {
            origin_points,
            magnitude,
            spread,
        }
    }

    pub fn create_resource_map(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        resolution: u32,
    ) -> Texture {
        let pixels = (0..resolution * resolution)
            .into_par_iter()
            .flat_map(|i| {
                let y = (i / resolution) as f32;
                let x = (i % resolution) as f32;

                let min_distance = self
                    .origin_points
                    .iter()
                    .map(|point| {
                        ((point.x - x) * (point.x - x) + (point.y - y) * (point.y - y)).sqrt()
                    })
                    .enumerate()
                    .reduce(|acc, (i, e)| {
                        if i == 0 {
                            return (0, e);
                        }
                        let k = self.magnitude;
                        let h = (0.5 + 0.5 * (e - acc.1) / k).clamp(0.0, 1.0);
                        (i, PerlinNoise::lerp(e, acc.1, h) - k * h * (1.0 - h))
                    })
                    .unwrap()
                    .1;
                if min_distance <= self.magnitude {
                    let val = PerlinNoise::lerp(
                        0.0,
                        255.0,
                        (self.magnitude - min_distance).clamp(0.0, self.spread) / self.spread,
                    ) as u8;
                    [val, 255, 0, 255]
                } else {
                    [0, 0, 0, 255]
                }
            })
            .collect::<Vec<_>>();

        let resource_map = image::RgbaImage::from_vec(resolution, resolution, pixels).unwrap();

        gamezap::texture::Texture::from_rgba(
            device,
            queue,
            &resource_map,
            Some("Resource Map"),
            true,
            true,
        )
        .unwrap()
    }
}
