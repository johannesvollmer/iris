#![feature(const_fn)]
#![feature(duration_float)]

#[macro_use]
extern crate derive_new;

extern crate nalgebra as na;

mod camera;
mod film;
mod geometry;
mod integrator;
mod math;
mod sampler;
mod scene;

use crate::camera::Camera;
use film::spectrum::Spectrum;
use math::*;
use rayon::prelude::*;
use sampler::Sampler;
use std::sync::Arc;

const TILE_SIZE: i32 = 16;

fn main() {
    render(500, 500, "out.png", 1);
}

fn render(width: i32, height: i32, filename: &str, spp: i32) {
    let start = std::time::SystemTime::now();

    let film = Arc::new(film::Film::new(width, height));

    let resolution = Bounds2i::new(
        Point2i::new(0, 0),
        Point2i::new(width as i32, height as i32),
    );

    let tile_dims = Vec2i::new(
        (resolution.max.x + TILE_SIZE - 1) / TILE_SIZE,
        (resolution.max.y + TILE_SIZE - 1) / TILE_SIZE,
    );

    let ntiles = tile_dims.x * tile_dims.y;

    let sampler = sampler::uniform::UniformSampler::new(spp as u32);

    let camera = camera::orthographic::OrthographicCamera::new(
        Transform::new(na::Projective3::identity()),
        Bounds2f::new(Point2f::new(0.0, 0.0), Point2f::new(1.0, 1.0)),
        0.0,
        0.0,
        film.clone(),
    );

    let bar = indicatif::ProgressBar::new(ntiles as u64);

    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{wide_bar} {pos}/{len} [{elapsed} - ETA {eta}]"),
    );

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
                let camera_sample = sampler.get_camera_sample(pixel);

                if let Some((mut ray_diff, weight)) =
                    camera.generate_ray_differential(&camera_sample)
                {
                    ray_diff
                        .scale_differentials(1.0 / (sampler.samples_per_pixel() as Float).sqrt());

                    let sample =
                        Spectrum::new(sampler.get_1d(), sampler.get_1d(), sampler.get_1d()); // Li

                    film_tile.add_sample(Point2f::from(pixel) + Vec2f::new(0.5, 0.5), &sample);
                }
            }
        }

        film.merge_tile(film_tile);
        bar.inc(1);
    });

    film.write_to_file(filename).unwrap();

    bar.finish_and_clear();

    let end = std::time::SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Rendered in {}s", duration.as_float_secs());
}
