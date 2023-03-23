use crate::{Quaternion, Vector3};
use std::ops::Mul;

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix4 {
    pub m00: f32,
    pub m01: f32,
    pub m02: f32,
    pub m03: f32,
    pub m10: f32,
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m20: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m30: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
}

impl Matrix4 {
    pub const IDENTITY: Matrix4 = Matrix4 {
        m00: 1.0, m10: 0.0, m20: 0.0, m30: 0.0,
        m01: 0.0, m11: 1.0, m21: 0.0, m31: 0.0,
        m02: 0.0, m12: 0.0, m22: 1.0, m32: 0.0,
        m03: 0.0, m13: 0.0, m23: 0.0, m33: 1.0,
    };
    
    pub fn from_perspective(fov: f32, aspect: f32, clip_near: f32, clip_far: f32) -> Matrix4 {
        let f = 1.0 / (fov / 2.0).tan();
        let fa = f / aspect;
        let nf = 1.0 / (clip_near - clip_far);

        Matrix4 {
            m00: fa, m10: 0.0, m20: 0.0, m30: 0.0,
            m01: 0.0, m11: f, m21: 0.0, m31: 0.0,
            m02: 0.0, m12: 0.0, m22: (clip_near + clip_far) * nf, m32: 2.0 * clip_far * clip_near * nf,
            m03: 0.0, m13: 0.0, m23: -1.0, m33: 0.0,
        }
    }

    pub fn from_rotation_translation_scale(rotation: Quaternion, translation: Vector3, scale: Vector3) -> Matrix4 {
        let x2 = rotation.x + rotation.x;
        let y2 = rotation.y + rotation.y;
        let z2 = rotation.z + rotation.z;

        let xx = rotation.x * x2;
        let xy = rotation.x * y2;
        let xz = rotation.x * z2;
        let yy = rotation.y * y2;
        let yz = rotation.y * z2;
        let zz = rotation.z * z2;
        let wx = rotation.w * x2;
        let wy = rotation.w * y2;
        let wz = rotation.w * z2;

        Matrix4 {
            m00: (1.0 - (yy + zz)) * scale.x, m10: (xy - wz) * scale.y,         m20: (xz + wy) * scale.z,         m30: translation.x,
            m01: (xy + wz) * scale.x,         m11: (1.0 - (xx + zz)) * scale.y, m21: (yz - wx) * scale.z,         m31: translation.y,
            m02: (xz - wy) * scale.x,         m12: (yz + wx) * scale.y,         m22: (1.0 - (xx + yy)) * scale.z, m32: translation.z,
            m03: 0.0,                         m13: 0.0,                         m23: 0.0,                         m33: 1.0,
        }
    }

    pub fn transpose(&self) -> Self {
        Matrix4 {
            m00: self.m00, m10: self.m01, m20: self.m02, m30: self.m03,
            m01: self.m10, m11: self.m11, m21: self.m12, m31: self.m13,
            m02: self.m20, m12: self.m21, m22: self.m22, m32: self.m21,
            m03: self.m30, m13: self.m31, m23: self.m32, m33: self.m33,
        }
    }

    pub fn invert(&self) -> Self {
        let b00 = self.m00 * self.m11 - self.m01 * self.m10;
        let b01 = self.m00 * self.m12 - self.m02 * self.m10;
        let b02 = self.m00 * self.m13 - self.m03 * self.m10;
        let b03 = self.m01 * self.m12 - self.m02 * self.m11;
        let b04 = self.m01 * self.m13 - self.m03 * self.m11;
        let b05 = self.m02 * self.m13 - self.m03 * self.m12;
        let b06 = self.m20 * self.m31 - self.m21 * self.m30;
        let b07 = self.m20 * self.m32 - self.m22 * self.m30;
        let b08 = self.m20 * self.m33 - self.m23 * self.m30;
        let b09 = self.m21 * self.m32 - self.m22 * self.m31;
        let b10 = self.m21 * self.m33 - self.m23 * self.m31;
        let b11 = self.m22 * self.m33 - self.m23 * self.m32;
        let determinant = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
        let determinant = 1.0 / determinant;

        Matrix4 {
            m00: (self.m11 * b11 - self.m12 * b10 + self.m13 * b09) * determinant, m10: (self.m12 * b08 - self.m10 * b11 - self.m13 * b07) * determinant, m20: (self.m10 * b10 - self.m11 * b08 + self.m13 * b06) * determinant, m30: (self.m11 * b07 - self.m10 * b09 - self.m12 * b06) * determinant,
            m01: (self.m02 * b10 - self.m01 * b11 - self.m03 * b09) * determinant, m11: (self.m00 * b11 - self.m02 * b08 + self.m03 * b07) * determinant, m21: (self.m01 * b08 - self.m00 * b10 - self.m03 * b06) * determinant, m31: (self.m00 * b09 - self.m01 * b07 + self.m02 * b06) * determinant,
            m02: (self.m31 * b05 - self.m32 * b04 + self.m33 * b03) * determinant, m12: (self.m32 * b02 - self.m30 * b05 - self.m33 * b01) * determinant, m22: (self.m30 * b04 - self.m31 * b02 + self.m33 * b00) * determinant, m32: (self.m31 * b01 - self.m30 * b03 - self.m32 * b00) * determinant,
            m03: (self.m22 * b04 - self.m21 * b05 - self.m23 * b03) * determinant, m13: (self.m20 * b05 - self.m22 * b02 + self.m23 * b01) * determinant, m23: (self.m21 * b02 - self.m20 * b04 - self.m23 * b00) * determinant, m33: (self.m20 * b03 - self.m21 * b01 + self.m22 * b00) * determinant,
        }
    }

    pub fn translate(mut self, translation: Vector3) -> Self {
        self.m30 = (self.m00 * translation.x) + (self.m10 * translation.y) + (self.m20 * translation.z) + self.m30;
        self.m31 = (self.m01 * translation.x) + (self.m11 * translation.y) + (self.m21 * translation.z) + self.m31;
        self.m32 = (self.m02 * translation.x) + (self.m12 * translation.y) + (self.m22 * translation.z) + self.m32;
        self.m33 = (self.m03 * translation.x) + (self.m13 * translation.y) + (self.m23 * translation.z) + self.m33;
        self
    }

    pub fn scale(mut self, scale: Vector3) -> Self {
        self.m00 = self.m00 * scale.x;
        self.m01 = self.m01 * scale.x;
        self.m02 = self.m02 * scale.x;
        self.m03 = self.m03 * scale.x;

        self.m10 = self.m10 * scale.y;
        self.m11 = self.m11 * scale.y;
        self.m12 = self.m12 * scale.y;
        self.m13 = self.m13 * scale.y;

        self.m20 = self.m20 * scale.z;
        self.m21 = self.m21 * scale.z;
        self.m22 = self.m22 * scale.z;
        self.m23 = self.m23 * scale.z;
        self
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: Matrix4) -> Self::Output {
        Matrix4 {
            m00: (rhs.m00 * self.m00) + (rhs.m01 * self.m10) + (rhs.m02 * self.m20) + (rhs.m03 * self.m30),
            m01: (rhs.m00 * self.m01) + (rhs.m01 * self.m11) + (rhs.m02 * self.m21) + (rhs.m03 * self.m31),
            m02: (rhs.m00 * self.m02) + (rhs.m01 * self.m12) + (rhs.m02 * self.m22) + (rhs.m03 * self.m32),
            m03: (rhs.m00 * self.m03) + (rhs.m01 * self.m13) + (rhs.m02 * self.m23) + (rhs.m03 * self.m33),

            m10: (rhs.m10 * self.m00) + (rhs.m11 * self.m10) + (rhs.m12 * self.m20) + (rhs.m13 * self.m30),
            m11: (rhs.m10 * self.m01) + (rhs.m11 * self.m11) + (rhs.m12 * self.m21) + (rhs.m13 * self.m31),
            m12: (rhs.m10 * self.m02) + (rhs.m11 * self.m12) + (rhs.m12 * self.m22) + (rhs.m13 * self.m32),
            m13: (rhs.m10 * self.m03) + (rhs.m11 * self.m13) + (rhs.m12 * self.m23) + (rhs.m13 * self.m33),

            m20: (rhs.m20 * self.m00) + (rhs.m21 * self.m10) + (rhs.m22 * self.m20) + (rhs.m23 * self.m30),
            m21: (rhs.m20 * self.m01) + (rhs.m21 * self.m11) + (rhs.m22 * self.m21) + (rhs.m23 * self.m31),
            m22: (rhs.m20 * self.m02) + (rhs.m21 * self.m12) + (rhs.m22 * self.m22) + (rhs.m23 * self.m32),
            m23: (rhs.m20 * self.m03) + (rhs.m21 * self.m13) + (rhs.m22 * self.m23) + (rhs.m23 * self.m33),

            m30: (rhs.m30 * self.m00) + (rhs.m31 * self.m10) + (rhs.m32 * self.m20) + (rhs.m33 * self.m30),
            m31: (rhs.m30 * self.m01) + (rhs.m31 * self.m11) + (rhs.m32 * self.m21) + (rhs.m33 * self.m31),
            m32: (rhs.m30 * self.m02) + (rhs.m31 * self.m12) + (rhs.m32 * self.m22) + (rhs.m33 * self.m32),
            m33: (rhs.m30 * self.m03) + (rhs.m31 * self.m13) + (rhs.m32 * self.m23) + (rhs.m33 * self.m33)
        }
    }
}