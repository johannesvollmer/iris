use crate::film::color::Color4f;

pub struct RenderBlock {
    pixels: Vec<Color4f>,
}

pub struct RenderTarget {
    width: usize,
    height: usize,
    blocks: Vec<RenderBlock>,
    block_size: (usize, usize),
}