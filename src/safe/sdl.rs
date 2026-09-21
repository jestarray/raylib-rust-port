//! Safe wrappers for the raw `rsdl` bindings.
//!
//! Every public function of `rsdl` is mirrored here with a `snake_case` name and a
//! signature that can be called without `unsafe`. `SDL_*` prefixed helpers keep their
//! `sdl_` prefix so they stay distinguishable from the raylib-level wrappers in
//! [`crate::safe::core`]; window-state flags use [`ConfigFlags`], cursor selection uses
//! [`MouseCursor`], and pointer out-parameters become `&mut` references.
//!
//! Opaque SDL handles that have no safe wrapper in this crate (a raw `*mut SDL_Surface`
//! from the pixel-format migration helpers and the native window handle) are passed
//! through unchanged; everything else is converted.
//!
//! The raw `rsdl` module is private, so this module is the only way to reach the SDL
//! window, monitor and platform helpers from outside the crate.

use std::ffi::{CStr, c_void};

use sdl3_sys::events::{SDL_EventType, SDL_TouchFingerEvent};
use sdl3_sys::scancode::SDL_Scancode;
use sdl3_sys::surface::SDL_Surface;
use sdl3_sys::touch::SDL_TouchID;
use sdl3_sys::video::{SDL_DisplayID, SDL_DisplayMode};

use crate::rsdl::*;
use crate::types::{ConfigFlags, Image, KeyboardKey, MouseCursor, Vector2};

/// Returns `None` when there is no controller at `joystick_index`; otherwise the name is
/// copied out of SDL's borrowed `*const c_char` into an owned [`String`].
pub fn sdl_game_controller_name_for_index(joystick_index: i32) -> Option<String> {
    let name = unsafe { SDL_GameControllerNameForIndex(joystick_index) };
    if name.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(name) }
                .to_string_lossy()
                .into_owned(),
        )
    }
}

pub fn sdl_get_num_video_displays() -> i32 {
    unsafe { SDL_GetNumVideoDisplays() }
}

pub fn sdl_event_state(event_type: SDL_EventType, state: i32) -> u8 {
    unsafe { SDL_EventState(event_type, state) }
}

/// Fills `mode` with the current display mode, leaving it untouched when SDL reports none.
pub fn sdl_get_current_display_mode_adapter(display_id: SDL_DisplayID, mode: &mut SDL_DisplayMode) {
    unsafe { SDL_GetCurrentDisplayMode_Adapter(display_id, mode) }
}

/// Returns the opaque SDL surface handle unchanged; free it with `SDL_DestroySurface`.
#[allow(clippy::too_many_arguments)]
pub fn sdl_create_rgb_surface(
    flags: u32,
    width: i32,
    height: i32,
    depth: i32,
    r_mask: u32,
    g_mask: u32,
    b_mask: u32,
    a_mask: u32,
) -> *mut SDL_Surface {
    unsafe { SDL_CreateRGBSurface(flags, width, height, depth, r_mask, g_mask, b_mask, a_mask) }
}

/// Writes the display DPI into all three out-parameters; returns `0`.
pub fn sdl_get_display_dpi(
    display_index: i32,
    ddpi: &mut f32,
    hdpi: &mut f32,
    vdpi: &mut f32,
) -> i32 {
    unsafe { SDL_GetDisplayDPI(display_index, ddpi, hdpi, vdpi) }
}

/// Returns the opaque SDL surface handle unchanged; free it with `SDL_DestroySurface`.
pub fn sdl_create_rgb_surface_with_format(
    flags: u32,
    width: i32,
    height: i32,
    depth: i32,
    format: u32,
) -> *mut SDL_Surface {
    unsafe { SDL_CreateRGBSurfaceWithFormat(flags, width, height, depth, format) }
}

/// The caller keeps ownership of `pixels`, which must stay alive for as long as the returned
/// surface is used. Returns the opaque SDL surface handle unchanged.
#[allow(clippy::too_many_arguments)]
pub fn sdl_create_rgb_surface_from(
    pixels: &mut [u8],
    width: i32,
    height: i32,
    depth: i32,
    pitch: i32,
    r_mask: u32,
    g_mask: u32,
    b_mask: u32,
    a_mask: u32,
) -> *mut SDL_Surface {
    unsafe {
        SDL_CreateRGBSurfaceFrom(
            pixels.as_mut_ptr().cast(),
            width,
            height,
            depth,
            pitch,
            r_mask,
            g_mask,
            b_mask,
            a_mask,
        )
    }
}

/// The caller keeps ownership of `pixels`, which must stay alive for as long as the returned
/// surface is used. Returns the opaque SDL surface handle unchanged.
pub fn sdl_create_rgb_surface_with_format_from(
    pixels: &mut [u8],
    width: i32,
    height: i32,
    depth: i32,
    pitch: i32,
    format: u32,
) -> *mut SDL_Surface {
    unsafe {
        SDL_CreateRGBSurfaceWithFormatFrom(
            pixels.as_mut_ptr().cast(),
            width,
            height,
            depth,
            pitch,
            format,
        )
    }
}

pub fn sdl_num_joysticks() -> i32 {
    unsafe { SDL_NumJoysticks() }
}

/// Returns `0` on success and `-1` on failure.
pub fn sdl_set_relative_mouse_mode_adapter(enabled: bool) -> i32 {
    unsafe { SDL_SetRelativeMouseMode_Adapter(enabled) }
}

pub fn sdl_get_relative_mouse_mode_adapter() -> bool {
    unsafe { SDL_GetRelativeMouseMode_Adapter() }
}

pub fn sdl_get_num_touch_fingers(touch_id: SDL_TouchID) -> i32 {
    unsafe { SDL_GetNumTouchFingers(touch_id) }
}

pub fn window_should_close() -> bool {
    unsafe { WindowShouldClose() }
}

pub fn toggle_fullscreen() {
    unsafe { ToggleFullscreen() }
}

pub fn toggle_borderless_windowed() {
    unsafe { ToggleBorderlessWindowed() }
}

pub fn maximize_window() {
    unsafe { MaximizeWindow() }
}

pub fn minimize_window() {
    unsafe { MinimizeWindow() }
}

pub fn restore_window() {
    unsafe { RestoreWindow() }
}

/// Flags accumulate, so several variants can be applied by calling this once per flag.
pub fn set_window_state(flags: ConfigFlags) {
    unsafe { SetWindowState(flags as u32) }
}

/// Flags accumulate, so call this once per flag to clear several.
pub fn clear_window_state(flags: ConfigFlags) {
    unsafe { ClearWindowState(flags as u32) }
}

pub fn set_window_icon(image: &mut Image) {
    unsafe { SetWindowIcon(image) }
}

/// The count is derived from `images.len()`.
pub fn set_window_icons(images: &[Image]) {
    SetWindowIcons(images, images.len() as i32)
}

pub fn set_window_title(title: &str) {
    unsafe { SetWindowTitle(title) }
}

pub fn set_window_position(x: i32, y: i32) {
    unsafe { SetWindowPosition(x, y) }
}

pub fn set_window_monitor(monitor: i32) {
    unsafe { SetWindowMonitor(monitor) }
}

pub fn set_window_min_size(width: i32, height: i32) {
    unsafe { SetWindowMinSize(width, height) }
}

pub fn set_window_max_size(width: i32, height: i32) {
    unsafe { SetWindowMaxSize(width, height) }
}

pub fn set_window_size(width: i32, height: i32) {
    unsafe { SetWindowSize(width, height) }
}

pub fn set_window_opacity(opacity: f32) {
    unsafe { SetWindowOpacity(opacity) }
}

pub fn set_window_focused() {
    unsafe { SetWindowFocused() }
}

/// The native handle is platform specific (an `HWND` on Windows, an X11 window id or
/// `wl_surface` on Linux, an `NSWindow` on Apple platforms) and is passed through as a raw
/// pointer.
pub fn get_window_handle() -> *mut c_void {
    unsafe { GetWindowHandle() }
}

pub fn get_monitor_count() -> i32 {
    unsafe { GetMonitorCount() }
}

pub fn get_current_monitor() -> i32 {
    unsafe { GetCurrentMonitor() }
}

pub fn get_monitor_position(monitor: i32) -> Vector2 {
    unsafe { GetMonitorPosition(monitor) }
}

pub fn get_monitor_width(monitor: i32) -> i32 {
    unsafe { GetMonitorWidth(monitor) }
}

pub fn get_monitor_height(monitor: i32) -> i32 {
    unsafe { GetMonitorHeight(monitor) }
}

pub fn get_monitor_physical_width(monitor: i32) -> i32 {
    unsafe { GetMonitorPhysicalWidth(monitor) }
}

pub fn get_monitor_physical_height(monitor: i32) -> i32 {
    unsafe { GetMonitorPhysicalHeight(monitor) }
}

pub fn get_monitor_refresh_rate(monitor: i32) -> i32 {
    unsafe { GetMonitorRefreshRate(monitor) }
}

pub fn get_monitor_name(monitor: i32) -> String {
    unsafe { GetMonitorName(monitor) }
}

pub fn get_window_position() -> Vector2 {
    unsafe { GetWindowPosition() }
}

pub fn get_window_scale_dpi() -> Vector2 {
    unsafe { GetWindowScaleDPI() }
}

pub fn set_clipboard_text(text: &str) {
    unsafe { SetClipboardText(text) }
}

pub fn get_clipboard_text() -> String {
    unsafe { GetClipboardText() }
}

pub fn get_clipboard_image() -> Option<Image> {
    unsafe { GetClipboardImage() }
}

pub fn show_cursor() {
    unsafe { ShowCursor() }
}

pub fn hide_cursor() {
    unsafe { HideCursor() }
}

pub fn enable_cursor() {
    unsafe { EnableCursor() }
}

pub fn disable_cursor() {
    unsafe { DisableCursor() }
}

pub fn swap_screen_buffer() {
    unsafe { SwapScreenBuffer() }
}

pub fn get_time() -> f64 {
    unsafe { GetTime() }
}

pub fn open_url(url: &str) {
    unsafe { OpenURL(url) }
}

pub fn set_gamepad_mappings(mappings: &str) -> i32 {
    SetGamepadMappings(mappings)
}

pub fn set_gamepad_vibration(gamepad: i32, left_motor: f32, right_motor: f32, duration: f32) {
    unsafe { SetGamepadVibration(gamepad, left_motor, right_motor, duration) }
}

pub fn set_mouse_position(x: i32, y: i32) {
    unsafe { SetMousePosition(x, y) }
}

pub fn set_mouse_cursor(cursor: MouseCursor) {
    unsafe { SetMouseCursor(cursor as i32) }
}

pub fn get_key_name(key: KeyboardKey) -> String {
    unsafe { GetKeyName(key as i32) }
}

pub fn poll_input_events() {
    unsafe { PollInputEvents() }
}

pub fn init_platform() -> i32 {
    unsafe { InitPlatform() }
}

pub fn close_platform() {
    unsafe { ClosePlatform() }
}

/// Returns the matching [`KeyboardKey`] discriminant, or `0` (`KEY_NULL`) when the
/// scancode has no raylib equivalent.
pub fn convert_scancode_to_key(sdl_scancode: SDL_Scancode) -> i32 {
    ConvertScancodeToKey(sdl_scancode)
}

pub fn get_codepoint_next_sdl(text: &str, codepoint_size: &mut i32) -> i32 {
    GetCodepointNextSDL(text, codepoint_size)
}

pub fn update_touch_points_sdl(event: SDL_TouchFingerEvent) {
    unsafe { UpdateTouchPointsSDL(event) }
}

pub fn copy_sdl_error() -> String {
    CopySDLError()
}
