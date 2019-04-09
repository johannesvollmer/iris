#![feature(duration_float)]

#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate bitflags;

extern crate nalgebra as na;

mod bxdf;
mod camera;
mod film;
mod geometry;
mod integrator;
mod light;
mod material;
mod math;
mod sampler;
mod scene;
mod texture;

use bumpalo::Bump;
use camera::Camera;
use film::spectrum::Spectrum;
use integrator::Integrator;
use math::*;
use rayon::prelude::*;
use sampler::Sampler;
use std::sync::Arc;

const TILE_SIZE: i32 = 16;

fn main() {
    if cfg!(debug_assertions) {
        render(100, 100, "out.png", 1);
    } else {
        render(500, 500, "out.png", 25);
    }
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

    let sampler = sampler::random::RandomSampler::new(spp as u32);

    let scene = test_scene();

    let integrator = integrator::whitted::Whitted::new(10);
    // let integrator = integrator::normals::Normals::new();

    let camera = camera::orthographic::OrthographicCamera::new(
        Transform::new(na::Projective3::identity()),
        Bounds2f::new(Point2f::new(0.0, 0.0), Point2f::new(1.0, 1.0)),
        0.0,
        0.0,
        film.clone(),
    );

    let progress_bar = indicatif::ProgressBar::new(ntiles as u64);

    progress_bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{wide_bar} {pos}/{len} [{elapsed} - ETA {eta}]"),
    );

    progress_bar.tick();

    let thread_work = |tile_idx: i32| {
        let horizontal = tile_idx % tile_dims.x;
        let vertical = tile_idx / tile_dims.x;
        let bounds = Bounds2i::new(
            Point2i::new(horizontal * TILE_SIZE, vertical * TILE_SIZE),
            Point2i::new(
                ((horizontal + 1) * TILE_SIZE).min(resolution.max.x),
                ((vertical + 1) * TILE_SIZE).min(resolution.max.y),
            ),
        );

        let mut film_tile = film::Film::get_film_tile(bounds);

        let mut sampler = sampler.clone_seed(tile_idx as u64);

        let arena = Bump::new();

        for pixel in bounds {
            sampler.start_pixel(pixel);

            while let Some(_) = sampler.next_sample() {
                let camera_sample = sampler.get_camera_sample(pixel);

                if let Some((mut ray_diff, _)) = camera.generate_ray_differential(&camera_sample) {
                    ray_diff
                        .scale_differentials(1.0 / (sampler.samples_per_pixel() as Float).sqrt());

                    let mut sample =
                        integrator.radiance(&ray_diff.ray, &scene, sampler.as_mut(), &arena, 0);
                    if cfg!(debug_assertions) {
                        if sample.has_nans() {
                            eprintln!("Sample at pixel {}, {} has NaNs", pixel.x, pixel.y);
                            sample = Spectrum::black();
                        } else if sample.has_infs() {
                            eprintln!("Sample at pixel ({}, {}) has infs", pixel.x, pixel.y);
                            sample = Spectrum::black();
                        } else if sample.has_negatives() {
                            eprintln!("Sample at pixel ({}, {}) has negatives", pixel.x, pixel.y);
                            sample = Spectrum::black();
                        } // TODO: Check sample.y() < 0
                    }

                    film_tile.add_sample(camera_sample.film, &sample);
                }
            }
        }

        film.merge_tile(film_tile);
        progress_bar.inc(1);
    };

    match std::env::var("THREADS") {
        Ok(_) => (0..ntiles).for_each(thread_work),
        Err(_) => (0..ntiles).into_par_iter().for_each(thread_work),
    }

    film.write_to_file(filename).unwrap();

    progress_bar.finish_and_clear();

    let end = std::time::SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Rendered in {}s", duration.as_float_secs());
}

fn test_scene() -> scene::Scene {
    use geometry::{primitive::Primitive, receiver::Receiver, sphere::Sphere};
    use light::emitter::Emitter;
    // use material::matte::Matte;
    use material::plastic::Plastic;
    use texture::constant::ConstantTexture;

    let mut geometry = Vec::new();

    geometry.push(Primitive::Receiver(Receiver::new(
        Arc::new(Sphere::new(0.3)),
        Arc::new(Plastic::new(
            Arc::new(ConstantTexture::new(Spectrum::from_rgb(0.25, 0.25, 0.25))),
            Arc::new(ConstantTexture::new(Spectrum::from_rgb(0.25, 0.25, 0.25))),
            Arc::new(ConstantTexture::new(0.20)),
        )),
        // Arc::new(Matte::new(
        //     Arc::new(ConstantTexture::new(Spectrum::from_rgb(0.25, 0.25, 0.25))),
        //     Some(Arc::new(ConstantTexture::new(0.0))),
        // )),
        Transform::translate(Vec3f::new(0.5, 0.5, 2.0)),
    )));

    // geometry.push(Primitive::Emitter(Emitter::new_point(
    //     Spectrum::from_rgb(0.0, 0.0, 1.0),
    //     Point3f::new(0.5, 0.5, 0.0),
    // )));

    // geometry.push(Primitive::Emitter(Emitter::new_spot(
    //     Spectrum::from_rgb(0.0, 0.0, 1.0),
    //     Point3f::new(0.5, 0.5, 0.0),
    //     Vec3f::new(0.0, 0.0, 1.0),
    //     3.0,
    //     5.0,
    // )));

    geometry.push(Primitive::Emitter(Emitter::new_area(
        Spectrum::from_rgb(0.0, 10.0, 0.0),
        Transform::translate(Vec3f::new(-0.5, 0.5, 1.0)),
        Arc::new(Sphere::new(0.1)),
    )));

    geometry.push(Primitive::Emitter(Emitter::new_area(
        Spectrum::from_rgb(10.0, 0.0, 0.0),
        Transform::translate(Vec3f::new(1.5, 0.5, 1.0)),
        Arc::new(Sphere::new(0.1)),
    )));

    scene::Scene::new(geometry)
}
