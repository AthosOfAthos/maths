use crate::IVector2;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

/// A single precision 2D Vector.
/// This struct is [repr(C)] with an alignment of 8 to satisfy std140
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    pub const ONE: Vector2 = Vector2 { x: 1.0, y: 1.0 };
    pub const UP: Vector2 = Vector2 { x: 0.0, y: 1.0 };
    pub const DOWN: Vector2 = Vector2 { x: 0.0, y: -1.0 };
    pub const LEFT: Vector2 = Vector2 { x: -1.0, y: 0.0 };
    pub const RIGHT: Vector2 = Vector2 { x: 1.0, y: 0.0 };

    pub fn magnitude(&self) -> f32 {
        ( (self.x * self.x) + (self.y * self.y) ).sqrt()
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.x /= magnitude;
        self.y /= magnitude;
    }

    pub fn normalized(&self) -> Vector2 {
        Vector2 {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
        }
    }
}

impl From<IVector2> for Vector2 {
    fn from(value: IVector2) -> Self {
        Vector2 {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

// F32 OPS
impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

// VECTOR2 OPS
impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl MulAssign<Vector2> for Vector2 {
    fn mul_assign(&mut self, rhs: Vector2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Div<Vector2> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl DivAssign<Vector2> for Vector2 {
    fn div_assign(&mut self, rhs: Vector2) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

#[cfg(test)]
mod tests {
    use crate::Vector2;
    #[test]
    fn test() {
        assert_eq!(Vector2 { x: 8.6, y: 1.5 } + Vector2 { x: -5.2, y: 0.1 }, Vector2 { x: 8.6 - 5.2, y: 1.5 + 0.1 });
        assert_eq!(Vector2 { x: 1.2, y: 5.0 } - Vector2 { x: 0.2, y: 1.0 }, Vector2 { x: 1.2 - 0.2, y: 5.0 - 1.0 });
        assert_eq!(Vector2 { x: 5.0, y: 1.0 } * Vector2 { x: 5.0, y: 6.9 }, Vector2 { x: 5.0 * 5.0, y: 1.0 * 6.9 });
        assert_eq!(Vector2 { x: 8.0, y: 5.6 } / Vector2 { x: 2.0, y: 1.0 }, Vector2 { x: 8.0 / 2.0, y: 5.6 / 1.0 });
    }
}