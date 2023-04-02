use core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use libm::sqrtf;

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vector4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

impl Vector4 {
	pub const ZERO: Self = Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
	pub const ONE: Self = Vector4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };

	pub const fn splat(val: f32) -> Self {
		Vector4 { x: val, y: val, z: val, w: val }
	}

	pub fn magnitude(&self) -> f32 {
		sqrtf((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w))
	}

	pub fn normalize(&mut self) {
		let magnitude = self.magnitude();
		self.x /= magnitude;
		self.y /= magnitude;
		self.z /= magnitude;
		self.w /= magnitude;
	}
}

// F32 OPS
impl Mul<f32> for Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: f32) -> Self::Output {
		Vector4 {
		    x: self.x * rhs,
		    y: self.y * rhs,
		    z: self.z * rhs,
		    w: self.w * rhs,
		}
    }
}
impl MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, rhs: f32) {
		self.x *= rhs;
		self.y *= rhs;
		self.z *= rhs;
		self.w *= rhs;
    }
}

impl Div<f32> for Vector4 {
    type Output = Vector4;
    fn div(self, rhs: f32) -> Self::Output {
		Vector4 {
		    x: self.x / rhs,
		    y: self.y / rhs,
		    z: self.z / rhs,
		    w: self.w / rhs,
		}
    }
}
impl DivAssign<f32> for Vector4 {
    fn div_assign(&mut self, rhs: f32) {
		self.x /= rhs;
		self.y /= rhs;
		self.z /= rhs;
		self.w /= rhs;
    }
}

// VECTOR4 OPS
impl Add<Vector4> for Vector4 {
    type Output = Vector4;
    fn add(self, rhs: Vector4) -> Self::Output {
		Vector4 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
			w: self.w + rhs.w,
		}
    }
}
impl AddAssign<Vector4> for Vector4 {
    fn add_assign(&mut self, rhs: Vector4) {
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
		self.w += rhs.w;
    }
}

impl Sub<Vector4> for Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: Vector4) -> Self::Output {
		Vector4 {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
			w: self.w - rhs.w,
		}
    }
}
impl SubAssign<Vector4> for Vector4 {
    fn sub_assign(&mut self, rhs: Vector4) {
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
		self.w -= rhs.w;
    }
}

impl Mul<Vector4> for Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Self::Output {
		Vector4 {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
			z: self.z * rhs.z,
			w: self.w * rhs.w,
		}
    }
}
impl MulAssign<Vector4> for Vector4 {
    fn mul_assign(&mut self, rhs: Vector4) {
		self.x *= rhs.x;
		self.y *= rhs.y;
		self.z *= rhs.z;
		self.w *= rhs.w;
    }
}

impl Div<Vector4> for Vector4 {
    type Output = Vector4;
    fn div(self, rhs: Vector4) -> Self::Output {
		Vector4 {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
			z: self.z / rhs.z,
			w: self.w / rhs.w,
		}
    }
}
impl DivAssign<Vector4> for Vector4 {
    fn div_assign(&mut self, rhs: Vector4) {
		self.x /= rhs.x;
		self.y /= rhs.y;
		self.z /= rhs.z;
		self.w /= rhs.w;
    }
}
