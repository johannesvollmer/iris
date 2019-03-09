#![feature(const_fn)]

#[macro_use]
extern crate derive_new;

mod film;
mod geometry;
mod math;
mod scene;

fn main() {
    let render_target = film::render::RenderTarget::new(128, 128);

    render_target
        .get_image()
        .write_to_file("out.png".to_string());
}
