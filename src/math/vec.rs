use super::Float;

#[derive(new, Copy, Clone, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(new, Copy, Clone, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

pub type Vec2f = Vec2<Float>;
pub type Vec2i = Vec2<i32>;
pub type Vec3f = Vec3<Float>;
pub type Vec3i = Vec3<i32>;

impl<T> Vec3<T> {
    pub fn dot(&self, other: &Self) -> T
        where T: Copy + std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>
    {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length_squared(&self) -> T
        where T: Copy + std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T>
    {
        self.dot(&self)
    }

    pub fn length(&self) -> T
        where T: num::Float
    {
        self.length_squared().sqrt()
    }
}

impl<T> std::ops::Add for Vec2<T>
    where T: std::ops::Add<T, Output = T> + Copy
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> std::ops::Add<T> for Vec2<T>
    where T: std::ops::Add<T, Output = T> + Copy
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        self + Self { x: other, y: other }
    }
}

impl<T> std::ops::Sub for Vec2<T>
    where T: std::ops::Sub<T, Output = T> + Copy
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> std::ops::Sub<T> for Vec2<T>
    where T: std::ops::Sub<T, Output = T> + Copy
{
    type Output = Self;

    fn sub(self, other: T) -> Self::Output {
        self - Self { x: other, y: other }
    }
}

impl<T> std::ops::Div<T> for Vec2<T>
    where T: std::ops::Div<T, Output = T> + Copy
{
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T> std::ops::Add for Vec3<T>
    where T: std::ops::Add<T, Output = T> + Copy
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> std::ops::Add<T> for Vec3<T>
    where T: std::ops::Add<T, Output = T> + Copy
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        self + Self { x: other, y: other, z: other }
    }
}

impl<T> std::ops::Sub for Vec3<T>
    where T: std::ops::Sub<T, Output = T> + Copy
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> std::ops::Sub<T> for Vec3<T>
    where T: std::ops::Sub<T, Output = T> + Copy
{
    type Output = Self;

    fn sub(self, other: T) -> Self::Output {
        self - Self { x: other, y: other, z: other }
    }
}

impl<T> std::ops::Div<T> for Vec3<T>
    where T: std::ops::Div<T, Output = T> + Copy
{
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}