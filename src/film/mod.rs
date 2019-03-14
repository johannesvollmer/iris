pub mod camera;
pub mod filter;
pub mod spectrum;

use std::sync::Mutex;
//use crate::film::color::Color4f;
use crate::film::filter::Filter;
use crate::film::spectrum::Spectrum;
use crate::math::*;

#[derive(Copy, Clone)]
pub struct Pixel {
    pub rgb: [f32; 3],
    pub filter_weight_sum: f32,
}

#[derive(Clone)]
pub struct FilmTilePixel {
    pub contrib_sum: Spectrum,
    pub filter_weight_sum: f32,
}

pub struct FilmTile {
    bounds: Bounds2i,
    filter: Box<dyn Filter>,
    filter_width: f32,
    pixels: Vec<FilmTilePixel>,
}

impl FilmTile {
    pub fn new(bounds: Bounds2i, filter: Box<Filter>) -> FilmTile {
        Self {
            bounds,
            filter_width: filter.width(),
            filter,
            pixels: vec![
                FilmTilePixel {
                    contrib_sum: Spectrum::new(0.0, 0.0, 0.0),
                    filter_weight_sum: 0.0
                };
                bounds.area() as usize
            ],
        }
    }

    pub fn get_pixel_mut(&mut self, p: Point2i) -> &mut FilmTilePixel {
        let width = self.bounds.max.x - self.bounds.min.x;
        let offset = (p.x - self.bounds.min.x) + (p.y - self.bounds.min.y) * width;
        &mut self.pixels[offset as usize]
    }

    pub fn add_sample(&mut self, point: Point2f, sample: &Spectrum) {
        let discrete = point - Vec2f::new(0.5, 0.5);
        let mut p_min: Point2i = (discrete - self.filter_width).ceil().into();
        let mut p_max: Point2i =
            Point2i::from((discrete + self.filter_width).floor()) + Vec2i::new(1, 1);

        // Clip min and max
        p_min.x = i32::max(self.bounds.min.x, p_min.x);
        p_min.y = i32::max(self.bounds.min.y, p_min.y);
        p_max.x = i32::min(self.bounds.max.x, p_max.x);
        p_max.y = i32::min(self.bounds.max.y, p_max.y);

        let bounds = Bounds2i::new(p_min, p_max);

        for point in bounds {
            let weight = self
                .filter
                .evaluate(point.x as f32 - discrete.x, point.y as f32 - discrete.y);
            let pixel = self.get_pixel_mut(point);
            pixel.contrib_sum += sample * weight;
            pixel.filter_weight_sum += weight;
        }
    }
}

pub struct Film {
    width: usize,
    height: usize,
    pixels: Mutex<Vec<Pixel>>,
}

impl Film {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: Mutex::new(vec![
                Pixel {
                    rgb: [0.0, 0.0, 0.0],
                    filter_weight_sum: 0.0
                };
                width * height
            ]),
        }
    }

    pub fn get_film_tile(bounds: Bounds2i) -> FilmTile {
        FilmTile::new(bounds, Box::new(filter::Triangle::new(4.0)))
    }

    pub fn merge_tile(&self, mut tile: FilmTile) {
        let mut pixels = self.pixels.lock().unwrap();
        for point in tile.bounds {
            let tile_pixel = tile.get_pixel_mut(point);
            let film_pixel = &mut pixels[(point.y * self.width as i32 + point.x) as usize];

            let rgb = tile_pixel.contrib_sum.rgb();
            film_pixel.rgb[0] += rgb[0];
            film_pixel.rgb[1] += rgb[1];
            film_pixel.rgb[2] += rgb[2];
            film_pixel.filter_weight_sum += tile_pixel.filter_weight_sum;
        }
    }

    pub fn write_to_file(self, filename: &str) -> std::io::Result<()> {
        let mut imgbuf = image::ImageBuffer::new(self.width as u32, self.height as u32);

        let pixels = self.pixels.lock().unwrap();

        for x in 0..self.width {
            for y in 0..self.height {
                let pixel_in = pixels[y * self.width + x];
                let pixel_out = imgbuf.get_pixel_mut(x as u32, y as u32);

                let weight = 1.0 / pixel_in.filter_weight_sum;

                let mut weighted = [0, 0, 0];

                for (i, component) in pixel_in.rgb.iter().enumerate() {
                    weighted[i] = (component * weight * 255.0).max(0.0) as u8;
                }

                *pixel_out = image::Rgb(weighted);
            }
        }

        imgbuf.save(filename)
    }
}
