use crate::geometry::primitive::{BVHPrimitive, Primitive};
use crate::geometry::HitInfo;
use crate::math::*;
use bvh::bvh::BVH;

pub struct Scene {
    bvh: BVH,
    geometry: Vec<BVHPrimitive>,
}

impl Scene {
    pub fn new(geometry: Vec<Primitive>) -> Self {
        assert!(geometry.len() > 0);
        let mut geometry: Vec<_> = geometry.into_iter().map(BVHPrimitive::new).collect();

        let bvh = BVH::build(&mut geometry);

        Self { bvh, geometry }
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
