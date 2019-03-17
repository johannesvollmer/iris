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

    pub fn ceil(&self) -> Self
    where
        T: num::Float,
    {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    pub fn floor(&self) -> Self
    where
        T: num::Float,
    {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
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

impl<T> std::ops::Sub<Vec3<T>> for Point3<T>
where
    T: std::ops::Sub<T, Output = T> + Copy,
{
    type Output = Point3<T>;

    fn sub(self, other: Vec3<T>) -> Self::Output {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> std::ops::Sub<T> for Point3<T>
where
    T: std::ops::Sub<T, Output = T> + Copy,
{
    type Output = Point3<T>;

    fn sub(self, other: T) -> Self::Output {
        Point3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl<T> std::ops::Add<T> for Point3<T>
where
    T: std::ops::Add<T, Output = T> + Copy,
{
    type Output = Point3<T>;

    fn add(self, other: T) -> Self::Output {
        Point3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl<T> std::ops::Add<Vec3<T>> for Point3<T>
where
    T: std::ops::Add<T, Output = T> + Copy,
{
    type Output = Point3<T>;

    fn add(self, other: Vec3<T>) -> Self::Output {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> From<Vec3<T>> for Point3<T>
where
    T: Copy
{
    fn from(other: Vec3<T>) -> Self {
        Self {
            x: other.x,
            y: other.y,
            z: other.z
        }
    }
}

impl<T: Copy> Point2<T> {
    pub fn to_vec(&self) -> Vec2<T> {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn ceil(&self) -> Self
    where
        T: num::Float,
    {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    pub fn floor(&self) -> Self
    where
        T: num::Float,
    {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
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

impl<T> std::ops::Sub<Vec2<T>> for Point2<T>
where
    T: std::ops::Sub<T, Output = T> + Copy,
{
    type Output = Point2<T>;

    fn sub(self, other: Vec2<T>) -> Self::Output {
        Point2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> std::ops::Add<Vec2<T>> for Point2<T>
where
    T: std::ops::Add<T, Output = T> + Copy,
{
    type Output = Point2<T>;

    fn add(self, other: Vec2<T>) -> Self::Output {
        Point2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> std::ops::Sub<T> for Point2<T>
where
    T: std::ops::Sub<T, Output = T> + Copy,
{
    type Output = Point2<T>;

    fn sub(self, other: T) -> Self::Output {
        Point2 {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl<T> std::ops::Add<T> for Point2<T>
where
    T: std::ops::Add<T, Output = T> + Copy,
{
    type Output = Point2<T>;

    fn add(self, other: T) -> Self::Output {
        Point2 {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl From<Point2f> for Point2i {
    fn from(p: Point2f) -> Point2i {
        Point2i {
            x: p.x as i32,
            y: p.y as i32,
        }
    }
}

impl From<Point2i> for Point2f {
    fn from(p: Point2i) -> Point2f {
        Point2f {
            x: p.x as f32,
            y: p.y as f32,
        }
    }
}
