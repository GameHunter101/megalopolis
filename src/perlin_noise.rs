use nalgebra::Vector2;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Debug)]
pub struct PerlinNoise {
    grid: Vec<Vec<Vector2<f32>>>,
    octaves: usize,
    persistence: f32,
    seed: u64,
}

impl PerlinNoise {
    pub fn new(size: usize, octaves: usize, persistence: f32, seed: u64) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        let mut grid = vec![vec![Vector2::zeros(); size + 1]; size + 1];

        for row in &mut grid {
            for vec in row {
                let comp_x = rng.gen_range::<f32, _>(-5.0..=5.0);
                let comp_y = rng.gen_range::<f32, _>(-5.0..=5.0);
                *vec = Vector2::new(comp_x, comp_y).normalize();
                print!("({comp_x:.2}, {comp_y:.2}) ");
            }
            println!();
        }

        Self {
            grid,
            octaves,
            persistence,
            seed,
        }
    }

    fn fade(t: f32) -> f32 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    /// Calculate the perlin noise value at a given `(x, y)` coordinate. The greatest `x` or `y`
    /// values should not exceed `self.size`
    pub fn evaluate(&self, x: f32, y: f32) -> f32 {
        let position_vector = Vector2::new(x, y);

        let x_floor = x.floor() as usize;
        let y_floor = y.floor() as usize;
        let x_ceil = x_floor + 1;
        let y_ceil = y_floor + 1;

        /* println!("floor: {x_floor:.2}, {y_floor:.2}");
        println!("ceil: {x_ceil:.2}, {y_ceil:.2}");
        println!(); */

        let bottom_left_vector = Vector2::new(x_floor as f32, y_floor as f32);
        let bottom_right_vector = Vector2::new(x_floor as f32, y_ceil as f32);
        let top_left_vector = Vector2::new(x_ceil as f32, y_floor as f32);
        let top_right_vector = Vector2::new(x_ceil as f32, y_ceil as f32);

        let bottom_left_perlin_vector = self.grid[y_floor][x_floor];
        let bottom_right_perlin_vector = self.grid[y_floor][x_ceil];
        let top_left_perlin_vector = self.grid[y_ceil][x_floor];
        let top_right_perlin_vector = self.grid[y_ceil][x_ceil];

        let bottom_left_dist = (position_vector - bottom_left_vector).normalize();
        let bottom_right_dist = (position_vector - bottom_right_vector).normalize();
        let top_left_dist = (position_vector - top_left_vector).normalize();
        let top_right_dist = (position_vector - top_right_vector).normalize();

        let bottom_left_displacement = bottom_left_dist.dot(&bottom_left_perlin_vector);
        let bottom_right_displacement = bottom_right_dist.dot(&bottom_right_perlin_vector);
        let top_left_displacement = top_left_dist.dot(&top_left_perlin_vector);
        let top_right_displacement = top_right_dist.dot(&top_right_perlin_vector);

        let x_frac = x.fract();
        let y_frac = y.fract();

        let bottom_left_t = Self::fade(1.0 - x_frac) * Self::fade(1.0 - y_frac);
        let bottom_right_t = Self::fade(x_frac) * Self::fade(1.0 - y_frac);
        let top_left_t = Self::fade(1.0 - x_frac) * Self::fade(y_frac);
        let top_right_t = Self::fade(x_frac) * Self::fade(y_frac);

        bottom_left_t * bottom_left_displacement
        + bottom_right_t * bottom_right_displacement
        + top_left_t * top_left_displacement
        + top_right_t * top_right_displacement
    }
}
