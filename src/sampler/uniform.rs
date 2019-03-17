use crate::math::*;
use crate::sampler::Sampler;
use rand::{rngs::StdRng, Rng, SeedableRng};

pub struct UniformSampler {
    rng: StdRng,
    pixel: Point2i,
    bounds: Bounds2i,
    spp: u32,
    samples_taken: u32,
}

impl UniformSampler {
    pub fn new(spp: u32) -> Self {
        UniformSampler {
            rng: SeedableRng::seed_from_u64(0),
            pixel: Point2i::new(0, 0),
            bounds: Bounds2i::new(Point2i::new(0, 0), Point2i::new(0, 0)),
            spp,
            samples_taken: 0,
        }
    }
}

impl Sampler for UniformSampler {
    fn get_bounds(&self) -> Bounds2i {
        self.bounds
    }

    fn start_pixel(&mut self, pixel: Point2i) {
        self.pixel = pixel;
        self.samples_taken = 0;
    }

    fn clone_seed(&self, seed: u64) -> Box<dyn Sampler> {
        Box::new(UniformSampler {
            rng: SeedableRng::seed_from_u64(seed),
            pixel: Point2i::new(0, 0),
            bounds: Bounds2i::new(Point2i::new(0, 0), Point2i::new(0, 0)),
            spp: self.spp,
            samples_taken: 0,
        })
    }

    fn next_sample(&mut self) -> Option<f32> {
        if self.samples_taken == self.spp {
            None
        } else {
            self.samples_taken += 1;
            Some(self.rng.gen())
        }
    }
}
