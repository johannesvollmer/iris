// Adapted from https://github.com/Twinklebear/tray_rust/blob/master/src/film/color.rs
/*
The MIT License (MIT)

Copyright (c) 2014 Will Usher

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use crate::math;
use std::f32;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color4f {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color4f {
    pub fn new(r: f32, g: f32, b: f32) -> Color4f {
        Color4f {
            r: r,
            g: g,
            b: b,
            a: 1.0,
        }
    }

    pub const fn with_alpha(r: f32, g: f32, b: f32, a: f32) -> Color4f {
        Color4f {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub const fn broadcast(x: f32) -> Color4f {
        Color4f {
            r: x,
            g: x,
            b: x,
            a: 1.0,
        }
    }

    pub const fn black() -> Color4f {
        Color4f::broadcast(0.0)
    }
    pub const fn white() -> Color4f {
        Color4f::broadcast(1.0)
    }

    pub fn clamp(&self) -> Color4f {
        Color4f {
            r: math::clamp(self.r, 0.0, 1.0),
            g: math::clamp(self.g, 0.0, 1.0),
            b: math::clamp(self.b, 0.0, 1.0),
            a: math::clamp(self.a, 0.0, 1.0),
        }
    }

    pub fn has_nans(&self) -> bool {
        f32::is_nan(self.r) || f32::is_nan(self.g) || f32::is_nan(self.b) || f32::is_nan(self.a)
    }

    pub fn has_infs(&self) -> bool {
        f32::is_infinite(self.r)
            || f32::is_infinite(self.g)
            || f32::is_infinite(self.b)
            || f32::is_infinite(self.a)
    }

    pub fn to_rgb8(&self) -> [u8; 3] {
        let convert = |x: f32| (math::clamp(x, 0.0, 1.0) * 255.0).round() as u8;
        [convert(self.r), convert(self.g), convert(self.b)]
    }
}

impl Add for Color4f {
    type Output = Color4f;
    fn add(self, rhs: Color4f) -> Color4f {
        Color4f {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl Sub for Color4f {
    type Output = Color4f;
    fn sub(self, rhs: Color4f) -> Color4f {
        Color4f {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        }
    }
}

impl Mul for Color4f {
    type Output = Color4f;
    fn mul(self, rhs: Color4f) -> Color4f {
        Color4f {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

impl Mul<f32> for Color4f {
    type Output = Color4f;
    fn mul(self, rhs: f32) -> Color4f {
        Color4f {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}

impl Mul<Color4f> for f32 {
    type Output = Color4f;
    fn mul(self, rhs: Color4f) -> Color4f {
        Color4f {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
            a: self * rhs.a,
        }
    }
}

impl Div for Color4f {
    type Output = Color4f;
    fn div(self, rhs: Color4f) -> Color4f {
        Color4f {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
            a: self.a / rhs.a,
        }
    }
}

impl Div<f32> for Color4f {
    type Output = Color4f;
    fn div(self, rhs: f32) -> Color4f {
        Color4f {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a / rhs,
        }
    }
}

impl Neg for Color4f {
    type Output = Color4f;
    fn neg(self) -> Color4f {
        Color4f {
            r: -self.r,
            g: -self.g,
            b: -self.b,
            a: -self.a,
        }
    }
}

impl Index<usize> for Color4f {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            3 => &self.a,
            _ => panic!("Invalid index into color"),
        }
    }
}

impl IndexMut<usize> for Color4f {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            3 => &mut self.a,
            _ => panic!("Invalid index into color"),
        }
    }
}
