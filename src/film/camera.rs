use crate::film::Film;
use std::sync::Arc;

pub trait Camera {
    fn get_film(&self) -> Arc<Film>;
}
