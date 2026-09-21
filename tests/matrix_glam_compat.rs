use glam::{Mat4, Vec3};
use raylib::{
    rlgl::{
        rlMatrixIdentity, rlMatrixInvert, rlMatrixMultiply, rlMatrixToFloatV, rlMatrixTranspose,
    },
    types::Matrix,
};

const EPSILON: f32 = 2.0e-5;

fn assert_float_eq(actual: f32, expected: f32) {
    let tolerance = EPSILON * actual.abs().max(expected.abs()).max(1.0);
    assert!(
        (actual - expected).abs() <= tolerance,
        "actual {actual:?}, expected {expected:?}, tolerance {tolerance:?}"
    );
}

fn assert_matrix_eq(actual: Mat4, expected: [f32; 16]) {
    for (index, (actual, expected)) in actual.to_cols_array().into_iter().zip(expected).enumerate()
    {
        let tolerance = EPSILON * actual.abs().max(expected.abs()).max(1.0);
        assert!(
            (actual - expected).abs() <= tolerance,
            "element {index}: actual {actual:?}, expected {expected:?}, tolerance {tolerance:?}"
        );
    }
}

fn legacy_transpose(m: [f32; 16]) -> [f32; 16] {
    let mut result = [0.0; 16];
    for column in 0..4 {
        for row in 0..4 {
            result[column * 4 + row] = m[row * 4 + column];
        }
    }
    result
}

// The old Matrix::multiply implementation computes right * left when its
// column-major data is interpreted using standard column-vector notation.
fn legacy_multiply(left: [f32; 16], right: [f32; 16]) -> [f32; 16] {
    let mut result = [0.0; 16];
    for column in 0..4 {
        for row in 0..4 {
            result[column * 4 + row] = (0..4)
                .map(|k| right[k * 4 + row] * left[column * 4 + k])
                .sum();
        }
    }
    result
}

fn legacy_determinant(m: [f32; 16]) -> f32 {
    let [
        m0,
        m1,
        m2,
        m3,
        m4,
        m5,
        m6,
        m7,
        m8,
        m9,
        m10,
        m11,
        m12,
        m13,
        m14,
        m15,
    ] = m;
    m0 * (m5 * (m10 * m15 - m11 * m14) - m9 * (m6 * m15 - m7 * m14) + m13 * (m6 * m11 - m7 * m10))
        - m4 * (m1 * (m10 * m15 - m11 * m14) - m9 * (m2 * m15 - m3 * m14)
            + m13 * (m2 * m11 - m3 * m10))
        + m8 * (m1 * (m6 * m15 - m7 * m14) - m5 * (m2 * m15 - m3 * m14) + m13 * (m2 * m7 - m3 * m6))
        - m12 * (m1 * (m6 * m11 - m7 * m10) - m5 * (m2 * m11 - m3 * m10) + m9 * (m2 * m7 - m3 * m6))
}

fn legacy_inverse(m: [f32; 16]) -> [f32; 16] {
    let [
        a00,
        a01,
        a02,
        a03,
        a10,
        a11,
        a12,
        a13,
        a20,
        a21,
        a22,
        a23,
        a30,
        a31,
        a32,
        a33,
    ] = m;
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

    [
        (a11 * b11 - a12 * b10 + a13 * b09) * inv_det,
        (-a01 * b11 + a02 * b10 - a03 * b09) * inv_det,
        (a31 * b05 - a32 * b04 + a33 * b03) * inv_det,
        (-a21 * b05 + a22 * b04 - a23 * b03) * inv_det,
        (-a10 * b11 + a12 * b08 - a13 * b07) * inv_det,
        (a00 * b11 - a02 * b08 + a03 * b07) * inv_det,
        (-a30 * b05 + a32 * b02 - a33 * b01) * inv_det,
        (a20 * b05 - a22 * b02 + a23 * b01) * inv_det,
        (a10 * b10 - a11 * b08 + a13 * b06) * inv_det,
        (-a00 * b10 + a01 * b08 - a03 * b06) * inv_det,
        (a30 * b04 - a31 * b02 + a33 * b00) * inv_det,
        (-a20 * b04 + a21 * b02 - a23 * b00) * inv_det,
        (-a10 * b09 + a11 * b07 - a12 * b06) * inv_det,
        (a00 * b09 - a01 * b07 + a02 * b06) * inv_det,
        (-a30 * b03 + a31 * b01 - a32 * b00) * inv_det,
        (a20 * b03 - a21 * b01 + a22 * b00) * inv_det,
    ]
}

fn legacy_rotation(axis: Vec3, angle: f32) -> [f32; 16] {
    let mut axis = axis;
    let length_squared = axis.length_squared();
    if length_squared != 1.0 && length_squared != 0.0 {
        axis *= length_squared.sqrt().recip();
    }
    let (sin, cos) = angle.sin_cos();
    let t = 1.0 - cos;
    let (x, y, z) = (axis.x, axis.y, axis.z);
    [
        x * x * t + cos,
        y * x * t + z * sin,
        z * x * t - y * sin,
        0.0,
        x * y * t - z * sin,
        y * y * t + cos,
        z * y * t + x * sin,
        0.0,
        x * z * t + y * sin,
        y * z * t - x * sin,
        z * z * t + cos,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ]
}

fn legacy_rotation_xyz(angle: Vec3) -> [f32; 16] {
    let (sinz, cosz) = (-angle.z).sin_cos();
    let (siny, cosy) = (-angle.y).sin_cos();
    let (sinx, cosx) = (-angle.x).sin_cos();
    [
        cosz * cosy,
        cosz * siny * sinx - sinz * cosx,
        cosz * siny * cosx + sinz * sinx,
        0.0,
        sinz * cosy,
        sinz * siny * sinx + cosz * cosx,
        sinz * siny * cosx - cosz * sinx,
        0.0,
        -siny,
        cosy * sinx,
        cosy * cosx,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ]
}

fn legacy_rotation_zyx(angle: Vec3) -> [f32; 16] {
    let (sz, cz) = angle.z.sin_cos();
    let (sy, cy) = angle.y.sin_cos();
    let (sx, cx) = angle.x.sin_cos();
    [
        cz * cy,
        cy * sz,
        -sy,
        0.0,
        cz * sy * sx - cx * sz,
        cz * cx + sz * sy * sx,
        cy * sx,
        0.0,
        sz * sx + cz * cx * sy,
        cx * sz * sy - cz * sx,
        cy * cx,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ]
}

fn legacy_frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> [f32; 16] {
    let rl = right - left;
    let tb = top - bottom;
    let fn_ = far - near;
    [
        near * 2.0 / rl,
        0.0,
        0.0,
        0.0,
        0.0,
        near * 2.0 / tb,
        0.0,
        0.0,
        (right + left) / rl,
        (top + bottom) / tb,
        -(far + near) / fn_,
        -1.0,
        0.0,
        0.0,
        -(far * near * 2.0) / fn_,
        0.0,
    ]
}

fn legacy_ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> [f32; 16] {
    let rl = right - left;
    let tb = top - bottom;
    let fn_ = far - near;
    [
        2.0 / rl,
        0.0,
        0.0,
        0.0,
        0.0,
        2.0 / tb,
        0.0,
        0.0,
        0.0,
        0.0,
        -2.0 / fn_,
        0.0,
        -(left + right) / rl,
        -(top + bottom) / tb,
        -(far + near) / fn_,
        1.0,
    ]
}

fn legacy_look_at(eye: Vec3, target: Vec3, up: Vec3) -> [f32; 16] {
    let mut vz = eye - target;
    let mut length = vz.length();
    if length == 0.0 {
        length = 1.0;
    }
    vz *= length.recip();

    let mut vx = up.cross(vz);
    length = vx.length();
    if length == 0.0 {
        length = 1.0;
    }
    vx *= length.recip();
    let vy = vz.cross(vx);

    [
        vx.x,
        vy.x,
        vz.x,
        0.0,
        vx.y,
        vy.y,
        vz.y,
        0.0,
        vx.z,
        vy.z,
        vz.z,
        0.0,
        -vx.dot(eye),
        -vy.dot(eye),
        -vz.dot(eye),
        1.0,
    ]
}

#[test]
fn alias_layout_and_rlgl_helpers_match_the_old_matrix() {
    let _: Matrix = Mat4::IDENTITY;
    assert_eq!(
        std::mem::size_of::<Matrix>(),
        16 * std::mem::size_of::<f32>()
    );
    assert_eq!(rlMatrixIdentity(), Mat4::IDENTITY);
    assert_eq!(Matrix::ZERO.to_cols_array(), [0.0; 16]);
    assert_eq!(
        raylib::rcore::WindowData::default().screenScale,
        Matrix::ZERO
    );

    let values = [
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ];
    let matrix = Mat4::from_cols_array(&values);
    assert_eq!(matrix.to_cols_array(), values);
    assert_eq!(rlMatrixToFloatV(matrix), values);
    assert_matrix_eq(rlMatrixTranspose(matrix), legacy_transpose(values));
}

#[test]
fn arithmetic_determinant_trace_and_inverse_match_the_old_methods() {
    let left = [
        1.2, -0.7, 2.1, 0.3, 0.5, 2.0, -1.0, 0.8, -0.4, 1.3, 0.9, -1.2, 3.0, -2.0, 4.5, 1.0,
    ];
    let right = [
        0.7, 1.1, -0.2, 0.4, -1.0, 0.3, 2.2, -0.5, 1.5, -0.8, 0.6, 1.7, 0.2, 2.4, -1.3, 0.9,
    ];
    let a = Mat4::from_cols_array(&left);
    let b = Mat4::from_cols_array(&right);

    assert_matrix_eq(a + b, std::array::from_fn(|i| left[i] + right[i]));
    assert_matrix_eq(a - b, std::array::from_fn(|i| left[i] - right[i]));
    assert_matrix_eq(a * 2.75, std::array::from_fn(|i| left[i] * 2.75));
    assert_matrix_eq(rlMatrixMultiply(a, b), legacy_multiply(left, right));
    assert_float_eq(a.determinant(), legacy_determinant(left));

    let old_trace = left[0] + left[5] + left[10] + left[15];
    let glam_trace = a.x_axis.x + a.y_axis.y + a.z_axis.z + a.w_axis.w;
    assert_float_eq(glam_trace, old_trace);

    let invertible = Mat4::from_scale_rotation_translation(
        Vec3::new(1.5, 0.75, 2.25),
        glam::Quat::from_rotation_y(0.7) * glam::Quat::from_rotation_x(-0.2),
        Vec3::new(3.0, -4.0, 2.0),
    );
    assert_matrix_eq(
        invertible.inverse(),
        legacy_inverse(invertible.to_cols_array()),
    );
    assert_matrix_eq(
        rlMatrixInvert(invertible),
        legacy_inverse(invertible.to_cols_array()),
    );
}

#[test]
fn affine_constructors_match_the_old_methods() {
    let translation = Vec3::new(3.5, -2.0, 8.25);
    assert_matrix_eq(
        Mat4::from_translation(translation),
        [
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            translation.x,
            translation.y,
            translation.z,
            1.0,
        ],
    );

    let scale = Vec3::new(1.5, -0.75, 2.25);
    assert_matrix_eq(
        Mat4::from_scale(scale),
        [
            scale.x, 0.0, 0.0, 0.0, 0.0, scale.y, 0.0, 0.0, 0.0, 0.0, scale.z, 0.0, 0.0, 0.0, 0.0,
            1.0,
        ],
    );

    let axis = Vec3::new(2.0, -3.0, 4.0);
    let angle = 0.73;
    assert_matrix_eq(
        Mat4::from_axis_angle(axis.normalize(), angle),
        legacy_rotation(axis, angle),
    );
    assert_matrix_eq(
        Mat4::from_rotation_x(angle),
        legacy_rotation(Vec3::X, angle),
    );
    assert_matrix_eq(
        Mat4::from_rotation_y(angle),
        legacy_rotation(Vec3::Y, angle),
    );
    assert_matrix_eq(
        Mat4::from_rotation_z(angle),
        legacy_rotation(Vec3::Z, angle),
    );

    let angles = Vec3::new(0.31, -0.52, 1.17);
    assert_matrix_eq(
        Mat4::from_rotation_x(angles.x)
            * Mat4::from_rotation_y(angles.y)
            * Mat4::from_rotation_z(angles.z),
        legacy_rotation_xyz(angles),
    );
    assert_matrix_eq(
        Mat4::from_rotation_z(angles.z)
            * Mat4::from_rotation_y(angles.y)
            * Mat4::from_rotation_x(angles.x),
        legacy_rotation_zyx(angles),
    );
}

#[test]
fn camera_and_projection_constructors_match_the_old_methods() {
    let (left, right, bottom, top, near, far) = (-1.3, 2.1, -0.9, 1.7, 0.1, 250.0);
    assert_matrix_eq(
        glam::camera::rh::proj::opengl::frustum(left, right, bottom, top, near, far),
        legacy_frustum(left, right, bottom, top, near, far),
    );

    let fov = 67.0_f32.to_radians();
    let aspect = 16.0 / 9.0;
    let perspective = glam::camera::rh::proj::opengl::perspective(fov, aspect, near, far);
    let frustum_top = near * (fov * 0.5).tan();
    assert_matrix_eq(
        perspective,
        legacy_frustum(
            -frustum_top * aspect,
            frustum_top * aspect,
            -frustum_top,
            frustum_top,
            near,
            far,
        ),
    );

    assert_matrix_eq(
        glam::camera::rh::proj::opengl::orthographic(left, right, bottom, top, near, far),
        legacy_ortho(left, right, bottom, top, near, far),
    );

    let eye = Vec3::new(4.0, 3.0, 8.0);
    let target = Vec3::new(-1.0, 0.5, 2.0);
    let up = Vec3::Y;
    assert_matrix_eq(
        glam::camera::rh::view::look_at_mat4(eye, target, up),
        legacy_look_at(eye, target, up),
    );
}
