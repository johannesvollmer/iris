use super::Float;
use crate::math::normal::Normal3f;

#[derive(new, Copy, Clone, Debug)]
pub struct LocalVec3f {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl LocalVec3f {
    pub fn dot(&self, other: &Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length_squared(&self) -> Float {
        self.dot(self)
    }

    // FloatODO: Separate type for this vector
    pub fn cos_theta(&self) -> Float {
        self.z
    }

    pub fn abs_cos_theta(&self) -> Float {
        self.z.abs()
    }

    /*pub fn cos_squared_theta(&self) -> Float
    where
        Float: std::ops::Mul<Output = Float> + Copy,
    {
        self.cos_theta() * self.cos_theta()
    }

    pub fn sin_squared_theta(&self) -> Float
    where
        Float: num::Float,
    {
        (Float::one() - self.cos_squared_theta()).max(Float::zero())
    }

    pub fn cross(&self, other: Self) -> Self
    {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn sin_theta(&self) -> Float
    where
        Float: num::Float,
    {
        self.sin_squared_theta().sqrt()
    }*/

    pub fn same_hemisphere(&self, other: &Self) -> bool {
        self.z * other.z > 0.0
    }

    pub fn length(&self) -> Float {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Self {
        *self / self.length()
    }
}

impl std::ops::Add for LocalVec3f {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Add<Float> for LocalVec3f {
    type Output = Self;

    fn add(self, other: Float) -> Self::Output {
        self + Self {
            x: other,
            y: other,
            z: other,
        }
    }
}

impl std::ops::Sub for LocalVec3f {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Sub<Float> for LocalVec3f {
    type Output = Self;

    fn sub(self, other: Float) -> Self::Output {
        self - Self {
            x: other,
            y: other,
            z: other,
        }
    }
}

impl std::ops::Div<Float> for LocalVec3f {
    type Output = Self;

    fn div(self, other: Float) -> Self::Output {
        Self::Output {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl std::ops::Mul<Float> for LocalVec3f {
    type Output = Self;

    fn mul(self, other: Float) -> Self::Output {
        Self::Output {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Neg for LocalVec3f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl From<Normal3f> for LocalVec3f {
    fn from(n: Normal3f) -> Self {
        Self {
            x: n.x,
            y: n.y,
            z: n.z,
        }
    }
}
