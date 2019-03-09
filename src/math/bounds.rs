use super::{Float, Point2, Point3, Vec2, Vec3};

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
pub type Bounds3i = Bounds3<i32>;

impl<T> Bounds2<T> {
    pub fn diagonal(&self) -> Vec2<T>
    where
        T: std::ops::Sub<T, Output = T> + Copy,
    {
        self.max - self.min
    }
}

impl<T> Bounds3<T> {
    pub fn diagonal(&self) -> Vec3<T>
    where
        T: std::ops::Sub<T, Output = T> + Copy,
    {
        self.max - self.min
    }
}
