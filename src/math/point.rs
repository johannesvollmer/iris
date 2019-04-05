use super::{Float, Vec2, Vec3f};

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

impl Point3f {
    pub fn to_vec(&self) -> Vec3f {
        Vec3f::new(self.x, self.y, self.z)
    }
}

impl std::ops::Sub for Point3f {
    type Output = Vec3f;
    fn sub(self, other: Self) -> Self::Output {
        self.to_vec() - other.to_vec()
    }
}

impl std::ops::Sub<Vec3f> for Point3f {
    type Output = Point3f;

    fn sub(self, other: Vec3f) -> Self::Output {
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

impl std::ops::Add<Vec3f> for Point3f
{
    type Output = Point3f;

    fn add(self, other: Vec3f) -> Self::Output {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> std::ops::Index<usize> for Point3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("out of bounds for Point3f"),
        }
    }
}

impl<T> std::ops::IndexMut<usize> for Point3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("out of bounds for Point3f"),
        }
    }
}

impl From<Vec3f> for Point3f
{
    fn from(other: Vec3f) -> Self {
        Self {
            x: other.x,
            y: other.y,
            z: other.z,
        }
    }
}

impl<T: Default> Default for Point3<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            z: T::default(),
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
            x: p.x as Float,
            y: p.y as Float,
        }
    }
}
