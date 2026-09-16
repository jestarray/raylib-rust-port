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

pub fn Vector3Transform(v: Vector3, mat: Matrix) -> Vector3 {
    let mut result = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let x = v.x;
    let y = v.y;
    let z = v.z;

    result.x = mat.m0 * x + mat.m4 * y + mat.m8 * z + mat.m12;
    result.y = mat.m1 * x + mat.m5 * y + mat.m9 * z + mat.m13;
    result.z = mat.m2 * x + mat.m6 * y + mat.m10 * z + mat.m14;

    result
}

// Projects a Vector3 from screen space into object space
// NOTE: Self-contained function, no other raymath functions are called
#[rustfmt::skip]
pub fn Vector3Unproject(source: Vector3, projection: Matrix, view: Matrix) -> Vector3 {
    let mut result = Vector3::default();

    // Calculate unprojected matrix (multiply view matrix by projection matrix) and invert it
    let matViewProj = Matrix::new(      // Matrix::multiply(view, projection);
        view.m0*projection.m0 + view.m1*projection.m4 + view.m2*projection.m8 + view.m3*projection.m12,
        view.m0*projection.m1 + view.m1*projection.m5 + view.m2*projection.m9 + view.m3*projection.m13,
        view.m0*projection.m2 + view.m1*projection.m6 + view.m2*projection.m10 + view.m3*projection.m14,
        view.m0*projection.m3 + view.m1*projection.m7 + view.m2*projection.m11 + view.m3*projection.m15,
        view.m4*projection.m0 + view.m5*projection.m4 + view.m6*projection.m8 + view.m7*projection.m12,
        view.m4*projection.m1 + view.m5*projection.m5 + view.m6*projection.m9 + view.m7*projection.m13,
        view.m4*projection.m2 + view.m5*projection.m6 + view.m6*projection.m10 + view.m7*projection.m14,
        view.m4*projection.m3 + view.m5*projection.m7 + view.m6*projection.m11 + view.m7*projection.m15,
        view.m8*projection.m0 + view.m9*projection.m4 + view.m10*projection.m8 + view.m11*projection.m12,
        view.m8*projection.m1 + view.m9*projection.m5 + view.m10*projection.m9 + view.m11*projection.m13,
        view.m8*projection.m2 + view.m9*projection.m6 + view.m10*projection.m10 + view.m11*projection.m14,
        view.m8*projection.m3 + view.m9*projection.m7 + view.m10*projection.m11 + view.m11*projection.m15,
        view.m12*projection.m0 + view.m13*projection.m4 + view.m14*projection.m8 + view.m15*projection.m12,
        view.m12*projection.m1 + view.m13*projection.m5 + view.m14*projection.m9 + view.m15*projection.m13,
        view.m12*projection.m2 + view.m13*projection.m6 + view.m14*projection.m10 + view.m15*projection.m14,
        view.m12*projection.m3 + view.m13*projection.m7 + view.m14*projection.m11 + view.m15*projection.m15,
    );

    // Calculate inverted matrix -> Matrix::invert(matViewProj);
    // Cache the matrix values (speed optimization)
    let a00 = matViewProj.m0;
    let a01 = matViewProj.m1;
    let a02 = matViewProj.m2;
    let a03 = matViewProj.m3;
    let a10 = matViewProj.m4;
    let a11 = matViewProj.m5;
    let a12 = matViewProj.m6;
    let a13 = matViewProj.m7;
    let a20 = matViewProj.m8;
    let a21 = matViewProj.m9;
    let a22 = matViewProj.m10;
    let a23 = matViewProj.m11;
    let a30 = matViewProj.m12;
    let a31 = matViewProj.m13;
    let a32 = matViewProj.m14;
    let a33 = matViewProj.m15;

    let b00 = a00*a11 - a01*a10;
    let b01 = a00*a12 - a02*a10;
    let b02 = a00*a13 - a03*a10;
    let b03 = a01*a12 - a02*a11;
    let b04 = a01*a13 - a03*a11;
    let b05 = a02*a13 - a03*a12;
    let b06 = a20*a31 - a21*a30;
    let b07 = a20*a32 - a22*a30;
    let b08 = a20*a33 - a23*a30;
    let b09 = a21*a32 - a22*a31;
    let b10 = a21*a33 - a23*a31;
    let b11 = a22*a33 - a23*a32;

    // Calculate the invert determinant (inlined to avoid double-caching)
    let invDet = 1.0/(b00*b11 - b01*b10 + b02*b09 + b03*b08 - b04*b07 + b05*b06);

    let matViewProjInv = Matrix::new(
        (a11*b11 - a12*b10 + a13*b09)*invDet,
        (-a01*b11 + a02*b10 - a03*b09)*invDet,
        (a31*b05 - a32*b04 + a33*b03)*invDet,
        (-a21*b05 + a22*b04 - a23*b03)*invDet,
        (-a10*b11 + a12*b08 - a13*b07)*invDet,
        (a00*b11 - a02*b08 + a03*b07)*invDet,
        (-a30*b05 + a32*b02 - a33*b01)*invDet,
        (a20*b05 - a22*b02 + a23*b01)*invDet,
        (a10*b10 - a11*b08 + a13*b06)*invDet,
        (-a00*b10 + a01*b08 - a03*b06)*invDet,
        (a30*b04 - a31*b02 + a33*b00)*invDet,
        (-a20*b04 + a21*b02 - a23*b00)*invDet,
        (-a10*b09 + a11*b07 - a12*b06)*invDet,
        (a00*b09 - a01*b07 + a02*b06)*invDet,
        (-a30*b03 + a31*b01 - a32*b00)*invDet,
        (a20*b03 - a21*b01 + a22*b00)*invDet,
    );

    // Create quaternion from source point
    let quat = Quaternion::new(source.x, source.y, source.z, 1.0);

    // Multiply quat point by unprojected matrix
    let qtransformed = Quaternion::new(     // Quaternion::transform(quat, matViewProjInv)
        matViewProjInv.m0*quat.x + matViewProjInv.m4*quat.y + matViewProjInv.m8*quat.z + matViewProjInv.m12*quat.w,
        matViewProjInv.m1*quat.x + matViewProjInv.m5*quat.y + matViewProjInv.m9*quat.z + matViewProjInv.m13*quat.w,
        matViewProjInv.m2*quat.x + matViewProjInv.m6*quat.y + matViewProjInv.m10*quat.z + matViewProjInv.m14*quat.w,
        matViewProjInv.m3*quat.x + matViewProjInv.m7*quat.y + matViewProjInv.m11*quat.z + matViewProjInv.m15*quat.w,
    );

    // Normalized world points in vectors
    result.x = qtransformed.x/qtransformed.w;
    result.y = qtransformed.y/qtransformed.w;
    result.z = qtransformed.z/qtransformed.w;

    result
}

#[allow(non_snake_case)]
pub fn QuaternionTransform(q: Quaternion, mat: Matrix) -> Quaternion {
    let mut result = Quaternion::ZERO;

    result.x = mat.m0 * q.x + mat.m4 * q.y + mat.m8 * q.z + mat.m12 * q.w;
    result.y = mat.m1 * q.x + mat.m5 * q.y + mat.m9 * q.z + mat.m13 * q.w;
    result.z = mat.m2 * q.x + mat.m6 * q.y + mat.m10 * q.z + mat.m14 * q.w;
    result.w = mat.m3 * q.x + mat.m7 * q.y + mat.m11 * q.z + mat.m15 * q.w;

    result
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
