#![no_std]
//! Bespoke vector math library
//!

mod ivector2;
pub use ivector2::IVector2;

mod matrix3;
pub use matrix3::Matrix3;

mod matrix4;
pub use matrix4::Matrix4;

mod quaternion;
pub use quaternion::Quaternion;

mod vector2;
pub use vector2::Vector2;

mod vector3;
pub use vector3::Vector3;