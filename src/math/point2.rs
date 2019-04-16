use super::{Float, Vec2};

#[derive(new, Copy, Clone, Debug)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

pub type Point2f = Point2<Float>;
pub type Point2i = Point2<i32>;

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

impl std::default::Default for Point2i {
    fn default() -> Self {
        Self { x: 0, y: 0 }
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
