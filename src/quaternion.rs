use std::ops::{Mul, MulAssign};
use crate::Vector3;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub const IDENTITY: Quaternion = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    pub fn from_euler(euler: Vector3) -> Quaternion {
        const HALF_TO_RAD: f32 = std::f32::consts::PI / 360.0;
        let x = euler.x * HALF_TO_RAD;
        let y = euler.y * HALF_TO_RAD;
        let z = euler.z * HALF_TO_RAD;

        let sx = x.sin();
        let cx = x.cos();
        let sy = y.sin();
        let cy = y.cos();
        let sz = z.sin();
        let cz = z.cos();

        Quaternion {
            x: sx * cy * cz - cx * sy * sz,
            y: cx * sy * cz + sx * cy * sz,
            z: cx * cy * sz - sx * sy * cz,
            w: cx * cy * cz + sx * sy * sz
        }
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Self::Output {
        Quaternion {
            x: self.x * rhs.w + self.w * rhs.x + self.y * rhs.z - self.z * rhs.y,
            y: self.y * rhs.w + self.w * rhs.y + self.z * rhs.x - self.x * rhs.z,
            z: self.z * rhs.w + self.w * rhs.z + self.x * rhs.y - self.y * rhs.x,
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z
        }
    }
}

impl MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, rhs: Quaternion) {
        let ax = self.x * rhs.w + self.w * rhs.x + self.y * rhs.z - self.z * rhs.y;
        let ay = self.y * rhs.w + self.w * rhs.y + self.z * rhs.x - self.x * rhs.z;
        let az = self.z * rhs.w + self.w * rhs.z + self.x * rhs.y - self.y * rhs.x;
        let aw = self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z;

        self.x = ax;
        self.y = ay;
        self.z = az;
        self.w = aw;
    }
}