use nalgebra::{ComplexField, Vector2};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Debug)]
pub struct PerlinNoise {
    grids: Vec<Vec<Vec<Vector2<f32>>>>,
    octaves: usize,
    persistence: f32,
}

impl PerlinNoise {
    pub fn new(size: usize, octaves: usize, persistence: f32, seed: u64) -> Self {
        Self {
            grids: (0..octaves).map(|i| Self::generate_grid(size, seed + i as u64)).collect(),
            octaves,
            persistence,
        }
    }

    fn generate_grid(size: usize, seed: u64) -> Vec<Vec<Vector2<f32>>> {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let mut grid = vec![vec![Vector2::zeros(); size + 2]; size + 2];

        for row in &mut grid {
            for vector in row.iter_mut() {
                let comp_x = rng.gen_range::<f32, _>(-5.0..=5.0);
                let comp_y = rng.gen_range::<f32, _>(-5.0..=5.0);
                *vector = Vector2::new(comp_x, comp_y).normalize();
            }
            let first_vector = row[0];
            *row.last_mut().unwrap() = first_vector;
        }

        for i in 0..(size + 2) {
            grid[size + 1][i] = grid[0][i];
        }

        grid
    }

    fn fade(t: f32) -> f32 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + t * (b - a)
    }

    /// Calculate the perlin noise value at a given `(x, y)` coordinate. The greatest `x` or `y`
    /// values should not exceed `self.size`
    pub fn evaluate(&self, x: f32, y: f32, octave: usize) -> f32 {
        let position_vector = Vector2::new(x, y);

        let x_floor = x.floor() as usize;
        let y_floor = y.floor() as usize;
        let x_ceil = x_floor + 1;
        let y_ceil = y_floor + 1;

        let top_left_vector = Vector2::new(x_floor as f32, y_floor as f32);
        let top_right_vector = Vector2::new(x_ceil as f32, y_floor as f32);
        let bottom_left_vector = Vector2::new(x_floor as f32, y_ceil as f32);
        let bottom_right_vector = Vector2::new(x_ceil as f32, y_ceil as f32);

        let top_left_distance = position_vector - top_left_vector;
        let top_right_distance = position_vector - top_right_vector;
        let bottom_left_distance = position_vector - bottom_left_vector;
        let bottom_right_distance = position_vector - bottom_right_vector;

        let top_left_perlin = self.grids[octave][y_floor][x_floor];
        let top_right_perlin = self.grids[octave][y_floor][x_ceil];
        let bottom_left_perlin = self.grids[octave][y_ceil][x_floor];
        let bottom_right_perlin = self.grids[octave][y_ceil][x_ceil];

        let top_left_displacement = top_left_perlin.dot(&top_left_distance);
        let top_right_displacement = top_right_perlin.dot(&top_right_distance);
        let bottom_left_displacement = bottom_left_perlin.dot(&bottom_left_distance);
        let bottom_right_displacement = bottom_right_perlin.dot(&bottom_right_distance);

        let top_lerp = Self::lerp(
            top_left_displacement,
            top_right_displacement,
            Self::fade(x % 1.0),
        );
        let bottom_lerp = Self::lerp(
            bottom_left_displacement,
            bottom_right_displacement,
            Self::fade(x % 1.0),
        );

        Self::lerp(top_lerp, bottom_lerp, Self::fade(y % 1.0))
    }

    pub fn octave_evaluate(&self, x: f32, y: f32) -> f32 {
        (0..self.octaves)
            .map(|i| {
                self.persistence.powi(i as i32)
                    * self.evaluate(2.0.powi(i as i32) * x, 2.0.powi(i as i32) * y, i)
            })
            .sum()
    }

    /// Calculates the Perlin noise map in reverse order. This is done so that there will be no
    /// tiling or seams in the final texture
    pub fn reverse_octave_evaluate(&self, x: f32, y: f32) -> f32 {
        (0..self.octaves)
            .map(|old_i| {
                let i = (self.octaves - 1 - old_i) as i32;

                self.evaluate(x / (2.0.powi(i)), 0.5.powi(i) * y, old_i)
                    * self.persistence.powi(old_i as i32)
            })
            .sum()
    }
}
