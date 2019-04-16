use super::{Float, Point2, Point2i, Point2f, Point3f, Vec2};
use bvh::aabb::AABB;
use num::traits::ToPrimitive;

#[derive(new, Default, Copy, Clone, Debug)]
pub struct Bounds3f {
    pub min: Point3f,
    pub max: Point3f,
}

#[derive(new, Copy, Clone, Debug)]
pub struct Bounds2<T> {
    pub min: Point2<T>,
    pub max: Point2<T>,
}

pub type Bounds2f = Bounds2<Float>;
pub type Bounds2i = Bounds2<i32>;

macro_rules! max {
    ($a:expr, $b:expr) => { if $a > $b { $a } else { $b } };
}

macro_rules! min {
    ($a:expr, $b:expr) => { if $a < $b { $a } else { $b } };
}

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

    pub fn intersection(&self, other: Self) -> Self
    where
        T: PartialOrd + Copy
    {
        let min = Point2::new(
            max!(self.min.x, other.min.x),
            max!(self.min.y, other.min.y),
        );
        let max = Point2::new(
            min!(self.max.x, other.max.x),
            min!(self.max.y, other.max.y),
        );
        Self::new(min, max)
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

impl Bounds3f {
    pub fn to_aabb(self) -> AABB {
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

impl From<Bounds2f> for Bounds2i {
    fn from(other: Bounds2f) -> Self {
        Self {
            min: Point2i::from(other.min),
            max: Point2i::from(other.max),
        }
    }
}

impl From<Bounds2i> for Bounds2f {
    fn from(other: Bounds2i) -> Self {
        Self {
            min: Point2f::from(other.min),
            max: Point2f::from(other.max),
        }
    }
}