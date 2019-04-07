macro_rules! define_point {
    ($pointbase:ident, $vecbase:ident) => {
        use crate::math::Float;

        #[derive(Default, Copy, Clone, Debug)]
        pub struct $pointbase {
            pub x: Float,
            pub y: Float,
            pub z: Float,
        }

        impl $pointbase {
            #[inline(always)]
            #[allow(dead_code)]
            pub fn new(x: Float, y: Float, z: Float) -> Self {
                Self { x, y, z }
            }

            #[allow(dead_code)]
            pub fn distance(self, other: Self) -> Float {
                self.distance_squared(other).sqrt()
            }

            #[allow(dead_code)]
            pub fn distance_squared(self, other: Self) -> Float {
                (self - other).length_squared()
            }

            #[allow(dead_code)]
            pub fn to_vec(self) -> $vecbase {
                $vecbase::new(self.x, self.y, self.z)
            }
        }

        impl std::cmp::PartialEq for $pointbase {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x && self.y == other.y && self.z == other.z
            }
        }

        impl std::cmp::PartialEq<Float> for $pointbase {
            fn eq(&self, other: &Float) -> bool {
                self.x == *other && self.y == *other && self.z == *other
            }
        }

        impl std::ops::Index<usize> for $pointbase {
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

        impl std::ops::IndexMut<usize> for $pointbase {
            fn index_mut(&mut self, index: usize) -> &mut Float {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    2 => &mut self.z,
                    _ => panic!("out of bounds"),
                }
            }
        }

        impl std::ops::Sub for $pointbase {
            type Output = $vecbase;

            fn sub(self, other: Self) -> $vecbase {
                $vecbase::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }

        impl std::ops::Add<$vecbase> for $pointbase {
            type Output = Self;

            fn add(self, other: $vecbase) -> Self {
                Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
            }
        }

        impl std::ops::AddAssign<$vecbase> for $pointbase {
            fn add_assign(&mut self, other: $vecbase) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        impl From<$vecbase> for $pointbase {
            fn from(v: $vecbase) -> Self {
                Self::new(v.x, v.y, v.z)
            }
        }
    };
}
