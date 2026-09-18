#![allow(unused_variables)]
#![allow(unsafe_op_in_unsafe_fn)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(clippy::missing_safety_doc, unused_parens, non_snake_case, static_mut_refs)]
#![allow(
    clippy::too_many_arguments,
    clippy::needless_return,
    clippy::manual_range_contains,
    clippy::field_reassign_with_default,
    clippy::manual_map,
    clippy::match_like_matches_macro,
    clippy::upper_case_acronyms,
    clippy::let_and_return,
    clippy::double_parens
)]
use sdl3_sys::joystick::SDL_JoystickID;
use sdl3_sys::mouse::SDL_Cursor;
use sdl3_sys::video::{SDL_GLContext, SDL_Window};

use crate::rcore::MAX_GAMEPADS;

#[repr(C)]
#[allow(non_snake_case)]
pub struct PlatformData {
    pub window: *mut SDL_Window,
    pub glContext: SDL_GLContext,

    pub gamepad: [*mut SDL_Gamepad; MAX_GAMEPADS],
    pub gamepadId: [SDL_JoystickID; MAX_GAMEPADS], // Joystick instance ids, they do not start from 0
    pub cursor: *mut SDL_Cursor,

    #[cfg(target_os = "linux")]
    pub windowHandleX11: std::os::raw::c_ulong, // unsigned long → c_ulong
}

use crate::rcore::*;
use crate::types::*;
use crate::types::{ConfigFlags::*, KeyboardKey::*, GamepadButton::*, GamepadAxis::*, PixelFormat::*};
use crate::rlgl::{rlGetVersion, rlLoadExtensions, rlGlVersion::*};
use log::{info, warn, error};
use std::ffi::{CStr, CString};
use sdl3_sys::{clipboard::*, events::*, gamepad::*, hints::*, init::*, joystick::*, keyboard::*, keycode::*, mouse::*, pixels::*, properties::*, error::*, filesystem::*, misc::*, rect::*, scancode::*, stdinc::*, surface::*, timer::*, touch::*, video::*};

// Size of the clipboard buffer used on GetClipboardText()
const MAX_CLIPBOARD_BUFFER_LENGTH: usize = 1024;
const SCANCODE_MAPPED_NUM: usize = 232;

//----------------------------------------------------------------------------------
// Global Variables Definition
//----------------------------------------------------------------------------------
use crate::rcore::CORE;                   // Global CORE state context

static mut platform: PlatformData = PlatformData {
    window: std::ptr::null_mut(),
    glContext: std::ptr::null_mut(),
    gamepad: [std::ptr::null_mut(); MAX_GAMEPADS],
    gamepadId: [SDL_JoystickID(0); MAX_GAMEPADS],
    cursor: std::ptr::null_mut(),
    #[cfg(target_os = "linux")]
    windowHandleX11: 0,
};   // Platform specific data

static mapScancodeToKey: [i32; SCANCODE_MAPPED_NUM] = [
    KEY_NULL as i32,           // SDL_SCANCODE_UNKNOWN
    0,
    0,
    0,
    KEY_A as i32,              // SDL_SCANCODE_A
    KEY_B as i32,              // SDL_SCANCODE_B
    KEY_C as i32,              // SDL_SCANCODE_C
    KEY_D as i32,              // SDL_SCANCODE_D
    KEY_E as i32,              // SDL_SCANCODE_E
    KEY_F as i32,              // SDL_SCANCODE_F
    KEY_G as i32,              // SDL_SCANCODE_G
    KEY_H as i32,              // SDL_SCANCODE_H
    KEY_I as i32,              // SDL_SCANCODE_I
    KEY_J as i32,              // SDL_SCANCODE_J
    KEY_K as i32,              // SDL_SCANCODE_K
    KEY_L as i32,              // SDL_SCANCODE_L
    KEY_M as i32,              // SDL_SCANCODE_M
    KEY_N as i32,              // SDL_SCANCODE_N
    KEY_O as i32,              // SDL_SCANCODE_O
    KEY_P as i32,              // SDL_SCANCODE_P
    KEY_Q as i32,              // SDL_SCANCODE_Q
    KEY_R as i32,              // SDL_SCANCODE_R
    KEY_S as i32,              // SDL_SCANCODE_S
    KEY_T as i32,              // SDL_SCANCODE_T
    KEY_U as i32,              // SDL_SCANCODE_U
    KEY_V as i32,              // SDL_SCANCODE_V
    KEY_W as i32,              // SDL_SCANCODE_W
    KEY_X as i32,              // SDL_SCANCODE_X
    KEY_Y as i32,              // SDL_SCANCODE_Y
    KEY_Z as i32,              // SDL_SCANCODE_Z
    KEY_ONE as i32,            // SDL_SCANCODE_1
    KEY_TWO as i32,            // SDL_SCANCODE_2
    KEY_THREE as i32,          // SDL_SCANCODE_3
    KEY_FOUR as i32,           // SDL_SCANCODE_4
    KEY_FIVE as i32,           // SDL_SCANCODE_5
    KEY_SIX as i32,            // SDL_SCANCODE_6
    KEY_SEVEN as i32,          // SDL_SCANCODE_7
    KEY_EIGHT as i32,          // SDL_SCANCODE_8
    KEY_NINE as i32,           // SDL_SCANCODE_9
    KEY_ZERO as i32,           // SDL_SCANCODE_0
    KEY_ENTER as i32,          // SDL_SCANCODE_RETURN
    KEY_ESCAPE as i32,         // SDL_SCANCODE_ESCAPE
    KEY_BACKSPACE as i32,      // SDL_SCANCODE_BACKSPACE
    KEY_TAB as i32,            // SDL_SCANCODE_TAB
    KEY_SPACE as i32,          // SDL_SCANCODE_SPACE
    KEY_MINUS as i32,          // SDL_SCANCODE_MINUS
    KEY_EQUAL as i32,          // SDL_SCANCODE_EQUALS
    KEY_LEFT_BRACKET as i32,   // SDL_SCANCODE_LEFTBRACKET
    KEY_RIGHT_BRACKET as i32,  // SDL_SCANCODE_RIGHTBRACKET
    KEY_BACKSLASH as i32,      // SDL_SCANCODE_BACKSLASH
    0,                  // SDL_SCANCODE_NONUSHASH
    KEY_SEMICOLON as i32,      // SDL_SCANCODE_SEMICOLON
    KEY_APOSTROPHE as i32,     // SDL_SCANCODE_APOSTROPHE
    KEY_GRAVE as i32,          // SDL_SCANCODE_GRAVE
    KEY_COMMA as i32,          // SDL_SCANCODE_COMMA
    KEY_PERIOD as i32,         // SDL_SCANCODE_PERIOD
    KEY_SLASH as i32,          // SDL_SCANCODE_SLASH
    KEY_CAPS_LOCK as i32,      // SDL_SCANCODE_CAPSLOCK
    KEY_F1 as i32,             // SDL_SCANCODE_F1
    KEY_F2 as i32,             // SDL_SCANCODE_F2
    KEY_F3 as i32,             // SDL_SCANCODE_F3
    KEY_F4 as i32,             // SDL_SCANCODE_F4
    KEY_F5 as i32,             // SDL_SCANCODE_F5
    KEY_F6 as i32,             // SDL_SCANCODE_F6
    KEY_F7 as i32,             // SDL_SCANCODE_F7
    KEY_F8 as i32,             // SDL_SCANCODE_F8
    KEY_F9 as i32,             // SDL_SCANCODE_F9
    KEY_F10 as i32,            // SDL_SCANCODE_F10
    KEY_F11 as i32,            // SDL_SCANCODE_F11
    KEY_F12 as i32,            // SDL_SCANCODE_F12
    KEY_PRINT_SCREEN as i32,   // SDL_SCANCODE_PRINTSCREEN
    KEY_SCROLL_LOCK as i32,    // SDL_SCANCODE_SCROLLLOCK
    KEY_PAUSE as i32,          // SDL_SCANCODE_PAUSE
    KEY_INSERT as i32,         // SDL_SCANCODE_INSERT
    KEY_HOME as i32,           // SDL_SCANCODE_HOME
    KEY_PAGE_UP as i32,        // SDL_SCANCODE_PAGEUP
    KEY_DELETE as i32,         // SDL_SCANCODE_DELETE
    KEY_END as i32,            // SDL_SCANCODE_END
    KEY_PAGE_DOWN as i32,      // SDL_SCANCODE_PAGEDOWN
    KEY_RIGHT as i32,          // SDL_SCANCODE_RIGHT
    KEY_LEFT as i32,           // SDL_SCANCODE_LEFT
    KEY_DOWN as i32,           // SDL_SCANCODE_DOWN
    KEY_UP as i32,             // SDL_SCANCODE_UP
    KEY_NUM_LOCK as i32,       // SDL_SCANCODE_NUMLOCKCLEAR
    KEY_KP_DIVIDE as i32,      // SDL_SCANCODE_KP_DIVIDE
    KEY_KP_MULTIPLY as i32,    // SDL_SCANCODE_KP_MULTIPLY
    KEY_KP_SUBTRACT as i32,    // SDL_SCANCODE_KP_MINUS
    KEY_KP_ADD as i32,         // SDL_SCANCODE_KP_PLUS
    KEY_KP_ENTER as i32,       // SDL_SCANCODE_KP_ENTER
    KEY_KP_1 as i32,           // SDL_SCANCODE_KP_1
    KEY_KP_2 as i32,           // SDL_SCANCODE_KP_2
    KEY_KP_3 as i32,           // SDL_SCANCODE_KP_3
    KEY_KP_4 as i32,           // SDL_SCANCODE_KP_4
    KEY_KP_5 as i32,           // SDL_SCANCODE_KP_5
    KEY_KP_6 as i32,           // SDL_SCANCODE_KP_6
    KEY_KP_7 as i32,           // SDL_SCANCODE_KP_7
    KEY_KP_8 as i32,           // SDL_SCANCODE_KP_8
    KEY_KP_9 as i32,           // SDL_SCANCODE_KP_9
    KEY_KP_0 as i32,           // SDL_SCANCODE_KP_0
    KEY_KP_DECIMAL as i32,     // SDL_SCANCODE_KP_PERIOD
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0,
    KEY_LEFT_CONTROL as i32,   //SDL_SCANCODE_LCTRL
    KEY_LEFT_SHIFT as i32,     //SDL_SCANCODE_LSHIFT
    KEY_LEFT_ALT as i32,       //SDL_SCANCODE_LALT
    KEY_LEFT_SUPER as i32,     //SDL_SCANCODE_LGUI
    KEY_RIGHT_CONTROL as i32,  //SDL_SCANCODE_RCTRL
    KEY_RIGHT_SHIFT as i32,    //SDL_SCANCODE_RSHIFT
    KEY_RIGHT_ALT as i32,      //SDL_SCANCODE_RALT
    KEY_RIGHT_SUPER as i32     //SDL_SCANCODE_RGUI
];

static CursorsLUT: [SDL_SystemCursor; 11] = [
    SDL_SYSTEM_CURSOR_DEFAULT,     // 0  MOUSE_CURSOR_DEFAULT
    SDL_SYSTEM_CURSOR_DEFAULT,     // 1  MOUSE_CURSOR_ARROW
    SDL_SYSTEM_CURSOR_TEXT,        // 2  MOUSE_CURSOR_IBEAM
    SDL_SYSTEM_CURSOR_CROSSHAIR,   // 3  MOUSE_CURSOR_CROSSHAIR
    SDL_SYSTEM_CURSOR_POINTER,     // 4  MOUSE_CURSOR_POINTING_HAND
    SDL_SYSTEM_CURSOR_EW_RESIZE,   // 5  MOUSE_CURSOR_RESIZE_EW
    SDL_SYSTEM_CURSOR_NS_RESIZE,   // 6  MOUSE_CURSOR_RESIZE_NS
    SDL_SYSTEM_CURSOR_NWSE_RESIZE, // 7  MOUSE_CURSOR_RESIZE_NWSE
    SDL_SYSTEM_CURSOR_NESW_RESIZE, // 8  MOUSE_CURSOR_RESIZE_NESW
    SDL_SYSTEM_CURSOR_MOVE,        // 9  MOUSE_CURSOR_RESIZE_ALL
    SDL_SYSTEM_CURSOR_NOT_ALLOWED  // 10 MOUSE_CURSOR_NOT_ALLOWED
    //SDL_SYSTEM_CURSOR_WAIT,      // No equivalent implemented on MouseCursor enum on raylib.h
    //SDL_SYSTEM_CURSOR_PROGRESS,  // No equivalent implemented on MouseCursor enum on raylib.h
];

// SDL3 migration layer made to avoid 'ifdefs' inside functions

// SDL3 migration:
// SDL_WINDOW_FULLSCREEN_DESKTOP has been removed,
// SDL_GetWindowFullscreenMode() can be called
// to see whether an exclusive fullscreen mode will be used
// or the borderless fullscreen desktop mode
const SDL_WINDOW_FULLSCREEN_DESKTOP: SDL_WindowFlags = SDL_WINDOW_FULLSCREEN;

const SDL_IGNORE: bool = false;
const SDL_DISABLE: bool = false;
const SDL_ENABLE: bool = true;

// SDL3 Migration: SDL_INIT_TIMER - no longer needed before calling SDL_AddTimer()
const SDL_INIT_TIMER: SDL_InitFlags = SDL_InitFlags(0); // It's a flag, so no problem in setting it to zero to be used in a bitor (|)

// SDL3 Migration: The SDL_WINDOW_SHOWN flag has been removed. Windows are shown by default and can be created hidden by using the SDL_WINDOW_HIDDEN flag
const SDL_WINDOW_SHOWN: SDL_WindowFlags = SDL_WindowFlags(0); // It's a flag, so no problem in setting it to zero to be used in a bitor (|)

// SDL3 Migration: Renamed
// IMPORTANT: Might need to call SDL_CleanupEvent somewhere see :https://github.com/libsdl-org/SDL/issues/3540#issuecomment-1793449852

// SDL2 implementation for SDL3 function
pub unsafe fn SDL_GameControllerNameForIndex(joystickIndex: i32) -> *const std::ffi::c_char
{
    // NOTE: SDL3 uses the IDs itself (SDL_JoystickID) instead of SDL2 joystick_index
    let mut name: *const std::ffi::c_char = std::ptr::null_mut();
    let mut numJoysticks: i32 = 0;
    let joysticks: *mut SDL_JoystickID = SDL_GetJoysticks(&mut numJoysticks);

    if (!joysticks.is_null())
    {
        if (joystickIndex < numJoysticks)
        {
            let instance_id: SDL_JoystickID = *joysticks.add(joystickIndex as usize);
            name = SDL_GetGamepadNameForID(instance_id);
        }

        SDL_free(joysticks.cast());
    }

    return name;
}

pub unsafe fn SDL_GetNumVideoDisplays() -> i32
{
    let mut monitorCount: i32 = 0;
    let displays: *mut SDL_DisplayID = SDL_GetDisplays(&mut monitorCount);

    // Safe because If 'mem' is NULL, SDL_free does nothing
    SDL_free(displays.cast());

    return monitorCount;
}

// SLD3 Migration: To emulate SDL2 this function should return 'SDL_DISABLE' or 'SDL_ENABLE'
// representing the *processing state* of the event before this function makes any changes to it
pub unsafe fn SDL_EventState(r#type: SDL_EventType, state: i32) -> u8
{
    let stateBefore: u8 = SDL_EventEnabled(r#type.0) as u8;

    match state
    {
        state if state == SDL_DISABLE as i32 => { SDL_SetEventEnabled(r#type.0, false); },
        state if state == SDL_ENABLE as i32 => { SDL_SetEventEnabled(r#type.0, true); },
        _ => { warn!("SDL: Event state of unknow type"); },
    }

    return stateBefore;
}

pub unsafe fn SDL_GetCurrentDisplayMode_Adapter(displayID: SDL_DisplayID, mode: *mut SDL_DisplayMode)
{
    let currentMode: *const SDL_DisplayMode = sdl3_sys::video::SDL_GetCurrentDisplayMode(displayID);

    if currentMode.is_null() { warn!("SDL: No possible to get current display mode"); }
    else { *mode = std::ptr::read(currentMode); }
}

// SDL3 Migration: Renamed
use self::SDL_GetCurrentDisplayMode_Adapter as SDL_GetCurrentDisplayMode;

pub unsafe fn SDL_CreateRGBSurface(flags: u32, width: i32, height: i32, depth: i32, Rmask: u32, Gmask: u32, Bmask: u32, Amask: u32) -> *mut SDL_Surface
{
    return SDL_CreateSurface(width, height, SDL_GetPixelFormatForMasks(depth, Rmask, Gmask, Bmask, Amask));
}

// SDL3 Migration:
// SDL_GetDisplayDPI() not reliable across platforms, approximately replaced by multiplying
// SDL_GetWindowDisplayScale() times 160 on iPhone and Android, and 96 on other platforms
// returns 0 on success or a negative error code on failure
pub unsafe fn SDL_GetDisplayDPI(displayIndex: i32, ddpi: *mut f32, hdpi: *mut f32, vdpi: *mut f32) -> i32
{
    let dpi: f32 = SDL_GetWindowDisplayScale(platform.window)*96.0;

    if !ddpi.is_null() { *ddpi = dpi; }
    if !hdpi.is_null() { *hdpi = dpi; }
    if !vdpi.is_null() { *vdpi = dpi; }

    return 0;
}

pub unsafe fn SDL_CreateRGBSurfaceWithFormat(flags: u32, width: i32, height: i32, depth: i32, format: u32) -> *mut SDL_Surface
{
    return SDL_CreateSurface(width, height, SDL_PixelFormat(format as i32));
}

pub unsafe fn SDL_CreateRGBSurfaceFrom(pixels: *mut std::ffi::c_void, width: i32, height: i32, depth: i32, pitch: i32, Rmask: u32, Gmask: u32, Bmask: u32, Amask: u32) -> *mut SDL_Surface
{
    return SDL_CreateSurfaceFrom(width, height, SDL_GetPixelFormatForMasks(depth, Rmask, Gmask, Bmask, Amask), pixels, pitch);
}

pub unsafe fn SDL_CreateRGBSurfaceWithFormatFrom(pixels: *mut std::ffi::c_void, width: i32, height: i32, depth: i32, pitch: i32, format: u32) -> *mut SDL_Surface
{
    return SDL_CreateSurfaceFrom(width, height, SDL_PixelFormat(format as i32), pixels, pitch);
}

pub unsafe fn SDL_NumJoysticks() -> i32
{
    let mut numJoysticks: i32 = 0;
    let joysticks: *mut SDL_JoystickID = SDL_GetJoysticks(&mut numJoysticks);
    SDL_free(joysticks.cast());
    return numJoysticks;
}

// SDL_SetRelativeMouseMode
// returns 0 on success or a negative error code on failure
// If relative mode is not supported, this returns -1
pub unsafe fn SDL_SetRelativeMouseMode_Adapter(enabled: bool) -> i32
{
    // SDL_SetWindowRelativeMouseMode(SDL_Window *window, bool enabled)
    // \returns true on success or false on failure; call SDL_GetError() for more
    if (SDL_SetWindowRelativeMouseMode(platform.window, enabled))
    {
        return 0; // success
    }
    else
    {
        return -1; // failure
    }
}

use self::SDL_SetRelativeMouseMode_Adapter as SDL_SetRelativeMouseMode;

pub unsafe fn SDL_GetRelativeMouseMode_Adapter() -> bool
{
    return SDL_GetWindowRelativeMouseMode(platform.window);
}


pub unsafe fn SDL_GetNumTouchFingers(touchID: SDL_TouchID) -> i32
{
    // SDL_Finger **SDL_GetTouchFingers(SDL_TouchID touchID, int *count)
    let mut count: i32 = 0;
    let fingers: *mut *mut SDL_Finger = SDL_GetTouchFingers(touchID, &mut count);
    SDL_free(fingers.cast());
    return count;
}


//----------------------------------------------------------------------------------
// Module Internal Functions Declaration
//----------------------------------------------------------------------------------
                                      // Initialize platform (graphics, inputs and more)
                                    // Close platform

  // Help convert SDL scancodes to raylib key
 // Get next codepoint in a byte sequence and bytes processed
 // Update CORE input touch point info from SDL touch data

//----------------------------------------------------------------------------------
// Module Functions Declaration
//----------------------------------------------------------------------------------
// NOTE: Functions declaration is provided by raylib.h

//----------------------------------------------------------------------------------
// Module Functions Definition: Window and Graphics Device
//----------------------------------------------------------------------------------

// Check if application should close
pub unsafe fn WindowShouldClose() -> bool
{
    if (CORE.Window.ready) { return CORE.Window.shouldClose; }
    else { return true; }
}

// Toggle fullscreen mode
pub unsafe fn ToggleFullscreen()
{
    let monitor= SDL_GetDisplayForWindow(platform.window);

    if (SDL_GetDisplayProperties(monitor) != 0) // Returns 0 on failure, so a value other than zero indicates that the monitor id is valid
    {
        if (((CORE.Window.flags & FLAG_FULLSCREEN_MODE as u32) != 0))
        {
            SDL_SetWindowFullscreen(platform.window, false);
            CORE.Window.flags &= !(FLAG_FULLSCREEN_MODE as u32);
        }
        else
        {
            SDL_SetWindowFullscreen(platform.window, SDL_WINDOW_FULLSCREEN != SDL_WindowFlags(0));
            CORE.Window.flags |= FLAG_FULLSCREEN_MODE as u32;
        }
    }
    else { warn!("SDL: Failed to find selected monitor"); }
}

// Toggle borderless windowed mode
pub unsafe fn ToggleBorderlessWindowed()
{
    let monitor= SDL_GetDisplayForWindow(platform.window);

    if (SDL_GetDisplayProperties(monitor) != 0) // Returns 0 on failure, so a value other than zero indicates that the monitor id is valid
    {
        if (((CORE.Window.flags & FLAG_BORDERLESS_WINDOWED_MODE as u32) != 0))
        {
            SDL_SetWindowFullscreen(platform.window, false);
            CORE.Window.flags &= !(FLAG_BORDERLESS_WINDOWED_MODE as u32);
        }
        else
        {
            SDL_SetWindowFullscreen(platform.window, SDL_WINDOW_FULLSCREEN_DESKTOP != SDL_WindowFlags(0));
            CORE.Window.flags |= FLAG_BORDERLESS_WINDOWED_MODE as u32;
        }
    }
    else { warn!("SDL: Failed to find selected monitor"); }
}

// Set window state: maximized, if resizable
pub unsafe fn MaximizeWindow()
{
    SDL_MaximizeWindow(platform.window);
    if (!((CORE.Window.flags & FLAG_WINDOW_MAXIMIZED as u32) != 0)) { CORE.Window.flags |= FLAG_WINDOW_MAXIMIZED as u32; }
}

// Set window state: minimized
pub unsafe fn MinimizeWindow()
{
    SDL_MinimizeWindow(platform.window);
    if (!((CORE.Window.flags & FLAG_WINDOW_MINIMIZED as u32) != 0)) { CORE.Window.flags |= FLAG_WINDOW_MINIMIZED as u32; }
}

// Restore window from being minimized/maximized
pub unsafe fn RestoreWindow()
{
    SDL_RestoreWindow(platform.window);
    // CORE.Window.flags will be removed on PollInputEvents()
}

// Set window configuration state using flags
pub unsafe fn SetWindowState(flags: u32)
{
    if (!CORE.Window.ready) { warn!("WINDOW: SetWindowState does nothing before window initialization, Use \"SetConfigFlags\" instead"); }

    CORE.Window.flags |= flags;

    if (((flags & FLAG_VSYNC_HINT as u32) != 0))
    {
        SDL_GL_SetSwapInterval(1);
    }
    if (((flags & FLAG_FULLSCREEN_MODE as u32) != 0))
    {
        let monitor= SDL_GetDisplayForWindow(platform.window);

        if (SDL_GetDisplayProperties(monitor) != 0) // Returns 0 on failure, so a value other than zero indicates that the monitor id is valid
        {
            SDL_SetWindowFullscreen(platform.window, SDL_WINDOW_FULLSCREEN != SDL_WindowFlags(0));
            CORE.Window.flags |= FLAG_FULLSCREEN_MODE as u32;
        }
        else { warn!("SDL: Failed to find selected monitor"); }
    }
    if (((flags & FLAG_WINDOW_RESIZABLE as u32) != 0))
    {
        SDL_SetWindowResizable(platform.window, true);
    }
    if (((flags & FLAG_WINDOW_UNDECORATED as u32) != 0))
    {
        SDL_SetWindowBordered(platform.window, false);
    }
    if (((flags & FLAG_WINDOW_HIDDEN as u32) != 0))
    {
        SDL_HideWindow(platform.window);
    }
    if (((flags & FLAG_WINDOW_MINIMIZED as u32) != 0))
    {
        SDL_MinimizeWindow(platform.window);
    }
    if (((flags & FLAG_WINDOW_MAXIMIZED as u32) != 0))
    {
        SDL_MaximizeWindow(platform.window);
    }
    if (((flags & FLAG_WINDOW_UNFOCUSED as u32) != 0))
    {
        warn!("SetWindowState() - FLAG_WINDOW_UNFOCUSED is not supported on PLATFORM_DESKTOP_SDL");
    }
    if (((flags & FLAG_WINDOW_TOPMOST as u32) != 0))
    {
        SDL_SetWindowAlwaysOnTop(platform.window, false);
    }
    if (((flags & FLAG_WINDOW_ALWAYS_RUN as u32) != 0))
    {
        CORE.Window.flags |= FLAG_WINDOW_ALWAYS_RUN as u32;
    }
    if (((flags & FLAG_WINDOW_TRANSPARENT as u32) != 0))
    {
        warn!("SetWindowState() - FLAG_WINDOW_TRANSPARENT is not supported on PLATFORM_DESKTOP_SDL");
    }
    if (((flags & FLAG_WINDOW_HIGHDPI as u32) != 0))
    {
        // NOTE: Such a function does not seem to exist
        warn!("SetWindowState() - FLAG_WINDOW_HIGHDPI is not supported on PLATFORM_DESKTOP_SDL");
    }
    if (((flags & FLAG_WINDOW_MOUSE_PASSTHROUGH as u32) != 0))
    {
        //SDL_SetWindowGrab(platform.window, false);
        warn!("SetWindowState() - FLAG_WINDOW_MOUSE_PASSTHROUGH is not supported on PLATFORM_DESKTOP_SDL");
    }
    if (((flags & FLAG_BORDERLESS_WINDOWED_MODE as u32) != 0))
    {
        let monitor= SDL_GetDisplayForWindow(platform.window);

        if (SDL_GetDisplayProperties(monitor) != 0) // Returns 0 on failure, so a value other than zero indicates that the monitor id is valid
        {
            SDL_SetWindowFullscreen(platform.window, SDL_WINDOW_FULLSCREEN_DESKTOP != SDL_WindowFlags(0));
        }
        else { warn!("SDL: Failed to find selected monitor"); }
    }
    if (((flags & FLAG_MSAA_4X_HINT as u32) != 0))
    {
        SDL_GL_SetAttribute(SDL_GL_MULTISAMPLEBUFFERS, 1); // Enable multisampling buffers
        SDL_GL_SetAttribute(SDL_GL_MULTISAMPLESAMPLES, 4); // Enable multisampling
    }
    if (((flags & FLAG_INTERLACED_HINT as u32) != 0))
    {
        warn!("SetWindowState() - FLAG_INTERLACED_HINT is not supported on PLATFORM_DESKTOP_SDL");
    }
}

// Clear window configuration state flags
pub unsafe fn ClearWindowState(flags: u32)
{
    CORE.Window.flags &= !flags;

    if (((flags & FLAG_VSYNC_HINT as u32) != 0))
    {
        SDL_GL_SetSwapInterval(0);
    }
    if (((flags & FLAG_FULLSCREEN_MODE as u32) != 0))
    {
        SDL_SetWindowFullscreen(platform.window, false);
    }
    if (((flags & FLAG_WINDOW_RESIZABLE as u32) != 0))
    {
        SDL_SetWindowResizable(platform.window, false);
    }
    if (((flags & FLAG_WINDOW_UNDECORATED as u32) != 0))
    {
        SDL_SetWindowBordered(platform.window, true);
    }
    if (((flags & FLAG_WINDOW_HIDDEN as u32) != 0))
    {
        SDL_ShowWindow(platform.window);
    }
    if (((flags & FLAG_WINDOW_MINIMIZED as u32) != 0))
    {
        SDL_RestoreWindow(platform.window);
    }
    if (((flags & FLAG_WINDOW_MAXIMIZED as u32) != 0))
    {
        SDL_RestoreWindow(platform.window);
    }
    if (((flags & FLAG_WINDOW_UNFOCUSED as u32) != 0))
    {
        //SDL_RaiseWindow(platform.window);
        warn!("ClearWindowState() - FLAG_WINDOW_UNFOCUSED is not supported on PLATFORM_DESKTOP_SDL");
    }
    if (((flags & FLAG_WINDOW_TOPMOST as u32) != 0))
    {
        SDL_SetWindowAlwaysOnTop(platform.window, false);
    }
    if (((flags & FLAG_WINDOW_TRANSPARENT as u32) != 0))
    {
        warn!("ClearWindowState() - FLAG_WINDOW_TRANSPARENT is not supported on PLATFORM_DESKTOP_SDL");
    }
    if (((flags & FLAG_WINDOW_HIGHDPI as u32) != 0))
    {
        // NOTE: There also doesn't seem to be a feature to disable high DPI once enabled
        warn!("ClearWindowState() - FLAG_WINDOW_HIGHDPI is not supported on PLATFORM_DESKTOP_SDL");
    }
    if (((flags & FLAG_WINDOW_MOUSE_PASSTHROUGH as u32) != 0))
    {
        //SDL_SetWindowGrab(platform.window, true);
        warn!("ClearWindowState() - FLAG_WINDOW_MOUSE_PASSTHROUGH is not supported on PLATFORM_DESKTOP_SDL");
    }
    if (((flags & FLAG_BORDERLESS_WINDOWED_MODE as u32) != 0))
    {
        SDL_SetWindowFullscreen(platform.window, false);
    }
    if (((flags & FLAG_MSAA_4X_HINT as u32) != 0))
    {
        SDL_GL_SetAttribute(SDL_GL_MULTISAMPLEBUFFERS, 0); // Disable multisampling buffers
        SDL_GL_SetAttribute(SDL_GL_MULTISAMPLESAMPLES, 0); // Disable multisampling
    }
    if (((flags & FLAG_INTERLACED_HINT as u32) != 0))
    {
        warn!("ClearWindowState() - FLAG_INTERLACED_HINT is not supported on PLATFORM_DESKTOP_SDL");
    }
}

// Set icon for window
pub unsafe fn SetWindowIcon(image: Image)
{
    let mut iconSurface: *mut SDL_Surface = std::ptr::null_mut();

    let mut rmask: u32 = 0; let mut gmask: u32 = 0; let mut bmask: u32 = 0; let mut amask: u32 = 0;
    let mut depth: i32 = 0;  // Depth in bits
    let mut pitch: i32 = 0;  // Pixel spacing (pitch) in bytes

    match image.format
    {
        format if format == PIXELFORMAT_UNCOMPRESSED_GRAYSCALE as i32 => {
            rmask = 0xFF; gmask = 0;
            bmask = 0; amask = 0;
            depth = 8; pitch = image.width;
        },
        format if format == PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA as i32 => {
            rmask = 0xFF; gmask = 0xFF00;
            bmask = 0; amask = 0;
            depth = 16; pitch = image.width*2;
        },
        format if format == PIXELFORMAT_UNCOMPRESSED_R5G6B5 as i32 => {
            rmask = 0xF800; gmask = 0x07E0;
            bmask = 0x001F; amask = 0;
            depth = 16; pitch = image.width*2;
        },
        format if format == PIXELFORMAT_UNCOMPRESSED_R8G8B8 as i32 => {
            // WARNING: SDL2 could be using BGR but SDL3 RGB
            rmask = 0xFF0000; gmask = 0x00FF00;
            bmask = 0x0000FF; amask = 0;
            depth = 24; pitch = image.width*3;
        },
        format if format == PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 as i32 => {
            rmask = 0xF800; gmask = 0x07C0;
            bmask = 0x003E; amask = 0x0001;
            depth = 16; pitch = image.width*2;
        },
        format if format == PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 as i32 => {
            rmask = 0xF000; gmask = 0x0F00;
            bmask = 0x00F0; amask = 0x000F;
            depth = 16; pitch = image.width*2;
        },
        format if format == PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32 => {
            rmask = 0xFF000000; gmask = 0x00FF0000;
            bmask = 0x0000FF00; amask = 0x000000FF;
            depth = 32; pitch = image.width*4;
        },
        format if format == PIXELFORMAT_UNCOMPRESSED_R32 as i32 => {
            rmask = 0xFFFFFFFF; gmask = 0;
            bmask = 0; amask = 0;
            depth = 32; pitch = image.width*4;
        },
        format if format == PIXELFORMAT_UNCOMPRESSED_R32G32B32 as i32 => {
            rmask = 0xFFFFFFFF; gmask = 0xFFFFFFFF;
            bmask = 0xFFFFFFFF; amask = 0;
            depth = 96; pitch = image.width*12;
        },
        format if format == PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 as i32 => {
            rmask = 0xFFFFFFFF; gmask = 0xFFFFFFFF;
            bmask = 0xFFFFFFFF; amask = 0xFFFFFFFF;
            depth = 128; pitch = image.width*16;
        },
        format if format == PIXELFORMAT_UNCOMPRESSED_R16 as i32 => {
            rmask = 0xFFFF; gmask = 0;
            bmask = 0; amask = 0;
            depth = 16; pitch = image.width*2;
        },
        format if format == PIXELFORMAT_UNCOMPRESSED_R16G16B16 as i32 => {
            rmask = 0xFFFF; gmask = 0xFFFF;
            bmask = 0xFFFF; amask = 0;
            depth = 48; pitch = image.width*6;
        },
        format if format == PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 as i32 => {
            rmask = 0xFFFF; gmask = 0xFFFF;
            bmask = 0xFFFF; amask = 0xFFFF;
            depth = 64; pitch = image.width*8;
        },
        _ => { return; }, // Compressed formats are not supported
    }

    iconSurface = SDL_CreateRGBSurfaceFrom(image.data, image.width, image.height, depth, pitch, rmask, gmask, bmask, amask);

    if (!iconSurface.is_null())
    {
        SDL_SetWindowIcon(platform.window, iconSurface);
        SDL_DestroySurface(iconSurface);
    }
}

// Set icon for window
pub unsafe fn SetWindowIcons(images: &[Image], count: i32)
{
    warn!("SetWindowIcons() not available on target platform");
}

// Set title for window
pub unsafe fn SetWindowTitle(title: &str)
{
    let title = CString::new(title).expect("window title contains a NUL byte").into_raw();
    SDL_SetWindowTitle(platform.window, title);

    CORE.Window.title = title;
}

// Set window position on screen (windowed mode)
pub unsafe fn SetWindowPosition(x: i32, y: i32)
{
    SDL_SetWindowPosition(platform.window, x, y);

    CORE.Window.position.x = (x) as f32;
    CORE.Window.position.y = (y) as f32;
}

// Set monitor for the current window
pub unsafe fn SetWindowMonitor(monitor: i32)
{
    if (SDL_GetDisplayProperties(SDL_DisplayID(monitor as u32)) != 0) // Returns 0 on failure, so a value other than zero indicates that the monitor id is valid
    {
        // NOTE 1: SDL started supporting moving exclusive fullscreen windows between displays on SDL3,
        // see commit https://github.com/libsdl-org/SDL/commit/3f5ef7dd422057edbcf3e736107e34be4b75d9ba
        // NOTE 2: A workaround for SDL2 is leaving fullscreen, moving the window, then entering full screen again
        let wasFullscreen: bool = (CORE.Window.flags & FLAG_FULLSCREEN_MODE as u32) != 0;

        let screenWidth: i32 = CORE.Window.screen.x as i32;
        let screenHeight: i32 = CORE.Window.screen.y as i32;
        let mut usableBounds: SDL_Rect = std::mem::zeroed();

        if (SDL_GetDisplayUsableBounds(SDL_DisplayID(monitor as u32), &mut usableBounds))
        {
            if (wasFullscreen) { ToggleFullscreen(); } // Leave fullscreen

            // If the screen size is larger than the monitor usable area, anchor it on the top left corner, otherwise, center it
            if ((screenWidth >= usableBounds.w) || (screenHeight >= usableBounds.h))
            {
                // NOTE 1: There's a known issue where if the window larger than the target display bounds,
                // when moving the windows to that display, the window could be clipped back
                // ending up positioned partly outside the target display
                // NOTE 2: The workaround for that is, previously to moving the window,
                // setting the window size to the target display size, so they match
                // NOTE 3: It wasn't done here because it can not be assumed that changing
                // the window size automatically is acceptable behavior by the user
                SDL_SetWindowPosition(platform.window, usableBounds.x, usableBounds.y);
                CORE.Window.position.x = (usableBounds.x) as f32;
                CORE.Window.position.y = (usableBounds.y) as f32;
            }
            else
            {
                let x: i32 = usableBounds.x + (usableBounds.w/2) - (screenWidth/2);
                let y: i32 = usableBounds.y + (usableBounds.h/2) - (screenHeight/2);
                SDL_SetWindowPosition(platform.window, x, y);
                CORE.Window.position.x = (x) as f32;
                CORE.Window.position.y = (y) as f32;
            }

            if (wasFullscreen) { ToggleFullscreen(); } // Re-enter fullscreen
        }
        else { warn!("SDL: Failed to get selected display usable bounds"); }
    }
    else { warn!("SDL: Failed to find selected monitor"); }
}

// Set window minimum dimensions (FLAG_WINDOW_RESIZABLE)
pub unsafe fn SetWindowMinSize(width: i32, height: i32)
{
    SDL_SetWindowMinimumSize(platform.window, width, height);

    CORE.Window.screenMin.x = (width) as f32;
    CORE.Window.screenMin.y = (height) as f32;
}

// Set window maximum dimensions (FLAG_WINDOW_RESIZABLE)
pub unsafe fn SetWindowMaxSize(width: i32, height: i32)
{
    SDL_SetWindowMaximumSize(platform.window, width, height);

    CORE.Window.screenMax.x = (width) as f32;
    CORE.Window.screenMax.y = (height) as f32;
}

// Set window dimensions
pub unsafe fn SetWindowSize(width: i32, height: i32)
{
    SDL_SetWindowSize(platform.window, width, height);

    CORE.Window.screen.x = (width) as f32;
    CORE.Window.screen.y = (height) as f32;
}

// Set window opacity, value opacity is between 0.0 and 1.0
pub unsafe fn SetWindowOpacity(mut opacity: f32)
{
    if (opacity >= 1.0) { opacity = 1.0; }
    else if (opacity <= 0.0) { opacity = 0.0; }

    SDL_SetWindowOpacity(platform.window, opacity);
}

// Set window focused
pub unsafe fn SetWindowFocused()
{
    SDL_RaiseWindow(platform.window);
}

// Get native window handle
// NOTE: Handle type depends on OS and windowing system
pub unsafe fn GetWindowHandle() -> *mut std::ffi::c_void
{
    let mut handle: *mut std::ffi::c_void = std::ptr::null_mut();

    // REF: https://github.com/libsdl-org/SDL/blob/main/include/SDL3/SDL_video.h#L1590
    let props: SDL_PropertiesID = SDL_GetWindowProperties(platform.window);
    #[cfg(target_os = "windows")]
    {
    handle = (SDL_GetPointerProperty(props, SDL_PROP_WINDOW_WIN32_HWND_POINTER, std::ptr::null_mut()) as *mut std::ffi::c_void); // Type: HWND
    }
    #[cfg(target_os = "linux")]
    {
    let windowId: std::os::raw::c_ulong = (SDL_GetNumberProperty(props, SDL_PROP_WINDOW_X11_WINDOW_NUMBER, 0) as std::os::raw::c_ulong); // Type: unsigned long (XID, Window)
    if (windowId != 0)
    {
        // X11 window ID
        platform.windowHandleX11 = windowId;
        handle = (&raw mut platform.windowHandleX11).cast();
    }
    else
    {
        // Wayland, get display surface pointer
        // NOTE: Alternative SDL_PROP_WINDOW_WAYLAND_DISPLAY_POINTER
        handle = (SDL_GetPointerProperty(props, SDL_PROP_WINDOW_WAYLAND_SURFACE_POINTER, std::ptr::null_mut()) as *mut std::ffi::c_void); // Type: struct wl_surface*
    }
    }
    #[cfg(target_vendor = "apple")]
    {
    handle = (SDL_GetPointerProperty(props, SDL_PROP_WINDOW_COCOA_WINDOW_POINTER, std::ptr::null_mut()) as *mut std::ffi::c_void); // Type: NSWindow*
    }

    return handle;
}

// Get number of monitors
pub unsafe fn GetMonitorCount() -> i32
{
    return SDL_GetNumVideoDisplays();
}

// Get current monitor where window is placed
pub unsafe fn GetCurrentMonitor() -> i32
{
    // Be aware that this returns an ID in SDL3 and a Index in SDL2
    let res = SDL_GetDisplayForWindow(platform.window);
    return res.value() as i32;
}

// Get selected monitor position
pub unsafe fn GetMonitorPosition(monitor: i32) -> Vector2
{
    if (SDL_GetDisplayProperties(SDL_DisplayID(monitor as u32)) != 0) // Returns 0 on failure, so a value other than zero indicates that the monitor id is valid
    {
        let mut displayBounds: SDL_Rect = std::mem::zeroed();

        if (SDL_GetDisplayUsableBounds(SDL_DisplayID(monitor as u32), &mut displayBounds))
        {
            return Vector2::new((displayBounds.x as f32), (displayBounds.y as f32));
        }
        else { warn!("SDL: Failed to get selected display usable bounds"); }
    }
    else { warn!("SDL: Failed to find selected monitor"); }
    return Vector2::new(0.0, 0.0);
}

// Get selected monitor width (currently used by monitor)
pub unsafe fn GetMonitorWidth(monitor: i32) -> i32
{
    let mut width: i32 = 0;

    if (SDL_GetDisplayProperties(SDL_DisplayID(monitor as u32)) != 0) // Returns 0 on failure, so a value other than zero indicates that the monitor id is valid
    {
        let mut mode: SDL_DisplayMode = std::mem::zeroed();
        SDL_GetCurrentDisplayMode(SDL_DisplayID(monitor as u32), &mut mode);
        width = mode.w;
    }
    else { warn!("SDL: Failed to find selected monitor"); }

    return width;
}

// Get selected monitor height (currently used by monitor)
pub unsafe fn GetMonitorHeight(monitor: i32) -> i32
{
    let mut height: i32 = 0;

    if (SDL_GetDisplayProperties(SDL_DisplayID(monitor as u32)) != 0) // Returns 0 on failure, so a value other than zero indicates that the monitor id is valid
    {
        let mut mode: SDL_DisplayMode = std::mem::zeroed();
        SDL_GetCurrentDisplayMode(SDL_DisplayID(monitor as u32), &mut mode);
        height = mode.h;
    }
    else { warn!("SDL: Failed to find selected monitor"); }

    return height;
}

// Get selected monitor physical width in millimetres
pub unsafe fn GetMonitorPhysicalWidth(monitor: i32) -> i32
{
    let mut width: i32 = 0;

    if (SDL_GetDisplayProperties(SDL_DisplayID(monitor as u32)) != 0) // Returns 0 on failure, so a value other than zero indicates that the monitor id is valid
    {
        let mut ddpi: f32 = 0.0;
        SDL_GetDisplayDPI(monitor, &mut ddpi, std::ptr::null_mut(), std::ptr::null_mut());
        let mut mode: SDL_DisplayMode = std::mem::zeroed();
        SDL_GetCurrentDisplayMode(SDL_DisplayID(monitor as u32), &mut mode);
        // Calculate size on inches, then convert to millimeter
        if (ddpi > 0.0) { width = (((mode.w as f32/ddpi)*25.4) as i32); }
    }
    else { warn!("SDL: Failed to find selected monitor"); }

    return width;
}

// Get selected monitor physical height in millimetres
pub unsafe fn GetMonitorPhysicalHeight(monitor: i32) -> i32
{
    let mut height: i32 = 0;

    if (SDL_GetDisplayProperties(SDL_DisplayID(monitor as u32)) != 0) // Returns 0 on failure, so a value other than zero indicates that the monitor id is valid
    {
        let mut ddpi: f32 = 0.0;
        SDL_GetDisplayDPI(monitor, &mut ddpi, std::ptr::null_mut(), std::ptr::null_mut());
        let mut mode: SDL_DisplayMode = std::mem::zeroed();
        SDL_GetCurrentDisplayMode(SDL_DisplayID(monitor as u32), &mut mode);
        // Calculate size on inches, then convert to millimeter
        if (ddpi > 0.0) { height = (((mode.h as f32/ddpi)*25.4) as i32); }
    }
    else { warn!("SDL: Failed to find selected monitor"); }

    return height;
}

// Get selected monitor refresh rate
pub unsafe fn GetMonitorRefreshRate(monitor: i32) -> i32
{
    let mut refresh: i32 = 0;

    if (SDL_GetDisplayProperties(SDL_DisplayID(monitor as u32)) != 0) // Returns 0 on failure, so a value other than zero indicates that the monitor id is valid
    {
        let mut mode: SDL_DisplayMode = std::mem::zeroed();
        SDL_GetCurrentDisplayMode(SDL_DisplayID(monitor as u32), &mut mode);
        refresh = mode.refresh_rate as i32;
    }
    else { warn!("SDL: Failed to find selected monitor"); }

    return refresh;
}

// Get the human-readable, UTF-8 encoded name of the selected monitor
pub unsafe fn GetMonitorName(monitor: i32) -> String
{
    if (SDL_GetDisplayProperties(SDL_DisplayID(monitor as u32)) != 0) // Returns 0 on failure, so a value other than zero indicates that the monitor id is valid
    {
        return CStr::from_ptr(SDL_GetDisplayName(SDL_DisplayID(monitor as u32))).to_string_lossy().into_owned();
    }
    else { warn!("SDL: Failed to find selected monitor"); }

    return String::new();
}

// Get window position XY on monitor
pub unsafe fn GetWindowPosition() -> Vector2
{
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    SDL_GetWindowPosition(platform.window, &mut x, &mut y);

    return Vector2::new((x as f32), (y as f32));
}

// Get window scale DPI factor for current monitor
pub unsafe fn GetWindowScaleDPI() -> Vector2
{
    let mut scale: Vector2 = Vector2::new(1.0, 1.0);

    // NOTE: SDL_GetWindowDisplayScale added on SDL3
    // REF: https://wiki.libsdl.org/SDL3/SDL_GetWindowDisplayScale
    scale.x = SDL_GetWindowDisplayScale(platform.window);
    scale.y = scale.x;

    return scale;
}

// Set clipboard text content
pub unsafe fn SetClipboardText(text: &str)
{
    let text = CString::new(text).expect("clipboard text contains a NUL byte");
    SDL_SetClipboardText(text.as_ptr());
}

// Get clipboard text content
pub unsafe fn GetClipboardText() -> String {
    let clipboard = unsafe { SDL_GetClipboardText() };
    if clipboard.is_null() {
        return String::new();
    }

    let bytes = unsafe { CStr::from_ptr(clipboard) }.to_bytes();

    let text = if bytes.len() >= MAX_CLIPBOARD_BUFFER_LENGTH {
        let mut text =
            String::from_utf8_lossy(&bytes[..MAX_CLIPBOARD_BUFFER_LENGTH - 4])
                .into_owned();
        text.push_str("...");
        text
    } else {
        String::from_utf8_lossy(bytes).into_owned()
    };

    unsafe { SDL_free(clipboard.cast()) };
    text
}

// Get clipboard image
pub unsafe fn GetClipboardImage() -> Image
{
    let image: Image = std::mem::zeroed();

#[cfg(feature = "SUPPORT_CLIPBOARD_IMAGE")]
{
#[cfg(not(feature = "SUPPORT_MODULE_RTEXTURES"))]
{
    warn!("Enabling SUPPORT_CLIPBOARD_IMAGE requires SUPPORT_MODULE_RTEXTURES to work properly");
    return image;
}

// It's nice to have support Bitmap on Linux as well, but not as necessary as Windows
#[cfg(all(not(feature = "SUPPORT_FILEFORMAT_BMP"), target_os = "windows"))]
{
    warn!("WARNING: Enabling SUPPORT_CLIPBOARD_IMAGE requires SUPPORT_FILEFORMAT_BMP, specially on Windows");
    return image;
}

// From what I've tested applications on Wayland saves images on clipboard as PNG
#[cfg(all(any(not(feature = "SUPPORT_FILEFORMAT_PNG"), not(feature = "SUPPORT_FILEFORMAT_JPG")), not(target_os = "windows")))]
{
    warn!("WARNING: Getting image from the clipboard might not work without SUPPORT_FILEFORMAT_PNG or SUPPORT_FILEFORMAT_JPG");
}
    // Let's hope compiler put these arrays in static memory
    let mut imageFormats: [&str; 4] = [
        "image/bmp",
        "image/png",
        "image/jpg",
        "image/tiff",
    ];
    let mut imageExtensions: [&str; 4] = [
        ".bmp",
        ".png",
        ".jpg",
        ".tiff",
    ];

    let mut dataSize: usize = 0;
    let mut fileData: *mut std::ffi::c_void = std::ptr::null_mut();

    for i in (0) as usize..(imageFormats.len()) as usize
    {
        fileData = SDL_GetClipboardData(CString::new(imageFormats[i]).unwrap().as_ptr(), &mut dataSize);

        if (!fileData.is_null())
        {
            image = LoadImageFromMemory(imageExtensions[i], fileData, (dataSize as i32));

            SDL_free(fileData.cast());

            if (IsImageValid(image))
            {
                info!("Clipboard: Got image from clipboard successfully: {}", imageExtensions[i]);
                return image;
            }
        }
    }

    if (!IsImageValid(image)) { warn!("Clipboard: Couldn't get clipboard data. ERROR: {}", CStr::from_ptr(SDL_GetError()).to_string_lossy()); }
} // SUPPORT_CLIPBOARD_IMAGE

    return image;
}

// Show mouse cursor
pub unsafe fn ShowCursor()
{
    SDL_ShowCursor();
    CORE.Input.Mouse.cursorHidden = false;
}

// Hide mouse cursor
pub unsafe fn HideCursor()
{
    SDL_HideCursor();
    CORE.Input.Mouse.cursorHidden = true;
}

// Enable cursor (unlock cursor)
pub unsafe fn EnableCursor()
{
    SDL_SetRelativeMouseMode(false);

    ShowCursor();
    CORE.Input.Mouse.cursorLocked = false;
}

// Disable cursor (lock cursor)
pub unsafe fn DisableCursor()
{
    SDL_SetRelativeMouseMode(true);

    HideCursor();
    CORE.Input.Mouse.cursorLocked = true;
}

// Swap back buffer with front buffer (screen drawing)
pub unsafe fn SwapScreenBuffer()
{
    SDL_GL_SwapWindow(platform.window);
}

//----------------------------------------------------------------------------------
// Module Functions Definition: Misc
//----------------------------------------------------------------------------------

// Get elapsed time measure in seconds
pub unsafe fn GetTime() -> f64
{
    let time: f64 = ((SDL_GetPerformanceCounter() as f64)/(SDL_GetPerformanceFrequency() as f64)) - (CORE.Time.base as f64)/(SDL_GetPerformanceFrequency() as f64);

    return time;
}

// Open URL with default system browser (if available)
// WARNING: This function is only safe to use if you control the URL given,
// a user could craft a malicious string to perform and undesired action
// NOTE: Some safety checks have been added to mitigate security issues
pub unsafe fn OpenURL(url: &str)
{
    // Security check to (partially) avoid malicious code
    if (url.contains('\'') || url.contains('"'))
    {
        // Filter characters: ' and "
        warn!("SYSTEM: Provided URL could be potentially malicious, avoid ['\"] characters");
    }
    else if (!url.starts_with("http://") && !url.starts_with("https://"))
    {
        // Only allow URL starting with "http://" or "https://" protocols
        warn!("SYSTEM: Provided URL must start with 'http://' or 'https://' protocols");
    }
    else { SDL_OpenURL(CString::new(url).expect("URL contains a NUL byte").as_ptr()); }
}

//----------------------------------------------------------------------------------
// Module Functions Definition: Inputs
//----------------------------------------------------------------------------------

// Set internal gamepad mappings
// Set internal gamepad mappings
pub fn SetGamepadMappings(mappings: &str) -> i32
{
    let mut succeed = true;

    for mapping in mappings.split('\n').filter(|mapping| !mapping.is_empty())
    {
        let mapping = match CString::new(mapping)
        {
            Ok(mapping) => mapping,
            Err(_) =>
            {
                succeed = false;
                continue;
            }
        };

        if unsafe { SDL_AddGamepadMapping(mapping.as_ptr()) } == -1
        {
            succeed = false;
        }
    }

    // To make return value consistent with the GLFW version.
    if succeed { 1 } else { 0 }
}

// Set gamepad vibration
pub unsafe fn SetGamepadVibration(gamepad: i32, mut leftMotor: f32, mut rightMotor: f32, mut duration: f32)
{
    if ((gamepad < MAX_GAMEPADS as i32) && CORE.Input.Gamepad.ready[gamepad as usize] && (duration > 0.0))
    {
        if (leftMotor < 0.0) { leftMotor = 0.0; }
        if (leftMotor > 1.0) { leftMotor = 1.0; }
        if (rightMotor < 0.0) { rightMotor = 0.0; }
        if (rightMotor > 1.0) { rightMotor = 1.0; }
        if (duration > MAX_GAMEPAD_VIBRATION_TIME) { duration = MAX_GAMEPAD_VIBRATION_TIME; }

        SDL_RumbleGamepad(platform.gamepad[gamepad as usize], ((leftMotor*65535.0) as u16), ((rightMotor*65535.0) as u16), ((duration*1000.0) as u32));
    }
}

// Set mouse position XY
pub unsafe fn SetMousePosition(x: i32, y: i32)
{
    SDL_WarpMouseInWindow(platform.window, x as f32, y as f32);

    CORE.Input.Mouse.currentPosition = Vector2::new((x as f32), (y as f32));
}

// Set mouse cursor
pub unsafe fn SetMouseCursor(cursor: i32)
{
    platform.cursor = SDL_CreateSystemCursor(CursorsLUT[cursor as usize]);
    SDL_SetCursor(platform.cursor);

    CORE.Input.Mouse.cursor = cursor;
}

// Get physical key name
pub unsafe fn GetKeyName(key: i32) -> String
{
    return CStr::from_ptr(SDL_GetKeyName(SDL_Keycode(key as u32))).to_string_lossy().into_owned();
}

// Register all input events
pub unsafe fn PollInputEvents()
{
#[cfg(feature = "SUPPORT_GESTURES_SYSTEM")]
{
    // NOTE: Gestures update must be called every frame to reset gestures correctly
    // because ProcessGestureEvent() is called on an event, not every frame
    UpdateGestures();
}

    // Reset keys/chars pressed registered
    CORE.Input.Keyboard.keyPressedQueueCount = 0;
    CORE.Input.Keyboard.charPressedQueueCount = 0;

    // Reset mouse wheel
    CORE.Input.Mouse.currentWheelMove.x = 0.0;
    CORE.Input.Mouse.currentWheelMove.y = 0.0;

    // Register previous mouse position
    if (CORE.Input.Mouse.cursorLocked) { CORE.Input.Mouse.currentPosition = Vector2::new(0.0, 0.0); }
    else { CORE.Input.Mouse.previousPosition = CORE.Input.Mouse.currentPosition; }

    // Reset last gamepad button/axis registered state
    let mut i: usize = 0;
    while ((i as i32) < SDL_NumJoysticks()) && (i < MAX_GAMEPADS)
    {
        // Check if gamepad is available
        if (CORE.Input.Gamepad.ready[i])
        {
            // Register previous gamepad button states
            for k in 0_usize..(MAX_GAMEPAD_BUTTONS)
            {
                CORE.Input.Gamepad.previousButtonState[i][k] = CORE.Input.Gamepad.currentButtonState[i][k];
            }
        }
        i += 1;
    }

    // Register previous touch states
    for i in 0_usize..(MAX_TOUCH_POINTS) { CORE.Input.Touch.previousTouchState[i] = CORE.Input.Touch.currentTouchState[i]; }

    // Map touch position to mouse position for convenience
    if (CORE.Input.Touch.pointCount == 0) { CORE.Input.Touch.position[0] = CORE.Input.Mouse.currentPosition; }

    let mut touchAction: i32 = -1;       // 0-TOUCH_ACTION_UP, 1-TOUCH_ACTION_DOWN, 2-TOUCH_ACTION_MOVE
    let mut realTouch: bool = false;     // Flag to differentiate real touch gestures from mouse ones

    // Register previous keys states
    // NOTE: Android supports up to 260 keys
    for i in 0_usize..(MAX_KEYBOARD_KEYS)
    {
        CORE.Input.Keyboard.previousKeyState[i] = CORE.Input.Keyboard.currentKeyState[i];
        CORE.Input.Keyboard.keyRepeatInFrame[i] = 0;
    }

    // Register previous mouse states
    for i in 0_usize..(MAX_MOUSE_BUTTONS) { CORE.Input.Mouse.previousButtonState[i] = CORE.Input.Mouse.currentButtonState[i]; }

    // Poll input events for current platform
    //-----------------------------------------------------------------------------
    // WARNING: Indexes into this array are obtained by using SDL_Scancode values, not SDL_Keycode values
    //const Uint8 *keys = SDL_GetKeyboardState(NULL);
    //for (int i = 0; i < 256; i++) CORE.Input.Keyboard.currentKeyState[i] = keys[i];

    CORE.Window.resizedLastFrame = false;

    if ((CORE.Window.eventWaiting) || (((CORE.Window.flags & FLAG_WINDOW_MINIMIZED as u32) != 0) && !((CORE.Window.flags & FLAG_WINDOW_ALWAYS_RUN as u32) != 0)))
    {
        SDL_WaitEvent(std::ptr::null_mut());
        CORE.Time.previous = GetTime();
    }

    let mut event: SDL_Event = std::mem::zeroed();
    while (SDL_PollEvent(&mut event))
    {
        // All input events can be processed after polling
        match SDL_EventType(event.r#type)
        {
            value if value == SDL_EventType::QUIT => { CORE.Window.shouldClose = true; },
            value if value == SDL_EventType::DROP_FILE => // Dropped file
            {
                if (CORE.Window.dropFileCount == 0)
                {
                    // When a new file is dropped, reserve a fixed number of slots for all possible dropped files
                    // at the moment limit the number of drops at once to 1024 files but this behaviour should probably be reviewed
                    // TODO: Pointers should probably be reallocated for any new file added...
                    //CORE.Window.dropFilepaths = (RL_CALLOC(1024, std::mem::size_of::<*mut std::ffi::c_char>()) as *mut *mut std::ffi::c_char);

                    //(*CORE.Window.dropFilepaths.add(CORE.Window.dropFileCount as usize)) = (RL_CALLOC(MAX_FILEPATH_LENGTH, std::mem::size_of::<std::ffi::c_char>()) as *mut std::ffi::c_char);

                    // const char *data;   // The text for SDL_EVENT_DROP_TEXT and the file name for SDL_EVENT_DROP_FILE, NULL for other events
                    // Event memory is now managed by SDL, so it should not be freed in SDL_EVENT_DROP_FILE,
                    // in case data needs to be hold onto the text in SDL_EVENT_TEXT_EDITING and SDL_EVENT_TEXT_INPUT events,
                    // a copy is required, SDL_TEXTINPUTEVENT_TEXT_SIZE is no longer necessary and has been removed
                    libc::snprintf((*CORE.Window.dropFilepaths.add(CORE.Window.dropFileCount as usize)), MAX_FILEPATH_LENGTH, c"%s".as_ptr(), event.drop.data);

                    CORE.Window.dropFileCount += 1;
                }
                else if (CORE.Window.dropFileCount < 1024)
                {
                    //(*CORE.Window.dropFilepaths.add(CORE.Window.dropFileCount as usize)) = (RL_CALLOC(MAX_FILEPATH_LENGTH, std::mem::size_of::<std::ffi::c_char>()) as *mut std::ffi::c_char);

                    libc::snprintf((*CORE.Window.dropFilepaths.add(CORE.Window.dropFileCount as usize)), MAX_FILEPATH_LENGTH, c"%s".as_ptr(), event.drop.data);

                    CORE.Window.dropFileCount += 1;
                }
                else { warn!("FILE: Maximum drag and drop files at once is limited to 1024 files!"); }

            } 

            // Window events are also polled (minimized, maximized, close...)
                    value if value == SDL_EventType::WINDOW_RESIZED || value == SDL_EventType::WINDOW_PIXEL_SIZE_CHANGED => {
                        let width: i32 = event.window.data1;
                        let height: i32 = event.window.data2;
                        SetupViewport(width, height);

                        // Consider content scaling if required
                        if (((CORE.Window.flags & FLAG_WINDOW_HIGHDPI as u32) != 0))
                        {
                            CORE.Window.screen.x = (((width as f32/GetWindowScaleDPI().x) as i32)) as f32;
                            CORE.Window.screen.y = (((height as f32/GetWindowScaleDPI().y) as i32)) as f32;
                        }
                        else
                        {
                            CORE.Window.screen.x = (width) as f32;
                            CORE.Window.screen.y = (height) as f32;
                        }
                        CORE.Window.currentFbo.x = (width) as f32;
                        CORE.Window.currentFbo.y = (height) as f32;
                        CORE.Window.resizedLastFrame = true;


                    },
                    value if value == SDL_EventType::WINDOW_MOUSE_ENTER => { CORE.Input.Mouse.cursorOnScreen = true; },
                    value if value == SDL_EventType::WINDOW_MOUSE_LEAVE => { CORE.Input.Mouse.cursorOnScreen = false; },
                    value if value == SDL_EventType::WINDOW_MINIMIZED => {
                        if (!((CORE.Window.flags & FLAG_WINDOW_MINIMIZED as u32) != 0)) { CORE.Window.flags |= FLAG_WINDOW_MINIMIZED as u32; }
                    },
                    value if value == SDL_EventType::WINDOW_MAXIMIZED => {
                        if (!((CORE.Window.flags & FLAG_WINDOW_MAXIMIZED as u32) != 0)) { CORE.Window.flags |= FLAG_WINDOW_MAXIMIZED as u32; }
                    },
                    value if value == SDL_EventType::WINDOW_RESTORED => {
                        if (!((SDL_GetWindowFlags(platform.window) & SDL_WindowFlags::MINIMIZED) != 0))
                        {
                            if (((CORE.Window.flags & FLAG_WINDOW_MINIMIZED as u32) != 0)) { CORE.Window.flags &= !(FLAG_WINDOW_MINIMIZED as u32); }
                        }

                        if (!((SDL_GetWindowFlags(platform.window) & SDL_WindowFlags::MAXIMIZED) != 0))
                        {
                            if (((CORE.Window.flags & SDL_WindowFlags::MAXIMIZED.0 as u32) != 0)) { CORE.Window.flags &= !(SDL_WindowFlags::MAXIMIZED.0 as u32); }
                        }
                    },
                    value if value == SDL_EventType::WINDOW_HIDDEN => {
                        if (!((CORE.Window.flags & FLAG_WINDOW_HIDDEN as u32) != 0)) { CORE.Window.flags |= FLAG_WINDOW_HIDDEN as u32; }
                    },
                    value if value == SDL_EventType::WINDOW_SHOWN => {
                        if (((CORE.Window.flags & FLAG_WINDOW_HIDDEN as u32) != 0)) { CORE.Window.flags &= !(FLAG_WINDOW_HIDDEN as u32); }
                    },
                    value if value == SDL_EventType::WINDOW_FOCUS_GAINED => {
                        if (((CORE.Window.flags & FLAG_WINDOW_UNFOCUSED as u32) != 0)) { CORE.Window.flags &= !(FLAG_WINDOW_UNFOCUSED as u32); }
                    },
                    value if value == SDL_EventType::WINDOW_FOCUS_LOST => {
                        if (!((CORE.Window.flags & FLAG_WINDOW_UNFOCUSED as u32) != 0)) { CORE.Window.flags |= FLAG_WINDOW_UNFOCUSED as u32; }
                    } 


            // Check keyboard events
            value if value == SDL_EventType::KEY_DOWN => {
                // SDL3 Migration: The following structures have been removed: SDL_Keysym
                let key: i32 = ConvertScancodeToKey(event.key.scancode);

                if (key != KEY_NULL as i32)
                {
                    // If key was up, add it to the key pressed queue
                    if ((CORE.Input.Keyboard.currentKeyState[key as usize] == 0) && (CORE.Input.Keyboard.keyPressedQueueCount < MAX_KEY_PRESSED_QUEUE as i32))
                    {
                        CORE.Input.Keyboard.keyPressedQueue[CORE.Input.Keyboard.keyPressedQueueCount as usize] = key;
                        CORE.Input.Keyboard.keyPressedQueueCount += 1;
                    }

                    CORE.Input.Keyboard.currentKeyState[key as usize] = 1;
                }

                if (event.key.repeat) { CORE.Input.Keyboard.keyRepeatInFrame[key as usize] = 1; }

                // Check for registered exit key to request exit game loop on next iteration
                if (CORE.Input.Keyboard.currentKeyState[CORE.Input.Keyboard.exitKey as usize] != 0) { CORE.Window.shouldClose = true; }

            },
            value if value == SDL_EventType::KEY_UP => {
                let key: i32 = ConvertScancodeToKey(event.key.scancode);
                if (key != KEY_NULL as i32) { CORE.Input.Keyboard.currentKeyState[key as usize] = 0; }
            },
            value if value == SDL_EventType::TEXT_INPUT => {
                // NOTE: event.text.text data comes an UTF-8 text sequence but register codepoints (int)

                // Check if there is space available in the queue
                if (CORE.Input.Keyboard.charPressedQueueCount < MAX_CHAR_PRESSED_QUEUE as i32)
                {
                    // Add character (codepoint) to the queue
                    let mut textLen: usize = libc::strlen(event.text.text);
                    let codepoint: u32 = (SDL_StepUTF8(&mut event.text.text, &mut textLen) as u32);

                    CORE.Input.Keyboard.charPressedQueue[CORE.Input.Keyboard.charPressedQueueCount as usize] = codepoint as i32;
                    CORE.Input.Keyboard.charPressedQueueCount += 1;
                }
            } 

            // Check mouse events
            value if value == SDL_EventType::MOUSE_BUTTON_DOWN => {
                // NOTE: SDL2 mouse button order is LEFT, MIDDLE, RIGHT, but raylib uses LEFT, RIGHT, MIDDLE like GLFW
                // The following conditions align SDL with raylib.h MouseButton enum order
                let mut btn: i32 = event.button.button as i32 - 1;
                if (btn == 2) { btn = 1; }
                else if (btn == 1) { btn = 2; }

                CORE.Input.Mouse.currentButtonState[btn as usize] = 1;
                CORE.Input.Touch.currentTouchState[btn as usize] = 1;

                touchAction = 1;
            },
            value if value == SDL_EventType::MOUSE_BUTTON_UP => {
                // NOTE: SDL2 mouse button order is LEFT, MIDDLE, RIGHT, but raylib uses LEFT, RIGHT, MIDDLE like GLFW
                // The following conditions align SDL with raylib.h MouseButton enum order
                let mut btn: i32 = event.button.button as i32 - 1;
                if (btn == 2) { btn = 1; }
                else if (btn == 1) { btn = 2; }

                CORE.Input.Mouse.currentButtonState[btn as usize] = 0;
                CORE.Input.Touch.currentTouchState[btn as usize] = 0;

                touchAction = 0;
            },
            value if value == SDL_EventType::MOUSE_WHEEL => {
                CORE.Input.Mouse.currentWheelMove.x = event.wheel.x;
                CORE.Input.Mouse.currentWheelMove.y = event.wheel.y;
            },
            value if value == SDL_EventType::MOUSE_MOTION => {
                if (CORE.Input.Mouse.cursorLocked)
                {
                    CORE.Input.Mouse.currentPosition.x = (event.motion.xrel as f32);
                    CORE.Input.Mouse.currentPosition.y = (event.motion.yrel as f32);
                    CORE.Input.Mouse.previousPosition = Vector2::new(0.0, 0.0);
                }
                else
                {
                    CORE.Input.Mouse.currentPosition.x = (event.motion.x as f32);
                    CORE.Input.Mouse.currentPosition.y = (event.motion.y as f32);
                }

                CORE.Input.Touch.position[0] = CORE.Input.Mouse.currentPosition;
                touchAction = 2;
            } 

            // Check Touch events
            value if value == SDL_EventType::FINGER_DOWN => {
                UpdateTouchPointsSDL(event.tfinger);
                touchAction = 1;
                realTouch = true;
            },
            value if value == SDL_EventType::FINGER_UP => {
                UpdateTouchPointsSDL(event.tfinger);
                touchAction = 0;
                realTouch = true;
            },
            value if value == SDL_EventType::FINGER_MOTION => {
                UpdateTouchPointsSDL(event.tfinger);
                touchAction = 2;
                realTouch = true;
            } 

            // Check Gamepad events
            value if value == SDL_EventType::JOYSTICK_ADDED => {
                let jid: SDL_JoystickID = event.jdevice.which; // Joystick device index

                // Check if already added at InitPlatform
                for i in 0_usize..(MAX_GAMEPADS)
                {
                    if (jid == platform.gamepadId[i]) { return; }
                }

                let mut nextAvailableSlot: usize = 0;
                while (nextAvailableSlot < MAX_GAMEPADS && CORE.Input.Gamepad.ready[nextAvailableSlot])
                {
                    nextAvailableSlot += 1;
                }

                if ((nextAvailableSlot < MAX_GAMEPADS) && !CORE.Input.Gamepad.ready[nextAvailableSlot])
                {
                    platform.gamepad[nextAvailableSlot] = SDL_OpenGamepad(jid);
                    platform.gamepadId[nextAvailableSlot] = SDL_GetJoystickID(SDL_GetGamepadJoystick(platform.gamepad[nextAvailableSlot]));

                    if (!platform.gamepad[nextAvailableSlot].is_null())
                    {
                        CORE.Input.Gamepad.ready[nextAvailableSlot] = true;
                        CORE.Input.Gamepad.axisCount[nextAvailableSlot] = SDL_GetNumJoystickAxes(SDL_GetGamepadJoystick(platform.gamepad[nextAvailableSlot]));
                        CORE.Input.Gamepad.axisState[nextAvailableSlot][GAMEPAD_AXIS_LEFT_TRIGGER as i32 as usize] = -1.0;
                        CORE.Input.Gamepad.axisState[nextAvailableSlot][GAMEPAD_AXIS_RIGHT_TRIGGER as i32 as usize] = -1.0;
                        CORE.Input.Gamepad.name[nextAvailableSlot].fill(0);
                        let controllerName = SDL_GameControllerNameForIndex(nextAvailableSlot as i32);
                        #[allow(clippy::unnecessary_cast)]
                        let controllerName = if controllerName.is_null() { c"noname".as_ptr() } else { controllerName } as *const std::ffi::c_char;
                        let destination = &mut CORE.Input.Gamepad.name[nextAvailableSlot];
                        let controllerNameLength = std::ffi::CStr::from_ptr(controllerName as *const std::ffi::c_char).to_bytes().len().min(destination.len().saturating_sub(1));destination.fill(0);
                        std::ptr::copy_nonoverlapping(controllerName, destination.as_mut_ptr() as *mut std::ffi::c_char, controllerNameLength);
                    }
                    else { warn!("PLATFORM: Unable to open game controller [ERROR: {}]", CStr::from_ptr(SDL_GetError()).to_string_lossy()); }
                }
            },
            value if value == SDL_EventType::JOYSTICK_REMOVED => {
                let jid: SDL_JoystickID = event.jdevice.which; // Joystick instance id

                for i in 0_usize..(MAX_GAMEPADS)
                {
                    if (platform.gamepadId[i] == jid)
                    {
                        SDL_CloseGamepad(platform.gamepad[i]);
                        CORE.Input.Gamepad.ready[i] = false;
                        CORE.Input.Gamepad.name[i].fill(0);
                        platform.gamepadId[i] = SDL_JoystickID((-1i32) as u32);
                        break;
                    }
                }
            },
            value if value == SDL_EventType::GAMEPAD_BUTTON_DOWN => {
                let mut button: i32 = -1;

                match SDL_GamepadButton(event.gbutton.button as i32)
                {
                    value if value == SDL_GamepadButton::NORTH => { button = GAMEPAD_BUTTON_RIGHT_FACE_UP as i32; },
                    value if value == SDL_GamepadButton::EAST => { button = GAMEPAD_BUTTON_RIGHT_FACE_RIGHT as i32; },
                    value if value == SDL_GamepadButton::SOUTH => { button = GAMEPAD_BUTTON_RIGHT_FACE_DOWN as i32; },
                    value if value == SDL_GamepadButton::WEST => { button = GAMEPAD_BUTTON_RIGHT_FACE_LEFT as i32; },
                    value if value == SDL_GamepadButton::LEFT_SHOULDER => { button = GAMEPAD_BUTTON_LEFT_TRIGGER_1 as i32; },
                    value if value == SDL_GamepadButton::RIGHT_SHOULDER => { button = GAMEPAD_BUTTON_RIGHT_TRIGGER_1 as i32; },
                    value if value == SDL_GamepadButton::BACK => { button = GAMEPAD_BUTTON_MIDDLE_LEFT as i32; },
                    value if value == SDL_GamepadButton::GUIDE => { button = GAMEPAD_BUTTON_MIDDLE as i32; },
                    value if value == SDL_GamepadButton::START => { button = GAMEPAD_BUTTON_MIDDLE_RIGHT as i32; },
                    value if value == SDL_GamepadButton::DPAD_UP => { button = GAMEPAD_BUTTON_LEFT_FACE_UP as i32; },
                    value if value == SDL_GamepadButton::DPAD_RIGHT => { button = GAMEPAD_BUTTON_LEFT_FACE_RIGHT as i32; },
                    value if value == SDL_GamepadButton::DPAD_DOWN => { button = GAMEPAD_BUTTON_LEFT_FACE_DOWN as i32; },
                    value if value == SDL_GamepadButton::DPAD_LEFT => { button = GAMEPAD_BUTTON_LEFT_FACE_LEFT as i32; },
                    value if value == SDL_GamepadButton::LEFT_STICK => { button = GAMEPAD_BUTTON_LEFT_THUMB as i32; },
                    value if value == SDL_GamepadButton::RIGHT_STICK => { button = GAMEPAD_BUTTON_RIGHT_THUMB as i32; },
                    _ => {  },
                }

                if (button >= 0)
                {
                    for i in 0_usize..(MAX_GAMEPADS)
                    {
                        if (platform.gamepadId[i] == event.gbutton.which)
                        {
                            CORE.Input.Gamepad.currentButtonState[i][button as usize] = 1;
                            CORE.Input.Gamepad.lastButtonPressed = button;
                            break;
                        }
                    }
                }
            },
            value if value == SDL_EventType::GAMEPAD_BUTTON_UP => {
                let mut button: i32 = -1;

                match SDL_GamepadButton(event.gbutton.button as i32)
                {
                    value if value == SDL_GamepadButton::NORTH => { button = GAMEPAD_BUTTON_RIGHT_FACE_UP as i32; },
                    value if value == SDL_GamepadButton::EAST => { button = GAMEPAD_BUTTON_RIGHT_FACE_RIGHT as i32; },
                    value if value == SDL_GamepadButton::SOUTH => { button = GAMEPAD_BUTTON_RIGHT_FACE_DOWN as i32; },
                    value if value == SDL_GamepadButton::WEST => { button = GAMEPAD_BUTTON_RIGHT_FACE_LEFT as i32; },
                    value if value == SDL_GamepadButton::LEFT_SHOULDER => { button = GAMEPAD_BUTTON_LEFT_TRIGGER_1 as i32; },
                    value if value == SDL_GamepadButton::RIGHT_SHOULDER => { button = GAMEPAD_BUTTON_RIGHT_TRIGGER_1 as i32; },
                    value if value == SDL_GamepadButton::BACK => { button = GAMEPAD_BUTTON_MIDDLE_LEFT as i32; },
                    value if value == SDL_GamepadButton::GUIDE => { button = GAMEPAD_BUTTON_MIDDLE as i32; },
                    value if value == SDL_GamepadButton::START => { button = GAMEPAD_BUTTON_MIDDLE_RIGHT as i32; },
                    value if value == SDL_GamepadButton::DPAD_UP => { button = GAMEPAD_BUTTON_LEFT_FACE_UP as i32; },
                    value if value == SDL_GamepadButton::DPAD_RIGHT => { button = GAMEPAD_BUTTON_LEFT_FACE_RIGHT as i32; },
                    value if value == SDL_GamepadButton::DPAD_DOWN => { button = GAMEPAD_BUTTON_LEFT_FACE_DOWN as i32; },
                    value if value == SDL_GamepadButton::DPAD_LEFT => { button = GAMEPAD_BUTTON_LEFT_FACE_LEFT as i32; },
                    value if value == SDL_GamepadButton::LEFT_STICK => { button = GAMEPAD_BUTTON_LEFT_THUMB as i32; },
                    value if value == SDL_GamepadButton::RIGHT_STICK => { button = GAMEPAD_BUTTON_RIGHT_THUMB as i32; },
                    _ => {  },
                }

                if (button >= 0)
                {
                    for i in 0_usize..(MAX_GAMEPADS)
                    {
                        if (platform.gamepadId[i] == event.gbutton.which)
                        {
                            CORE.Input.Gamepad.currentButtonState[i][button as usize] = 0;
                            if (CORE.Input.Gamepad.lastButtonPressed == button) { CORE.Input.Gamepad.lastButtonPressed = 0; }
                            break;
                        }
                    }
                }
            },
            value if value == SDL_EventType::GAMEPAD_AXIS_MOTION => {
                let mut axis: i32 = -1;

                match SDL_GamepadAxis(event.jaxis.axis as i32)
                {
                    value if value == SDL_GamepadAxis::LEFTX => { axis = GAMEPAD_AXIS_LEFT_X as i32; },
                    value if value == SDL_GamepadAxis::LEFTY => { axis = GAMEPAD_AXIS_LEFT_Y as i32; },
                    value if value == SDL_GamepadAxis::RIGHTX => { axis = GAMEPAD_AXIS_RIGHT_X as i32; },
                    value if value == SDL_GamepadAxis::RIGHTY => { axis = GAMEPAD_AXIS_RIGHT_Y as i32; },
                    value if value == SDL_GamepadAxis::LEFT_TRIGGER => { axis = GAMEPAD_AXIS_LEFT_TRIGGER as i32; },
                    value if value == SDL_GamepadAxis::RIGHT_TRIGGER => { axis = GAMEPAD_AXIS_RIGHT_TRIGGER as i32; },
                    _ => {  },
                }

                if (axis >= 0)
                {
                    for i in 0_usize..(MAX_GAMEPADS)
                    {
                        if (platform.gamepadId[i] == event.jaxis.which)
                        {
                            // SDL axis value range is -32768 to 32767, normalizing it to raylib's -1.0 to 1.0f range
                            let value: f32 = event.jaxis.value as f32/32767_f32;
                            CORE.Input.Gamepad.axisState[i][axis as usize] = value;

                            // Register button state for triggers in addition to their axes
                            if ((axis == GAMEPAD_AXIS_LEFT_TRIGGER as i32) || (axis == GAMEPAD_AXIS_RIGHT_TRIGGER as i32))
                            {
                                let button: i32 = if (axis == GAMEPAD_AXIS_LEFT_TRIGGER as i32) { GAMEPAD_BUTTON_LEFT_TRIGGER_2 as i32 } else { GAMEPAD_BUTTON_RIGHT_TRIGGER_2 as i32 };
                                let pressed: i32 = (value > 0.1) as i32;
                                CORE.Input.Gamepad.currentButtonState[i][button as usize] = pressed as i8;
                                if (pressed != 0) { CORE.Input.Gamepad.lastButtonPressed = button; }
                                else if (CORE.Input.Gamepad.lastButtonPressed == button) { CORE.Input.Gamepad.lastButtonPressed = 0; }
                            }
                            break;
                        }
                    }
                }
            },
            _ => {  },
        }

#[cfg(feature = "SUPPORT_GESTURES_SYSTEM")]
{
        if (touchAction > -1)
        {
            // Process mouse events as touches to be able to use mouse-gestures
            let mut gestureEvent: GestureEvent = std::mem::zeroed();

            // Register touch actions
            gestureEvent.touchAction = touchAction;

            // Assign a pointer ID
            gestureEvent.pointId[0] = 0;

            // Register touch points count
            gestureEvent.pointCount = 1;

            // Register touch points position, only one point registered
            if (touchAction == 2 || realTouch) { gestureEvent.position[0] = CORE.Input.Touch.position[0]; }
            else { gestureEvent.position[0] = GetMousePosition(); }

            // Normalize gestureEvent.position[0] for CORE.Window.screen.width and CORE.Window.screen.height
            gestureEvent.position[0].x /= (GetScreenWidth() as f32);
            gestureEvent.position[0].y /= (GetScreenHeight() as f32);

            // Gesture data is sent to gestures-system for processing
            ProcessGestureEvent(gestureEvent);

            touchAction = -1;
        }
}
    }
    //-----------------------------------------------------------------------------
}

//----------------------------------------------------------------------------------
// Module Internal Functions Definition
//----------------------------------------------------------------------------------

// Initialize platform: graphics, inputs and more
pub unsafe fn InitPlatform() -> i32
{
    // Initialize SDL internal global state, only required systems
    // NOTE: Not all systems need to be initialized, SDL_INIT_AUDIO is not required, managed by miniaudio
    if !SDL_Init(SDL_INIT_VIDEO | SDL_INIT_TIMER | SDL_INIT_EVENTS | SDL_INIT_GAMEPAD) {
        error!("SDL: Failed to initialize SDL: {}", CStr::from_ptr(SDL_GetError()).to_string_lossy());
        SDL_Quit();
        return -1;
    }

    // Initialize graphic device: display/window and graphic context
    //----------------------------------------------------------------------------
    let mut flags: SDL_WindowFlags = SDL_WindowFlags(0);
    flags |= SDL_WINDOW_SHOWN;
    flags |= SDL_WINDOW_INPUT_FOCUS;
    flags |= SDL_WINDOW_MOUSE_FOCUS;
    flags |= SDL_WINDOW_MOUSE_CAPTURE;  // Window has mouse captured

    // Check window creation flags
    if (((CORE.Window.flags & FLAG_FULLSCREEN_MODE as u32) != 0)) { flags |= SDL_WINDOW_FULLSCREEN; }

    //if (!FLAG_IS_SET(CORE.Window.flags, FLAG_WINDOW_HIDDEN)) FLAG_SET(flags, SDL_WINDOW_HIDDEN);
    if (((CORE.Window.flags & FLAG_WINDOW_UNDECORATED as u32) != 0)) { flags |= SDL_WINDOW_BORDERLESS; }
    if (((CORE.Window.flags & FLAG_WINDOW_RESIZABLE as u32) != 0)) { flags |= SDL_WINDOW_RESIZABLE; }
    if (((CORE.Window.flags & FLAG_WINDOW_MINIMIZED as u32) != 0)) { flags |= SDL_WINDOW_MINIMIZED; }
    if (((CORE.Window.flags & FLAG_WINDOW_MAXIMIZED as u32) != 0)) { flags |= SDL_WINDOW_MAXIMIZED; }
    if (((CORE.Window.flags & FLAG_WINDOW_UNFOCUSED as u32) != 0))
    {
        flags &= !SDL_WINDOW_INPUT_FOCUS;
        flags &= !SDL_WINDOW_MOUSE_FOCUS;
    }
    if (((CORE.Window.flags & FLAG_WINDOW_TOPMOST as u32) != 0)) { flags |= SDL_WINDOW_ALWAYS_ON_TOP; }
    if (((CORE.Window.flags & FLAG_WINDOW_MOUSE_PASSTHROUGH as u32) != 0)) { flags &= !SDL_WINDOW_MOUSE_CAPTURE; }
    if (((CORE.Window.flags & FLAG_WINDOW_HIGHDPI as u32) != 0)) { flags |= SDL_WINDOW_HIGH_PIXEL_DENSITY; }

    //if (FLAG_IS_SET(CORE.Window.flags, FLAG_WINDOW_TRANSPARENT)) FLAG_SET(flags, SDL_WINDOW_TRANSPARENT);     // Alternative: SDL_GL_ALPHA_SIZE = 8
    //if (FLAG_IS_SET(CORE.Window.flags, FLAG_FULLSCREEN_DESKTOP)) FLAG_SET(flags, SDL_WINDOW_FULLSCREEN_DESKTOP);

    // NOTE: Some OpenGL context attributes must be set before window creation

    if (rlGetVersion() != RL_OPENGL_SOFTWARE)
    {
        // Add the flag telling the window to use an OpenGL context
        flags |= SDL_WINDOW_OPENGL;

        // Check selection OpenGL version
        if (rlGetVersion() == RL_OPENGL_21)
        {
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 2);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 1);
        }
        else if (rlGetVersion() == RL_OPENGL_33)
        {
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE.0);
        }
        else if (rlGetVersion() == RL_OPENGL_43)
        {
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 4);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE.0);
    #[cfg(feature = "RLGL_ENABLE_OPENGL_DEBUG_CONTEXT")]
    {
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, SDL_GL_CONTEXT_DEBUG_FLAG.0);   // Enable OpenGL Debug Context
    }
        }
        else if (rlGetVersion() == RL_OPENGL_ES_20)                 // Request OpenGL ES 2.0 context
        {
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_ES.0);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 2);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 0);
        }
        else if (rlGetVersion() == RL_OPENGL_ES_30)                 // Request OpenGL ES 3.0 context
        {
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_ES.0);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 0);
        }

        if (((CORE.Window.flags & FLAG_MSAA_4X_HINT as u32) != 0))
        {
            SDL_GL_SetAttribute(SDL_GL_MULTISAMPLEBUFFERS, 1);
            SDL_GL_SetAttribute(SDL_GL_MULTISAMPLESAMPLES, 4);
        }
    }

    // Init window
    platform.window = SDL_CreateWindow(CORE.Window.title, CORE.Window.screen.x as i32, CORE.Window.screen.y as i32, flags);
    if platform.window.is_null() {
        error!("SDL: Failed to create window: {}", CStr::from_ptr(SDL_GetError()).to_string_lossy());
        SDL_Quit();
        return -1;
    }

    // NOTE: SDL3 no longer enables text input by default,
    // it is needed to be enabled manually to keep GetCharPressed() working
    // REF: https://github.com/libsdl-org/SDL/commit/72fc6f86e5d605a3787222bc7dc18c5379047f4a
    let enableOSK: *const std::ffi::c_char = SDL_GetHint(SDL_HINT_ENABLE_SCREEN_KEYBOARD);
    if enableOSK.is_null() { SDL_SetHint(SDL_HINT_ENABLE_SCREEN_KEYBOARD, c"0".as_ptr()); }
    if (!SDL_StartTextInput(platform.window)) { warn!("SDL: Failed to start text input: {}", CStr::from_ptr(SDL_GetError()).to_string_lossy()); }
    if enableOSK.is_null() { SDL_SetHint(SDL_HINT_ENABLE_SCREEN_KEYBOARD, std::ptr::null_mut()); }

    // Init OpenGL context
    if (rlGetVersion() != RL_OPENGL_SOFTWARE)
    {
        platform.glContext = SDL_GL_CreateContext(platform.window);
    }

    if (!platform.window.is_null() && ((rlGetVersion() == RL_OPENGL_SOFTWARE) || !platform.glContext.is_null()))
    {
        CORE.Window.ready = true;

        let mut displayMode: SDL_DisplayMode = std::mem::zeroed();
        SDL_GetCurrentDisplayMode(SDL_DisplayID(GetCurrentMonitor() as u32), &mut displayMode);

        CORE.Window.display.x = (displayMode.w) as f32;
        CORE.Window.display.y = (displayMode.h) as f32;

        // Android chooses the window size. Keep logical and framebuffer sizes
        // distinct so the demo also handles displays with content scaling.
        let (mut width, mut height) = (0, 0);
        SDL_GetWindowSize(platform.window, &mut width, &mut height);
        CORE.Window.screen.x = width as f32;
        CORE.Window.screen.y = height as f32;
        SDL_GetWindowSizeInPixels(platform.window, &mut width, &mut height);
        CORE.Window.render.x = width as f32;
        CORE.Window.render.y = height as f32;
        CORE.Window.currentFbo.x = CORE.Window.render.x;
        CORE.Window.currentFbo.y = CORE.Window.render.y;

        info!("DISPLAY: Device initialized successfully {}", if ((CORE.Window.flags & FLAG_WINDOW_HIGHDPI as u32) != 0) { "(HighDPI)" } else { "" });
        info!("    > Display size: {} x {}", CORE.Window.display.x, CORE.Window.display.y);
        info!("    > Screen size:  {} x {}", CORE.Window.screen.x, CORE.Window.screen.y);
        info!("    > Render size:  {} x {}", CORE.Window.render.x, CORE.Window.render.y);
        info!("    > Viewport offsets: {}, {}", CORE.Window.renderOffset.x, CORE.Window.renderOffset.y);

        if !platform.glContext.is_null()
        {
            SDL_GL_SetSwapInterval(if (((CORE.Window.flags & FLAG_VSYNC_HINT as u32) != 0)) { 1 } else { 0 });

            // Load OpenGL extensions
            // NOTE: GL procedures address loader is required to load extensions
            rlLoadExtensions(SDL_GL_GetProcAddress as *const () as *mut std::ffi::c_void);
        }
    }
    else
    {
        error!("PLATFORM: Failed to initialize graphics device: {}", CStr::from_ptr(SDL_GetError()).to_string_lossy());
        SDL_DestroyWindow(platform.window);
        platform.window = std::ptr::null_mut();
        SDL_Quit();
        return -1;
    }

    //----------------------------------------------------------------------------

    // Initialize input events system
    //----------------------------------------------------------------------------
    // Initialize gamepads
    for i in 0_usize..(MAX_GAMEPADS)
    {
        platform.gamepadId[i] = SDL_JoystickID((-1i32) as u32); // Set all gamepad initial instance ids as invalid to not conflict with instance id zero
    }

    let mut numJoysticks: i32 = 0;
    let joysticks: *mut SDL_JoystickID = SDL_GetJoysticks(&mut numJoysticks);

    let mut i: usize = 0;
    while ((i as i32) < numJoysticks) && (i < MAX_GAMEPADS)
    {
        platform.gamepad[i] = SDL_OpenGamepad(*joysticks.add(i));
        platform.gamepadId[i] = SDL_GetJoystickID(SDL_GetGamepadJoystick(platform.gamepad[i]));

        if (!platform.gamepad[i].is_null())
        {
            CORE.Input.Gamepad.ready[i] = true;
            CORE.Input.Gamepad.axisCount[i] = SDL_GetNumJoystickAxes(SDL_GetGamepadJoystick(platform.gamepad[i]));
            CORE.Input.Gamepad.axisState[i][GAMEPAD_AXIS_LEFT_TRIGGER as i32 as usize] = -1.0;
            CORE.Input.Gamepad.axisState[i][GAMEPAD_AXIS_RIGHT_TRIGGER as i32 as usize] = -1.0;
            #[allow(clippy::unnecessary_cast)] // DO NOT REMOVE THE AS *const CASTING OR IT WILL RUN INTO u8 vs i8 problem when compiling on android/desktop
            let joystickName = SDL_GetJoystickNameForID(*joysticks.add(i as usize)) as *const i8;
            let destination = &mut CORE.Input.Gamepad.name[i];
            let joystickNameLength = std::ffi::CStr::from_ptr(joystickName as *const std::ffi::c_char).to_bytes().len().min(destination.len().saturating_sub(1));
            destination.fill(0);
            std::ptr::copy_nonoverlapping(joystickName, destination.as_mut_ptr(), joystickNameLength);
            CORE.Input.Gamepad.name[i][MAX_GAMEPAD_NAME_LENGTH - 1] = 0;
        }
        else { warn!("PLATFORM: Unable to open game controller [ERROR: {}]", CStr::from_ptr(SDL_GetError()).to_string_lossy()); }
        i += 1;
    }

    // Disable mouse events being interpreted as touch events
    // NOTE: This is wanted because there are SDL_FINGER* events available which provide unique data
    // Due to the way PollInputEvents() and rgestures.h are currently implemented, setting this won't break SUPPORT_MOUSE_GESTURES
    SDL_SetHint(SDL_HINT_TOUCH_MOUSE_EVENTS, c"0".as_ptr());

    SDL_EventState(SDL_EVENT_DROP_FILE, SDL_ENABLE as i32);
    //----------------------------------------------------------------------------

    // Initialize timing system
    //----------------------------------------------------------------------------
    // Get base time from window initialization
    CORE.Time.base = SDL_GetPerformanceCounter();

    #[cfg(all(target_os = "windows", feature = "SUPPORT_WINMM_HIGHRES_TIMER", not(feature = "SUPPORT_BUSY_WAIT_LOOP")))]
    {
    SDL_SetHint(SDL_HINT_TIMER_RESOLUTION, c"1".as_ptr()); // SDL equivalent of timeBeginPeriod() and timeEndPeriod()
    }

    // NOTE: No need to call InitTimer(), let SDL manage it internally
    //----------------------------------------------------------------------------

    // Initialize storage system
    //----------------------------------------------------------------------------
    // Define base path for storage
    CORE.Storage.basePath = SDL_GetBasePath() as *mut std::ffi::c_char; // Alternative: GetWorkingDirectory();
    //----------------------------------------------------------------------------

    info!("PLATFORM: DESKTOP (SDL3): Initialized successfully");

    return 0;
}

// Close platform
pub unsafe fn ClosePlatform()
{
    SDL_DestroyCursor(platform.cursor); // Free cursor
    if !platform.glContext.is_null() { SDL_GL_DestroyContext(platform.glContext); } // Deinitialize OpenGL context
    SDL_DestroyWindow(platform.window);
    SDL_Quit(); // Deinitialize SDL internal global state
}

// Scancode to keycode mapping
pub fn ConvertScancodeToKey(sdlScancode: SDL_Scancode) -> i32
{
    if ((sdlScancode.0 >= 0) && ((sdlScancode.0 as usize) < SCANCODE_MAPPED_NUM))
    {
        return mapScancodeToKey[sdlScancode.0 as usize];
    }

    return KEY_NULL as i32; // No equivalent key in raylib
}

// Get next codepoint in a byte sequence and bytes processed
pub fn GetCodepointNextSDL(text: &str, codepointSize: &mut i32) -> i32
{
    let ptr = text.as_bytes();
    let mut codepoint: i32 = 0x3f;       // Codepoint (defaults to '?')
    *codepointSize = 1;

    // Get current codepoint and bytes processed
    if (0xf0 == (0xf8 & (ptr.first().copied().unwrap_or(0) as i32)))
    {
        // 4 byte UTF-8 codepoint
        if (((((ptr.get(1).copied().unwrap_or(0) as i32) & 0xC0) ^ 0x80) != 0) || ((((ptr.get(2).copied().unwrap_or(0) as i32) & 0xC0) ^ 0x80) != 0) || ((((ptr.get(3).copied().unwrap_or(0) as i32) & 0xC0) ^ 0x80) != 0)) { return codepoint; } // 10xxxxxx checks
        codepoint = ((0x07 & (ptr.first().copied().unwrap_or(0) as i32)) << 18) | ((0x3f & (ptr.get(1).copied().unwrap_or(0) as i32)) << 12) | ((0x3f & (ptr.get(2).copied().unwrap_or(0) as i32)) << 6) | (0x3f & (ptr.get(3).copied().unwrap_or(0) as i32));
        *codepointSize = 4;
    }
    else if (0xe0 == (0xf0 & (ptr.first().copied().unwrap_or(0) as i32)))
    {
        // 3 byte UTF-8 codepoint */
        if (((((ptr.get(1).copied().unwrap_or(0) as i32) & 0xC0) ^ 0x80) != 0) || ((((ptr.get(2).copied().unwrap_or(0) as i32) & 0xC0) ^ 0x80) != 0)) { return codepoint; } // 10xxxxxx checks
        codepoint = ((0x0f & (ptr.first().copied().unwrap_or(0) as i32)) << 12) | ((0x3f & (ptr.get(1).copied().unwrap_or(0) as i32)) << 6) | (0x3f & (ptr.get(2).copied().unwrap_or(0) as i32));
        *codepointSize = 3;
    }
    else if (0xc0 == (0xe0 & (ptr.first().copied().unwrap_or(0) as i32)))
    {
        // 2 byte UTF-8 codepoint
        if ((((ptr.get(1).copied().unwrap_or(0) as i32) & 0xC0) ^ 0x80) != 0) { return codepoint; } // 10xxxxxx checks
        codepoint = ((0x1f & (ptr.first().copied().unwrap_or(0) as i32)) << 6) | (0x3f & (ptr.get(1).copied().unwrap_or(0) as i32));
        *codepointSize = 2;
    }
    else if (0x00 == (0x80 & (ptr.first().copied().unwrap_or(0) as i32)))
    {
        // 1 byte UTF-8 codepoint
        codepoint = (ptr.first().copied().unwrap_or(0) as i32);
        *codepointSize = 1;
    }

    return codepoint;
}

// Update CORE input touch point info from SDL touch data
pub unsafe fn UpdateTouchPointsSDL(event: SDL_TouchFingerEvent)
{
    let mut count: i32 = 0;
    let fingers: *mut *mut SDL_Finger = SDL_GetTouchFingers(event.touchID, &mut count);
    CORE.Input.Touch.pointCount = count;

    for i in 0_usize..(CORE.Input.Touch.pointCount) as usize
    {
        let finger: *mut SDL_Finger = *fingers.add(i);
        CORE.Input.Touch.pointId[i] = (*finger).id.0 as i32;
        CORE.Input.Touch.position[i].x = (*finger).x*CORE.Window.screen.x;
        CORE.Input.Touch.position[i].y = (*finger).y*CORE.Window.screen.y;
        CORE.Input.Touch.currentTouchState[i] = 1;
    }

    SDL_free(fingers.cast());


    for i in (CORE.Input.Touch.pointCount) as usize..(MAX_TOUCH_POINTS) { CORE.Input.Touch.currentTouchState[i] = 0; }
}
