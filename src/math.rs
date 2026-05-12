use crate::types::{Matrix, Quaternion, Vector2, Vector3, Vector4};

pub const PI: f32 = std::f32::consts::PI;
pub const DEG2RAD: f32 = PI / 180.0;
pub const RAD2DEG: f32 = 180.0 / PI;

/// Clamp float value
#[inline]
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Calculate linear interpolation between two floats
#[inline]
pub fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    start + amount * (end - start)
}

/// Normalize input value within input range
#[inline]
pub fn normalize(value: f32, start: f32, end: f32) -> f32 {
    (value - start) / (end - start)
}

/// Remap input value within input range to output range
#[inline]
pub fn remap(
    value: f32,
    input_start: f32,
    input_end: f32,
    output_start: f32,
    output_end: f32,
) -> f32 {
    (value - input_start) / (input_end - input_start) * (output_end - output_start) + output_start
}

/// Wrap input value from min to max
#[inline]
pub fn wrap(value: f32, min: f32, max: f32) -> f32 {
    let result = value - (max - min) * ((value - min) / (max - min)).floor();
    if result == max {
        return min;
    }
    result
}

/// Check whether two given floats are almost equal
#[inline]
pub fn float_equals(x: f32, y: f32) -> bool {
    (x - y).abs() <= std::f32::EPSILON * x.abs().max(y.abs()).max(1.0)
}

// NOTE: glam handles Vector2, Vector3, Matrix, and Quaternion natively.
// Matrix math operations like MatrixLookAt, MatrixOrtho, etc. can be ported here
// if glam's Mat4::look_at_rh or Mat4::orthographic_rh doesn't match raymath exactly.
// Raylib uses right-handed, column-major matrices. Glam also defaults to this.

//#[inline]
//pub fn matrix_frustum(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> Matrix {
//    let mut result = Matrix::ZERO;
//
//    let rl = (right - left) as f32;
//    let tb = (top - bottom) as f32;
//    let fn_ = (far - near) as f32;
//
//    result.x_axis.x = ((near * 2.0) as f32) / rl;
//    result.y_axis.y = ((near * 2.0) as f32) / tb;
//    result.z_axis.x = ((right + left) as f32) / rl;
//    result.z_axis.y = ((top + bottom) as f32) / tb;
//    result.z_axis.z = -((far + near) as f32) / fn_;
//    result.z_axis.w = -1.0;
//    result.w_axis.z = -((far * near * 2.0) as f32) / fn_;
//
//    result
//}
