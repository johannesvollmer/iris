use super::Float;
use crate::math::normal::Normal3f;
use crate::math::point::Point3;

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

impl<T> Vec3<T> {
    pub fn dot(&self, other: &Self) -> T
    where
        T: Copy + std::ops::Mul<Output = T> + std::ops::Add<Output = T>,
    {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length_squared(&self) -> T
    where
        T: Copy + std::ops::Mul<Output = T> + std::ops::Add<Output = T>,
    {
        self.dot(self)
    }

    pub fn length(&self) -> T
    where
        T: num::Float,
    {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Self
    where
        T: num::Float,
    {
        *self / self.length()
    }

    pub fn cross(&self, other: Self) -> Self
    where
        T: num::Float,
    {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn abs(&self) -> Self
    where
        T: num::Float,
    {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    /*pub fn from_spherical(sin_theta: T, cos_theta: T, phi: T) -> Self
    where
        T: Copy + std::ops::Mul<T, Output = T> + num::Float,
    {
        Self {
            x: sin_theta * phi.cos(),
            y: cos_theta * phi.cos(),
            z: cos_theta,
        }
    }

    pub fn from_spherical_frame(
        sin_theta: T,
        cos_theta: T,
        phi: T,
        x: Self,
        y: Self,
        z: Self,
    ) -> Self
    where
        T: Copy + std::ops::Mul<T, Output = T> + num::Float,
    {
        x * sin_theta * phi.cos() + y * cos_theta * phi.cos() + z * cos_theta
    }

    pub fn spherical_theta(&self) -> T
    where
        T: Copy + num::Float + num::FromPrimitive,
    {
        misc::clamp(
            self.z,
            T::from_f32(-1.0).unwrap(),
            T::from_f32(1.0).unwrap(),
        )
        .acos()
    }

    pub fn spherical_phi(&self) -> T
    where
        T: Copy + num::Float + num::FromPrimitive,
    {
        let p = num::Float::atan2(self.y, self.x);
        if p < T::zero() {
            p + T::from_f32(2.0 * std::f32::consts::PI).unwrap()
        } else {
            p
        }
    }*/
}

impl<T> std::ops::Add for Vec2<T>
where
    T: std::ops::Add<T, Output = T> + Copy,
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
where
    T: std::ops::Add<T, Output = T> + Copy,
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        self + Self { x: other, y: other }
    }
}

impl<T> std::ops::Sub for Vec2<T>
where
    T: std::ops::Sub<T, Output = T> + Copy,
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
where
    T: std::ops::Sub<T, Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, other: T) -> Self::Output {
        self - Self { x: other, y: other }
    }
}

impl<T> std::ops::Div<T> for Vec2<T>
where
    T: std::ops::Div<T, Output = T> + Copy,
{
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T> std::ops::Mul<T> for Vec2<T>
where
    T: std::ops::Mul<T, Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T> std::ops::Add for Vec3<T>
where
    T: std::ops::Add<T, Output = T> + Copy,
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
where
    T: std::ops::Add<T, Output = T> + Copy,
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        self + Self {
            x: other,
            y: other,
            z: other,
        }
    }
}

impl<T> std::ops::Sub for Vec3<T>
where
    T: std::ops::Sub<T, Output = T> + Copy,
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
where
    T: std::ops::Sub<T, Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, other: T) -> Self::Output {
        self - Self {
            x: other,
            y: other,
            z: other,
        }
    }
}

impl<T> std::ops::Div<T> for Vec3<T>
where
    T: std::ops::Div<T, Output = T> + Copy,
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

impl<T> std::ops::Mul<T> for Vec3<T>
where
    T: std::ops::Mul<T, Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T> std::ops::Neg for Vec3<T>
where
    T: std::ops::Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> From<Point3<T>> for Vec3<T>
where
    T: Copy,
{
    fn from(other: Point3<T>) -> Self {
        Self {
            x: other.x,
            y: other.y,
            z: other.z,
        }
    }
}

impl From<Normal3f> for Vec3f {
    fn from(n: Normal3f) -> Self {
        Self {
            x: n.x,
            y: n.y,
            z: n.z,
        }
    }
}
