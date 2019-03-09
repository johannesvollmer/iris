use crate::film::color::Color4f;

#[derive(Clone)]
pub struct Image {
    pixels: Vec<Color4f>,
    dimensions: (u32, u32),
}

impl Image {
    pub fn new(pixels: Vec<Color4f>, dimensions: (u32, u32)) -> Self {
        assert!(dimensions.0 * dimensions.1 == pixels.len() as u32);
        Self { pixels, dimensions }
    }

    pub fn write_to_file(self, filename: String) {
        let mut imgbuf = image::ImageBuffer::new(self.dimensions.0, self.dimensions.1);

        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                let pixel_in = self.pixels[(y * self.dimensions.0 + x) as usize];
                let pixel_out = imgbuf.get_pixel_mut(x, y);

                *pixel_out = image::Rgb(pixel_in.to_rgb8());
            }
        }

        imgbuf.save(filename).unwrap();
    }
}
