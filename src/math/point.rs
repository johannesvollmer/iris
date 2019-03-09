use super::{Float, Vec2, Vec3};

#[derive(new, Copy, Clone, Debug)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(new, Copy, Clone, Debug)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

pub type Point2f = Point2<Float>;
pub type Point2i = Point2<i32>;
pub type Point3f = Point3<Float>;
pub type Point3i = Point3<i32>;

impl<T: Copy> Point3<T> {
    pub fn to_vec(&self) -> Vec3<T> {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl<T> std::ops::Sub for Point3<T>
where
    T: std::ops::Sub<T, Output = T> + Copy,
{
    type Output = Vec3<T>;

    fn sub(self, other: Self) -> Self::Output {
        self.to_vec() - other.to_vec()
    }
}

impl<T: Copy> Point2<T> {
    pub fn to_vec(&self) -> Vec2<T> {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl<T> std::ops::Sub for Point2<T>
where
    T: std::ops::Sub<T, Output = T> + Copy,
{
    type Output = Vec2<T>;

    fn sub(self, other: Self) -> Self::Output {
        self.to_vec() - other.to_vec()
    }
}
