#![feature(duration_float)]

#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate bitflags;

extern crate nalgebra as na;

mod bxdf;
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
use film::spectrum::Spectrum;
use integrator::Integrator;
use math::*;
use rayon::prelude::*;
use sampler::Sampler;
use std::sync::Arc;

const TILE_SIZE: i32 = 16;

fn main() {
    if cfg!(debug_assertions) {
        render(100, 100, "out.exr", 1);
    } else {
        render(500, 500, "out.exr", 25);
    }
}

fn render(width: i32, height: i32, filename: &str, spp: i32) {
    let start = std::time::SystemTime::now();

    let filter = Box::new(film::filter::Mitchell::new(2.0, 1.0 / 3.0, 1.0 / 3.0));
    let film = Arc::new(film::Film::new(width, height, TILE_SIZE, filter));

    let sampler = sampler::random::RandomSampler::new(spp as u32);

    let scene = test_scene();

    let integrator = integrator::path::Path::new(4, 8);
    // let integrator = integrator::whitted::Whitted::new(10);
    // let integrator = integrator::normals::Normals::new();

    let camera = film::camera::PerspectiveCamera::new(
        Transform::look_at(
            Point3f::new(0.0, 1.0, 0.0),
            Point3f::new(0.0, 1.0, 2.0),
            Vec3f::new(0.0, 1.0, 0.0),
        )
        .inverse(),
        70.0,
        &*film,
    );

    let progress_bar = indicatif::ProgressBar::new(film.ntiles as u64);

    progress_bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{wide_bar} {pos}/{len} [{elapsed} - ETA {eta}]"),
    );

    progress_bar.tick();

    let thread_work = |tile_idx: i32| {
        let mut film_tile = film.get_film_tile(tile_idx);

        let mut sampler = sampler.clone_seed(tile_idx as u64);

        let arena = Bump::new();

        for pixel in film_tile.sample_bounds {
            sampler.start_pixel(pixel);

            while let Some(_) = sampler.next_sample() {
                let camera_sample = sampler.get_camera_sample(pixel);

                if let Some((ray, _)) = camera.generate_ray(&camera_sample) {
                    let mut sample = integrator.radiance(&ray, &scene, sampler.as_mut(), &arena, 0);
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
                        } else if sample.y() < 0.0 {
                            eprintln!(
                                "Sample at pixel ({}. {}) has negative luminance",
                                pixel.x, pixel.y
                            );
                            sample = Spectrum::black();
                        }
                    }

                    film_tile.add_sample(camera_sample.film, &sample);
                }
            }
        }

        film.merge_tile(film_tile);
        progress_bar.inc(1);
    };

    match std::env::var("THREADS") {
        Ok(_) => (0..film.ntiles).for_each(thread_work),
        Err(_) => (0..film.ntiles).into_par_iter().for_each(thread_work),
    }

    film.write_to_file(filename).unwrap();

    progress_bar.finish_and_clear();

    let end = std::time::SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Rendered in {}s", duration.as_float_secs());
}

fn test_scene() -> scene::Scene {
    use geometry::{disk::Disk, primitive::Primitive, receiver::Receiver, sphere::Sphere};
    use light::emitter::Emitter;
    use material::matte::Matte;
    use material::plastic::Plastic;
    use texture::constant::ConstantTexture;

    let mut geometry = Vec::new();

    geometry.push(Primitive::Receiver(Receiver::new(
        Arc::new(Sphere::new(0.5)),
        Arc::new(Plastic::new(
            Arc::new(ConstantTexture::new(Spectrum::from_rgb(0.8, 0.8, 0.8))),
            Arc::new(ConstantTexture::new(Spectrum::from_rgb(0.0, 0.0, 0.0))),
            Arc::new(ConstantTexture::new(0.20)),
        )),
        // Arc::new(Mirror::new(
        //     Arc::new(ConstantTexture::new(Spectrum::all(0.5)))
        // )),
        // Arc::new(Matte::new(
        //     Arc::new(ConstantTexture::new(Spectrum::from_rgb(1.0, 1.0, 1.0))),
        //     Some(Arc::new(ConstantTexture::new(0.2))),
        // )),
        Transform::translate(Vec3f::new(0.0, 0.5, 2.3)),
    )));

    // Floor
    geometry.push(Primitive::Receiver(Receiver::new(
        Arc::new(Disk::new(10.0, 0.0)),
        Arc::new(Matte::new(
            Arc::new(ConstantTexture::new(Spectrum::from_rgb(0.74, 0.74, 0.74))),
            None,
        )),
        Transform::translate(Vec3f::new(0.0, 0.0, 1.0))
            * Transform::rotation(Vec3f::new(1.0, 0.0, 0.0), 360.0 - 90.0),
    )));

    // Left
    geometry.push(Primitive::Receiver(Receiver::new(
        Arc::new(Disk::new(10.0, 0.0)),
        Arc::new(Matte::new(
            Arc::new(ConstantTexture::new(Spectrum::from_rgb(0.8, 0.0, 0.0))),
            None,
        )),
        Transform::translate(Vec3f::new(-1.3, 0.0, 1.0))
            * Transform::rotation(Vec3f::new(0.0, 1.0, 0.0), 90.0),
    )));

    // Right
    geometry.push(Primitive::Receiver(Receiver::new(
        Arc::new(Disk::new(10.0, 0.0)),
        Arc::new(Matte::new(
            Arc::new(ConstantTexture::new(Spectrum::from_rgb(0.0, 0.8, 0.0))),
            None,
        )),
        Transform::translate(Vec3f::new(1.3, 0.0, 1.0))
            * Transform::rotation(Vec3f::new(0.0, 1.0, 0.0), 360.0 - 90.0),
    )));

    // Back
    geometry.push(Primitive::Receiver(Receiver::new(
        Arc::new(Disk::new(10.0, 0.0)),
        Arc::new(Matte::new(
            Arc::new(ConstantTexture::new(Spectrum::from_rgb(0.74, 0.74, 0.74))),
            None,
        )),
        Transform::translate(Vec3f::new(0.0, 0.0, 3.0))
            * Transform::rotation(Vec3f::new(1.0, 0.0, 0.0), 180.0),
    )));

    // Ceiling
    geometry.push(Primitive::Receiver(Receiver::new(
        Arc::new(Disk::new(10.0, 0.0)),
        Arc::new(Matte::new(
            Arc::new(ConstantTexture::new(Spectrum::from_rgb(0.74, 0.74, 0.74))),
            None,
        )),
        Transform::translate(Vec3f::new(0.0, 2.0, 1.0))
            * Transform::rotation(Vec3f::new(1.0, 0.0, 0.0), 90.0),
    )));

    // Ceiling light
    geometry.push(Primitive::Emitter(Emitter::new_area(
        Spectrum::from_rgb(1.0, 1.0, 1.0) * 25.0,
        Transform::translate(Vec3f::new(0.0, 1.99, 2.3))
            * Transform::rotation(Vec3f::new(1.0, 0.0, 0.0), 90.0),
        Arc::new(Disk::new(0.3, 0.0)),
        Arc::new(Matte::new(
            Arc::new(ConstantTexture::new(Spectrum::from_rgb(1.0, 1.0, 1.0))),
            None,
        )),
    )));

    scene::Scene::new(geometry)
}
