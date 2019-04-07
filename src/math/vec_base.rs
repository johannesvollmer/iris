macro_rules! define_vec {
    ($vecbase:ident) => {
        use crate::math::Float;

        #[derive(Default, Copy, Clone, Debug)]
        pub struct $vecbase {
            pub x: Float,
            pub y: Float,
            pub z: Float,
        }

        impl $vecbase {
            #[inline(always)]
            #[allow(dead_code)]
            pub fn new(x: Float, y: Float, z: Float) -> Self {
                $vecbase { x, y, z }
            }

            #[allow(dead_code)]
            pub fn dot(self, other: Self) -> Float {
                self.x * other.x + self.y * other.y + self.z * other.z
            }

            #[allow(dead_code)]
            pub fn cross(self, other: Self) -> Self {
                Self::new(
                    self.y * other.z - self.z * other.y,
                    self.z * other.x - self.x * other.z,
                    self.x * other.y - self.y * other.x,
                )
            }

            #[allow(dead_code)]
            pub fn abs(self) -> Self {
                Self::new(self.x.abs(), self.y.abs(), self.z.abs())
            }

            #[allow(dead_code)]
            pub fn length_squared(self) -> Float {
                self.dot(self)
            }

            #[allow(dead_code)]
            pub fn length(self) -> Float {
                self.length_squared().sqrt()
            }

            #[allow(dead_code)]
            pub fn normalized(self) -> Self {
                self / self.length()
            }
        }

        impl std::cmp::PartialEq for $vecbase {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x && self.y == other.y && self.z == other.z
            }
        }

        impl std::cmp::PartialEq<Float> for $vecbase {
            fn eq(&self, other: &Float) -> bool {
                self.x == *other && self.y == *other && self.z == *other
            }
        }

        impl std::ops::Index<usize> for $vecbase {
            type Output = Float;

            fn index(&self, index: usize) -> &Float {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    2 => &self.z,
                    _ => panic!("out of bounds"),
                }
            }
        }

        impl std::ops::IndexMut<usize> for $vecbase {
            fn index_mut(&mut self, index: usize) -> &mut Float {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    2 => &mut self.z,
                    _ => panic!("out of bounds"),
                }
            }
        }

        impl std::ops::Add for $vecbase {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
            }
        }

        impl std::ops::AddAssign for $vecbase {
            fn add_assign(&mut self, other: Self) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        impl std::ops::Add<Float> for $vecbase {
            type Output = Self;

            fn add(self, other: Float) -> Self {
                Self::new(self.x + other, self.y + other, self.z + other)
            }
        }

        impl std::ops::AddAssign<Float> for $vecbase {
            fn add_assign(&mut self, other: Float) {
                self.x += other;
                self.y += other;
                self.z += other;
            }
        }

        impl std::ops::Sub for $vecbase {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }

        impl std::ops::SubAssign for $vecbase {
            fn sub_assign(&mut self, other: Self) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }

        impl std::ops::Sub<Float> for $vecbase {
            type Output = Self;

            fn sub(self, other: Float) -> Self {
                Self::new(self.x - other, self.y - other, self.z - other)
            }
        }

        impl std::ops::SubAssign<Float> for $vecbase {
            fn sub_assign(&mut self, other: Float) {
                self.x -= other;
                self.y -= other;
                self.z -= other;
            }
        }

        impl std::ops::Mul for $vecbase {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
            }
        }

        impl std::ops::MulAssign for $vecbase {
            fn mul_assign(&mut self, other: Self) {
                self.x *= other.x;
                self.y *= other.y;
                self.z *= other.z;
            }
        }

        impl std::ops::Mul<Float> for $vecbase {
            type Output = Self;

            fn mul(self, other: Float) -> Self {
                Self::new(self.x * other, self.y * other, self.z * other)
            }
        }

        impl std::ops::MulAssign<Float> for $vecbase {
            fn mul_assign(&mut self, other: Float) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
            }
        }

        impl std::ops::Div for $vecbase {
            type Output = Self;

            fn div(self, other: Self) -> Self {
                Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
            }
        }

        impl std::ops::DivAssign for $vecbase {
            fn div_assign(&mut self, other: Self) {
                self.x /= other.x;
                self.y /= other.y;
                self.z /= other.z;
            }
        }

        impl std::ops::Div<Float> for $vecbase {
            type Output = Self;

            fn div(self, other: Float) -> Self {
                Self::new(self.x / other, self.y / other, self.z / other)
            }
        }

        impl std::ops::DivAssign<Float> for $vecbase {
            fn div_assign(&mut self, other: Float) {
                self.x /= other;
                self.y /= other;
                self.z /= other;
            }
        }

        impl std::ops::Neg for $vecbase {
            type Output = Self;

            fn neg(self) -> Self {
                Self::new(-self.x, -self.y, -self.z)
            }
        }
    };
}
