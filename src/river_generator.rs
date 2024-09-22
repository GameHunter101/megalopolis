use nalgebra::Vector2;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;

use crate::perlin_noise::PerlinNoise;

#[derive(Debug)]
/// A river is represented by a bezier curve. The curviness of the river is produced iteratively.
/// The size parameter is the radius around the curve at which pixels will be considered to be part
/// of the river. Might introduce a falloff parameter later
pub struct River {
    pub starting_point: Vector2<f32>,
    pub ending_point: Vector2<f32>,
    pub control_points: [Vector2<f32>; 2],
    size: f32,
    rng: ChaCha8Rng,
}

impl River {
    pub fn new(terrain_size: f32, size: f32, seed: u64) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let starting_side = rng.gen_range(0..4);
        let starting_offset = rng.gen_range(0.0..terrain_size);
        let starting_point = match starting_side {
            0 => Vector2::new(starting_offset, 0.0),
            1 => Vector2::new(starting_offset, terrain_size),
            2 => Vector2::new(0.0, starting_offset),
            3 => Vector2::new(terrain_size, starting_offset),
            _ => Vector2::zeros(),
        };

        let ending_side = {
            let mut rand = rng.gen_range(0..4);
            while rand == starting_side {
                rand = rng.gen_range(0..4);
            }
            rand
        };
        let ending_offset = rng.gen_range(0.0..terrain_size);
        let ending_point = match ending_side {
            0 => Vector2::new(ending_offset, 0.0),
            1 => Vector2::new(ending_offset, terrain_size),
            2 => Vector2::new(0.0, ending_offset),
            3 => Vector2::new(terrain_size, ending_offset),
            _ => Vector2::zeros(),
        };

        let control_points = [
            PerlinNoise::lerp(starting_point, ending_point, 1.0 / 3.0)
                + Vector2::new(rng.gen_range(-0.1..=0.1), rng.gen_range(-0.1..=0.1)),
            PerlinNoise::lerp(starting_point, ending_point, 2.0 / 3.0)
                + Vector2::new(rng.gen_range(-0.1..=0.1), rng.gen_range(-0.1..=0.1)),
        ];

        dbg!(&starting_point, &ending_point);

        Self {
            starting_point,
            ending_point,
            control_points,
            size,
            rng,
        }
    }

    /// Iteratively shifts around the bezier control points, favoring to go outwards instead of
    /// inwards.
    pub fn random_shift(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.control_points = self
                .control_points
                .iter()
                .enumerate()
                .map(|(i, control_point)| {
                    let persuasion_vector = (*control_point
                        - PerlinNoise::lerp(
                            self.starting_point,
                            self.ending_point,
                            (i as f32 + 1.0) / 3.0,
                        ))
                    .normalize();
                    let random_vector =
                        Vector2::new(self.rng.gen_range(-1.0..1.0), self.rng.gen_range(-1.0..1.0))
                            .normalize();

                    let weighted_vector = random_vector * persuasion_vector.dot(&random_vector);

                    control_point + weighted_vector
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
        }
    }

    const NEWTON_METHOD_ITERATIONS: usize = 10;
    const EVALUATION_POINT_COUNT: usize = 10;
    pub fn create_texture(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        terrain_size: f32,
        resolution: u32,
    ) -> gamezap::texture::Texture {
        let bezier_points = [
            self.starting_point,
            self.control_points[0],
            self.control_points[1],
            self.ending_point,
        ];
        let river_size = self.size;
        let pixels = (0..resolution + 2)
            .into_par_iter()
            .flat_map(|y| {
                (0..resolution + 2).into_par_iter().flat_map(move |x| {
                    let position = Vector2::new(
                        x as f32 * terrain_size / resolution as f32,
                        y as f32 * terrain_size / resolution as f32,
                    );

                    let mut t_values = Vec::with_capacity(Self::EVALUATION_POINT_COUNT + 3);
                    t_values.push(0.0);

                    for i in 0..=Self::EVALUATION_POINT_COUNT {
                        t_values.push(
                            Self::newton_method_evaluate(
                                &bezier_points,
                                position,
                                i as f32 / Self::EVALUATION_POINT_COUNT as f32,
                                Self::NEWTON_METHOD_ITERATIONS,
                            )
                            .clamp(0.0, 1.0),
                        );
                    }

                    t_values.push(1.0);

                    let mut min_val = 10000000.0_f32;
                    for t in &t_values {
                        let dist =
                            Self::bezier_evaluate(&bezier_points, *t).metric_distance(&position);
                        if !dist.is_nan() && dist.is_finite() {
                            min_val = min_val.min(dist);
                        }
                    }

                    /* for t in &t_values {
                        let current_distance =
                            Self::bezier_evaluate(&bezier_points, *t).metric_distance(&position);
                        if current_distance < river_size {
                            return [255_u8, 0, 0, 255];
                        }
                    }
                    [0, 0, 0, 255] */
                    let faded_val = Self::fade(min_val / (resolution as f32));

                    let final_val = if faded_val > river_size {
                        0.0
                    } else {
                        faded_val / river_size
                    };

                    [
                        PerlinNoise::lerp(0.0, 255.0, final_val)
                            as u8,
                        0,
                        0,
                        255,
                    ]
                })
            })
            .collect::<Vec<_>>();

        let height_map =
            image::RgbaImage::from_vec(resolution + 2, resolution + 2, pixels).unwrap();
        /* let height_map = image::RgbaImage::from_fn(resolution + 2, resolution + 2, |x, y| {
        }); */

        gamezap::texture::Texture::from_rgba(
            device,
            queue,
            &height_map,
            Some("River height map"),
            true,
            true,
        )
        .unwrap()
    }

    fn fade(t: f32) -> f32 {
        t * (1.5 + t * (1.0 - 1.65 * t))
    }

    fn bezier_evaluate(bezier_points: &[Vector2<f32>], t: f32) -> Vector2<f32> {
        // Bezier polynomial coefficients
        let t_0_coefficient = bezier_points[0];
        let t_1_coefficient = -3.0 * bezier_points[0] + 3.0 * bezier_points[1];
        let t_2_coefficient =
            3.0 * bezier_points[0] - 6.0 * bezier_points[1] + 3.0 * bezier_points[2];
        let t_3_coefficient = -1.0 * bezier_points[0] + 3.0 * bezier_points[1]
            - 3.0 * bezier_points[2]
            + bezier_points[3];

        t_0_coefficient
            + t * t_1_coefficient
            + t * t * t_2_coefficient
            + t * t * t * t_3_coefficient
    }

    fn bezier_derivative_evaluate(bezier_points: &[Vector2<f32>], t: f32) -> Vector2<f32> {
        // Bezier polynomial derivative coefficients
        let t_0_coefficient = -3.0 * bezier_points[0] + 3.0 * bezier_points[1];
        let t_1_coefficient =
            2.0 * (3.0 * bezier_points[0] - 6.0 * bezier_points[1] + 3.0 * bezier_points[2]);
        let t_2_coefficient = 3.0
            * (-1.0 * bezier_points[0] + 3.0 * bezier_points[1] - 3.0 * bezier_points[2]
                + bezier_points[3]);

        t_0_coefficient + t * t_1_coefficient + t * t * t_2_coefficient
    }

    fn bezier_second_derivative_evaluate(bezier_points: &[Vector2<f32>], t: f32) -> Vector2<f32> {
        // Bezier polynomial second derivative coefficients
        let t_0_coefficient =
            2.0 * (3.0 * bezier_points[0] - 6.0 * bezier_points[1] + 3.0 * bezier_points[2]);
        let t_1_coefficient = 6.0
            * (-1.0 * bezier_points[0] + 3.0 * bezier_points[1] - 3.0 * bezier_points[2]
                + bezier_points[3]);

        t_0_coefficient + t * t_1_coefficient
    }

    fn newton_method_evaluate(
        bezier_points: &[Vector2<f32>],
        position: Vector2<f32>,
        initial_t: f32,
        remaining_iterations: usize,
    ) -> f32 {
        if remaining_iterations == 0 {
            return initial_t;
        }
        let bezier_point = Self::bezier_evaluate(bezier_points, initial_t);
        let bezier_derivative = Self::bezier_derivative_evaluate(bezier_points, initial_t);
        let bezier_second_derivative =
            Self::bezier_second_derivative_evaluate(bezier_points, initial_t);

        let pointing_vector = bezier_point - position;
        let angle_between_tangent_and_pointing_vectors = pointing_vector.dot(&bezier_derivative);

        let angle_between_tangent_and_pointing_derivative = bezier_derivative
            .dot(&bezier_derivative)
            + pointing_vector.dot(&bezier_second_derivative);

        Self::newton_method_evaluate(
            bezier_points,
            position,
            initial_t
                - angle_between_tangent_and_pointing_vectors
                    / angle_between_tangent_and_pointing_derivative,
            remaining_iterations - 1,
        )
    }
}
