use super::Float;

#[derive(new, Copy, Clone, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

pub type Vec2f = Vec2<Float>;
pub type Vec2i = Vec2<i32>;

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

impl From<(Float, Float)> for Vec2f {
    fn from(t: (Float, Float)) -> Self {
        Self { x: t.0, y: t.1 }
    }
}
