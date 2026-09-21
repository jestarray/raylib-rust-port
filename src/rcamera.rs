#![allow(unused_variables)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(clippy::missing_safety_doc, unused_parens, static_mut_refs)]
#![allow(
    clippy::too_many_arguments,
    clippy::needless_return,
    clippy::manual_range_contains,
    clippy::field_reassign_with_default,
    clippy::manual_map,
    clippy::match_like_matches_macro,
    clippy::upper_case_acronyms,
    clippy::let_and_return,
    clippy::double_parens,
)]
use std::f32::consts::FRAC_1_SQRT_2;

use crate::{math::DEG2RAD, rcore::{GetFrameTime, GetGamepadAxisMovement, GetMouseDelta, GetMouseWheelMove, IsGamepadAvailable, IsKeyDown, IsKeyPressed, IsMouseButtonDown}, rlgl::{RL_CULL_DISTANCE_FAR, RL_CULL_DISTANCE_NEAR}, types::{Camera, GamepadAxis::{GAMEPAD_AXIS_LEFT_X, GAMEPAD_AXIS_LEFT_Y, GAMEPAD_AXIS_RIGHT_X, GAMEPAD_AXIS_RIGHT_Y}, Matrix, MouseButton::MOUSE_BUTTON_MIDDLE, Vector3}};
use crate::types::KeyboardKey::*;
use crate::types::CameraProjection::*;
use crate::types::CameraMode::*;
use glam::Quat;
pub const CAMERA_CULL_DISTANCE_NEAR: f64 =   RL_CULL_DISTANCE_NEAR;
pub const CAMERA_CULL_DISTANCE_FAR: f64 =   RL_CULL_DISTANCE_FAR;
pub const CAMERA_MOVE_SPEED: f32 = 5.4;                     // Units per second
pub const CAMERA_ROTATION_SPEED: f32 = 0.03;
pub const CAMERA_PAN_SPEED: f32 = 2.0;

// Camera mouse movement sensitivity
pub const CAMERA_MOUSE_MOVE_SENSITIVITY: f32 = 0.003;

// Camera orbital speed in CAMERA_ORBITAL mode
pub const CAMERA_ORBITAL_SPEED: f32 = 0.5;                  // Radians per second
//----------------------------------------------------------------------------------
// Module Functions Definition
//----------------------------------------------------------------------------------
// Returns the cameras forward vector (normalized)
pub fn GetCameraForward(camera: &Camera) -> Vector3
{
    return (camera.target - camera.position).normalize();
}

// Returns the cameras up vector (normalized)
// Note: The up vector might not be perpendicular to the forward vector
pub fn GetCameraUp(camera: &Camera) -> Vector3
{
    return camera.up.normalize();
}

// Returns the cameras right vector (normalized)
pub fn GetCameraRight(camera: &Camera) -> Vector3
{
    let forward = GetCameraForward(camera);
    let up = GetCameraUp(camera);

    return forward.cross(up).normalize();
}

// Moves the camera in its forward direction
pub fn CameraMoveForward(camera: &mut Camera, distance: f32, moveInWorldPlane: bool)
{
    let mut forward = GetCameraForward(camera);

    if moveInWorldPlane
    {
        // Project vector onto world plane (the plane defined by the up vector)
        if camera.up.z.abs() > FRAC_1_SQRT_2 { forward.z = 0.0; }
        else if camera.up.x.abs() > FRAC_1_SQRT_2 { forward.x = 0.0; }
        else { forward.y = 0.0; }

        forward = forward.normalize();
    }

    // Scale by distance
    forward *= distance;

    // Move position and target
    camera.position += forward;
    camera.target += forward;
}

// Moves the camera in its up direction
pub fn CameraMoveUp(camera: &mut Camera, distance: f32)
{
    let mut up = GetCameraUp(camera);

    // Scale by distance
    up *= distance;

    // Move position and target
    camera.position += up;
    camera.target += up;
}

// Moves the camera target in its current right direction
pub fn CameraMoveRight(camera: &mut Camera, distance: f32, moveInWorldPlane: bool)
{
    let mut right = GetCameraRight(camera);

    if moveInWorldPlane
    {
        // Project vector onto world plane (the plane defined by the up vector)
        if camera.up.z.abs() > FRAC_1_SQRT_2 { right.z = 0.0; }
        else if camera.up.x.abs() > FRAC_1_SQRT_2 { right.x = 0.0; }
        else { right.y = 0.0; }

        right = right.normalize();
    }

    // Scale by distance
    right *= distance;

    // Move position and target
    camera.position += right;
    camera.target += right;
}

// Moves the camera position closer/farther to/from the camera target
pub fn CameraMoveToTarget(camera: &mut Camera, delta: f32)
{
    let mut distance = camera.position.distance(camera.target);

    // Apply delta
    distance += delta;

    // Distance must be greater than 0
    if distance <= 0.0 { distance = 0.001; }

    // Set new distance by moving the position along the forward vector
    let forward = GetCameraForward(camera);
    camera.position = camera.target + (forward * -distance);
}

// Rotates the camera around its up vector
// Yaw is "looking left and right"
// If rotateAroundTarget is false, the camera rotates around its position
// Note: angle must be provided in radians
pub fn CameraYaw(camera: &mut Camera, angle: f32, rotateAroundTarget: bool)
{
    // Rotation axis
    let up = GetCameraUp(camera);

    // View vector
    let mut targetPosition = camera.target - camera.position;

    // Rotate view vector around up axis (using glam's Quat operator overloading)
    targetPosition = Quat::from_axis_angle(up, angle) * targetPosition;

    if rotateAroundTarget
    {
        // Move position relative to target
        camera.position = camera.target - targetPosition;
    }
    else // rotate around camera.position
    {
        // Move target relative to position
        camera.target = camera.position + targetPosition;
    }
}

// Rotates the camera around its right vector, pitch is "looking up and down"
//  - lockView prevents camera overrotation (aka "somersaults")
//  - rotateAroundTarget defines if rotation is around target or around its position
//  - rotateUp rotates the up direction as well (typically only useful in CAMERA_FREE)
// NOTE: [angle] must be provided in radians
pub fn CameraPitch(camera: &mut Camera, mut angle: f32, lockView: bool, rotateAroundTarget: bool, rotateUp: bool)
{
    // Up direction
    let up = GetCameraUp(camera);

    // View vector
    let mut targetPosition = camera.target - camera.position;

    if lockView
    {
        // In these camera modes, clamp the Pitch angle
        // to allow only viewing straight up or down

        // Clamp view up
        let mut maxAngleUp = up.angle_between(targetPosition);
        maxAngleUp -= 0.001; // avoid numerical errors
        if angle > maxAngleUp { angle = maxAngleUp; }

        // Clamp view down
        let mut maxAngleDown = (-up).angle_between(targetPosition);
        maxAngleDown *= -1.0; // downwards angle is negative
        maxAngleDown += 0.001; // avoid numerical errors
        if angle < maxAngleDown { angle = maxAngleDown; }
    }

    // Rotation axis
    let right = GetCameraRight(camera);

    // Rotate view vector around right axis
    targetPosition = Quat::from_axis_angle(right, angle) * targetPosition;

    if rotateAroundTarget
    {
        // Move position relative to target
        camera.position = camera.target - targetPosition;
    }
    else // Rotate around camera.position
    {
        // Move target relative to position
        camera.target = camera.position + targetPosition;
    }

    if rotateUp
    {
        // Rotate up direction around right axis
        camera.up = Quat::from_axis_angle(right, angle) * camera.up;
    }
}

// Rotates the camera around its forward vector
// Roll is "turning your head sideways to the left or right"
// Note: angle must be provided in radians
pub fn CameraRoll(camera: &mut Camera, angle: f32)
{
    // Rotation axis
    let forward = GetCameraForward(camera);

    // Rotate up direction around forward axis
    camera.up = Quat::from_axis_angle(forward, angle) * camera.up;
}

// Returns the camera view matrix
pub fn GetCameraViewMatrix(camera: &Camera) -> Matrix
{
    return glam::camera::rh::view::look_at_mat4(camera.position, camera.target, camera.up);
}

// Returns the camera projection matrix
pub fn GetCameraProjectionMatrix(camera: &Camera, aspect: f32) -> Matrix
{
    if camera.projection == CAMERA_PERSPECTIVE as i32
    {
        return glam::camera::rh::proj::opengl::perspective(
            camera.fovy*DEG2RAD,
            aspect,
            CAMERA_CULL_DISTANCE_NEAR as f32,
            CAMERA_CULL_DISTANCE_FAR as f32,
        );
    }
    else if camera.projection == CAMERA_ORTHOGRAPHIC as i32
    {
        let top = (camera.fovy as f64)/2.0;
        let right = top*(aspect as f64);

        return glam::camera::rh::proj::opengl::orthographic(
            -right as f32,
            right as f32,
            -top as f32,
            top as f32,
            CAMERA_CULL_DISTANCE_NEAR as f32,
            CAMERA_CULL_DISTANCE_FAR as f32,
        );
    }

    return Matrix::IDENTITY;
}

// Update camera position for selected mode
// Camera mode: CAMERA_FREE, CAMERA_FIRST_PERSON, CAMERA_THIRD_PERSON, CAMERA_ORBITAL or CUSTOM
pub unsafe fn  UpdateCamera(camera: &mut Camera, mode: i32)
{
    let mousePositionDelta = GetMouseDelta();

    let moveInWorldPlane = ((mode == CAMERA_FIRST_PERSON as i32) || (mode == CAMERA_THIRD_PERSON as i32));
    let rotateAroundTarget = ((mode == CAMERA_THIRD_PERSON as i32) || (mode == CAMERA_ORBITAL as i32));
    let lockView = ((mode == CAMERA_FREE as i32) || (mode == CAMERA_FIRST_PERSON as i32) || (mode == CAMERA_THIRD_PERSON as i32) || (mode == CAMERA_ORBITAL as i32));
    let rotateUp = false;

    // Camera speeds based on frame time
    let cameraMoveSpeed = CAMERA_MOVE_SPEED*GetFrameTime();
    let cameraRotationSpeed = CAMERA_ROTATION_SPEED*GetFrameTime();
    let cameraPanSpeed = CAMERA_PAN_SPEED*GetFrameTime();
    let cameraOrbitalSpeed = CAMERA_ORBITAL_SPEED*GetFrameTime();

    if mode == CAMERA_CUSTOM as i32 {}
    else if mode == CAMERA_ORBITAL as i32
    {
        let rotation = Quat::from_axis_angle(GetCameraUp(camera), cameraOrbitalSpeed);
        let mut view = camera.position - camera.target;
        view = rotation * view;
        camera.position = camera.target + view;
    }
    else
    {
        // Camera rotation
        if IsKeyDown(KEY_DOWN) { CameraPitch(camera, -cameraRotationSpeed, lockView, rotateAroundTarget, rotateUp); }
        if IsKeyDown(KEY_UP) { CameraPitch(camera, cameraRotationSpeed, lockView, rotateAroundTarget, rotateUp); }
        if IsKeyDown(KEY_RIGHT) { CameraYaw(camera, -cameraRotationSpeed, rotateAroundTarget); }
        if IsKeyDown(KEY_LEFT) { CameraYaw(camera, cameraRotationSpeed, rotateAroundTarget); }
        if IsKeyDown(KEY_Q) { CameraRoll(camera, -cameraRotationSpeed); }
        if IsKeyDown(KEY_E) { CameraRoll(camera, cameraRotationSpeed); }

        // Camera movement
        // Camera pan (for CAMERA_FREE)
        if (mode == CAMERA_FREE as i32) && IsMouseButtonDown(MOUSE_BUTTON_MIDDLE as i32)
        {
            let mouseDelta = GetMouseDelta();
            if mouseDelta.x > 0.0 { CameraMoveRight(camera, cameraPanSpeed, moveInWorldPlane); }
            if mouseDelta.x < 0.0 { CameraMoveRight(camera, -cameraPanSpeed, moveInWorldPlane); }
            if mouseDelta.y > 0.0 { CameraMoveUp(camera, -cameraPanSpeed); }
            if mouseDelta.y < 0.0 { CameraMoveUp(camera, cameraPanSpeed); }
        }
        else
        {
            // Mouse support
            CameraYaw(camera, -mousePositionDelta.x*CAMERA_MOUSE_MOVE_SENSITIVITY, rotateAroundTarget);
            CameraPitch(camera, -mousePositionDelta.y*CAMERA_MOUSE_MOVE_SENSITIVITY, lockView, rotateAroundTarget, rotateUp);
        }

        // Keyboard support
        if IsKeyDown(KEY_W) { CameraMoveForward(camera, cameraMoveSpeed, moveInWorldPlane); }
        if IsKeyDown(KEY_A) { CameraMoveRight(camera, -cameraMoveSpeed, moveInWorldPlane); }
        if IsKeyDown(KEY_S) { CameraMoveForward(camera, -cameraMoveSpeed, moveInWorldPlane); }
        if IsKeyDown(KEY_D) { CameraMoveRight(camera, cameraMoveSpeed, moveInWorldPlane); }

        // Gamepad movement
        if IsGamepadAvailable(0)
        {
            // Gamepad controller support
            CameraYaw(camera, -(GetGamepadAxisMovement(0, GAMEPAD_AXIS_RIGHT_X as i32)*2.0)*CAMERA_MOUSE_MOVE_SENSITIVITY, rotateAroundTarget);
            CameraPitch(camera, -(GetGamepadAxisMovement(0, GAMEPAD_AXIS_RIGHT_Y as i32)*2.0)*CAMERA_MOUSE_MOVE_SENSITIVITY, lockView, rotateAroundTarget, rotateUp);

            if GetGamepadAxisMovement(0, GAMEPAD_AXIS_LEFT_Y as i32) <= -0.25 { CameraMoveForward(camera, cameraMoveSpeed, moveInWorldPlane); }
            if GetGamepadAxisMovement(0, GAMEPAD_AXIS_LEFT_X as i32) <= -0.25 { CameraMoveRight(camera, -cameraMoveSpeed, moveInWorldPlane); }
            if GetGamepadAxisMovement(0, GAMEPAD_AXIS_LEFT_Y as i32) >= 0.25 { CameraMoveForward(camera, -cameraMoveSpeed, moveInWorldPlane); }
            if GetGamepadAxisMovement(0, GAMEPAD_AXIS_LEFT_X as i32) >= 0.25 { CameraMoveRight(camera, cameraMoveSpeed, moveInWorldPlane); }
        }

        if mode == CAMERA_FREE as i32
        {
            if IsKeyDown(KEY_SPACE) { CameraMoveUp(camera, cameraMoveSpeed); }
            if IsKeyDown(KEY_LEFT_CONTROL) { CameraMoveUp(camera, -cameraMoveSpeed); }
        }
    }

    if (mode == CAMERA_THIRD_PERSON as i32) || (mode == CAMERA_ORBITAL as i32) || (mode == CAMERA_FREE as i32)
    {
        // Zoom target distance
        CameraMoveToTarget(camera, -GetMouseWheelMove());
        if IsKeyPressed(KEY_KP_SUBTRACT) { CameraMoveToTarget(camera, 2.0); }
        if IsKeyPressed(KEY_KP_ADD) { CameraMoveToTarget(camera, -2.0); }
    }
}
// #endif // !RCAMERA_STANDALONE

// Update camera movement, movement/rotation values should be provided by user
pub fn UpdateCameraPro(camera: &mut Camera, movement: Vector3, rotation: Vector3, zoom: f32)
{
    // Required values
    // movement.x - Move forward/backward
    // movement.y - Move right/left
    // movement.z - Move up/down
    // rotation.x - yaw
    // rotation.y - pitch
    // rotation.z - roll
    // zoom - Move towards target

    let lockView = true;
    let rotateAroundTarget = false;
    let rotateUp = false;
    let moveInWorldPlane = true;

    // Camera rotation
    CameraPitch(camera, -rotation.y*DEG2RAD, lockView, rotateAroundTarget, rotateUp);
    CameraYaw(camera, -rotation.x*DEG2RAD, rotateAroundTarget);
    CameraRoll(camera, rotation.z*DEG2RAD);

    // Camera movement
    CameraMoveForward(camera, movement.x, moveInWorldPlane);
    CameraMoveRight(camera, movement.y, moveInWorldPlane);
    CameraMoveUp(camera, movement.z);

    // Zoom target distance
    CameraMoveToTarget(camera, zoom);
}
