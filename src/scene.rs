use crate::geometry::primitive::{BVHPrimitive, Primitive};
use crate::geometry::HitInfo;
use crate::light::emitter::Emitter;
use crate::light::Light;
use crate::math::*;
use bvh::bvh::BVH;

pub struct Scene {
    bvh: BVH,
    geometry: Vec<BVHPrimitive>,
    pub lights: Vec<Emitter>,
}

impl Scene {
    pub fn new(geometry: Vec<Primitive>) -> Self {
        assert!(geometry.len() > 0);

        let mut lights = Vec::new();

        let mut bvh_geom = geometry
            .into_iter()
            .filter_map(|g| match g {
                Primitive::Emitter(ref e) => {
                    let is_delta = e.is_delta();
                    lights.push(e.clone());
                    if !is_delta {
                        Some(BVHPrimitive::new(g))
                    } else {
                        None
                    }
                }
                Primitive::Receiver(_) => Some(BVHPrimitive::new(g)),
            })
            .collect::<Vec<BVHPrimitive>>();

        let bvh = BVH::build(&mut bvh_geom);

        Self {
            bvh,
            geometry: bvh_geom,
            lights,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        let bvh_ray = bvh::ray::Ray::new(
            na::Point3::new(ray.o.x, ray.o.y, ray.o.z),
            na::Vector3::new(ray.d.x, ray.d.y, ray.d.z),
        );

        let hits = self.bvh.traverse(&bvh_ray, &self.geometry);

        hits.iter()
            .filter_map(|hit| hit.intersect(ray).map(|isect| (ray.t_max, isect)))
            .min_by_key(|(t, _)| ordered_float::NotNan::new(*t).unwrap())
            .map(|(_, i)| i)
    }
}
