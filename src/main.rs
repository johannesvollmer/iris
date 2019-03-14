#![feature(const_fn)]

#[macro_use]
extern crate derive_new;

mod film;
mod geometry;
mod math;
mod scene;

use math::*;

fn main() {

    let bounds = Bounds2i::new(Point2i::new(0, 0), Point2i::new(128, 128));

    let film = film::Film::new(128, 128);

    let mut film_tile = film::Film::get_film_tile(bounds);

    for point in bounds {
        film_tile.add_sample(Point2f::new(point.x as f32 + 0.5, point.y as f32 + 0.5), &film::spectrum::Spectrum::new(1.0, 1.0, 0.0));
    }

    film.merge_tile(film_tile);

    film.write_to_file("out.png").unwrap();
}
