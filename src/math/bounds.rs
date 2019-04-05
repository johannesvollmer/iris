use super::{Float, Point2, Point2i, Point3, Vec2};
use bvh::aabb::AABB;

#[derive(new, Copy, Clone, Debug)]
pub struct Bounds3<T> {
    pub min: Point3<T>,
    pub max: Point3<T>,
}

#[derive(new, Copy, Clone, Debug)]
pub struct Bounds2<T> {
    pub min: Point2<T>,
    pub max: Point2<T>,
}

pub type Bounds2f = Bounds2<Float>;
pub type Bounds2i = Bounds2<i32>;
pub type Bounds3f = Bounds3<Float>;

impl<T> Bounds2<T> {
    pub fn diagonal(&self) -> Vec2<T>
    where
        T: std::ops::Sub<T, Output = T> + Copy,
    {
        self.max - self.min
    }

    pub fn area(&self) -> T
    where
        T: std::ops::Sub<T, Output = T> + std::ops::Mul<T, Output = T> + Copy,
    {
        let diagonal = self.diagonal();
        diagonal.x * diagonal.y
    }

    /*pub fn contains(&self, point: Point2<T>) -> bool
        where T: std::ops::Sub<T, Output = T> + PartialOrd + Copy,
    {
        (self.bounds.min.x..self.bounds.max.x).contains(&point.x) &&
        (self.bounds.min.y..self.bounds.max.y).contains(&point.y)
    }*/
}

pub struct BoundsIter {
    bounds: Bounds2i,
    point: Point2i,
}

impl BoundsIter {
    fn new(bounds: Bounds2i) -> Self {
        Self {
            bounds,
            point: bounds.min,
        }
    }
}

impl Iterator for BoundsIter {
    type Item = Point2i;

    fn next(&mut self) -> Option<Self::Item> {
        if self.point.y >= self.bounds.max.y {
            None
        } else if self.point.x + 1 == self.bounds.max.x {
            let out = Some(self.point);
            self.point.y += 1;
            self.point.x = self.bounds.min.x;
            out
        } else {
            let out = Some(self.point);
            self.point.x += 1;
            out
        }
    }
}

impl IntoIterator for Bounds2i {
    type Item = <BoundsIter as Iterator>::Item;
    type IntoIter = BoundsIter;

    fn into_iter(self) -> Self::IntoIter {
        BoundsIter::new(self)
    }
}

impl<T> Bounds3<T> {
    pub fn to_aabb(self) -> AABB
    where
        T: num::Float,
    {
        AABB::with_bounds(
            na::Point3::new(
                self.min.x.to_f32().unwrap(),
                self.min.y.to_f32().unwrap(),
                self.min.z.to_f32().unwrap(),
            ),
            na::Point3::new(
                self.max.x.to_f32().unwrap(),
                self.max.y.to_f32().unwrap(),
                self.max.z.to_f32().unwrap(),
            ),
        )
    }
}

impl<T: Default> Default for Bounds3<T> {
    fn default() -> Self {
        Self {
            min: Point3::default(),
            max: Point3::default(),
        }
    }
}