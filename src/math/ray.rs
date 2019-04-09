use super::{misc::offset_ray_origin, Float, LocalPoint3f, LocalVec3f, Normal3f, Point3f, Vec3f};
use num::Float as _;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub o: Point3f,
    pub d: Vec3f,
    pub t_max: Float,
    pub time: Float,
    // TODO: Medium
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

    // pub fn at(&self, t: Float) -> Point3f {
    //     self.o + self.d * t
    // }

    pub fn as_local(&self) -> LocalRay {
        LocalRay {
            o: self.o.as_local(),
            d: self.d.as_local(),
            t_max: self.t_max,
            time: self.time,
        }
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
        let d = other - o;
        Ray {
            o,
            d,
            t_max: 1.0 - 0.001,
            time,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LocalRay {
    pub o: LocalPoint3f,
    pub d: LocalVec3f,
    pub t_max: Float,
    pub time: Float,
}

impl LocalRay {
    pub fn at(&self, t: Float) -> LocalPoint3f {
        self.o + self.d * t
    }
}

impl LocalRay {
    pub fn global_t(&self, local_t: Float, global: &Ray) -> Float {
        let t_scale_factor = global.d.length() / self.d.length();
        local_t / t_scale_factor
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
