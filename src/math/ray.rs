use crate::math::{misc::offset_ray_origin, Float, Normal3f, Point3f, Vec3f};
use num::Float as _;

#[derive(Debug, Copy, Clone)]
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
            t_max: Float::infinity(),
            time: 0.0,
        }
    }

    pub fn at(&self, t: Float) -> Point3f {
        self.o + self.d * t
    }

    pub fn spawn(point: Point3f, dir: Vec3f, err: Vec3f, normal: Normal3f, time: Float) -> Self {
        Ray {
            o: offset_ray_origin(point, err, normal, dir),
            d: dir,
            t_max: Float::infinity(),
            time,
        }
    }

    pub fn spawn_to(
        point: Point3f,
        other: Point3f,
        err: Vec3f,
        normal: Normal3f,
        time: Float,
    ) -> Self {
        let o = offset_ray_origin(point, err, normal, other - point);
        let d = point - o;
        Ray {
            o,
            d,
            t_max: 1.0 - 0.001,
            time,
        }
    }
}

#[derive(Debug)]
pub struct RayDifferentialInfo {
    pub rx_origin: Point3f,
    pub ry_origin: Point3f,
    pub rx_direction: Vec3f,
    pub ry_direction: Vec3f,
}

#[derive(Debug)]
pub struct RayDifferential {
    pub ray: Ray,
    pub info: Option<RayDifferentialInfo>,
}

impl RayDifferential {
    pub fn scale_differentials(&mut self, s: Float) {
        if let Some(info) = &mut self.info {
            self.info = Some(RayDifferentialInfo {
                rx_origin: self.ray.o + (info.rx_origin - self.ray.o) * s,
                ry_origin: self.ray.o + (info.ry_origin - self.ray.o) * s,
                rx_direction: self.ray.d + (info.rx_direction - self.ray.d) * s,
                ry_direction: self.ray.d + (info.ry_direction - self.ray.d) * s,
            });
        }
    }
}
