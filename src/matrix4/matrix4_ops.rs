use std::ops::Mul;
use crate::Matrix4;

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