use std::sync::Mutex;
use crate::film::color::Color4f;
use crate::film::image::Image;

const TILE_SIZE: usize = 16;
const TILE_AREA: usize = TILE_SIZE * TILE_SIZE;

pub struct ImageSample {
    x: f32,
    y: f32,
    color: Color4f,
}

#[derive(Clone)]
pub struct RenderBlock {
    pub pixels: Vec<Color4f>,
}

impl RenderBlock {
    pub fn new() -> RenderBlock {
        Self {
            pixels: vec![Color4f::black(); TILE_AREA],
        }
    }
}

pub struct RenderTarget {
    width: usize,
    height: usize,
    blocks: Vec<Mutex<RenderBlock>>,
}

impl RenderTarget {
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width * height % TILE_SIZE == 0);
        let nblocks = width * height / TILE_AREA;
        let blocks = (0..nblocks).map(|_| Mutex::new(RenderBlock::new())).collect();

        Self {
            width,
            height,
            blocks,
        }
    }

    pub fn get_image(self) -> Image {
        let default_block = RenderBlock::new();

        let mut pixels_flat: Vec<Color4f> = Vec::with_capacity(self.width * self.height);

        for block in self.blocks {
            let block = block.lock().unwrap();
            pixels_flat.extend_from_slice(&block.pixels);
        }

        Image::new(pixels_flat, (self.width as u32, self.height as u32))
    }

    pub fn write(&self, samples: &[ImageSample], region: &Bounds2f) {
        
    }
}
