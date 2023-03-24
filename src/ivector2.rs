use crate::Vector2;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[repr(C, align(8))]
#[derive(Clone, Copy, Debug)]
pub struct IVector2 {
	pub x: i32,
	pub y: i32,
}

impl IVector2 {
	pub const ZERO: Self = IVector2::splat(0);
	pub const ONE: Self = IVector2::splat(1);
	pub const UP: Self = IVector2 { x: 0, y: 1 };
	pub const DOWN: Self = IVector2 { x: 0, y: -1 };
	pub const LEFT: Self = IVector2 { x: 1, y: 0 };
	pub const RIGHT: Self = IVector2 { x: -1, y: 0 };

	pub const fn splat(value: i32) -> Self {
		IVector2 { x: value, y: value }
	}
}

impl PartialEq for IVector2 {
    fn eq(&self, other: &IVector2) -> bool {
		(self.x == other.x) && (self.y == other.y)
    }
}
impl Eq for IVector2 {}

impl From<Vector2> for IVector2 {
    fn from(value: Vector2) -> Self {
		IVector2 {
			x: value.x as i32,
			y: value.y as i32,
		}
    }
}

impl Hash for IVector2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
		unsafe {
			const SIZE: usize = std::mem::size_of::<IVector2>();
			let ptr = self as *const IVector2 as *const u8;
			let bytes = std::slice::from_raw_parts(ptr, SIZE);
			state.write(bytes);
		}
    }
}

impl Add<IVector2> for IVector2 {
    type Output = IVector2;
    fn add(self, rhs: IVector2) -> Self::Output {
		IVector2 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
    }
}

impl AddAssign<IVector2> for IVector2 {
    fn add_assign(&mut self, rhs: IVector2) {
		self.x += rhs.x;
		self.y += rhs.y;
    }
}

impl Sub<IVector2> for IVector2 {
    type Output = IVector2;
    fn sub(self, rhs: IVector2) -> Self::Output {
		IVector2 {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
    }
}

impl SubAssign<IVector2> for IVector2 {
    fn sub_assign(&mut self, rhs: IVector2) {
		self.x -= rhs.x;
		self.y -= rhs.y;
    }
}

impl Mul<IVector2> for IVector2 {
    type Output = IVector2;
    fn mul(self, rhs: IVector2) -> Self::Output {
		IVector2 {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		}
    }
}

impl MulAssign<IVector2> for IVector2 {
    fn mul_assign(&mut self, rhs: IVector2) {
		self.x *= rhs.x;
		self.y *= rhs.y;
    }
}

impl Div<IVector2> for IVector2 {
    type Output = IVector2;
    fn div(self, rhs: IVector2) -> Self::Output {
		IVector2 {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
		}
    }
}

impl DivAssign<IVector2> for IVector2 {
    fn div_assign(&mut self, rhs: IVector2) {
		self.x /= rhs.x;
		self.y /= rhs.y;
    }
}

#[cfg(test)]
mod ivector2_tests{
	use crate::IVector2;
	
	#[test]
	fn hash() {
		use std::hash::{Hash, Hasher};
		use std::collections::hash_map::DefaultHasher;
		let zero_hash = {
			let mut hasher = DefaultHasher::new();
			IVector2::ZERO.hash(&mut hasher);
			hasher.finish()
		};
		let one_hash = {
			let mut hasher = DefaultHasher::new();
			IVector2::ONE.hash(&mut hasher);
			hasher.finish()
		};
		let zero_two_hash = {
			let mut hasher = DefaultHasher::new();
			IVector2::ZERO.hash(&mut hasher);
			hasher.finish()
		};

		assert_ne!(zero_hash, one_hash);
		assert_eq!(zero_hash, zero_two_hash);
	}
}