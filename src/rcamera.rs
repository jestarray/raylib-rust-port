use crate::types::{Camera, Vector3, Matrix, CameraProjection};
use crate::math::*;
use glam::{Vec3, Mat4, Quat};

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
pub fn camera_pitch(camera: &mut Camera, angle: f32, lock_view: bool, rotate_around_target: bool, rotate_up: bool) {
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
    Mat4::look_at_rh(camera.position, camera.target, camera.up)
}

// Returns the camera projection matrix
pub fn get_camera_projection_matrix(camera: &Camera, aspect: f32) -> Matrix {
    const CAMERA_CULL_DISTANCE_NEAR: f32 = 0.01; // Should match RL_CULL_DISTANCE_NEAR
    const CAMERA_CULL_DISTANCE_FAR: f32 = 1000.0; // Should match RL_CULL_DISTANCE_FAR

    if camera.projection == CameraProjection::Perspective as i32 {
        Mat4::perspective_rh(camera.fovy * DEG2RAD, aspect, CAMERA_CULL_DISTANCE_NEAR, CAMERA_CULL_DISTANCE_FAR)
    } else {
        let top = camera.fovy as f32 / 2.0;
        let right = top * aspect;
        Mat4::orthographic_rh(-right, right, -top, top, CAMERA_CULL_DISTANCE_NEAR, CAMERA_CULL_DISTANCE_FAR)
    }
}
