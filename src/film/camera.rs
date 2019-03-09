use crate::film::render::RenderTarget;
use std::sync::Arc;

pub trait Camera {
    fn get_render_target(&self) -> Arc<RenderTarget>;
}
