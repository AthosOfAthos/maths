use crate::{Vector2, Vector3};

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3 {
	pub x: Vector3,
	pub y: Vector3,
	pub z: Vector3,
}

impl Matrix3 {
	pub const IDENTITY: Self = Matrix3 {
		x: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
		y: Vector3 { x: 0.0, y: 1.0, z: 0.0 },
		z: Vector3 { x: 0.0, y: 0.0, z: 1.0 },
	};

	pub fn from_translation(translation: Vector2) -> Self {
		Matrix3 {
			x: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
			y: Vector3 { x: 0.0, y: 1.0, z: 0.0 },
			z: Vector3 { x: translation.x, y: translation.y, z: 1.0 },
		}
	}

	pub fn ortho(width: f32, height: f32) -> Self {
		Matrix3 {
			x: Vector3 { x: (2.0 / width), y: 0.0, z: 0.0 },
			y: Vector3 { x: 0.0, y: (2.0 / height), z: 0.0 },
			z: Vector3 { x: 0.0, y: 0.0, z: 1.0 },
		}
	}
}
