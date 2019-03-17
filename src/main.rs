#![feature(const_fn)]

#[macro_use]
extern crate derive_new;

extern crate nalgebra as na;

mod film;
mod geometry;
mod math;
mod sampler;
mod scene;
mod camera;

use film::spectrum::Spectrum;
use math::*;
use rayon::prelude::*;
use sampler::Sampler;

const TILE_SIZE: i32 = 16;

fn main() {
    render(512, 512, "out.png");
}

fn render(width: usize, height: usize, filename: &str) {
    let film = film::Film::new(width, height);

    let resolution = Bounds2i::new(
        Point2i::new(0, 0),
        Point2i::new(width as i32, height as i32),
    );

    let tile_dims = Vec2i::new(
        (resolution.max.x + TILE_SIZE - 1) / TILE_SIZE,
        (resolution.max.y + TILE_SIZE - 1) / TILE_SIZE,
    );

    let ntiles = tile_dims.x * tile_dims.y;

    let sampler = sampler::uniform::UniformSampler::new(1);

    (0..ntiles).into_par_iter().for_each(|tile_idx| {
        let horizontal = tile_idx % tile_dims.x;
        let vertical = tile_idx / tile_dims.y;
        let bounds = Bounds2i::new(
            Point2i::new(horizontal * TILE_SIZE, vertical * TILE_SIZE),
            Point2i::new(
                ((horizontal + 1) * TILE_SIZE).min(resolution.max.x),
                ((vertical + 1) * TILE_SIZE).min(resolution.max.y),
            ),
        );

        let mut film_tile = film::Film::get_film_tile(bounds);

        let mut sampler = sampler.clone_seed(tile_idx as u64);

        for pixel in bounds {
            sampler.start_pixel(pixel);

            while let Some(_) = sampler.next_sample() {
                let sample = Spectrum::new(1.0, 0.0, 1.0);
                film_tile.add_sample(Point2f::from(pixel) + Vec2f::new(0.5, 0.5), &sample);
            }
        }

        film.merge_tile(film_tile);
    });

    film.write_to_file("out.png").unwrap();
}
