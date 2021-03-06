use crate::Matrix4;

impl Matrix4 {
    pub const IDENTITY: Matrix4 = Matrix4 {
        m00: 1.0, m10: 0.0, m20: 0.0, m30: 0.0,
        m01: 0.0, m11: 1.0, m21: 0.0, m31: 0.0,
        m02: 0.0, m12: 0.0, m22: 1.0, m32: 0.0,
        m03: 0.0, m13: 0.0, m23: 0.0, m33: 1.0,
    };
}