#![feature(const_fn)]

#[macro_use]
extern crate derive_new;

mod film;
mod geometry;
mod math;
mod scene;

use film::spectrum::Spectrum;
use math::*;
use rand::prelude::*;
use rayon::prelude::*;

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

        let mut rng = rand::thread_rng();
        let sample = Spectrum::new(rng.gen(), rng.gen(), rng.gen());

        for point in bounds {
            film_tile.add_sample(Point2f::from(point) + Vec2f::new(0.5, 0.5), &sample);
        }

        film.merge_tile(film_tile);
    });

    film.write_to_file("out.png").unwrap();
}
