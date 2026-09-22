//! Safe wrappers for the raw `rcore` bindings.
//!
//! Every public function of `rcore` is mirrored here with a `snake_case` name and a
//! signature that can be called without `unsafe`:
//!
//! * `i32`/`u32` parameters that name a value from [`crate::types`] use the matching enum
//!   ([`ConfigFlags`], [`BlendMode`], [`TraceLogLevel`], [`KeyboardKey`], [`MouseButton`],
//!   [`GamepadButton`], [`GamepadAxis`], [`ShaderUniformDataType`]).
//! * file/path parameters are generic over [`AsRef<Path>`](AsRef).
//! * raw pointers become references: `*const c_void` uniform payloads turn into `&T` or
//!   `&[T]`, and `*mut AutomationEventList` becomes `&mut AutomationEventList`.
//! * redundant length arguments are derived from the slice that is passed in.
//!
//! The raw `rcore` module is private, so this module is the only way to reach window,
//! drawing and input functionality from outside the crate.

use std::path::Path;

use log::warn;

use crate::rcore::TraceLogCallback;
use crate::rcore::*;
use crate::types::{
    AutomationEvent, AutomationEventList, BlendMode, Camera, Camera2D, Color, ConfigFlags,
    GamepadAxis, GamepadButton, KeyboardKey, Matrix, MouseButton, Ray, RenderTexture2D, Shader,
    ShaderUniformDataType, Texture2D, TraceLogLevel, Vector2, Vector3, VrDeviceInfo,
    VrStereoConfig,
};

// The `CORE` global's data types, re-exported so callers can name what [`state`] returns
// and read the individual sections. The raw `CORE` static itself is deliberately *not*
// exported: it is a `static mut`, and handing it out would let callers mutate raylib's
// internal state (and, in edition 2024, they could not even take a reference to it).
pub use crate::rcore::{
    CoreData, GamepadData, InputData, KeyboardData, MouseData, StorageData, TimeData, TouchData,
    WindowData,
};

/// Read-only, zero-cost view of raylib's live global `CORE` state.
///
/// Handy for debugging and for the few values that have no dedicated accessor:
///
/// ```no_run
/// let state = raylib::core::state();
/// println!("screen: {:?}", state.Window.screen);
/// println!("exit key: {}", state.Input.Keyboard.exitKey);
/// println!("{state:#?}"); // `CoreData` derives `Debug`
/// ```
///
/// This borrows the live state rather than copying it, so do not hold the reference across a
/// call that mutates core state. Use [`state_snapshot`] when you need to keep it around.
pub fn state() -> &'static CoreData {
    // `addr_of!` takes the address without forming a reference to the `static mut`, which
    // edition 2024 rejects outright (`static_mut_refs` is a hard error there).
    unsafe { &*std::ptr::addr_of!(crate::rcore::CORE) }
}

/// Owned copy of raylib's global `CORE` state, taken at the moment of the call.
///
/// Unlike [`state`], the result is detached from the live state, so it is safe to store and
/// inspect later. It is a deep copy, so prefer [`state`] for a one-off read.
pub fn state_snapshot() -> CoreData {
    unsafe { (*std::ptr::addr_of!(crate::rcore::CORE)).clone() }
}

/// Convert a path-like value into the UTF-8 `&str` that the raw bindings expect, logging a
/// warning when the path is not valid UTF-8.
///
/// Shared by the wrapper modules; not part of the public API.
pub(crate) fn path_to_str(path: &Path) -> Option<&str> {
    match path.to_str() {
        Some(path) => Some(path),
        None => {
            warn!("SYSTEM: Path is not valid UTF-8: {:?}", path);
            None
        }
    }
}

//----------------------------------------------------------------------------------
// Window and Graphics Device
//----------------------------------------------------------------------------------

pub fn init_window(width: i32, height: i32, title: &str) {
    unsafe { InitWindow(width, height, title) }
}

pub fn close_window() {
    unsafe { CloseWindow() }
}

pub fn is_window_ready() -> bool {
    unsafe { IsWindowReady() }
}

pub fn is_window_fullscreen() -> bool {
    unsafe { IsWindowFullscreen() }
}

pub fn is_window_hidden() -> bool {
    unsafe { IsWindowHidden() }
}

pub fn is_window_minimized() -> bool {
    unsafe { IsWindowMinimized() }
}

pub fn is_window_maximized() -> bool {
    unsafe { IsWindowMaximized() }
}

pub fn is_window_focused() -> bool {
    unsafe { IsWindowFocused() }
}

pub fn is_window_resized() -> bool {
    unsafe { IsWindowResized() }
}

pub fn is_window_state(flag: ConfigFlags) -> bool {
    unsafe { IsWindowState(flag as u32) }
}

pub fn get_screen_width() -> i32 {
    unsafe { GetScreenWidth() }
}

pub fn get_screen_height() -> i32 {
    unsafe { GetScreenHeight() }
}

pub fn get_render_width() -> i32 {
    unsafe { GetRenderWidth() }
}

pub fn get_render_height() -> i32 {
    unsafe { GetRenderHeight() }
}

pub fn enable_event_waiting() {
    unsafe { EnableEventWaiting() }
}

pub fn disable_event_waiting() {
    unsafe { DisableEventWaiting() }
}

pub fn is_cursor_hidden() -> bool {
    unsafe { IsCursorHidden() }
}

pub fn is_cursor_on_screen() -> bool {
    unsafe { IsCursorOnScreen() }
}

//----------------------------------------------------------------------------------
// Drawing
//----------------------------------------------------------------------------------

pub fn clear_background(color: Color) {
    unsafe { ClearBackground(color) }
}

pub fn begin_drawing() {
    unsafe { BeginDrawing() }
}

pub fn end_drawing() {
    unsafe { EndDrawing() }
}

//----------------------------------------------------------------------------------
// 2D/3D Camera and render modes
//----------------------------------------------------------------------------------

pub fn begin_mode_2d(camera: Camera2D) {
    unsafe { BeginMode2D(camera) }
}

pub fn end_mode_2d() {
    unsafe { EndMode2D() }
}

pub fn begin_mode_3d(camera: Camera) {
    unsafe { BeginMode3D(camera) }
}

pub fn end_mode_3d() {
    unsafe { EndMode3D() }
}

pub fn begin_texture_mode(target: RenderTexture2D) {
    unsafe { BeginTextureMode(target) }
}

pub fn end_texture_mode() {
    unsafe { EndTextureMode() }
}

pub fn begin_shader_mode(shader: &mut Shader) {
    unsafe { BeginShaderMode(shader) }
}

pub fn end_shader_mode() {
    unsafe { EndShaderMode() }
}

pub fn begin_blend_mode(mode: BlendMode) {
    unsafe { BeginBlendMode(mode as i32) }
}

pub fn end_blend_mode() {
    unsafe { EndBlendMode() }
}

pub fn begin_scissor_mode(x: i32, y: i32, width: i32, height: i32) {
    unsafe { BeginScissorMode(x, y, width, height) }
}

pub fn end_scissor_mode() {
    unsafe { EndScissorMode() }
}

pub fn begin_vr_stereo_mode(config: VrStereoConfig) {
    unsafe { BeginVrStereoMode(config) }
}

pub fn end_vr_stereo_mode() {
    unsafe { EndVrStereoMode() }
}

pub fn load_vr_stereo_config(device: VrDeviceInfo) -> VrStereoConfig {
    unsafe { LoadVrStereoConfig(device) }
}

pub fn unload_vr_stereo_config(config: VrStereoConfig) {
    UnloadVrStereoConfig(config)
}

//----------------------------------------------------------------------------------
// Shaders
//----------------------------------------------------------------------------------

pub fn load_shader<P: AsRef<Path>, Q: AsRef<Path>>(
    vs_file_name: Option<P>,
    fs_file_name: Option<Q>,
) -> Shader {
    let vs_file_name = match vs_file_name.as_ref() {
        Some(path) => path_to_str(path.as_ref()),
        None => None,
    };
    let fs_file_name = match fs_file_name.as_ref() {
        Some(path) => path_to_str(path.as_ref()),
        None => None,
    };

    unsafe { LoadShader(vs_file_name, fs_file_name) }
}

pub fn load_shader_from_memory(vs_code: Option<&str>, fs_code: Option<&str>) -> Shader {
    unsafe { LoadShaderFromMemory(vs_code, fs_code) }
}

pub fn is_shader_valid(shader: Shader) -> bool {
    IsShaderValid(shader)
}

pub fn unload_shader(shader: &mut Shader) {
    unsafe { UnloadShader(shader) }
}

pub fn get_shader_location(shader: &Shader, uniform_name: &str) -> i32 {
    unsafe { GetShaderLocation(shader, uniform_name) }
}

pub fn get_shader_location_attrib(shader: Shader, attrib_name: &str) -> i32 {
    unsafe { GetShaderLocationAttrib(shader, attrib_name) }
}

/// Takes a reference to a single value of the type named by `uniform_type`.
pub fn set_shader_value<T>(
    shader: &mut Shader,
    loc_index: i32,
    value: &T,
    uniform_type: ShaderUniformDataType,
) {
    unsafe {
        SetShaderValue(
            shader,
            loc_index,
            std::ptr::from_ref(value).cast(),
            uniform_type as i32,
        )
    }
}

/// Takes a slice whose `len()` becomes the uniform count. Each element of `value` must be
/// one `uniform_type` value, so e.g. a `vec2` uniform is passed as `&[[f32; 2]]`.
pub fn set_shader_value_v<T>(
    shader: &mut Shader,
    loc_index: i32,
    value: &[T],
    uniform_type: ShaderUniformDataType,
) {
    unsafe {
        SetShaderValueV(
            shader,
            loc_index,
            value.as_ptr().cast(),
            uniform_type as i32,
            value.len() as i32,
        )
    }
}

pub fn set_shader_value_matrix(shader: Shader, loc_index: i32, mat: Matrix) {
    unsafe { SetShaderValueMatrix(shader, loc_index, mat) }
}

pub fn set_shader_value_texture(shader: Shader, loc_index: i32, texture: Texture2D) {
    unsafe { SetShaderValueTexture(shader, loc_index, texture) }
}

//----------------------------------------------------------------------------------
// Screen-space conversions
//----------------------------------------------------------------------------------

pub fn get_screen_to_world_ray(position: Vector2, camera: Camera) -> Ray {
    unsafe { GetScreenToWorldRay(position, camera) }
}

pub fn get_screen_to_world_ray_ex(
    position: Vector2,
    camera: Camera,
    width: i32,
    height: i32,
) -> Ray {
    unsafe { GetScreenToWorldRayEx(position, camera, width, height) }
}

pub fn get_camera_matrix(camera: Camera) -> Matrix {
    GetCameraMatrix(camera)
}

pub fn get_camera_matrix_2d(camera: Camera2D) -> Matrix {
    GetCameraMatrix2D(camera)
}

pub fn get_world_to_screen(position: Vector3, camera: Camera) -> Vector2 {
    unsafe { GetWorldToScreen(position, camera) }
}

pub fn get_world_to_screen_ex(
    position: Vector3,
    camera: Camera,
    width: i32,
    height: i32,
) -> Vector2 {
    unsafe { GetWorldToScreenEx(position, camera, width, height) }
}

pub fn get_world_to_screen_2d(position: Vector2, camera: Camera2D) -> Vector2 {
    GetWorldToScreen2D(position, camera)
}

pub fn get_screen_to_world_2d(position: Vector2, camera: Camera2D) -> Vector2 {
    GetScreenToWorld2D(position, camera)
}

//----------------------------------------------------------------------------------
// Timing
//----------------------------------------------------------------------------------

pub fn set_target_fps(fps: i32) {
    unsafe { SetTargetFPS(fps) }
}

pub fn get_fps() -> i32 {
    unsafe { GetFPS() }
}

pub fn get_frame_time() -> f32 {
    unsafe { GetFrameTime() }
}

pub fn wait_time(seconds: f64) {
    WaitTime(seconds)
}

pub fn get_random_value(min: i32, max: i32) -> i32 {
    GetRandomValue(min, max)
}

pub fn take_screenshot<P: AsRef<Path>>(file_name: P) {
    if let Some(file_name) = path_to_str(file_name.as_ref()) {
        unsafe { TakeScreenshot(file_name) }
    }
}

//----------------------------------------------------------------------------------
// Configuration and logging
//----------------------------------------------------------------------------------

/// Flags accumulate, so call this once per flag to request several; it has no effect after
/// [`init_window`] has run.
pub fn set_config_flags(flags: u32) {
    unsafe { SetConfigFlags(flags) }
}

pub fn set_trace_log_level(log_type: TraceLogLevel) {
    unsafe { SetTraceLogLevel(log_type as i32) }
}

pub fn trace_log(log_type: TraceLogLevel, text: std::fmt::Arguments<'_>) {
    TraceLog(log_type as i32, text)
}

pub fn set_trace_log_callback(callback: Option<TraceLogCallback>) {
    unsafe { SetTraceLogCallback(callback) }
}

//----------------------------------------------------------------------------------
// Filesystem
//----------------------------------------------------------------------------------

pub fn load_file_data<P: AsRef<Path>>(file_name: P) -> Result<Vec<u8>, String> {
    LoadFileData(file_name)
}

/// [`load_file_data`] hands back an owned `Vec<u8>`, so releasing the buffer normally just
/// means dropping it. This exists for parity with the C API and leaves the slice untouched.
pub fn unload_file_data(data: &mut [u8]) {
    unsafe { UnloadFileData(data.as_mut_ptr()) }
}

pub fn save_file_data<P: AsRef<Path>>(file_name: P, data: &[u8]) -> bool {
    match path_to_str(file_name.as_ref()) {
        Some(file_name) => SaveFileData(file_name, data),
        None => false,
    }
}

/// The exported length is derived from `data.len()`.
pub fn export_data_as_code<P: AsRef<Path>>(data: &[u8], file_name: P) -> bool {
    match path_to_str(file_name.as_ref()) {
        Some(file_name) => ExportDataAsCode(data, data.len() as i32, file_name),
        None => false,
    }
}

pub fn load_file_text<P: AsRef<Path>>(file_name: P) -> Option<String> {
    match path_to_str(file_name.as_ref()) {
        Some(file_name) => LoadFileText(file_name),
        None => None,
    }
}

pub fn unload_file_text(text: Option<String>) {
    UnloadFileText(text)
}

pub fn save_file_text<P: AsRef<Path>>(file_name: P, text: &str) -> bool {
    match path_to_str(file_name.as_ref()) {
        Some(file_name) => SaveFileText(file_name, text),
        None => false,
    }
}

pub fn file_exists<P: AsRef<Path>>(file_name: P) -> bool {
    match path_to_str(file_name.as_ref()) {
        Some(file_name) => FileExists(file_name),
        None => false,
    }
}

pub fn is_file_extension<P: AsRef<Path>>(file_name: P, ext: &str) -> bool {
    match path_to_str(file_name.as_ref()) {
        Some(file_name) => IsFileExtension(file_name, ext),
        None => false,
    }
}

pub fn directory_exists<P: AsRef<Path>>(dir_path: P) -> bool {
    match path_to_str(dir_path.as_ref()) {
        Some(dir_path) => DirectoryExists(dir_path),
        None => false,
    }
}

pub fn get_file_extension<P: AsRef<Path> + ?Sized>(file_name: &P) -> Option<&str> {
    match path_to_str(file_name.as_ref()) {
        Some(file_name) => GetFileExtension(file_name),
        None => None,
    }
}

pub fn get_file_name<P: AsRef<Path> + ?Sized>(file_path: &P) -> &str {
    match path_to_str(file_path.as_ref()) {
        Some(file_path) => GetFileName(file_path),
        None => "",
    }
}

pub fn get_file_name_without_ext<P: AsRef<Path>>(file_path: P) -> String {
    match path_to_str(file_path.as_ref()) {
        Some(file_path) => GetFileNameWithoutExt(file_path),
        None => String::new(),
    }
}

pub fn get_directory_path<P: AsRef<Path>>(file_path: P) -> String {
    match path_to_str(file_path.as_ref()) {
        Some(file_path) => GetDirectoryPath(file_path),
        None => String::new(),
    }
}

pub fn get_working_directory() -> String {
    GetWorkingDirectory()
}

pub fn is_path_file<P: AsRef<Path>>(path: P) -> bool {
    match path_to_str(path.as_ref()) {
        Some(path) => IsPathFile(path),
        None => false,
    }
}

pub fn is_path_directory<P: AsRef<Path>>(path: P) -> bool {
    match path_to_str(path.as_ref()) {
        Some(path) => IsPathDirectory(path),
        None => false,
    }
}

pub fn is_path_absolute<P: AsRef<Path>>(path: P) -> bool {
    match path_to_str(path.as_ref()) {
        Some(path) => IsPathAbsolute(path),
        None => false,
    }
}

pub fn is_file_dropped() -> bool {
    unsafe { IsFileDropped() }
}

//----------------------------------------------------------------------------------
// Automation events
//----------------------------------------------------------------------------------

pub fn load_automation_event_list<P: AsRef<Path>>(file_name: Option<P>) -> AutomationEventList {
    LoadAutomationEventList(file_name)
}

pub fn unload_automation_event_list(list: &mut AutomationEventList) {
    UnloadAutomationEventList(list)
}

pub fn export_automation_event_list<P: AsRef<Path>>(
    list: AutomationEventList,
    file_name: P,
) -> bool {
    match path_to_str(file_name.as_ref()) {
        Some(file_name) => unsafe { ExportAutomationEventList(list, file_name) },
        None => false,
    }
}

/// The list is kept in a static, so `list` must outlive the recording session.
pub fn set_automation_event_list(list: &mut AutomationEventList) {
    unsafe { SetAutomationEventList(list) }
}

pub fn set_automation_event_base_frame(frame: i32) {
    unsafe { SetAutomationEventBaseFrame(frame) }
}

pub fn start_automation_event_recording() {
    unsafe { StartAutomationEventRecording() }
}

pub fn stop_automation_event_recording() {
    unsafe { StopAutomationEventRecording() }
}

pub fn play_automation_event(event: AutomationEvent) {
    unsafe { PlayAutomationEvent(event) }
}

#[cfg(feature = "SUPPORT_AUTOMATION_EVENTS")]
pub fn record_automation_event() {
    unsafe { RecordAutomationEvent() }
}

//----------------------------------------------------------------------------------
// Keyboard input
//----------------------------------------------------------------------------------

pub fn is_key_pressed(key: KeyboardKey) -> bool {
    unsafe { IsKeyPressed(key) }
}

pub fn is_key_pressed_repeat(key: KeyboardKey) -> bool {
    unsafe { IsKeyPressedRepeat(key as i32) }
}

pub fn is_key_down(key: KeyboardKey) -> bool {
    unsafe { IsKeyDown(key) }
}

pub fn is_key_released(key: KeyboardKey) -> bool {
    unsafe { IsKeyReleased(key as i32) }
}

pub fn is_key_up(key: KeyboardKey) -> bool {
    unsafe { IsKeyUp(key as i32) }
}

/// Returns the raw key code (see [`KeyboardKey`]) of the last key pressed in the queue, or
/// `0` when the queue is empty.
pub fn get_key_pressed() -> i32 {
    unsafe { GetKeyPressed() }
}

/// Returns the next unicode codepoint in the queue, or `0` when the queue is empty.
pub fn get_char_pressed() -> i32 {
    unsafe { GetCharPressed() }
}

pub fn set_exit_key(key: KeyboardKey) {
    unsafe { SetExitKey(key as i32) }
}

//----------------------------------------------------------------------------------
// Gamepad input
//----------------------------------------------------------------------------------

pub fn is_gamepad_available(gamepad: i32) -> bool {
    unsafe { IsGamepadAvailable(gamepad) }
}

pub fn get_gamepad_name(gamepad: i32) -> Option<String> {
    unsafe { GetGamepadName(gamepad) }
}

pub fn is_gamepad_button_pressed(gamepad: i32, button: GamepadButton) -> bool {
    unsafe { IsGamepadButtonPressed(gamepad, button as i32) }
}

pub fn is_gamepad_button_down(gamepad: i32, button: GamepadButton) -> bool {
    unsafe { IsGamepadButtonDown(gamepad, button) }
}

pub fn is_gamepad_button_released(gamepad: i32, button: GamepadButton) -> bool {
    unsafe { IsGamepadButtonReleased(gamepad, button as i32) }
}

pub fn is_gamepad_button_up(gamepad: i32, button: GamepadButton) -> bool {
    unsafe { IsGamepadButtonUp(gamepad, button as i32) }
}

/// Returns the raw button index (see [`GamepadButton`]) of the last button pressed, or `0`
/// when none was pressed.
pub fn get_gamepad_button_pressed() -> i32 {
    unsafe { GetGamepadButtonPressed() }
}

pub fn get_gamepad_axis_count(gamepad: i32) -> i32 {
    unsafe { GetGamepadAxisCount(gamepad) }
}

pub fn get_gamepad_axis_movement(gamepad: i32, axis: GamepadAxis) -> f32 {
    unsafe { GetGamepadAxisMovement(gamepad, axis as i32) }
}

//----------------------------------------------------------------------------------
// Mouse input
//----------------------------------------------------------------------------------

pub fn is_mouse_button_pressed(button: MouseButton) -> bool {
    unsafe { IsMouseButtonPressed(button) }
}

pub fn is_mouse_button_down(button: MouseButton) -> bool {
    unsafe { IsMouseButtonDown(button as i32) }
}

pub fn is_mouse_button_released(button: MouseButton) -> bool {
    unsafe { IsMouseButtonReleased(button as i32) }
}

pub fn is_mouse_button_up(button: MouseButton) -> bool {
    unsafe { IsMouseButtonUp(button as i32) }
}

pub fn get_mouse_x() -> i32 {
    unsafe { GetMouseX() }
}

pub fn get_mouse_y() -> i32 {
    unsafe { GetMouseY() }
}

pub fn get_mouse_position() -> Vector2 {
    unsafe { GetMousePosition() }
}

pub fn get_mouse_delta() -> Vector2 {
    unsafe { GetMouseDelta() }
}

pub fn set_mouse_offset(offset_x: i32, offset_y: i32) {
    unsafe { SetMouseOffset(offset_x, offset_y) }
}

pub fn set_mouse_scale(scale_x: f32, scale_y: f32) {
    unsafe { SetMouseScale(scale_x, scale_y) }
}

pub fn get_mouse_wheel_move() -> f32 {
    unsafe { GetMouseWheelMove() }
}

pub fn get_mouse_wheel_move_v() -> Vector2 {
    unsafe { GetMouseWheelMoveV() }
}

//----------------------------------------------------------------------------------
// Touch input
//----------------------------------------------------------------------------------

pub fn get_touch_x() -> i32 {
    unsafe { GetTouchX() }
}

pub fn get_touch_y() -> i32 {
    unsafe { GetTouchY() }
}

pub fn get_touch_position(index: i32) -> Vector2 {
    unsafe { GetTouchPosition(index) }
}

pub fn get_touch_point_id(index: i32) -> i32 {
    unsafe { GetTouchPointId(index) }
}

pub fn get_touch_point_count() -> i32 {
    unsafe { GetTouchPointCount() }
}

//----------------------------------------------------------------------------------
// Misc
//----------------------------------------------------------------------------------

pub fn init_timer() {
    unsafe { InitTimer() }
}

pub fn setup_viewport(width: i32, height: i32) {
    unsafe { SetupViewport(width, height) }
}

/// Only available without the `SUPPORT_MODULE_RTEXT` feature.
#[cfg(not(feature = "SUPPORT_MODULE_RTEXT"))]
pub fn text_format(text: std::fmt::Arguments<'_>) -> String {
    TextFormat(text)
}
