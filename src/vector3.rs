mod f32_ops;
mod vector3_ops;

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub const ZERO: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Vector3 = Vector3 { x: 1.0, y: 1.0, z: 1.0 };
    pub const UP: Vector3 = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
    pub const DOWN: Vector3 = Vector3 { x: 0.0, y: -1.0, z: 0.0 };
    pub const LEFT: Vector3 = Vector3 { x: -1.0, y: 0.0, z: 0.0 };
    pub const RIGHT: Vector3 = Vector3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const FORWARD: Vector3 = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
    pub const BACK: Vector3 = Vector3 { x: 0.0, y: 0.0, z: -1.0 };

    pub fn magnitude(&self) -> f32 {
        ( (self.x * self.x) + (self.y * self.y) + (self.z * self.z) ).sqrt()
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.x /= magnitude;
        self.y /= magnitude;
        self.z /= magnitude;
    }
}

#[test]
fn test() {
    assert_eq!(Vector3{ x: 2.0, y: 2.0, z: 2.0 } + Vector3{ x: 2.0, y: 2.0, z: 2.0 }, Vector3{ x: 2.0 + 2.0, y: 2.0 + 2.0, z: 2.0 + 2.0 });
}