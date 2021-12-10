mod f32_ops;
mod vector2_ops;

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