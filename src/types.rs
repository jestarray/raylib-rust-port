use glam::{Vec2, Vec3, Vec4};
use std::ffi::c_void;

pub type Vector2 = Vec2;
pub type Vector3 = Vec3;
pub type Vector4 = Vec4;
pub type Quaternion = Vec4;
//pub type Matrix = Mat4;

use std::ops::Mul;
pub const RAYLIB_VERSION: &str = "6.1-dev";
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Matrix {
    // First row
    pub m0: f32,
    pub m4: f32,
    pub m8: f32,
    pub m12: f32,

    // Second row
    pub m1: f32,
    pub m5: f32,
    pub m9: f32,
    pub m13: f32,

    // Third row
    pub m2: f32,
    pub m6: f32,
    pub m10: f32,
    pub m14: f32,

    // Fourth row
    pub m3: f32,
    pub m7: f32,
    pub m11: f32,
    pub m15: f32,
}

impl Matrix {
    pub const ZERO: Self = Self {
        m0: 0.0,
        m4: 0.0,
        m8: 0.0,
        m12: 0.0,

        m1: 0.0,
        m5: 0.0,
        m9: 0.0,
        m13: 0.0,

        m2: 0.0,
        m6: 0.0,
        m10: 0.0,
        m14: 0.0,

        m3: 0.0,
        m7: 0.0,
        m11: 0.0,
        m15: 0.0,
    };
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        m0: f32,
        m4: f32,
        m8: f32,
        m12: f32,
        m1: f32,
        m5: f32,
        m9: f32,
        m13: f32,
        m2: f32,
        m6: f32,
        m10: f32,
        m14: f32,
        m3: f32,
        m7: f32,
        m11: f32,
        m15: f32,
    ) -> Self {
        Self {
            m0,
            m4,
            m8,
            m12,
            m1,
            m5,
            m9,
            m13,
            m2,
            m6,
            m10,
            m14,
            m3,
            m7,
            m11,
            m15,
        }
    }
    pub const IDENTITY: Self = Self::identity();
    pub const fn identity() -> Self {
        Self {
            m0: 1.0,
            m4: 0.0,
            m8: 0.0,
            m12: 0.0,

            m1: 0.0,
            m5: 1.0,
            m9: 0.0,
            m13: 0.0,

            m2: 0.0,
            m6: 0.0,
            m10: 1.0,
            m14: 0.0,

            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }

    /// Explicit conversion to column-major memory layout
    pub fn to_array(self) -> [f32; 16] {
        [
            self.m0, self.m1, self.m2, self.m3, self.m4, self.m5, self.m6, self.m7, self.m8,
            self.m9, self.m10, self.m11, self.m12, self.m13, self.m14, self.m15,
        ]
    }

    pub fn transpose(self) -> Self {
        Self {
            m0: self.m0,
            m1: self.m4,
            m2: self.m8,
            m3: self.m12,

            m4: self.m1,
            m5: self.m5,
            m6: self.m9,
            m7: self.m13,

            m8: self.m2,
            m9: self.m6,
            m10: self.m10,
            m11: self.m14,

            m12: self.m3,
            m13: self.m7,
            m14: self.m11,
            m15: self.m15,
        }
    }

    pub fn invert(self) -> Self {
        let a00 = self.m0;
        let a01 = self.m1;
        let a02 = self.m2;
        let a03 = self.m3;

        let a10 = self.m4;
        let a11 = self.m5;
        let a12 = self.m6;
        let a13 = self.m7;

        let a20 = self.m8;
        let a21 = self.m9;
        let a22 = self.m10;
        let a23 = self.m11;

        let a30 = self.m12;
        let a31 = self.m13;
        let a32 = self.m14;
        let a33 = self.m15;

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        let inv_det = 1.0 / (b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06);

        Self {
            m0: (a11 * b11 - a12 * b10 + a13 * b09) * inv_det,
            m1: (-a01 * b11 + a02 * b10 - a03 * b09) * inv_det,
            m2: (a31 * b05 - a32 * b04 + a33 * b03) * inv_det,
            m3: (-a21 * b05 + a22 * b04 - a23 * b03) * inv_det,

            m4: (-a10 * b11 + a12 * b08 - a13 * b07) * inv_det,
            m5: (a00 * b11 - a02 * b08 + a03 * b07) * inv_det,
            m6: (-a30 * b05 + a32 * b02 - a33 * b01) * inv_det,
            m7: (a20 * b05 - a22 * b02 + a23 * b01) * inv_det,

            m8: (a10 * b10 - a11 * b08 + a13 * b06) * inv_det,
            m9: (-a00 * b10 + a01 * b08 - a03 * b06) * inv_det,
            m10: (a30 * b04 - a31 * b02 + a33 * b00) * inv_det,
            m11: (-a20 * b04 + a21 * b02 - a23 * b00) * inv_det,

            m12: (-a10 * b09 + a11 * b07 - a12 * b06) * inv_det,
            m13: (a00 * b09 - a01 * b07 + a02 * b06) * inv_det,
            m14: (-a30 * b03 + a31 * b01 - a32 * b00) * inv_det,
            m15: (a20 * b03 - a21 * b01 + a22 * b00) * inv_det,
        }
    }
    /// Compute matrix determinant.
    pub fn determinant(self) -> f32 {
        let m0 = self.m0;
        let m1 = self.m1;
        let m2 = self.m2;
        let m3 = self.m3;
        let m4 = self.m4;
        let m5 = self.m5;
        let m6 = self.m6;
        let m7 = self.m7;
        let m8 = self.m8;
        let m9 = self.m9;
        let m10 = self.m10;
        let m11 = self.m11;
        let m12 = self.m12;
        let m13 = self.m13;
        let m14 = self.m14;
        let m15 = self.m15;

        m0 * (m5 * (m10 * m15 - m11 * m14) - m9 * (m6 * m15 - m7 * m14)
            + m13 * (m6 * m11 - m7 * m10))
            - m4 * (m1 * (m10 * m15 - m11 * m14) - m9 * (m2 * m15 - m3 * m14)
                + m13 * (m2 * m11 - m3 * m10))
            + m8 * (m1 * (m6 * m15 - m7 * m14) - m5 * (m2 * m15 - m3 * m14)
                + m13 * (m2 * m7 - m3 * m6))
            - m12
                * (m1 * (m6 * m11 - m7 * m10) - m5 * (m2 * m11 - m3 * m10)
                    + m9 * (m2 * m7 - m3 * m6))
    }

    /// Get the trace of the matrix.
    pub fn trace(self) -> f32 {
        self.m0 + self.m5 + self.m10 + self.m15
    }

    /// Add two matrices.
    pub fn add(self, right: Self) -> Self {
        Self {
            m0: self.m0 + right.m0,
            m4: self.m4 + right.m4,
            m8: self.m8 + right.m8,
            m12: self.m12 + right.m12,

            m1: self.m1 + right.m1,
            m5: self.m5 + right.m5,
            m9: self.m9 + right.m9,
            m13: self.m13 + right.m13,

            m2: self.m2 + right.m2,
            m6: self.m6 + right.m6,
            m10: self.m10 + right.m10,
            m14: self.m14 + right.m14,

            m3: self.m3 + right.m3,
            m7: self.m7 + right.m7,
            m11: self.m11 + right.m11,
            m15: self.m15 + right.m15,
        }
    }

    /// Subtract two matrices (`self - right`).
    pub fn subtract(self, right: Self) -> Self {
        Self {
            m0: self.m0 - right.m0,
            m4: self.m4 - right.m4,
            m8: self.m8 - right.m8,
            m12: self.m12 - right.m12,

            m1: self.m1 - right.m1,
            m5: self.m5 - right.m5,
            m9: self.m9 - right.m9,
            m13: self.m13 - right.m13,

            m2: self.m2 - right.m2,
            m6: self.m6 - right.m6,
            m10: self.m10 - right.m10,
            m14: self.m14 - right.m14,

            m3: self.m3 - right.m3,
            m7: self.m7 - right.m7,
            m11: self.m11 - right.m11,
            m15: self.m15 - right.m15,
        }
    }

    /// Multiply two matrices.
    ///
    /// NOTE: When multiplying matrices, the order matters.
    pub fn multiply(self, right: Self) -> Self {
        self * right
    }

    /// Multiply all matrix components by a value.
    pub fn multiply_value(self, value: f32) -> Self {
        Self {
            m0: self.m0 * value,
            m4: self.m4 * value,
            m8: self.m8 * value,
            m12: self.m12 * value,

            m1: self.m1 * value,
            m5: self.m5 * value,
            m9: self.m9 * value,
            m13: self.m13 * value,

            m2: self.m2 * value,
            m6: self.m6 * value,
            m10: self.m10 * value,
            m14: self.m14 * value,

            m3: self.m3 * value,
            m7: self.m7 * value,
            m11: self.m11 * value,
            m15: self.m15 * value,
        }
    }

    /// Get translation matrix.
    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        Self {
            m0: 1.0,
            m4: 0.0,
            m8: 0.0,
            m12: x,

            m1: 0.0,
            m5: 1.0,
            m9: 0.0,
            m13: y,

            m2: 0.0,
            m6: 0.0,
            m10: 1.0,
            m14: z,

            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }

    /// Create rotation matrix from axis and angle.
    ///
    /// NOTE: Angle must be provided in radians.
    pub fn rotate(axis: Vector3, angle: f32) -> Self {
        let mut x = axis.x;
        let mut y = axis.y;
        let mut z = axis.z;

        let length_squared = x * x + y * y + z * z;

        if length_squared != 1.0 && length_squared != 0.0 {
            let inverse_length = 1.0 / length_squared.sqrt();

            x *= inverse_length;
            y *= inverse_length;
            z *= inverse_length;
        }

        let sinres = angle.sin();
        let cosres = angle.cos();
        let t = 1.0 - cosres;

        Self {
            m0: x * x * t + cosres,
            m4: x * y * t - z * sinres,
            m8: x * z * t + y * sinres,
            m12: 0.0,

            m1: y * x * t + z * sinres,
            m5: y * y * t + cosres,
            m9: y * z * t - x * sinres,
            m13: 0.0,

            m2: z * x * t - y * sinres,
            m6: z * y * t + x * sinres,
            m10: z * z * t + cosres,
            m14: 0.0,

            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }

    /// Get x-rotation matrix.
    ///
    /// NOTE: Angle must be provided in radians.
    pub fn rotate_x(angle: f32) -> Self {
        let cosres = angle.cos();
        let sinres = angle.sin();

        Self {
            m0: 1.0,
            m4: 0.0,
            m8: 0.0,
            m12: 0.0,

            m1: 0.0,
            m5: cosres,
            m9: -sinres,
            m13: 0.0,

            m2: 0.0,
            m6: sinres,
            m10: cosres,
            m14: 0.0,

            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }

    /// Get y-rotation matrix.
    ///
    /// NOTE: Angle must be provided in radians.
    pub fn rotate_y(angle: f32) -> Self {
        let cosres = angle.cos();
        let sinres = angle.sin();

        Self {
            m0: cosres,
            m4: 0.0,
            m8: sinres,
            m12: 0.0,

            m1: 0.0,
            m5: 1.0,
            m9: 0.0,
            m13: 0.0,

            m2: -sinres,
            m6: 0.0,
            m10: cosres,
            m14: 0.0,

            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }

    /// Get z-rotation matrix.
    ///
    /// NOTE: Angle must be provided in radians.
    pub fn rotate_z(angle: f32) -> Self {
        let cosres = angle.cos();
        let sinres = angle.sin();

        Self {
            m0: cosres,
            m4: -sinres,
            m8: 0.0,
            m12: 0.0,

            m1: sinres,
            m5: cosres,
            m9: 0.0,
            m13: 0.0,

            m2: 0.0,
            m6: 0.0,
            m10: 1.0,
            m14: 0.0,

            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }

    /// Get xyz-rotation matrix.
    ///
    /// NOTE: Angles must be provided in radians.
    pub fn rotate_xyz(angle: Vector3) -> Self {
        let cosz = (-angle.z).cos();
        let sinz = (-angle.z).sin();
        let cosy = (-angle.y).cos();
        let siny = (-angle.y).sin();
        let cosx = (-angle.x).cos();
        let sinx = (-angle.x).sin();

        Self {
            m0: cosz * cosy,
            m4: sinz * cosy,
            m8: -siny,
            m12: 0.0,

            m1: cosz * siny * sinx - sinz * cosx,
            m5: sinz * siny * sinx + cosz * cosx,
            m9: cosy * sinx,
            m13: 0.0,

            m2: cosz * siny * cosx + sinz * sinx,
            m6: sinz * siny * cosx - cosz * sinx,
            m10: cosy * cosx,
            m14: 0.0,

            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }

    /// Get zyx-rotation matrix.
    ///
    /// NOTE: Angles must be provided in radians.
    pub fn rotate_zyx(angle: Vector3) -> Self {
        let cz = angle.z.cos();
        let sz = angle.z.sin();
        let cy = angle.y.cos();
        let sy = angle.y.sin();
        let cx = angle.x.cos();
        let sx = angle.x.sin();

        Self {
            m0: cz * cy,
            m4: cz * sy * sx - cx * sz,
            m8: sz * sx + cz * cx * sy,
            m12: 0.0,

            m1: cy * sz,
            m5: cz * cx + sz * sy * sx,
            m9: cx * sz * sy - cz * sx,
            m13: 0.0,

            m2: -sy,
            m6: cy * sx,
            m10: cy * cx,
            m14: 0.0,

            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }

    /// Get scaling matrix.
    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Self {
            m0: x,
            m4: 0.0,
            m8: 0.0,
            m12: 0.0,

            m1: 0.0,
            m5: y,
            m9: 0.0,
            m13: 0.0,

            m2: 0.0,
            m6: 0.0,
            m10: z,
            m14: 0.0,

            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }

    /// Get perspective projection matrix.
    pub fn frustum(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near_plane: f64,
        far_plane: f64,
    ) -> Self {
        let rl = (right - left) as f32;
        let tb = (top - bottom) as f32;
        let fn_ = (far_plane - near_plane) as f32;

        Self {
            m0: near_plane as f32 * 2.0 / rl,
            m4: 0.0,
            m8: (right as f32 + left as f32) / rl,
            m12: 0.0,

            m1: 0.0,
            m5: near_plane as f32 * 2.0 / tb,
            m9: (top as f32 + bottom as f32) / tb,
            m13: 0.0,

            m2: 0.0,
            m6: 0.0,
            m10: -(far_plane as f32 + near_plane as f32) / fn_,
            m14: -(far_plane as f32 * near_plane as f32 * 2.0) / fn_,

            m3: 0.0,
            m7: 0.0,
            m11: -1.0,
            m15: 0.0,
        }
    }

    /// Get perspective projection matrix.
    ///
    /// NOTE: `fov_y` must be provided in radians.
    pub fn perspective(fov_y: f64, aspect: f64, near_plane: f64, far_plane: f64) -> Self {
        let top = near_plane * (fov_y * 0.5).tan();
        let bottom = -top;
        let right = top * aspect;
        let left = -right;

        Self::frustum(left, right, bottom, top, near_plane, far_plane)
    }

    /// Get orthographic projection matrix.
    pub fn ortho(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near_plane: f64,
        far_plane: f64,
    ) -> Self {
        let rl = (right - left) as f32;
        let tb = (top - bottom) as f32;
        let fn_ = (far_plane - near_plane) as f32;

        Self {
            m0: 2.0 / rl,
            m4: 0.0,
            m8: 0.0,
            m12: -(left as f32 + right as f32) / rl,

            m1: 0.0,
            m5: 2.0 / tb,
            m9: 0.0,
            m13: -(top as f32 + bottom as f32) / tb,

            m2: 0.0,
            m6: 0.0,
            m10: -2.0 / fn_,
            m14: -(far_plane as f32 + near_plane as f32) / fn_,

            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }

    /// Get camera look-at matrix.
    pub fn look_at(eye: Vector3, target: Vector3, up: Vector3) -> Self {
        // Vector3Subtract(eye, target)
        let mut vz = Vector3 {
            x: eye.x - target.x,
            y: eye.y - target.y,
            z: eye.z - target.z,
        };

        // Vector3Normalize(vz)
        let mut length = (vz.x * vz.x + vz.y * vz.y + vz.z * vz.z).sqrt();

        if length == 0.0 {
            length = 1.0;
        }

        let inverse_length = 1.0 / length;
        vz.x *= inverse_length;
        vz.y *= inverse_length;
        vz.z *= inverse_length;

        // Vector3CrossProduct(up, vz)
        let mut vx = Vector3 {
            x: up.y * vz.z - up.z * vz.y,
            y: up.z * vz.x - up.x * vz.z,
            z: up.x * vz.y - up.y * vz.x,
        };

        // Vector3Normalize(vx)
        length = (vx.x * vx.x + vx.y * vx.y + vx.z * vx.z).sqrt();

        if length == 0.0 {
            length = 1.0;
        }

        let inverse_length = 1.0 / length;
        vx.x *= inverse_length;
        vx.y *= inverse_length;
        vx.z *= inverse_length;

        // Vector3CrossProduct(vz, vx)
        let vy = Vector3 {
            x: vz.y * vx.z - vz.z * vx.y,
            y: vz.z * vx.x - vz.x * vx.z,
            z: vz.x * vx.y - vz.y * vx.x,
        };

        Self {
            m0: vx.x,
            m4: vx.y,
            m8: vx.z,
            m12: -(vx.x * eye.x + vx.y * eye.y + vx.z * eye.z),

            m1: vy.x,
            m5: vy.y,
            m9: vy.z,
            m13: -(vy.x * eye.x + vy.y * eye.y + vy.z * eye.z),

            m2: vz.x,
            m6: vz.y,
            m10: vz.z,
            m14: -(vz.x * eye.x + vz.y * eye.y + vz.z * eye.z),

            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

impl Mul for Matrix {
    type Output = Self;

    #[rustfmt::skip]
    fn mul(self, right: Self) -> Self::Output {
        let left = self;
        Self {
            m0: left.m0 * right.m0 + left.m1 * right.m4 + left.m2 * right.m8 + left.m3 * right.m12,
            m1: left.m0 * right.m1 + left.m1 * right.m5 + left.m2 * right.m9 + left.m3 * right.m13,
            m2: left.m0 * right.m2 + left.m1 * right.m6 + left.m2 * right.m10 + left.m3 * right.m14,
            m3: left.m0 * right.m3 + left.m1 * right.m7 + left.m2 * right.m11 + left.m3 * right.m15,
            m4: left.m4 * right.m0 + left.m5 * right.m4 + left.m6 * right.m8 + left.m7 * right.m12,
            m5: left.m4 * right.m1 + left.m5 * right.m5 + left.m6 * right.m9 + left.m7 * right.m13,
            m6: left.m4 * right.m2 + left.m5 * right.m6 + left.m6 * right.m10 + left.m7 * right.m14,
            m7: left.m4 * right.m3 + left.m5 * right.m7 + left.m6 * right.m11 + left.m7 * right.m15,
            m8: left.m8 * right.m0 + left.m9 * right.m4 + left.m10 * right.m8 + left.m11 * right.m12,
            m9: left.m8 * right.m1 + left.m9 * right.m5 + left.m10 * right.m9 + left.m11 * right.m13,
            m10: left.m8 * right.m2 + left.m9 * right.m6 + left.m10 * right.m10 + left.m11 * right.m14,
            m11: left.m8 * right.m3 + left.m9 * right.m7 + left.m10 * right.m11 + left.m11 * right.m15,
            m12: left.m12 * right.m0 + left.m13 * right.m4 + left.m14 * right.m8 + left.m15 * right.m12,
            m13: left.m12 * right.m1 + left.m13 * right.m5 + left.m14 * right.m9 + left.m15 * right.m13,
            m14: left.m12 * right.m2 + left.m13 * right.m6 + left.m14 * right.m10 + left.m15 * right.m14,
            m15: left.m12 * right.m3 + left.m13 * right.m7 + left.m14 * right.m11 + left.m15 * right.m15,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const LIGHTGRAY: Color = Color::new(200, 200, 200, 255);
    pub const GRAY: Color = Color::new(130, 130, 130, 255);
    pub const DARKGRAY: Color = Color::new(80, 80, 80, 255);
    pub const YELLOW: Color = Color::new(253, 249, 0, 255);
    pub const GOLD: Color = Color::new(255, 203, 0, 255);
    pub const ORANGE: Color = Color::new(255, 161, 0, 255);
    pub const PINK: Color = Color::new(255, 109, 194, 255);
    pub const RED: Color = Color::new(230, 41, 55, 255);
    pub const MAROON: Color = Color::new(190, 33, 55, 255);
    pub const GREEN: Color = Color::new(0, 228, 48, 255);
    pub const LIME: Color = Color::new(0, 158, 47, 255);
    pub const DARKGREEN: Color = Color::new(0, 117, 44, 255);
    pub const SKYBLUE: Color = Color::new(102, 191, 255, 255);
    pub const BLUE: Color = Color::new(0, 121, 241, 255);
    pub const DARKBLUE: Color = Color::new(0, 82, 172, 255);
    pub const PURPLE: Color = Color::new(200, 122, 255, 255);
    pub const VIOLET: Color = Color::new(135, 60, 190, 255);
    pub const DARKPURPLE: Color = Color::new(112, 31, 126, 255);
    pub const BEIGE: Color = Color::new(211, 176, 131, 255);
    pub const BROWN: Color = Color::new(127, 106, 79, 255);
    pub const DARKBROWN: Color = Color::new(76, 63, 47, 255);
    pub const WHITE: Color = Color::new(255, 255, 255, 255);
    pub const BLACK: Color = Color::new(0, 0, 0, 255);
    pub const BLANK: Color = Color::new(0, 0, 0, 0);
    pub const MAGENTA: Color = Color::new(255, 0, 255, 255);
    pub const RAYWHITE: Color = Color::new(245, 245, 245, 255);
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Image {
    pub data: *mut c_void,
    pub width: i32,
    pub height: i32,
    pub mipmaps: i32,
    pub format: i32,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            data: std::ptr::null_mut(),
            width: 0,
            height: 0,
            mipmaps: 1,
            format: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Texture {
    pub id: u32,
    pub width: i32,
    pub height: i32,
    pub mipmaps: i32,
    pub format: i32,
}
pub type Texture2D = Texture;
pub type TextureCubemap = Texture;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RenderTexture {
    pub id: u32,
    pub texture: Texture,
    pub depth: Texture,
}
pub type RenderTexture2D = RenderTexture;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum CameraProjection {
    Perspective = 0,
    Orthographic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum CameraMode {
    Custom = 0,
    Free,
    Orbital,
    FirstPerson,
    ThirdPerson,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Camera3D {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fovy: f32,
    pub projection: i32,
}
pub type Camera = Camera3D;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Camera2D {
    pub offset: Vector2,
    pub target: Vector2,
    pub rotation: f32,
    pub zoom: f32,
}

// Some Basic Colors
pub const LIGHTGRAY: Color = Color {
    r: 200,
    g: 200,
    b: 200,
    a: 255,
};
pub const GRAY: Color = Color {
    r: 130,
    g: 130,
    b: 130,
    a: 255,
};
pub const DARKGRAY: Color = Color {
    r: 80,
    g: 80,
    b: 80,
    a: 255,
};
pub const YELLOW: Color = Color {
    r: 253,
    g: 249,
    b: 0,
    a: 255,
};
pub const GOLD: Color = Color {
    r: 255,
    g: 203,
    b: 0,
    a: 255,
};
pub const ORANGE: Color = Color {
    r: 255,
    g: 161,
    b: 0,
    a: 255,
};
pub const PINK: Color = Color {
    r: 255,
    g: 109,
    b: 194,
    a: 255,
};
pub const RED: Color = Color {
    r: 230,
    g: 41,
    b: 55,
    a: 255,
};
pub const MAROON: Color = Color {
    r: 190,
    g: 33,
    b: 55,
    a: 255,
};
pub const GREEN: Color = Color {
    r: 0,
    g: 228,
    b: 48,
    a: 255,
};
pub const LIME: Color = Color {
    r: 0,
    g: 158,
    b: 47,
    a: 255,
};
pub const DARKGREEN: Color = Color {
    r: 0,
    g: 117,
    b: 44,
    a: 255,
};
pub const SKYBLUE: Color = Color {
    r: 102,
    g: 191,
    b: 255,
    a: 255,
};
pub const BLUE: Color = Color {
    r: 0,
    g: 121,
    b: 241,
    a: 255,
};
pub const DARKBLUE: Color = Color {
    r: 0,
    g: 82,
    b: 172,
    a: 255,
};
pub const PURPLE: Color = Color {
    r: 200,
    g: 122,
    b: 255,
    a: 255,
};
pub const VIOLET: Color = Color {
    r: 135,
    g: 60,
    b: 190,
    a: 255,
};
pub const DARKPURPLE: Color = Color {
    r: 112,
    g: 31,
    b: 126,
    a: 255,
};
pub const BEIGE: Color = Color {
    r: 211,
    g: 176,
    b: 131,
    a: 255,
};
pub const BROWN: Color = Color {
    r: 127,
    g: 106,
    b: 79,
    a: 255,
};
pub const DARKBROWN: Color = Color {
    r: 76,
    g: 63,
    b: 47,
    a: 255,
};
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};
pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};
pub const BLANK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};
pub const MAGENTA: Color = Color {
    r: 255,
    g: 0,
    b: 255,
    a: 255,
};
pub const RAYWHITE: Color = Color {
    r: 245,
    g: 245,
    b: 245,
    a: 255,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NPatchLayout {
    NinePatch = 0,        // Npatch layout: 3x3 tiles
    ThreePatchVertical,   // Npatch layout: 1x3 tiles
    ThreePatchHorizontal, // Npatch layout: 3x1 tiles
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NPatchInfo {
    pub source: Rectangle, // Texture source rectangle
    pub left: i32,         // Left border offset
    pub top: i32,          // Top border offset
    pub right: i32,        // Right border offset
    pub bottom: i32,       // Bottom border offset
    pub layout: i32,       // Layout of the n-patch: 3x3, 1x3 or 3x1
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct GlyphInfo {
    pub value: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub advance_x: i32,
    pub image: Image,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Font {
    pub baseSize: i32,
    pub glyphCount: i32,
    pub glyphPadding: i32,
    pub texture: Texture,
    pub recs: Vec<Rectangle>,
    pub glyphs: Vec<GlyphInfo>,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ConfigFlags {
    FLAG_VSYNC_HINT = 64,
    FLAG_FULLSCREEN_MODE = 2,
    FLAG_WINDOW_RESIZABLE = 4,
    FLAG_WINDOW_UNDECORATED = 8,
    FLAG_WINDOW_HIDDEN = 128,
    FLAG_WINDOW_MINIMIZED = 512,
    FLAG_WINDOW_MAXIMIZED = 1024,
    FLAG_WINDOW_UNFOCUSED = 2048,
    FLAG_WINDOW_TOPMOST = 4096,
    FLAG_WINDOW_ALWAYS_RUN = 256,
    FLAG_WINDOW_TRANSPARENT = 16,
    FLAG_WINDOW_HIGHDPI = 8192,
    FLAG_WINDOW_MOUSE_PASSTHROUGH = 16384,
    FLAG_BORDERLESS_WINDOWED_MODE = 32768,
    FLAG_MSAA_4X_HINT = 32,
    FLAG_INTERLACED_HINT = 65536,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TraceLogLevel {
    LOG_ALL = 0,
    LOG_TRACE = 1,
    LOG_DEBUG = 2,
    LOG_INFO = 3,
    LOG_WARNING = 4,
    LOG_ERROR = 5,
    LOG_FATAL = 6,
    LOG_NONE = 7,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KeyboardKey {
    KEY_NULL = 0,
    KEY_APOSTROPHE = 39,
    KEY_COMMA = 44,
    KEY_MINUS = 45,
    KEY_PERIOD = 46,
    KEY_SLASH = 47,
    KEY_ZERO = 48,
    KEY_ONE = 49,
    KEY_TWO = 50,
    KEY_THREE = 51,
    KEY_FOUR = 52,
    KEY_FIVE = 53,
    KEY_SIX = 54,
    KEY_SEVEN = 55,
    KEY_EIGHT = 56,
    KEY_NINE = 57,
    KEY_SEMICOLON = 59,
    KEY_EQUAL = 61,
    KEY_A = 65,
    KEY_B = 66,
    KEY_C = 67,
    KEY_D = 68,
    KEY_E = 69,
    KEY_F = 70,
    KEY_G = 71,
    KEY_H = 72,
    KEY_I = 73,
    KEY_J = 74,
    KEY_K = 75,
    KEY_L = 76,
    KEY_M = 77,
    KEY_N = 78,
    KEY_O = 79,
    KEY_P = 80,
    KEY_Q = 81,
    KEY_R = 82,
    KEY_S = 83,
    KEY_T = 84,
    KEY_U = 85,
    KEY_V = 86,
    KEY_W = 87,
    KEY_X = 88,
    KEY_Y = 89,
    KEY_Z = 90,
    KEY_LEFT_BRACKET = 91,
    KEY_BACKSLASH = 92,
    KEY_RIGHT_BRACKET = 93,
    KEY_GRAVE = 96,
    KEY_SPACE = 32,
    KEY_ESCAPE = 256,
    KEY_ENTER = 257,
    KEY_TAB = 258,
    KEY_BACKSPACE = 259,
    KEY_INSERT = 260,
    KEY_DELETE = 261,
    KEY_RIGHT = 262,
    KEY_LEFT = 263,
    KEY_DOWN = 264,
    KEY_UP = 265,
    KEY_PAGE_UP = 266,
    KEY_PAGE_DOWN = 267,
    KEY_HOME = 268,
    KEY_END = 269,
    KEY_CAPS_LOCK = 280,
    KEY_SCROLL_LOCK = 281,
    KEY_NUM_LOCK = 282,
    KEY_PRINT_SCREEN = 283,
    KEY_PAUSE = 284,
    KEY_F1 = 290,
    KEY_F2 = 291,
    KEY_F3 = 292,
    KEY_F4 = 293,
    KEY_F5 = 294,
    KEY_F6 = 295,
    KEY_F7 = 296,
    KEY_F8 = 297,
    KEY_F9 = 298,
    KEY_F10 = 299,
    KEY_F11 = 300,
    KEY_F12 = 301,
    KEY_LEFT_SHIFT = 340,
    KEY_LEFT_CONTROL = 341,
    KEY_LEFT_ALT = 342,
    KEY_LEFT_SUPER = 343,
    KEY_RIGHT_SHIFT = 344,
    KEY_RIGHT_CONTROL = 345,
    KEY_RIGHT_ALT = 346,
    KEY_RIGHT_SUPER = 347,
    KEY_KB_MENU = 348,
    KEY_KP_0 = 320,
    KEY_KP_1 = 321,
    KEY_KP_2 = 322,
    KEY_KP_3 = 323,
    KEY_KP_4 = 324,
    KEY_KP_5 = 325,
    KEY_KP_6 = 326,
    KEY_KP_7 = 327,
    KEY_KP_8 = 328,
    KEY_KP_9 = 329,
    KEY_KP_DECIMAL = 330,
    KEY_KP_DIVIDE = 331,
    KEY_KP_MULTIPLY = 332,
    KEY_KP_SUBTRACT = 333,
    KEY_KP_ADD = 334,
    KEY_KP_ENTER = 335,
    KEY_KP_EQUAL = 336,
    KEY_BACK = 4,
    KEY_MENU = 5,
    KEY_VOLUME_UP = 24,
    KEY_VOLUME_DOWN = 25,
}
impl From<KeyboardKey> for i32 {
    fn from(val: KeyboardKey) -> Self {
        val as i32
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MouseButton {
    MOUSE_BUTTON_LEFT = 0,
    MOUSE_BUTTON_RIGHT = 1,
    MOUSE_BUTTON_MIDDLE = 2,
    MOUSE_BUTTON_SIDE = 3,
    MOUSE_BUTTON_EXTRA = 4,
    MOUSE_BUTTON_FORWARD = 5,
    MOUSE_BUTTON_BACK = 6,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MouseCursor {
    MOUSE_CURSOR_DEFAULT = 0,
    MOUSE_CURSOR_ARROW = 1,
    MOUSE_CURSOR_IBEAM = 2,
    MOUSE_CURSOR_CROSSHAIR = 3,
    MOUSE_CURSOR_POINTING_HAND = 4,
    MOUSE_CURSOR_RESIZE_EW = 5,
    MOUSE_CURSOR_RESIZE_NS = 6,
    MOUSE_CURSOR_RESIZE_NWSE = 7,
    MOUSE_CURSOR_RESIZE_NESW = 8,
    MOUSE_CURSOR_RESIZE_ALL = 9,
    MOUSE_CURSOR_NOT_ALLOWED = 10,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GamepadButton {
    GAMEPAD_BUTTON_UNKNOWN = 0,      // Unknown button, for error checking
    GAMEPAD_BUTTON_LEFT_FACE_UP,     // Gamepad left DPAD up button
    GAMEPAD_BUTTON_LEFT_FACE_RIGHT,  // Gamepad left DPAD right button
    GAMEPAD_BUTTON_LEFT_FACE_DOWN,   // Gamepad left DPAD down button
    GAMEPAD_BUTTON_LEFT_FACE_LEFT,   // Gamepad left DPAD left button
    GAMEPAD_BUTTON_RIGHT_FACE_UP,    // Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
    GAMEPAD_BUTTON_RIGHT_FACE_RIGHT, // Gamepad right button right (i.e. PS3: Circle, Xbox: B)
    GAMEPAD_BUTTON_RIGHT_FACE_DOWN,  // Gamepad right button down (i.e. PS3: Cross, Xbox: A)
    GAMEPAD_BUTTON_RIGHT_FACE_LEFT,  // Gamepad right button left (i.e. PS3: Square, Xbox: X)
    GAMEPAD_BUTTON_LEFT_TRIGGER_1, // Gamepad top/back trigger left (first), it could be a trailing button
    GAMEPAD_BUTTON_LEFT_TRIGGER_2, // Gamepad top/back trigger left (second), it could be a trailing button
    GAMEPAD_BUTTON_RIGHT_TRIGGER_1, // Gamepad top/back trigger right (first), it could be a trailing button
    GAMEPAD_BUTTON_RIGHT_TRIGGER_2, // Gamepad top/back trigger right (second), it could be a trailing button
    GAMEPAD_BUTTON_MIDDLE_LEFT,     // Gamepad center buttons, left one (i.e. PS3: Select)
    GAMEPAD_BUTTON_MIDDLE,          // Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
    GAMEPAD_BUTTON_MIDDLE_RIGHT,    // Gamepad center buttons, right one (i.e. PS3: Start)
    GAMEPAD_BUTTON_LEFT_THUMB,      // Gamepad joystick pressed button left
    GAMEPAD_BUTTON_RIGHT_THUMB,     // Gamepad joystick pressed button right
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GamepadAxis {
    GAMEPAD_AXIS_LEFT_X = 0,
    GAMEPAD_AXIS_LEFT_Y = 1,
    GAMEPAD_AXIS_RIGHT_X = 2,
    GAMEPAD_AXIS_RIGHT_Y = 3,
    GAMEPAD_AXIS_LEFT_TRIGGER = 4,
    GAMEPAD_AXIS_RIGHT_TRIGGER = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MaterialMapIndex {
    MATERIAL_MAP_ALBEDO = 0,
    MATERIAL_MAP_METALNESS = 1,
    MATERIAL_MAP_NORMAL = 2,
    MATERIAL_MAP_ROUGHNESS = 3,
    MATERIAL_MAP_OCCLUSION = 4,
    MATERIAL_MAP_EMISSION = 5,
    MATERIAL_MAP_HEIGHT = 6,
    MATERIAL_MAP_CUBEMAP = 7,
    MATERIAL_MAP_IRRADIANCE = 8,
    MATERIAL_MAP_PREFILTER = 9,
    MATERIAL_MAP_BRDF = 10,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ShaderLocationIndex {
    SHADER_LOC_VERTEX_POSITION = 0,
    SHADER_LOC_VERTEX_TEXCOORD01 = 1,
    SHADER_LOC_VERTEX_TEXCOORD02 = 2,
    SHADER_LOC_VERTEX_NORMAL = 3,
    SHADER_LOC_VERTEX_TANGENT = 4,
    SHADER_LOC_VERTEX_COLOR = 5,
    SHADER_LOC_MATRIX_MVP = 6,
    SHADER_LOC_MATRIX_VIEW = 7,
    SHADER_LOC_MATRIX_PROJECTION = 8,
    SHADER_LOC_MATRIX_MODEL = 9,
    SHADER_LOC_MATRIX_NORMAL = 10,
    SHADER_LOC_VECTOR_VIEW = 11,
    SHADER_LOC_COLOR_DIFFUSE = 12,
    SHADER_LOC_COLOR_SPECULAR = 13,
    SHADER_LOC_COLOR_AMBIENT = 14,
    SHADER_LOC_MAP_ALBEDO = 15,
    SHADER_LOC_MAP_METALNESS = 16,
    SHADER_LOC_MAP_NORMAL = 17,
    SHADER_LOC_MAP_ROUGHNESS = 18,
    SHADER_LOC_MAP_OCCLUSION = 19,
    SHADER_LOC_MAP_EMISSION = 20,
    SHADER_LOC_MAP_HEIGHT = 21,
    SHADER_LOC_MAP_CUBEMAP = 22,
    SHADER_LOC_MAP_IRRADIANCE = 23,
    SHADER_LOC_MAP_PREFILTER = 24,
    SHADER_LOC_MAP_BRDF = 25,
    SHADER_LOC_VERTEX_BONEIDS = 26,
    SHADER_LOC_VERTEX_BONEWEIGHTS = 27,
    SHADER_LOC_MATRIX_BONETRANSFORMS = 28,
    SHADER_LOC_VERTEX_INSTANCETRANSFORM = 29,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ShaderUniformDataType {
    SHADER_UNIFORM_FLOAT = 0,
    SHADER_UNIFORM_VEC2 = 1,
    SHADER_UNIFORM_VEC3 = 2,
    SHADER_UNIFORM_VEC4 = 3,
    SHADER_UNIFORM_INT = 4,
    SHADER_UNIFORM_IVEC2 = 5,
    SHADER_UNIFORM_IVEC3 = 6,
    SHADER_UNIFORM_IVEC4 = 7,
    SHADER_UNIFORM_UINT = 8,
    SHADER_UNIFORM_UIVEC2 = 9,
    SHADER_UNIFORM_UIVEC3 = 10,
    SHADER_UNIFORM_UIVEC4 = 11,
    SHADER_UNIFORM_SAMPLER2D = 12,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ShaderAttributeDataType {
    SHADER_ATTRIB_FLOAT = 0,
    SHADER_ATTRIB_VEC2 = 1,
    SHADER_ATTRIB_VEC3 = 2,
    SHADER_ATTRIB_VEC4 = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PixelFormat {
    PIXELFORMAT_UNCOMPRESSED_GRAYSCALE = 1,
    PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA = 2,
    PIXELFORMAT_UNCOMPRESSED_R5G6B5 = 3,
    PIXELFORMAT_UNCOMPRESSED_R8G8B8 = 4,
    PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 = 5,
    PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 = 6,
    PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 = 7,
    PIXELFORMAT_UNCOMPRESSED_R32 = 8,
    PIXELFORMAT_UNCOMPRESSED_R32G32B32 = 9,
    PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 = 10,
    PIXELFORMAT_UNCOMPRESSED_R16 = 11,
    PIXELFORMAT_UNCOMPRESSED_R16G16B16 = 12,
    PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 = 13,
    PIXELFORMAT_COMPRESSED_DXT1_RGB = 14,
    PIXELFORMAT_COMPRESSED_DXT1_RGBA = 15,
    PIXELFORMAT_COMPRESSED_DXT3_RGBA = 16,
    PIXELFORMAT_COMPRESSED_DXT5_RGBA = 17,
    PIXELFORMAT_COMPRESSED_ETC1_RGB = 18,
    PIXELFORMAT_COMPRESSED_ETC2_RGB = 19,
    PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA = 20,
    PIXELFORMAT_COMPRESSED_PVRT_RGB = 21,
    PIXELFORMAT_COMPRESSED_PVRT_RGBA = 22,
    PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA = 23,
    PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA = 24,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextureFilter {
    TEXTURE_FILTER_POINT = 0,
    TEXTURE_FILTER_BILINEAR = 1,
    TEXTURE_FILTER_TRILINEAR = 2,
    TEXTURE_FILTER_ANISOTROPIC_4X = 3,
    TEXTURE_FILTER_ANISOTROPIC_8X = 4,
    TEXTURE_FILTER_ANISOTROPIC_16X = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextureWrap {
    TEXTURE_WRAP_REPEAT = 0,
    TEXTURE_WRAP_CLAMP = 1,
    TEXTURE_WRAP_MIRROR_REPEAT = 2,
    TEXTURE_WRAP_MIRROR_CLAMP = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CubemapLayout {
    CUBEMAP_LAYOUT_AUTO_DETECT = 0,
    CUBEMAP_LAYOUT_LINE_VERTICAL = 1,
    CUBEMAP_LAYOUT_LINE_HORIZONTAL = 2,
    CUBEMAP_LAYOUT_CROSS_THREE_BY_FOUR = 3,
    CUBEMAP_LAYOUT_CROSS_FOUR_BY_THREE = 4,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FontType {
    FONT_DEFAULT = 0,
    FONT_BITMAP = 1,
    FONT_SDF = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BlendMode {
    BLEND_ALPHA = 0,
    BLEND_ADDITIVE = 1,
    BLEND_MULTIPLIED = 2,
    BLEND_ADD_COLORS = 3,
    BLEND_SUBTRACT_COLORS = 4,
    BLEND_ALPHA_PREMULTIPLY = 5,
    BLEND_CUSTOM = 6,
    BLEND_CUSTOM_SEPARATE = 7,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Gesture {
    GESTURE_NONE = 0,
    GESTURE_TAP = 1,
    GESTURE_DOUBLETAP = 2,
    GESTURE_HOLD = 4,
    GESTURE_DRAG = 8,
    GESTURE_SWIPE_RIGHT = 16,
    GESTURE_SWIPE_LEFT = 32,
    GESTURE_SWIPE_UP = 64,
    GESTURE_SWIPE_DOWN = 128,
    GESTURE_PINCH_IN = 256,
    GESTURE_PINCH_OUT = 512,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Shader {
    pub id: u32,
    pub locs: Vec<i32>,
}

// use crate::Matrix; // or import the matching C-compatible Matrix type

// VrDeviceInfo, Head-Mounted-Display device parameters
#[repr(C)]
#[allow(non_snake_case)]
pub struct VrDeviceInfo {
    pub hResolution: i32,               // Horizontal resolution in pixels
    pub vResolution: i32,               // Vertical resolution in pixels
    pub hScreenSize: f32,               // Horizontal size in meters
    pub vScreenSize: f32,               // Vertical size in meters
    pub eyeToScreenDistance: f32,       // Distance between eye and display in meters
    pub lensSeparationDistance: f32,    // Lens separation distance in meters
    pub interpupillaryDistance: f32,    // IPD (distance between pupils) in meters
    pub lensDistortionValues: [f32; 4], // Lens distortion constant parameters
    pub chromaAbCorrection: [f32; 4],   // Chromatic aberration correction parameters
}

// VrStereoConfig, VR stereo rendering configuration for simulator
#[repr(C)]
#[allow(non_snake_case)]
pub struct VrStereoConfig {
    pub projection: [Matrix; 2],     // VR projection matrices (per eye)
    pub viewOffset: [Matrix; 2],     // VR view offset matrices (per eye)
    pub leftLensCenter: [f32; 2],    // VR left lens center
    pub rightLensCenter: [f32; 2],   // VR right lens center
    pub leftScreenCenter: [f32; 2],  // VR left screen center
    pub rightScreenCenter: [f32; 2], // VR right screen center
    pub scale: [f32; 2],             // VR distortion scale
    pub scaleIn: [f32; 2],           // VR distortion scale in
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct FilePathList {
    pub count: u32,                        // Filepaths entries count
    pub paths: *mut *mut std::ffi::c_char, // Filepaths entries
}

// Automation event
#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone, Default)]
pub struct AutomationEvent {
    pub frame: u32,       // Event frame
    pub type_: u32,       // Event type (AutomationEventType)
    pub params: [i32; 4], // Event parameters (if required)
}

// Automation event list
#[repr(C)]
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct AutomationEventList {
    pub capacity: u32,                // Events max entries (MAX_AUTOMATION_EVENTS)
    pub count: u32,                   // Events entries count
    pub events: Vec<AutomationEvent>, // Events entries
}
impl AutomationEventList {
    pub fn with_capacity(cap: u32) -> Self {
        Self {
            capacity: cap,
            count: 0,
            events: vec![AutomationEvent::default(); cap as usize],
        }
    }
}

#[repr(C)]
pub struct Ray {
    pub position: Vector3,  // Ray position (origin)
    pub direction: Vector3, // Ray direction (normalized)
}
