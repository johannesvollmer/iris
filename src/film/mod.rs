use crate::film::filter::Filter;
use crate::film::spectrum::{RGBSpectrum, Spectrum};
use crate::math::*;
use std::sync::{Arc, Mutex};

pub mod camera;
pub mod filter;
pub mod image;
pub mod spectrum;

#[derive(Copy, Clone)]
pub struct Pixel {
    pub rgb: RGBSpectrum,
    pub filter_weight_sum: Float,
}

#[derive(Clone, Default)]
pub struct FilmTilePixel {
    pub contrib_sum: Spectrum,
    pub filter_weight_sum: Float,
}

pub struct FilmTile {
    pub pixel_bounds: Bounds2i,
    pub sample_bounds: Bounds2i,
    filter: Arc<dyn Filter>,
    filter_radius: Float,
    pixels: Vec<FilmTilePixel>,
}

impl FilmTile {
    pub fn new(
        sample_bounds: Bounds2i,
        pixel_bounds: Bounds2i,
        filter: Arc<dyn Filter>,
    ) -> FilmTile {
        Self {
            pixel_bounds,
            sample_bounds,
            filter_radius: filter.radius(),
            filter,
            pixels: vec![FilmTilePixel::default(); pixel_bounds.area() as usize],
        }
    }

    fn get_pixel_mut(&mut self, p: Point2i) -> &mut FilmTilePixel {
        let width = self.pixel_bounds.max.x - self.pixel_bounds.min.x;
        let offset = (p.x - self.pixel_bounds.min.x) + (p.y - self.pixel_bounds.min.y) * width;
        &mut self.pixels[offset as usize]
    }

    pub fn add_sample(&mut self, sample_point: Point2f, sample: &Spectrum) {
        let discrete = sample_point - Vec2f::new(0.5, 0.5);

        // TODO: Check NaNs etc
        let p_min = Point2f::from((discrete - self.filter_radius).ceil());
        let p_max = Point2f::from((discrete + self.filter_radius).floor() + Vec2f::new(1.0, 1.0));

        let bounds =
            Bounds2i::from(Bounds2f::new(p_min, p_max).intersection(self.pixel_bounds.into()));
        debug_assert!(bounds.max.x >= bounds.min.x && bounds.max.y >= bounds.min.y);

        if bounds.area() > 0 {
            for point in bounds {
                let relative = Point2f::from(point) - discrete;
                debug_assert!(relative.x.abs() <= self.filter_radius);
                debug_assert!(relative.y.abs() <= self.filter_radius);

                let weight = self.filter.evaluate(relative.x.abs(), relative.y.abs());

                let pixel = self.get_pixel_mut(point);
                pixel.contrib_sum += *sample * weight;
                pixel.filter_weight_sum += weight;
            }
        }
    }
}

pub struct Film {
    pub full_resolution: Point2i,
    pub ntiles: i32,
    sample_bounds: Bounds2i,
    tile_size: i32,
    tile_dims: Point2i,
    filter: Arc<dyn Filter + Send + Sync>,
    filter_radius: Float,
    pixels: Mutex<Vec<Pixel>>,
}

impl Film {
    pub fn new(
        width: i32,
        height: i32,
        tile_size: i32,
        filter: Box<dyn Filter + Send + Sync>,
    ) -> Self {
        let tile_dims = Point2i::new(
            (width + tile_size - 1) / tile_size,
            (height + tile_size - 1) / tile_size,
        );

        let filter_radius = filter.radius();

        let half = Vec2f::new(0.5, 0.5);
        let sample_bounds = Bounds2f::new(
            (Point2f::new(0.0, 0.0) + half - filter_radius).floor(),
            (Point2f::new(width as Float, height as Float) - half + filter_radius).ceil(),
        );

        Self {
            full_resolution: Point2i::new(width, height),
            ntiles: tile_dims.x * tile_dims.y,
            sample_bounds: sample_bounds.into(),
            tile_size,
            tile_dims,
            filter: filter.into(),
            filter_radius,
            pixels: Mutex::new(vec![
                Pixel {
                    rgb: RGBSpectrum::default(),
                    filter_weight_sum: 0.0
                };
                (width * height) as usize
            ]),
        }
    }

    pub fn get_film_tile(&self, tile_idx: i32) -> FilmTile {
        let x_tile = tile_idx % self.tile_dims.x;
        let y_tile = tile_idx / self.tile_dims.x;

        let x_min = self.sample_bounds.min.x + x_tile * self.tile_size;
        let y_min = self.sample_bounds.min.y + y_tile * self.tile_size;
        let x_max = (x_min + self.tile_size).min(self.sample_bounds.max.x);
        let y_max = (y_min + self.tile_size).min(self.sample_bounds.max.y);

        let bounds = Bounds2i::new(Point2i::new(x_min, y_min), Point2i::new(x_max, y_max));

        // let sample_bounds = bounds.intersection(self.sample_bounds);
        let sample_bounds = bounds;

        let pixel_bounds = {
            let half = Vec2f::new(0.5, 0.5);
            let float_bounds = Bounds2f::from(sample_bounds);
            let p_min = Point2i::from((float_bounds.min - half - self.filter_radius).ceil());
            let p_max = Point2i::from((float_bounds.max - half + self.filter_radius).ceil())
                + Vec2i::new(1, 1);
            Bounds2i::new(p_min, p_max)
                .intersection(Bounds2i::new(Point2i::default(), self.full_resolution))
        };

        FilmTile::new(sample_bounds, pixel_bounds, self.filter.clone())
    }

    pub fn merge_tile(&self, mut tile: FilmTile) {
        let mut pixels = self.pixels.lock().unwrap();
        for point in tile.pixel_bounds {
            let tile_pixel = tile.get_pixel_mut(point);
            let film_pixel = &mut pixels[(point.y * self.full_resolution.x + point.x) as usize];

            film_pixel.rgb += tile_pixel.contrib_sum.to_rgb_spectrum();
            film_pixel.filter_weight_sum += tile_pixel.filter_weight_sum;
        }
    }

    pub fn write_to_file(&self, filename: &str) {
        let pixels = self.pixels.lock().unwrap();
        let (resx, resy) = (self.full_resolution.x as u32, self.full_resolution.y as u32);

        let mut hdr_buffer = Vec::with_capacity((resx * resy) as usize);

        for y in 0..resy {
            for x in 0..resx {
                let pixel_in = pixels[(y * resx + x) as usize];
                let rgb = pixel_in.rgb / pixel_in.filter_weight_sum;

                // Some filters have negative lobes. Clamp at zero
                hdr_buffer.push(rgb.max(0.0));
            }
        }

        let path = std::path::PathBuf::from(filename);
        let image = image::Image::new(hdr_buffer, resx, resy, path);

        #[cfg(not(feature = "hdr"))]
        image.write_ldr(0.0, image::Tonemap::HableFilmic);

        #[cfg(feature = "hdr")]
        image.write_hdr();
    }
}
