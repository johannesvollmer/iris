use super::spectrum::RGBSpectrum;
use crate::math::*;
use num::traits::Float as _;

#[cfg(not(feature = "rgb16"))]
type ImgOut = u8;
#[cfg(not(feature = "rgb16"))]
const PIXEL_RANGE: Float = 255.99;

#[cfg(feature = "rgb16")]
type ImgOut = u16;
#[cfg(feature = "rgb16")]
const PIXEL_RANGE: Float = 65535.99;

pub enum Tonemap {
    Reinhard,
    ReinhardBurn(Float),
    // ReinhardBurnMax,
    // HableFilmic,
    // TODO: Implement more camera response functions from http://www.cs.columbia.edu/CAVE/software/softlib/dorf.php
}

#[derive(new)]
pub struct Image {
    hdr_buffer: Vec<RGBSpectrum>,
    resx: u32,
    resy: u32,
    path: std::path::PathBuf,
}

impl Image {
    #[cfg(not(feature = "hdr"))]
    pub fn write_ldr(mut self, exposure: Float, tonemap: Tonemap) {
        // Apply exposure
        for pixel in &mut self.hdr_buffer {
            *pixel *= 2.0.powf(exposure);
        }

        // Apply tonemapping
        match tonemap {
            Tonemap::Reinhard => self.apply_reinhard(Float::infinity()),
            Tonemap::ReinhardBurn(max_luminance) => self.apply_reinhard(max_luminance),
        }

        let imgbuf = image::ImageBuffer::from_fn(self.resx, self.resy, |x, y| {
            let mut scaled = [0, 0, 0];

            let px = self.hdr_buffer[(y * self.resx + x) as usize].to_rgb();
            for (i, component) in px.iter().enumerate() {
                debug_assert!(*component >= 0.0 && *component <= 1.0);
                scaled[i] = (num::clamp(*component, 0.0, 1.0) * PIXEL_RANGE) as ImgOut;
            }

            image::Rgb(scaled)
        });

        self.path.set_extension("png");

        #[cfg(not(feature = "rgb16"))]
        imgbuf.save(self.path.to_str().unwrap()).unwrap();

        #[cfg(feature = "rgb16")]
        {
            use byteorder::{BigEndian, WriteBytesExt};

            let raw = imgbuf.into_raw();
            let mut u8vec: Vec<u8> = Vec::with_capacity(raw.len() * 2);
            raw.into_iter()
                .for_each(|x| u8vec.write_u16::<BigEndian>(x).unwrap());

            image::save_buffer(
                self.path,
                &u8vec,
                self.resx,
                self.resy,
                image::ColorType::RGB(16),
            )
            .unwrap();
        }
    }

    #[cfg(feature = "hdr")]
    pub fn write_hdr(mut self) {
        use openexr::{FrameBuffer, Header, PixelType, ScanlineOutputFile};

        let buf = self
            .hdr_buffer
            .into_iter()
            .map(|pixel| pixel.to_rgb())
            .collect();

        self.path.set_extension("exr");

        let mut file = std::fs::File::create(self.path).unwrap();
        let mut output_file = ScanlineOutputFile::new(
            &mut file,
            Header::new()
                .set_resolution(self.resx, self.resy)
                .add_channel("R", PixelType::FLOAT)
                .add_channel("G", PixelType::FLOAT)
                .add_channel("B", PixelType::FLOAT),
        )
        .unwrap();

        let mut fb = FrameBuffer::new(self.resx, self.resy);
        fb.insert_channels(&["R", "G", "B"], &buf);

        output_file.write_pixels(&fb).unwrap();
    }

    // http://www.cs.utah.edu/~reinhard/cdrom/tonemap.pdf
    fn apply_reinhard(&mut self, max_luminance: Float) {
        for pixel in &mut self.hdr_buffer {
            let l_in = pixel.y().max(0.0001);

            let l_out = if max_luminance.is_finite() {
                (l_in * (1.0 + (l_in / max_luminance.powi(2)))) / (1.0 + l_in)
            } else {
                l_in / (1.0 + l_in)
            };

            *pixel = remap_color(*pixel, l_in, l_out);
        }
    }
}

// https://www.cl.cam.ac.uk/~rkm38/pdfs/mantiuk09cctm.pdf
fn remap_color(c_in: RGBSpectrum, l_in: Float, l_out: Float) -> RGBSpectrum {
    ((c_in) / l_in) * l_out
}