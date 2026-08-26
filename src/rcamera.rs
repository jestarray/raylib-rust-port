use crate::math::*;
use crate::types::{Camera, CameraProjection, Matrix, Vector3};
use glam::{Mat4, Quat, Vec3};

// Returns the cameras forward vector (normalized)
pub fn get_camera_forward(camera: &Camera) -> Vector3 {
    (camera.target - camera.position).normalize()
}

// Returns the cameras up vector (normalized)
pub fn get_camera_up(camera: &Camera) -> Vector3 {
    camera.up.normalize()
}

// Returns the cameras right vector (normalized)
pub fn get_camera_right(camera: &Camera) -> Vector3 {
    let forward = get_camera_forward(camera);
    let up = get_camera_up(camera);

    forward.cross(up).normalize()
}

// Moves the camera in its forward direction
pub fn camera_move_forward(camera: &mut Camera, distance: f32, move_in_world_plane: bool) {
    let mut forward = get_camera_forward(camera);

    if move_in_world_plane {
        // Project vector onto world plane (the plane defined by the up vector)
        if camera.up.z.abs() > 0.7071 {
            forward.z = 0.0;
        } else if camera.up.x.abs() > 0.7071 {
            forward.x = 0.0;
        } else {
            forward.y = 0.0;
        }

        forward = forward.normalize();
    }

    // Scale by distance
    forward *= distance;

    // Move position and target
    camera.position += forward;
    camera.target += forward;
}

// Moves the camera in its up direction
pub fn camera_move_up(camera: &mut Camera, distance: f32) {
    let up = get_camera_up(camera);

    // Scale by distance
    let up_scaled = up * distance;

    // Move position and target
    camera.position += up_scaled;
    camera.target += up_scaled;
}

// Moves the camera target in its current right direction
pub fn camera_move_right(camera: &mut Camera, distance: f32, move_in_world_plane: bool) {
    let mut right = get_camera_right(camera);

    if move_in_world_plane {
        // Project vector onto world plane (the plane defined by the up vector)
        if camera.up.z.abs() > 0.7071 {
            right.z = 0.0;
        } else if camera.up.x.abs() > 0.7071 {
            right.x = 0.0;
        } else {
            right.y = 0.0;
        }

        right = right.normalize();
    }

    // Scale by distance
    right *= distance;

    // Move position and target
    camera.position += right;
    camera.target += right;
}

// Moves the camera position closer/farther to/from the camera target
pub fn camera_move_to_target(camera: &mut Camera, delta: f32) {
    let mut distance = camera.position.distance(camera.target);

    // Apply delta
    distance += delta;

    // Distance must be greater than 0
    if distance <= 0.0 {
        distance = 0.001;
    }

    // Set new distance by moving the position along the forward vector
    let forward = get_camera_forward(camera);
    camera.position = camera.target + (forward * -distance);
}

// Rotates the camera around its up vector
// Yaw is "looking left and right"
// If rotateAroundTarget is false, the camera rotates around its position
// Note: angle must be provided in radians
pub fn camera_yaw(camera: &mut Camera, angle: f32, rotate_around_target: bool) {
    // Rotation axis
    let up = get_camera_up(camera);

    // View vector
    let mut target_position = camera.target - camera.position;

    // Rotate view vector around up axis
    target_position = Quat::from_axis_angle(up, angle) * target_position;

    if rotate_around_target {
        // Move position relative to target
        camera.position = camera.target - target_position;
    } else {
        // Move target relative to position
        camera.target = camera.position + target_position;
    }
}

// Rotates the camera around its right vector, pitch is "looking up and down"
pub fn camera_pitch(
    camera: &mut Camera,
    angle: f32,
    lock_view: bool,
    rotate_around_target: bool,
    rotate_up: bool,
) {
    let up = get_camera_up(camera);
    let mut target_position = camera.target - camera.position;

    let mut actual_angle = angle;

    if lock_view {
        // Clamp view up
        let max_angle_up = up.angle_between(target_position);
        let max_angle_up_clamped = max_angle_up - 0.001;
        if actual_angle > max_angle_up_clamped {
            actual_angle = max_angle_up_clamped;
        }

        // Clamp view down
        let max_angle_down = (-up).angle_between(target_position);
        let max_angle_down_clamped = -max_angle_down + 0.001;
        if actual_angle < max_angle_down_clamped {
            actual_angle = max_angle_down_clamped;
        }
    }

    // Rotation axis
    let right = get_camera_right(camera);

    // Rotate view vector around right axis
    target_position = Quat::from_axis_angle(right, actual_angle) * target_position;

    if rotate_around_target {
        camera.position = camera.target - target_position;
    } else {
        camera.target = camera.position + target_position;
    }

    if rotate_up {
        camera.up = Quat::from_axis_angle(right, actual_angle) * camera.up;
    }
}

// Rotates the camera around its forward vector
pub fn camera_roll(camera: &mut Camera, angle: f32) {
    let forward = get_camera_forward(camera);
    camera.up = Quat::from_axis_angle(forward, angle) * camera.up;
}

// Returns the camera view matrix
pub fn get_camera_view_matrix(camera: &Camera) -> Matrix {
    MatrixLookAt(camera.position, camera.target, camera.up)
}

// Returns the camera projection matrix
pub fn get_camera_projection_matrix(camera: &Camera, aspect: f32) -> Matrix {
    const CAMERA_CULL_DISTANCE_NEAR: f32 = 0.01; // Should match RL_CULL_DISTANCE_NEAR
    const CAMERA_CULL_DISTANCE_FAR: f32 = 1000.0; // Should match RL_CULL_DISTANCE_FAR

    if camera.projection == CameraProjection::Perspective as i32 {
        MatrixPerspective(
            camera.fovy.to_radians() as f64,
            aspect as f64,
            CAMERA_CULL_DISTANCE_NEAR as f64,
            CAMERA_CULL_DISTANCE_FAR as f64,
        )
    } else {
        let top = camera.fovy as f32 / 2.0;
        let right = top * aspect;
        MatrixOrtho(
            -right as f64,
            right as f64,
            -top as f64,
            top as f64,
            CAMERA_CULL_DISTANCE_NEAR as f64,
            CAMERA_CULL_DISTANCE_FAR as f64,
        )
    }
}

pub fn MatrixOrtho(
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    nearPlane: f64,
    farPlane: f64,
) -> Matrix {
    let mut result = Matrix {
        m0: 0.0,
        m1: 0.0,
        m2: 0.0,
        m3: 0.0,
        m4: 0.0,
        m5: 0.0,
        m6: 0.0,
        m7: 0.0,
        m8: 0.0,
        m9: 0.0,
        m10: 0.0,
        m11: 0.0,
        m12: 0.0,
        m13: 0.0,
        m14: 0.0,
        m15: 0.0,
    };

    let rl: f32 = (right - left) as f32;
    let tb: f32 = (top - bottom) as f32;
    let fn_ = (farPlane - nearPlane) as f32;

    result.m0 = 2.0 / rl;
    result.m1 = 0.0;
    result.m2 = 0.0;
    result.m3 = 0.0;

    result.m4 = 0.0;
    result.m5 = 2.0 / tb;
    result.m6 = 0.0;
    result.m7 = 0.0;

    result.m8 = 0.0;
    result.m9 = 0.0;
    result.m10 = -2.0 / fn_;
    result.m11 = 0.0;

    result.m12 = -((left as f32 + right as f32) / rl);
    result.m13 = -((top as f32 + bottom as f32) / tb);
    result.m14 = -((farPlane as f32 + nearPlane as f32) / fn_);
    result.m15 = 1.0;

    result
}

pub fn MatrixPerspective(fovY: f64, aspect: f64, nearPlane: f64, farPlane: f64) -> Matrix {
    let mut result = Matrix {
        m0: 0.0,
        m1: 0.0,
        m2: 0.0,
        m3: 0.0,
        m4: 0.0,
        m5: 0.0,
        m6: 0.0,
        m7: 0.0,
        m8: 0.0,
        m9: 0.0,
        m10: 0.0,
        m11: 0.0,
        m12: 0.0,
        m13: 0.0,
        m14: 0.0,
        m15: 0.0,
    };

    let top: f64 = nearPlane * (fovY * 0.5).tan();
    let bottom: f64 = -top;
    let right: f64 = top * aspect;
    let left: f64 = -right;

    // MatrixFrustum(-right, right, -top, top, near, far);
    let rl: f32 = (right - left) as f32;
    let tb: f32 = (top - bottom) as f32;
    let fn_ = (farPlane - nearPlane) as f32;

    result.m0 = ((nearPlane as f32) * 2.0) / rl;
    result.m5 = ((nearPlane as f32) * 2.0) / tb;
    result.m8 = ((right + left) as f32) / rl;
    result.m9 = ((top + bottom) as f32) / tb;
    result.m10 = -((farPlane + nearPlane) as f32) / fn_;
    result.m11 = -1.0;
    result.m14 = -((farPlane as f32 * nearPlane as f32 * 2.0) / fn_);

    result
}

pub fn MatrixLookAt(eye: Vector3, target: Vector3, up: Vector3) -> Matrix {
    let mut result = Matrix {
        m0: 0.0,
        m1: 0.0,
        m2: 0.0,
        m3: 0.0,
        m4: 0.0,
        m5: 0.0,
        m6: 0.0,
        m7: 0.0,
        m8: 0.0,
        m9: 0.0,
        m10: 0.0,
        m11: 0.0,
        m12: 0.0,
        m13: 0.0,
        m14: 0.0,
        m15: 0.0,
    };

    let mut length: f32 = 0.0;
    let mut ilength: f32 = 0.0;

    // Vector3Subtract(eye, target)
    let mut vz = Vector3 {
        x: eye.x - target.x,
        y: eye.y - target.y,
        z: eye.z - target.z,
    };

    // normalize vz
    let mut v = vz;
    length = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if length == 0.0 {
        length = 1.0;
    }
    ilength = 1.0 / length;

    vz.x *= ilength;
    vz.y *= ilength;
    vz.z *= ilength;

    // cross(up, vz) -> vx
    let mut vx = Vector3 {
        x: up.y * vz.z - up.z * vz.y,
        y: up.z * vz.x - up.x * vz.z,
        z: up.x * vz.y - up.y * vz.x,
    };

    // normalize vx
    v = vx;
    length = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if length == 0.0 {
        length = 1.0;
    }
    ilength = 1.0 / length;

    vx.x *= ilength;
    vx.y *= ilength;
    vx.z *= ilength;

    // cross(vz, vx) -> vy
    let vy = Vector3 {
        x: vz.y * vx.z - vz.z * vx.y,
        y: vz.z * vx.x - vz.x * vx.z,
        z: vz.x * vx.y - vz.y * vx.x,
    };

    // fill matrix (column-major like raylib/OpenGL style)
    result.m0 = vx.x;
    result.m1 = vy.x;
    result.m2 = vz.x;
    result.m3 = 0.0;

    result.m4 = vx.y;
    result.m5 = vy.y;
    result.m6 = vz.y;
    result.m7 = 0.0;

    result.m8 = vx.z;
    result.m9 = vy.z;
    result.m10 = vz.z;
    result.m11 = 0.0;

    result.m12 = -(vx.x * eye.x + vx.y * eye.y + vx.z * eye.z);
    result.m13 = -(vy.x * eye.x + vy.y * eye.y + vy.z * eye.z);
    result.m14 = -(vz.x * eye.x + vz.y * eye.y + vz.z * eye.z);
    result.m15 = 1.0;

    result
}
