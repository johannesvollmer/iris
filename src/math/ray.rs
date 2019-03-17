use crate::math::{Float, Point3f, Vec3f};

#[derive(Copy, Clone)]
pub struct Ray {
    pub o: Point3f,
    pub d: Vec3f,
    pub t_max: Float,
    pub time: Float,
    // medium
}

impl Ray {
    pub fn new(o: Point3f, d: Vec3f) -> Self {
        Self {
            o,
            d,
            t_max: std::f32::INFINITY,
            time: 0.0,
        }
    }

    pub fn at(&self, t: Float) -> Point3f {
        self.o + self.d * t
    }
}

pub struct RayDifferentialInfo {
    pub rx_origin: Point3f,
    pub ry_origin: Point3f,
    pub rx_direction: Vec3f,
    pub ry_direction: Vec3f,
}

pub struct RayDifferential {
    pub ray: Ray,
    pub info: Option<RayDifferentialInfo>,
}

impl RayDifferential {
    pub fn new(ray: Ray) -> Self {
        Self { ray, info: None }
    }
}
