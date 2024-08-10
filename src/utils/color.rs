use nannou::prelude::*;
use rand::Rng;

pub fn random_color() -> Rgba {
    let mut rng = rand::thread_rng();
    rgba(
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
    )
}

pub fn random_palette(n: usize) -> Vec<Rgba> {
    (0..n).map(|_| random_color()).collect()
}