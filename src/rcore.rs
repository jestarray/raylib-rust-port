#![allow(unused_variables)]
#![allow(unsafe_op_in_unsafe_fn)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens, non_snake_case, static_mut_refs)]
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
use std::ffi::{c_char, CString};

use crate::{
    math::{QuaternionTransform, Vector3Transform, Vector3Unproject}, rlgl::{rlGetVersion, rlGlVersion}, types::{Color, Matrix, RAYLIB_VERSION, Texture2D, Vector2},
};
use crate::rcore_desktop_sdl::*;

pub const MAX_TRACELOG_MSG_LENGTH: usize = 256;
pub const MAX_FILEPATH_CAPACITY: usize = 8192;

#[cfg(target_os = "windows")]
pub const MAX_FILEPATH_LENGTH: usize = 256;
#[cfg(not(target_os = "windows"))]
pub const MAX_FILEPATH_LENGTH: usize = 4096;

pub const MAX_KEYBOARD_KEYS: usize = 512;
pub const MAX_MOUSE_BUTTONS: usize = 8;
pub const MAX_GAMEPADS: usize = 4;
pub const MAX_GAMEPAD_NAME_LENGTH: usize = 128;
pub const MAX_GAMEPAD_AXES: usize = 8;
pub const MAX_GAMEPAD_BUTTONS: usize = 32;
pub const MAX_GAMEPAD_VIBRATION_TIME: f32 = 2.0;
pub const MAX_TOUCH_POINTS: usize = 8;
pub const MAX_KEY_PRESSED_QUEUE: usize = 16;
pub const MAX_CHAR_PRESSED_QUEUE: usize = 16;
pub const MAX_DECOMPRESSION_SIZE: usize = 64;
pub const MAX_AUTOMATION_EVENTS: usize = 16384;

// --- Component Structs (Converted from anonymous nested structs) ---
#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct WindowData {
    pub title: *mut c_char,
    pub flags: u32,
    pub ready: bool,
    pub shouldClose: bool,
    pub resizedLastFrame: bool,
    pub eventWaiting: bool,
    pub usingFbo: bool,

    pub display: Vector2,
    pub screen: Vector2,
    pub position: Vector2,
    pub previousScreen: Vector2,
    pub previousPosition: Vector2,
    pub render: Vector2,
    pub renderOffset: Vector2,
    pub currentFbo: Vector2,
    pub screenMin: Vector2,
    pub screenMax: Vector2,
    pub screenScale: Matrix,

    pub dropFilepaths: *mut *mut c_char,
    pub dropFileCount: u32,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StorageData {
    pub basePath: *mut c_char,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct KeyboardData {
    pub exitKey: i32,
    pub currentKeyState: [i8; MAX_KEYBOARD_KEYS],
    pub previousKeyState: [i8; MAX_KEYBOARD_KEYS],
    pub keyRepeatInFrame: [i8; MAX_KEYBOARD_KEYS],
    pub keyPressedQueue: [i32; MAX_KEY_PRESSED_QUEUE],
    pub keyPressedQueueCount: i32,
    pub charPressedQueue: [i32; MAX_CHAR_PRESSED_QUEUE],
    pub charPressedQueueCount: i32,
}
impl Default for KeyboardData {
    fn default() -> Self {
        Self {
            exitKey: 0,
            currentKeyState: [0; MAX_KEYBOARD_KEYS],
            previousKeyState: [0; MAX_KEYBOARD_KEYS],
            keyRepeatInFrame: [0; MAX_KEYBOARD_KEYS],
            keyPressedQueue: [0; MAX_KEY_PRESSED_QUEUE],
            keyPressedQueueCount: 0,
            charPressedQueue: [0; MAX_CHAR_PRESSED_QUEUE],
            charPressedQueueCount: 0,
        }
    }
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct MouseData {
    pub offset: Vector2,
    pub scale: Vector2,
    pub currentPosition: Vector2,
    pub previousPosition: Vector2,
    pub lockedPosition: Vector2,
    pub cursor: i32,
    pub cursorHidden: bool,
    pub cursorLocked: bool,
    pub cursorOnScreen: bool,
    pub currentButtonState: [i8; MAX_MOUSE_BUTTONS],
    pub previousButtonState: [i8; MAX_MOUSE_BUTTONS],
    pub currentWheelMove: Vector2,
    pub previousWheelMove: Vector2,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TouchData {
    pub pointCount: i32,
    pub pointId: [i32; MAX_TOUCH_POINTS],
    pub position: [Vector2; MAX_TOUCH_POINTS],
    pub previousPosition: [Vector2; MAX_TOUCH_POINTS],
    pub currentTouchState: [i8; MAX_TOUCH_POINTS],
    pub previousTouchState: [i8; MAX_TOUCH_POINTS],
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GamepadData {
    pub lastButtonPressed: i32,
    pub axisCount: [i32; MAX_GAMEPADS],
    pub ready: [bool; MAX_GAMEPADS],
    pub name: [[i8; MAX_GAMEPAD_NAME_LENGTH]; MAX_GAMEPADS],
    pub currentButtonState: [[i8; MAX_GAMEPAD_BUTTONS]; MAX_GAMEPADS],
    pub previousButtonState: [[i8; MAX_GAMEPAD_BUTTONS]; MAX_GAMEPADS],
    pub axisState: [[f32; MAX_GAMEPAD_AXES]; MAX_GAMEPADS],
}
impl Default for GamepadData {
    fn default() -> Self {
        Self {
            lastButtonPressed: 0,
            axisCount: [0; MAX_GAMEPADS],
            ready: [false; MAX_GAMEPADS],
            // For nested arrays, you initialize the inner array then the outer
            name: [[0; MAX_GAMEPAD_NAME_LENGTH]; MAX_GAMEPADS],
            currentButtonState: [[0; MAX_GAMEPAD_BUTTONS]; MAX_GAMEPADS],
            previousButtonState: [[0; MAX_GAMEPAD_BUTTONS]; MAX_GAMEPADS],
            axisState: [[0.0; MAX_GAMEPAD_AXES]; MAX_GAMEPADS],
        }
    }
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InputData {
    pub Keyboard: KeyboardData,
    pub Mouse: MouseData,
    pub Touch: TouchData,
    pub Gamepad: GamepadData,
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct TimeData {
    pub current: f64,
    pub previous: f64,
    pub update: f64,
    pub draw: f64,
    pub frame: f64,
    pub target: f64,
    pub base: u64,
    pub frameCounter: u32,
}

// --- Main CoreData Struct ---
#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct CoreData {
    pub Window: WindowData,
    pub Storage: StorageData,
    pub Input: InputData,
    pub Time: TimeData,
}

pub static mut CORE: CoreData = CoreData {
    Window: WindowData {
        title: std::ptr::null_mut(),
        flags: 0,
        ready: false,
        shouldClose: false,
        resizedLastFrame: false,
        eventWaiting: false,
        usingFbo: false,
        display: Vector2::ZERO,
        screen: Vector2::ZERO,
        position: Vector2::ZERO,
        previousScreen: Vector2::ZERO,
        previousPosition: Vector2::ZERO,
        render: Vector2::ZERO,
        renderOffset: Vector2::ZERO,
        currentFbo: Vector2::ZERO,
        screenMin: Vector2::ZERO,
        screenMax: Vector2::ZERO,
        screenScale: Matrix {
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
        },
        dropFilepaths: std::ptr::null_mut(),
        dropFileCount: 0,
    },
    Storage: StorageData {
        basePath: std::ptr::null_mut(),
    },
    Input: InputData {
        Keyboard: KeyboardData {
            exitKey: 0,
            currentKeyState: [0; MAX_KEYBOARD_KEYS],
            previousKeyState: [0; MAX_KEYBOARD_KEYS],
            keyRepeatInFrame: [0; MAX_KEYBOARD_KEYS],
            keyPressedQueue: [0; MAX_KEY_PRESSED_QUEUE],
            keyPressedQueueCount: 0,
            charPressedQueue: [0; MAX_CHAR_PRESSED_QUEUE],
            charPressedQueueCount: 0,
        },
        Mouse: MouseData {
            offset: Vector2::ZERO,
            scale: Vector2::ZERO,
            currentPosition: Vector2::ZERO,
            previousPosition: Vector2::ZERO,
            lockedPosition: Vector2::ZERO,
            cursor: 0,
            cursorHidden: false,
            cursorLocked: false,
            cursorOnScreen: false,
            currentButtonState: [0; MAX_MOUSE_BUTTONS],
            previousButtonState: [0; MAX_MOUSE_BUTTONS],
            currentWheelMove: Vector2::ZERO,
            previousWheelMove: Vector2::ZERO,
        },
        Touch: TouchData {
            pointCount: 0,
            pointId: [0; MAX_TOUCH_POINTS],
            position: [Vector2::ZERO; MAX_TOUCH_POINTS],
            previousPosition: [Vector2::ZERO; MAX_TOUCH_POINTS],
            currentTouchState: [0; MAX_TOUCH_POINTS],
            previousTouchState: [0; MAX_TOUCH_POINTS],
        },
        Gamepad: GamepadData {
            lastButtonPressed: 0,
            axisCount: [0; MAX_GAMEPADS],
            ready: [false; MAX_GAMEPADS],
            name: [[0; MAX_GAMEPAD_NAME_LENGTH]; MAX_GAMEPADS],
            currentButtonState: [[0; MAX_GAMEPAD_BUTTONS]; MAX_GAMEPADS],
            previousButtonState: [[0; MAX_GAMEPAD_BUTTONS]; MAX_GAMEPADS],
            axisState: [[0.0; MAX_GAMEPAD_AXES]; MAX_GAMEPADS],
        },
    },
    Time: TimeData {
        current: 0.0,
        previous: 0.0,
        update: 0.0,
        draw: 0.0,
        frame: 0.0,
        target: 0.0,
        base: 0,
        frameCounter: 0,
    },
};

use std::ffi::{c_void, CStr};
use std::fmt::Write;
use log::{info, warn};
use crate::math::DEG2RAD;
use crate::rlgl::*;
use crate::types::{AutomationEvent, AutomationEventList, BlendMode, Camera, Camera2D, CameraProjection, ConfigFlags, GamepadAxis, GamepadButton, KeyboardKey, MouseButton, MouseCursor, Quaternion, Ray, RenderTexture2D, Shader, ShaderLocationIndex, TraceLogLevel, Vector3, VrDeviceInfo, VrStereoConfig};
#[cfg(feature = "SUPPORT_MODULE_RTEXT")]
use crate::rtext::{LoadFontDefault};

//----------------------------------------------------------------------------------
// Global Variables Definition
//----------------------------------------------------------------------------------
pub const raylib_version: &str = "6.1-dev";  // raylib version exported symbol, required for some bindings

static mut windowTitle: Option<CString> = None; // Own the UTF-8 title passed to the platform as a C string

static mut logTypeLevel: i32 = TraceLogLevel::LOG_INFO as i32; // Minimum log type level

#[cfg(feature = "SUPPORT_SCREEN_CAPTURE")]
static mut screenshotCounter: i32 = 0;                   // Screenshots counter

pub type TraceLogCallback = fn(i32, std::fmt::Arguments<'_>);
pub type LoadFileDataCallback = unsafe fn(&str, &mut i32) -> *mut u8;
pub type SaveFileDataCallback = unsafe fn(&str, *const c_void, i32) -> bool;
pub type LoadFileTextCallback = fn(&str) -> Option<String>;
pub type SaveFileTextCallback = fn(&str, &str) -> bool;
static mut traceLog: Option<TraceLogCallback> = None;
static mut loadFileData: Option<LoadFileDataCallback> = None;
static mut saveFileData: Option<SaveFileDataCallback> = None;
static mut loadFileText: Option<LoadFileTextCallback> = None;
static mut saveFileText: Option<SaveFileTextCallback> = None;

// Automation events type
#[cfg(feature = "SUPPORT_AUTOMATION_EVENTS")]
#[repr(u32)]
pub enum AutomationEventType {
    EVENT_NONE = 0,
    // Input events
    INPUT_KEY_UP,                   // param[0]: key
    INPUT_KEY_DOWN,                 // param[0]: key
    INPUT_KEY_PRESSED,              // param[0]: key
    INPUT_KEY_RELEASED,             // param[0]: key
    INPUT_MOUSE_BUTTON_UP,          // param[0]: button
    INPUT_MOUSE_BUTTON_DOWN,        // param[0]: button
    INPUT_MOUSE_POSITION,           // param[0]: x, param[1]: y
    INPUT_MOUSE_WHEEL_MOTION,       // param[0]: x delta, param[1]: y delta
    INPUT_GAMEPAD_CONNECT,          // param[0]: gamepad
    INPUT_GAMEPAD_DISCONNECT,       // param[0]: gamepad
    INPUT_GAMEPAD_BUTTON_UP,        // param[0]: button
    INPUT_GAMEPAD_BUTTON_DOWN,      // param[0]: button
    INPUT_GAMEPAD_AXIS_MOTION,      // param[0]: axis, param[1]: delta
    INPUT_TOUCH_UP,                 // param[0]: id
    INPUT_TOUCH_DOWN,               // param[0]: id
    INPUT_TOUCH_POSITION,           // param[0]: x, param[1]: y
    INPUT_GESTURE,                  // param[0]: gesture
    // Window events
    WINDOW_CLOSE,                   // no params
    WINDOW_MAXIMIZE,                // no params
    WINDOW_MINIMIZE,                // no params
    WINDOW_RESIZE,                  // param[0]: width, param[1]: height
    // Custom events
    ACTION_TAKE_SCREENSHOT,         // no params
    ACTION_SETTARGETFPS             // param[0]: fps
}

// Event type name strings, required for export
#[cfg(feature = "SUPPORT_AUTOMATION_EVENTS")]
static autoEventTypeName: [&str; 24] = [
    "EVENT_NONE",
    "INPUT_KEY_UP",
    "INPUT_KEY_DOWN",
    "INPUT_KEY_PRESSED",
    "INPUT_KEY_RELEASED",
    "INPUT_MOUSE_BUTTON_UP",
    "INPUT_MOUSE_BUTTON_DOWN",
    "INPUT_MOUSE_POSITION",
    "INPUT_MOUSE_WHEEL_MOTION",
    "INPUT_GAMEPAD_CONNECT",
    "INPUT_GAMEPAD_DISCONNECT",
    "INPUT_GAMEPAD_BUTTON_UP",
    "INPUT_GAMEPAD_BUTTON_DOWN",
    "INPUT_GAMEPAD_AXIS_MOTION",
    "INPUT_TOUCH_UP",
    "INPUT_TOUCH_DOWN",
    "INPUT_TOUCH_POSITION",
    "INPUT_GESTURE",
    "WINDOW_CLOSE",
    "WINDOW_MAXIMIZE",
    "WINDOW_MINIMIZE",
    "WINDOW_RESIZE",
    "ACTION_TAKE_SCREENSHOT",
    "ACTION_SETTARGETFPS"
];

#[cfg(feature = "SUPPORT_AUTOMATION_EVENTS")]
static mut currentEventList: *mut AutomationEventList = std::ptr::null_mut();        // Current automation events list, set by user, keep internal pointer
#[cfg(feature = "SUPPORT_AUTOMATION_EVENTS")]
static mut automationEventRecording: bool = false;               // Recording automation events flag
//static short automationEventEnabled = 0b0000001111111111; // TODO: Automation events enabled for recording/playing

//----------------------------------------------------------------------------------
// Module Functions Definition: Window and Graphics Device
//----------------------------------------------------------------------------------

// Initialize window and OpenGL context
pub unsafe fn InitWindow(width: i32, height: i32, title: &str)
{
    info!("Initializing raylib {}", RAYLIB_VERSION);

#[cfg(feature = "PLATFORM_DESKTOP_GLFW")]
{
    info!("Platform backend: DESKTOP (GLFW)");
}
#[cfg(all(not(any(feature = "PLATFORM_DESKTOP_GLFW")), feature = "PLATFORM_DESKTOP_SDL"))]
{
    info!("Platform backend: DESKTOP (SDL)");
}
#[cfg(all(not(any(feature = "PLATFORM_DESKTOP_GLFW", feature = "PLATFORM_DESKTOP_SDL")), feature = "PLATFORM_DESKTOP_RGFW"))]
{
    info!("Platform backend: DESKTOP (RGFW)");
}
#[cfg(all(not(any(feature = "PLATFORM_DESKTOP_GLFW", feature = "PLATFORM_DESKTOP_SDL", feature = "PLATFORM_DESKTOP_RGFW")), feature = "PLATFORM_DESKTOP_WIN32"))]
{
    info!("Platform backend: DESKTOP (WIN32)");
}
#[cfg(all(not(any(feature = "PLATFORM_DESKTOP_GLFW", feature = "PLATFORM_DESKTOP_SDL", feature = "PLATFORM_DESKTOP_RGFW", feature = "PLATFORM_DESKTOP_WIN32")), feature = "PLATFORM_WEB_RGFW"))]
{
    info!("Platform backend: WEB (RGFW) (HTML5)");
}
#[cfg(all(not(any(feature = "PLATFORM_DESKTOP_GLFW", feature = "PLATFORM_DESKTOP_SDL", feature = "PLATFORM_DESKTOP_RGFW", feature = "PLATFORM_DESKTOP_WIN32", feature = "PLATFORM_WEB_RGFW")), feature = "PLATFORM_WEB"))]
{
    info!("Platform backend: WEB (HTML5)");
}
#[cfg(all(not(any(feature = "PLATFORM_DESKTOP_GLFW", feature = "PLATFORM_DESKTOP_SDL", feature = "PLATFORM_DESKTOP_RGFW", feature = "PLATFORM_DESKTOP_WIN32", feature = "PLATFORM_WEB_RGFW", feature = "PLATFORM_WEB")), feature = "PLATFORM_DRM"))]
{
    info!("Platform backend: NATIVE DRM");
}
#[cfg(all(not(any(feature = "PLATFORM_DESKTOP_GLFW", feature = "PLATFORM_DESKTOP_SDL", feature = "PLATFORM_DESKTOP_RGFW", feature = "PLATFORM_DESKTOP_WIN32", feature = "PLATFORM_WEB_RGFW", feature = "PLATFORM_WEB", feature = "PLATFORM_DRM")), feature = "PLATFORM_ANDROID"))]
{
    info!("Platform backend: ANDROID");
}
#[cfg(all(not(any(feature = "PLATFORM_DESKTOP_GLFW", feature = "PLATFORM_DESKTOP_SDL", feature = "PLATFORM_DESKTOP_RGFW", feature = "PLATFORM_DESKTOP_WIN32", feature = "PLATFORM_WEB_RGFW", feature = "PLATFORM_WEB", feature = "PLATFORM_DRM", feature = "PLATFORM_ANDROID")), feature = "PLATFORM_MEMORY"))]
{
    info!("Platform backend: MEMORY (No OS)");
}
#[cfg(not(any(feature = "PLATFORM_DESKTOP_GLFW", feature = "PLATFORM_DESKTOP_SDL", feature = "PLATFORM_DESKTOP_RGFW", feature = "PLATFORM_DESKTOP_WIN32", feature = "PLATFORM_WEB_RGFW", feature = "PLATFORM_WEB", feature = "PLATFORM_DRM", feature = "PLATFORM_ANDROID", feature = "PLATFORM_MEMORY")))]
{
    // TODO: Include your custom platform backend!
    // i.e software rendering backend or console backend!
    info!("Platform backend: CUSTOM");
}

    info!("Supported raylib modules:");
    info!("    > rcore:..... loaded (mandatory)");
    info!("    > rlgl:...... loaded (mandatory)");
#[cfg(feature = "SUPPORT_MODULE_RSHAPES")]
{
    info!("    > rshapes:... loaded (optional)");
}
#[cfg(not(any(feature = "SUPPORT_MODULE_RSHAPES")))]
{
    info!("    > rshapes:... not loaded (optional)");
}
#[cfg(feature = "SUPPORT_MODULE_RTEXTURES")]
{
    info!("    > rtextures:. loaded (optional)");
}
#[cfg(not(any(feature = "SUPPORT_MODULE_RTEXTURES")))]
{
    info!("    > rtextures:. not loaded (optional)");
}
#[cfg(feature = "SUPPORT_MODULE_RTEXT")]
{
    info!("    > rtext:..... loaded (optional)");
}
#[cfg(not(any(feature = "SUPPORT_MODULE_RTEXT")))]
{
    info!("    > rtext:..... not loaded (optional)");
}
#[cfg(feature = "SUPPORT_MODULE_RMODELS")]
{
    info!("    > rmodels:... loaded (optional)");
}
#[cfg(not(any(feature = "SUPPORT_MODULE_RMODELS")))]
{
    info!("    > rmodels:... not loaded (optional)");
}
#[cfg(feature = "SUPPORT_MODULE_RAUDIO")]
{
    info!("    > raudio:.... loaded (optional)");
}
#[cfg(not(any(feature = "SUPPORT_MODULE_RAUDIO")))]
{
    info!("    > raudio:.... not loaded (optional)");
}

    // Initialize window data
    CORE.Window.screen.x = width as f32;
    CORE.Window.screen.y = height as f32;

    CORE.Window.eventWaiting = false;
    CORE.Window.screenScale = Matrix::identity(); // No draw scaling required by default
    if (!title.is_empty())
    {
        windowTitle = Some(CString::new(title).unwrap());
        CORE.Window.title = windowTitle.as_ref().unwrap().as_ptr().cast_mut();
    }

    // Initialize global input state
    CORE.Input = InputData::default(); // Reset CORE.Input structure to 0
    CORE.Input.Keyboard.exitKey = KeyboardKey::KEY_ESCAPE as i32;
    CORE.Input.Mouse.scale = Vector2 { x: 1.0, y: 1.0 };
    CORE.Input.Mouse.cursor = MouseCursor::MOUSE_CURSOR_ARROW as i32;
    CORE.Input.Gamepad.lastButtonPressed = GamepadButton::GAMEPAD_BUTTON_UNKNOWN as i32;

    // Initialize platform
    //--------------------------------------------------------------
    let result: i32 = InitPlatform();

    if (result != 0)
    {
        warn!("SYSTEM: Failed to initialize platform");
        return;
    }

    // Initialize render dimensions for embedded platforms
    // NOTE: On desktop platforms (GLFW, SDL, etc.), CORE.Window.render.width/height are set during window creation
    // On embedded platforms with no window manager, InitPlatform() doesn't set these values, so they should be initialized
    // here from screen dimensions (which are set from the InitWindow parameters)
    if ((CORE.Window.render.x == 0.0) || (CORE.Window.render.y == 0.0))
    {
        CORE.Window.render.x = CORE.Window.screen.x;
        CORE.Window.render.y = CORE.Window.screen.y;
    }
    //--------------------------------------------------------------

    // Initialize rlgl default data (buffers and shaders)
    // NOTE: Current fbo size stored as globals in rlgl for convenience
    rlglInit(CORE.Window.render.x as i32, CORE.Window.render.y as i32);

    // Setup default viewport
    SetupViewport(CORE.Window.render.x as i32, CORE.Window.render.y as i32);

#[cfg(feature = "SUPPORT_MODULE_RTEXT")]
{
    // Load default font
    // WARNING: External function: Module required: rtext
    LoadFontDefault();
    #[cfg(feature = "SUPPORT_MODULE_RSHAPES")]
    {
    // Set font white rectangle for shapes drawing, so shapes and text can be batched together
    // WARNING: rshapes module is required, if not available, default internal white rectangle is used
    let mut rec: Rectangle = *GetFontDefault().recs.add(95);
    if (((CORE.Window.flags & ConfigFlags::FLAG_MSAA_4X_HINT as u32) == ConfigFlags::FLAG_MSAA_4X_HINT as u32))
    {
        // NOTE: Try to maximize rec padding to avoid pixel bleeding on MSAA filtering
        SetShapesTexture(GetFontDefault().texture, Rectangle { x: rec.x + 2.0, y: rec.y + 2.0, width: 1.0, height: 1.0 });
    }
    else
    {
        // NOTE: Set up a 1px padding on char rectangle to avoid pixel bleeding
        SetShapesTexture(GetFontDefault().texture, Rectangle { x: rec.x + 1.0, y: rec.y + 1.0, width: rec.width - 2.0, height: rec.height - 2.0 });
    }
    }
}
#[cfg(not(any(feature = "SUPPORT_MODULE_RTEXT")))]
{
    #[cfg(feature = "SUPPORT_MODULE_RSHAPES")]
    {
    // Set default texture and rectangle to be used for shapes drawing
    // NOTE: rlgl default texture is a 1x1 pixel UNCOMPRESSED_R8G8B8A8
    let mut texture: Texture2D = Texture2D { id: rlGetTextureIdDefault(), width: 1, height: 1, mipmaps: 1, format: PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32 };
    SetShapesTexture(texture, Rectangle { x: 0.0, y: 0.0, width: 1.0, height: 1.0 });    // WARNING: Module required: rshapes
    }
}

    CORE.Time.frameCounter = 0;
    CORE.Window.shouldClose = false;

    // Initialize random seed using available timer source instead of standard time() on embedded platforms
    // SetRandomSeed() // removed

    info!("SYSTEM: Working Directory: {}", GetWorkingDirectory());
}

// Close window and unload OpenGL context
pub unsafe fn CloseWindow()
{
#[cfg(feature = "SUPPORT_MODULE_RTEXT")]
{
    crate::rtext::UnloadFontDefault();        // WARNING: Module required: rtext
}

    rlglClose();                // De-init rlgl

    // De-initialize platform
    //--------------------------------------------------------------
    ClosePlatform();
    windowTitle = None;
    CORE.Window.title = std::ptr::null_mut();
    //--------------------------------------------------------------

    CORE.Window.ready = false;
    info!("Window closed successfully");
}

// Check if window has been initialized successfully
pub unsafe fn IsWindowReady() -> bool
{
    return CORE.Window.ready;
}

// Check if window is currently fullscreen
pub unsafe fn IsWindowFullscreen() -> bool
{
    return ((CORE.Window.flags & ConfigFlags::FLAG_FULLSCREEN_MODE as u32) == ConfigFlags::FLAG_FULLSCREEN_MODE as u32);
}

// Check if window is currently hidden
pub unsafe fn IsWindowHidden() -> bool
{
    return ((CORE.Window.flags & ConfigFlags::FLAG_WINDOW_HIDDEN as u32) == ConfigFlags::FLAG_WINDOW_HIDDEN as u32);
}

// Check if window has been minimized
pub unsafe fn IsWindowMinimized() -> bool
{
    return ((CORE.Window.flags & ConfigFlags::FLAG_WINDOW_MINIMIZED as u32) == ConfigFlags::FLAG_WINDOW_MINIMIZED as u32);
}

// Check if window has been maximized
pub unsafe fn IsWindowMaximized() -> bool
{
    return ((CORE.Window.flags & ConfigFlags::FLAG_WINDOW_MAXIMIZED as u32) == ConfigFlags::FLAG_WINDOW_MAXIMIZED as u32);
}

// Check if window has the focus
pub unsafe fn IsWindowFocused() -> bool
{
    return !((CORE.Window.flags & ConfigFlags::FLAG_WINDOW_UNFOCUSED as u32) == ConfigFlags::FLAG_WINDOW_UNFOCUSED as u32);
}

// Check if window has been resizedLastFrame
pub unsafe fn IsWindowResized() -> bool
{
    return CORE.Window.resizedLastFrame;
}

// Check if one specific window flag is enabled
pub unsafe fn IsWindowState(flag: u32) -> bool
{
    return ((CORE.Window.flags & flag) == flag);
}

// Get current screen width
pub unsafe fn GetScreenWidth() -> i32
{
    return CORE.Window.screen.x as i32;
}

// Get current screen height
pub unsafe fn GetScreenHeight() -> i32
{
    return CORE.Window.screen.y as i32;
}

// Get current render width which is equal to screen width*dpi scale
pub unsafe fn GetRenderWidth() -> i32
{
    let mut width: i32 = 0;

    if (CORE.Window.usingFbo) { width = CORE.Window.currentFbo.x as i32; }
    else { width = CORE.Window.render.x as i32; }

    return width;
}

// Get current screen height which is equal to screen height*dpi scale
pub unsafe fn GetRenderHeight() -> i32
{
    let mut height: i32 = 0;

    if (CORE.Window.usingFbo) { height = CORE.Window.currentFbo.y as i32; }
    else { height = CORE.Window.render.y as i32; }

    return height;
}

// Enable waiting for events on EndDrawing(), no automatic event polling
pub unsafe fn EnableEventWaiting()
{
    CORE.Window.eventWaiting = true;
}

// Disable waiting for events on EndDrawing(), automatic events polling
pub unsafe fn DisableEventWaiting()
{
    CORE.Window.eventWaiting = false;
}

// Check if cursor is not visible
pub unsafe fn IsCursorHidden() -> bool
{
    return CORE.Input.Mouse.cursorHidden;
}

// Check if cursor is on the current screen
pub unsafe fn IsCursorOnScreen() -> bool
{
    return CORE.Input.Mouse.cursorOnScreen;
}

//----------------------------------------------------------------------------------
// Module Functions Definition: Screen Drawing
//----------------------------------------------------------------------------------

// Clear background (framebuffer) to color
pub unsafe fn ClearBackground(color: Color)
{
    rlClearColor(color.r, color.g, color.b, color.a);   // Set clear color
    rlClearScreenBuffers();                             // Clear current framebuffers
}

// Begin canvas (framebuffer) drawing
pub unsafe fn BeginDrawing()
{
    // WARNING: Previously to BeginDrawing() other render textures drawing could happen,
    // consequently the measure for update vs draw is not accurate (only the total frame time is accurate)

    CORE.Time.current = GetTime();      // Number of elapsed seconds since InitTimer()
    CORE.Time.update = CORE.Time.current - CORE.Time.previous;
    CORE.Time.previous = CORE.Time.current;

    rlLoadIdentity();                   // Reset current matrix (modelview)
    rlMultMatrixf(CORE.Window.screenScale.to_array().as_ptr()); // Apply screen scaling

    //rlTranslatef(0.375, 0.375, 0);    // HACK to have 2D pixel-perfect drawing on OpenGL 1.1
                                        // NOTE: Not required with OpenGL 3.3+
}

// End canvas (framebuffer) drawing and swap buffers (double buffering)
pub unsafe fn EndDrawing()
{
    rlDrawRenderBatchActive();      // Update and draw internal render batch

#[cfg(feature = "SUPPORT_AUTOMATION_EVENTS")]
{
    if (automationEventRecording) { RecordAutomationEvent(); }    // Event recording
}

#[cfg(not(feature = "SUPPORT_CUSTOM_FRAME_CONTROL"))]
{
    SwapScreenBuffer();                  // Copy back buffer to front buffer (screen)

    // Frame time control system
    CORE.Time.current = GetTime();
    CORE.Time.draw = CORE.Time.current - CORE.Time.previous;
    CORE.Time.previous = CORE.Time.current;

    CORE.Time.frame = CORE.Time.update + CORE.Time.draw;

    // Wait for some milliseconds...
    if (CORE.Time.frame < CORE.Time.target)
    {
        WaitTime(CORE.Time.target - CORE.Time.frame);

        CORE.Time.current = GetTime();
        let waitTime: f64 = CORE.Time.current - CORE.Time.previous;
        CORE.Time.previous = CORE.Time.current;

        CORE.Time.frame += waitTime;    // Total frame time: update + draw + wait
    }

    PollInputEvents();      // Poll user events (before next frame update)
}

#[cfg(feature = "SUPPORT_SCREEN_CAPTURE")]
{
    if (IsKeyPressed(KeyboardKey::KEY_F12 as i32))
    {
        TakeScreenshot(&format!("screenshot{:03}.png", screenshotCounter));
        screenshotCounter += 1;
    }
}

    CORE.Time.frameCounter += 1;
}

// Initialize 2D mode with custom camera (2D)
pub unsafe fn BeginMode2D(camera: Camera2D)
{
    rlDrawRenderBatchActive();      // Update and draw internal render batch

    rlLoadIdentity();               // Reset current matrix (modelview)

    // Apply 2d camera transformation to modelview
    rlMultMatrixf(GetCameraMatrix2D(camera).to_array().as_ptr());
}

// End 2D mode with custom camera
pub unsafe fn EndMode2D()
{
    rlDrawRenderBatchActive();      // Update and draw internal render batch

    rlLoadIdentity();               // Reset current matrix (modelview)

    if (rlGetActiveFramebuffer() == 0) { rlMultMatrixf(CORE.Window.screenScale.to_array().as_ptr()); } // Apply screen scaling if required
}

// Initializes 3D mode with custom camera (3D)
pub unsafe fn BeginMode3D(camera: Camera)
{
    rlDrawRenderBatchActive();      // Update and draw internal render batch

    rlMatrixMode(RL_PROJECTION);    // Switch to projection matrix
    rlPushMatrix();                 // Save previous matrix, which contains the settings for the 2d ortho projection
    rlLoadIdentity();               // Reset current matrix (projection)

    let aspect: f32 = (CORE.Window.currentFbo.x as f32)/(CORE.Window.currentFbo.y as f32);

    // NOTE: zNear and zFar values are important when computing depth buffer values
    if (camera.projection == CameraProjection::Perspective as i32)
    {
        // Setup perspective projection
        let top: f64 = rlGetCullDistanceNear()*(camera.fovy as f64*0.5*DEG2RAD as f64).tan();
        let right: f64 = top*aspect as f64;

        rlFrustum(-right, right, -top, top, rlGetCullDistanceNear(), rlGetCullDistanceFar());
    }
    else if (camera.projection == CameraProjection::Orthographic as i32)
    {
        // Setup orthographic projection
        let top: f64 = camera.fovy as f64/2.0;
        let right: f64 = top*aspect as f64;

        rlOrtho(-right, right, -top,top, rlGetCullDistanceNear(), rlGetCullDistanceFar());
    }

    rlMatrixMode(RL_MODELVIEW);     // Switch back to modelview matrix
    rlLoadIdentity();               // Reset current matrix (modelview)

    // Setup Camera view
    let matView: Matrix = Matrix::look_at(camera.position, camera.target, camera.up);
    rlMultMatrixf(matView.to_array().as_ptr());      // Multiply modelview matrix by view matrix (camera)

    rlEnableDepthTest();            // Enable DEPTH_TEST for 3D
}

// End 3D mode and returns to default 2D orthographic mode
pub unsafe fn EndMode3D()
{
    rlDrawRenderBatchActive();      // Update and draw internal render batch

    rlMatrixMode(RL_PROJECTION);    // Switch to projection matrix
    rlPopMatrix();                  // Restore previous matrix (projection) from matrix stack

    rlMatrixMode(RL_MODELVIEW);     // Switch back to modelview matrix
    rlLoadIdentity();               // Reset current matrix (modelview)

    if (rlGetActiveFramebuffer() == 0) { rlMultMatrixf(CORE.Window.screenScale.to_array().as_ptr()); } // Apply screen scaling if required

    rlDisableDepthTest();           // Disable DEPTH_TEST for 2D
}

// Initializes render texture for drawing
pub unsafe fn BeginTextureMode(target: RenderTexture2D)
{
    rlDrawRenderBatchActive();      // Update and draw internal render batch

    rlEnableFramebuffer(target.id); // Enable render target

    // Set viewport and RLGL internal framebuffer size
    rlViewport(0, 0, target.texture.width, target.texture.height);
    rlSetFramebufferWidth(target.texture.width);
    rlSetFramebufferHeight(target.texture.height);

    rlMatrixMode(RL_PROJECTION);    // Switch to projection matrix
    rlLoadIdentity();               // Reset current matrix (projection)

    // Set orthographic projection to current framebuffer size
    // NOTE: Configured top-left corner as (0, 0)
    rlOrtho(0.0, target.texture.width as f64, target.texture.height as f64, 0.0, 0.0, 1.0);

    rlMatrixMode(RL_MODELVIEW);     // Switch back to modelview matrix
    rlLoadIdentity();               // Reset current matrix (modelview)

    //rlScalef(0.0f, -1.0f, 0.0f);  // Flip Y-drawing (?)

    // Setup current width/height for proper aspect ratio
    // calculation when using BeginTextureMode()
    CORE.Window.currentFbo.x = target.texture.width as f32;
    CORE.Window.currentFbo.y = target.texture.height as f32;
    CORE.Window.usingFbo = true;
}

// End drawing to render texture
pub unsafe fn EndTextureMode()
{
    rlDrawRenderBatchActive();      // Update and draw internal render batch

    rlDisableFramebuffer();         // Disable render target (fbo)

    // Set viewport to default framebuffer size
    SetupViewport(CORE.Window.render.x as i32, CORE.Window.render.y as i32);

    // Go back to the modelview state from BeginDrawing, back to the main framebuffer
    rlMatrixMode(RL_MODELVIEW);     // Switch back to modelview matrix
    rlLoadIdentity();               // Reset current matrix (modelview)
    rlMultMatrixf(CORE.Window.screenScale.to_array().as_ptr()); // Apply screen scaling if required

    // Reset current fbo to screen size
    CORE.Window.currentFbo.x = CORE.Window.render.x;
    CORE.Window.currentFbo.y = CORE.Window.render.y;
    CORE.Window.usingFbo = false;
}

// Begin custom shader mode
pub unsafe fn BeginShaderMode(shader: Shader)
{
    rlSetShader(shader.id, shader.locs);
}

// End custom shader mode (returns to default shader)
pub unsafe fn EndShaderMode()
{
    rlSetShader(rlGetShaderIdDefault(), rlGetShaderLocsDefault());
}

// Begin blending mode (alpha, additive, multiplied, subtract, custom)
// NOTE: Blend modes supported are enumerated in BlendMode enum
pub unsafe fn BeginBlendMode(mode: i32)
{
    rlSetBlendMode(mode);
}

// End blending mode (reset to default: alpha blending)
pub unsafe fn EndBlendMode()
{
    rlSetBlendMode(BlendMode::BLEND_ALPHA as i32);
}

// Begin scissor mode (define screen area for following drawing)
// NOTE: Scissor rec refers to bottom-left corner, changing it to upper-left
pub unsafe fn BeginScissorMode(x: i32, y: i32, width: i32, height: i32)
{
    rlDrawRenderBatchActive();      // Update and draw internal render batch

    rlEnableScissorTest();

#[cfg(target_vendor = "apple")]
    if (!CORE.Window.usingFbo)
    {
        let scale = GetWindowScaleDPI();
        rlScissor((x as f32*scale.x) as i32, (GetScreenHeight() as f32*scale.y - (y + height) as f32*scale.y) as i32, (width as f32*scale.x) as i32, (height as f32*scale.y) as i32);
    }
    else
    {
        rlScissor(x, CORE.Window.currentFbo.y as i32 - (y + height), width, height);
    }
    #[cfg(not(target_vendor = "apple"))]
    if (!CORE.Window.usingFbo && ((CORE.Window.flags & ConfigFlags::FLAG_WINDOW_HIGHDPI as u32) == ConfigFlags::FLAG_WINDOW_HIGHDPI as u32))
    {
        let scale = GetWindowScaleDPI();
        rlScissor((x as f32*scale.x) as i32, (CORE.Window.currentFbo.y - (y + height) as f32*scale.y) as i32, (width as f32*scale.x) as i32, (height as f32*scale.y) as i32);
    }
    else
    {
        rlScissor(x, CORE.Window.currentFbo.y as i32 - (y + height), width, height);
    }
}

// End scissor mode
pub unsafe fn EndScissorMode()
{
    rlDrawRenderBatchActive();      // Update and draw internal render batch
    rlDisableScissorTest();
}

//----------------------------------------------------------------------------------
// Module Functions Definition: VR Stereo Rendering
//----------------------------------------------------------------------------------

// Begin VR drawing configuration
pub unsafe fn BeginVrStereoMode(config: VrStereoConfig)
{
    rlEnableStereoRender();

    // Set stereo render matrices
    rlSetMatrixProjectionStereo(config.projection[0], config.projection[1]);
    rlSetMatrixViewOffsetStereo(config.viewOffset[0], config.viewOffset[1]);
}

// End VR drawing process (and desktop mirror)
pub unsafe fn EndVrStereoMode()
{
    rlDisableStereoRender();
}

// Load VR stereo config for VR simulator device parameters
pub unsafe fn LoadVrStereoConfig(device: VrDeviceInfo) -> VrStereoConfig
{
    let mut config: VrStereoConfig = unsafe { std::mem::zeroed() };

    if (rlGetVersion() != rlGlVersion::RL_OPENGL_11)
    {
        // Compute aspect ratio
        let aspect: f32 = ((device.hResolution as f32)*0.5)/(device.vResolution as f32);

        // Compute lens parameters
        let lensShift: f32 = (device.hScreenSize*0.25 - device.lensSeparationDistance*0.5)/device.hScreenSize;
        config.leftLensCenter[0] = 0.25 + lensShift;
        config.leftLensCenter[1] = 0.5;
        config.rightLensCenter[0] = 0.75 - lensShift;
        config.rightLensCenter[1] = 0.5;
        config.leftScreenCenter[0] = 0.25;
        config.leftScreenCenter[1] = 0.5;
        config.rightScreenCenter[0] = 0.75;
        config.rightScreenCenter[1] = 0.5;

        // Compute distortion scale parameters
        // NOTE: To get lens max radius, lensShift must be normalized to [-1..1]
        let lensRadius: f32 = (-1.0 - 4.0*lensShift).abs();
        let lensRadiusSq: f32 = lensRadius*lensRadius;
        let distortionScale: f32 = device.lensDistortionValues[0] +
                                device.lensDistortionValues[1]*lensRadiusSq +
                                device.lensDistortionValues[2]*lensRadiusSq*lensRadiusSq +
                                device.lensDistortionValues[3]*lensRadiusSq*lensRadiusSq*lensRadiusSq;

        let normScreenWidth: f32 = 0.5;
        let normScreenHeight: f32 = 1.0;
        config.scaleIn[0] = 2.0/normScreenWidth;
        config.scaleIn[1] = 2.0/normScreenHeight/aspect;
        config.scale[0] = normScreenWidth*0.5/distortionScale;
        config.scale[1] = normScreenHeight*0.5*aspect/distortionScale;

        // Fovy is normally computed with: 2*atan2f(device.vScreenSize, 2*device.eyeToScreenDistance)
        // ...but with lens distortion it is increased (see Oculus SDK Documentation)
        let fovy: f32 = 2.0*(device.vScreenSize*0.5*distortionScale).atan2(device.eyeToScreenDistance);     // Really need distortionScale?
       // float fovy = 2.0f*(float)atan2f(device.vScreenSize*0.5f, device.eyeToScreenDistance);

        // Compute camera projection matrices
        let projOffset: f32 = 4.0*lensShift;      // Scaled to projection space coordinates [-1..1]
        let proj: Matrix = Matrix::perspective(fovy as f64, aspect as f64, rlGetCullDistanceNear(), rlGetCullDistanceFar());

        config.projection[0] = Matrix::multiply(proj, Matrix::translate(projOffset, 0.0, 0.0));
        config.projection[1] = Matrix::multiply(proj, Matrix::translate(-projOffset, 0.0, 0.0));

        // Compute camera transformation matrices
        // NOTE: Camera movement might seem more natural if modelling the head
        // Axis of rotation is the base of the head, so adding some y (base of head to eye level
        // and -z (center of head to eye protrusion) to the camera positions
        config.viewOffset[0] = Matrix::translate(device.interpupillaryDistance*0.5, 0.075, 0.045);
        config.viewOffset[1] = Matrix::translate(-device.interpupillaryDistance*0.5, 0.075, 0.045);

        // Compute eyes Viewports
        /*
        config.eyeViewportRight[0] = 0;
        config.eyeViewportRight[1] = 0;
        config.eyeViewportRight[2] = device.hResolution/2;
        config.eyeViewportRight[3] = device.vResolution;

        config.eyeViewportLeft[0] = device.hResolution/2;
        config.eyeViewportLeft[1] = 0;
        config.eyeViewportLeft[2] = device.hResolution/2;
        config.eyeViewportLeft[3] = device.vResolution;
        */
    }
    else { warn!("RLGL: VR Simulator not supported on OpenGL 1.1"); }

    return config;
}

// Unload VR stereo config properties
pub fn UnloadVrStereoConfig(config: VrStereoConfig)
{
    info!("UnloadVrStereoConfig not implemented in rcore.c");
}

//----------------------------------------------------------------------------------
// Module Functions Definition: Shaders Management
//----------------------------------------------------------------------------------

// Load shader from files and bind default locations
// NOTE: If shader filename is NULL, using default vertex/fragment shaders
pub unsafe fn LoadShader(vsFileName: Option<&str>, fsFileName: Option<&str>) -> Shader
{
    let mut shader: Shader = unsafe { std::mem::zeroed() };

    let mut vShaderStr: Option<String> = None;
    let mut fShaderStr: Option<String> = None;

    if let Some(vsFileName) = vsFileName { vShaderStr = LoadFileText(vsFileName); }
    if let Some(fsFileName) = fsFileName { fShaderStr = LoadFileText(fsFileName); }

    if (vShaderStr.is_none() && fShaderStr.is_none()) { warn!("SHADER: Shader files provided are not valid, using default shader"); }

    shader = LoadShaderFromMemory(vShaderStr.as_deref(), fShaderStr.as_deref());

    UnloadFileText(vShaderStr);
    UnloadFileText(fShaderStr);

    return shader;
}

// Load shader from code strings and bind default locations
pub unsafe fn LoadShaderFromMemory(vsCode: Option<&str>, fsCode: Option<&str>) -> Shader
{
    let mut shader: Shader = unsafe { std::mem::zeroed() };

    let vsCode = vsCode.map(|code| CString::new(code).unwrap());
    let fsCode = fsCode.map(|code| CString::new(code).unwrap());
    shader.id = rlLoadShaderProgram(vsCode.as_ref().map_or(std::ptr::null(), |code| code.as_ptr()), fsCode.as_ref().map_or(std::ptr::null(), |code| code.as_ptr()));

    if (shader.id == 0)
    {
        // Shader could not be loaded but still loading the location points to avoid potential crashes
        // NOTE: All locations set to -1 (no location found)
        shader.locs = libc::calloc(RL_MAX_SHADER_LOCATIONS, std::mem::size_of::<i32>()) as *mut i32;
        for i in 0..RL_MAX_SHADER_LOCATIONS { *shader.locs.add((i) as usize) = -1; }
    }
    else if (shader.id == rlGetShaderIdDefault()) { shader.locs = rlGetShaderLocsDefault(); }
    else if (shader.id > 0)
    {
        // After custom shader loading, trying to set default location names
        // Default shader attribute locations have been binded before linking:
        //  - vertex position location    = 0
        //  - vertex texcoord location    = 1
        //  - vertex normal location      = 2
        //  - vertex color location       = 3
        //  - vertex tangent location     = 4
        //  - vertex texcoord2 location   = 5
        //  - vertex boneIndices location = 6
        //  - vertex boneWeights location = 7

        // NOTE: If any location is not found, loc point becomes -1

        // Load shader locations array
        // NOTE: All locations set to -1 (no location)
        shader.locs = libc::calloc(RL_MAX_SHADER_LOCATIONS, std::mem::size_of::<i32>()) as *mut i32;
        for i in 0..RL_MAX_SHADER_LOCATIONS { *shader.locs.add((i) as usize) = -1; }

        // Get handles to GLSL input attribute locations
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_VERTEX_POSITION as i32) as usize) = rlGetLocationAttrib(shader.id, RL_DEFAULT_SHADER_ATTRIB_NAME_POSITION.as_ptr());
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_VERTEX_TEXCOORD01 as i32) as usize) = rlGetLocationAttrib(shader.id, RL_DEFAULT_SHADER_ATTRIB_NAME_TEXCOORD.as_ptr());
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_VERTEX_TEXCOORD02 as i32) as usize) = rlGetLocationAttrib(shader.id, RL_DEFAULT_SHADER_ATTRIB_NAME_TEXCOORD2.as_ptr());
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_VERTEX_NORMAL as i32) as usize) = rlGetLocationAttrib(shader.id, RL_DEFAULT_SHADER_ATTRIB_NAME_NORMAL.as_ptr());
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_VERTEX_TANGENT as i32) as usize) = rlGetLocationAttrib(shader.id, RL_DEFAULT_SHADER_ATTRIB_NAME_TANGENT.as_ptr());
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_VERTEX_COLOR as i32) as usize) = rlGetLocationAttrib(shader.id, RL_DEFAULT_SHADER_ATTRIB_NAME_COLOR.as_ptr());
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_VERTEX_BONEIDS as i32) as usize) = rlGetLocationAttrib(shader.id, RL_DEFAULT_SHADER_ATTRIB_NAME_BONEINDICES.as_ptr());
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_VERTEX_BONEWEIGHTS as i32) as usize) = rlGetLocationAttrib(shader.id, RL_DEFAULT_SHADER_ATTRIB_NAME_BONEWEIGHTS.as_ptr());
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_VERTEX_INSTANCETRANSFORM as i32) as usize) = rlGetLocationAttrib(shader.id, RL_DEFAULT_SHADER_ATTRIB_NAME_INSTANCETRANSFORM.as_ptr());

        // Get handles to GLSL uniform locations (vertex shader)
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_MATRIX_MVP as i32) as usize) = rlGetLocationUniform(shader.id, RL_DEFAULT_SHADER_UNIFORM_NAME_MVP.as_ptr());
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_MATRIX_VIEW as i32) as usize) = rlGetLocationUniform(shader.id, RL_DEFAULT_SHADER_UNIFORM_NAME_VIEW.as_ptr());
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_MATRIX_PROJECTION as i32) as usize) = rlGetLocationUniform(shader.id, RL_DEFAULT_SHADER_UNIFORM_NAME_PROJECTION.as_ptr());
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_MATRIX_MODEL as i32) as usize) = rlGetLocationUniform(shader.id, RL_DEFAULT_SHADER_UNIFORM_NAME_MODEL.as_ptr());
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_MATRIX_NORMAL as i32) as usize) = rlGetLocationUniform(shader.id, RL_DEFAULT_SHADER_UNIFORM_NAME_NORMAL.as_ptr());
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_MATRIX_BONETRANSFORMS as i32) as usize) = rlGetLocationUniform(shader.id, RL_DEFAULT_SHADER_UNIFORM_NAME_BONEMATRICES.as_ptr());

        // Get handles to GLSL uniform locations (fragment shader)
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_COLOR_DIFFUSE as i32) as usize) = rlGetLocationUniform(shader.id, RL_DEFAULT_SHADER_UNIFORM_NAME_COLOR.as_ptr());
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_MAP_ALBEDO as i32) as usize) = rlGetLocationUniform(shader.id, RL_DEFAULT_SHADER_SAMPLER2D_NAME_TEXTURE0.as_ptr());  // SHADER_LOC_MAP_ALBEDO
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_MAP_METALNESS as i32) as usize) = rlGetLocationUniform(shader.id, RL_DEFAULT_SHADER_SAMPLER2D_NAME_TEXTURE1.as_ptr()); // SHADER_LOC_MAP_METALNESS
        *shader.locs.add((ShaderLocationIndex::SHADER_LOC_MAP_NORMAL as i32) as usize) = rlGetLocationUniform(shader.id, RL_DEFAULT_SHADER_SAMPLER2D_NAME_TEXTURE2.as_ptr());
    }

    return shader;
}

// Check if shader is valid (loaded on GPU)
pub fn IsShaderValid(shader: Shader) -> bool
{
    return ((shader.id > 0) &&          // Validate shader id (GPU loaded successfully)
            (!shader.locs.is_null()));     // Validate memory has been allocated for default shader locations

    // The following locations are tried to be set automatically (locs[i] >= 0),
    // any of them can be checked for validation but the only mandatory one is, afaik, SHADER_LOC_VERTEX_POSITION
    // NOTE: Users can also setup manually their own attributes/uniforms and do not used the default raylib ones

    // Vertex shader attribute locations (default)
    // shader.locs[SHADER_LOC_VERTEX_POSITION]      // Set by default internal shader
    // shader.locs[SHADER_LOC_VERTEX_TEXCOORD01]    // Set by default internal shader
    // shader.locs[SHADER_LOC_VERTEX_TEXCOORD02]
    // shader.locs[SHADER_LOC_VERTEX_NORMAL]
    // shader.locs[SHADER_LOC_VERTEX_TANGENT]
    // shader.locs[SHADER_LOC_VERTEX_COLOR]         // Set by default internal shader

    // Vertex shader uniform locations (default)
    // shader.locs[SHADER_LOC_MATRIX_MVP]           // Set by default internal shader
    // shader.locs[SHADER_LOC_MATRIX_VIEW]
    // shader.locs[SHADER_LOC_MATRIX_PROJECTION]
    // shader.locs[SHADER_LOC_MATRIX_MODEL]
    // shader.locs[SHADER_LOC_MATRIX_NORMAL]

    // Fragment shader uniform locations (default)
    // shader.locs[SHADER_LOC_COLOR_DIFFUSE]        // Set by default internal shader
    // shader.locs[SHADER_LOC_MAP_DIFFUSE]          // Set by default internal shader
    // shader.locs[SHADER_LOC_MAP_SPECULAR]
    // shader.locs[SHADER_LOC_MAP_NORMAL]
}

// Unload shader from GPU memory (VRAM)
pub unsafe fn UnloadShader(shader: Shader)
{
    if (shader.id != rlGetShaderIdDefault())
    {
        rlUnloadShaderProgram(shader.id);

        // NOTE: If shader loading failed, it should be 0
        libc::free(shader.locs.cast());
    }
}

// Get shader uniform location
pub unsafe fn GetShaderLocation(shader: Shader, uniformName: &str) -> i32
{
    return rlGetLocationUniform(shader.id, CString::new(uniformName).unwrap().as_ptr());
}

// Get shader attribute location
pub unsafe fn GetShaderLocationAttrib(shader: Shader, attribName: &str) -> i32
{
    return rlGetLocationAttrib(shader.id, CString::new(attribName).unwrap().as_ptr());
}

// Set shader uniform value
pub unsafe fn SetShaderValue(shader: Shader, locIndex: i32, value: *const std::ffi::c_void, uniformType: i32)
{
    SetShaderValueV(shader, locIndex, value, uniformType, 1);
}

// Set shader uniform value vector
pub unsafe fn SetShaderValueV(shader: Shader, locIndex: i32, value: *const std::ffi::c_void, uniformType: i32, count: i32)
{
    if (locIndex > -1)
    {
        rlEnableShader(shader.id);
        rlSetUniform(locIndex, value, uniformType, count);
        //rlDisableShader();      // Avoid resetting current shader program, in case other uniforms are set
    }
}

// Set shader uniform value (matrix 4x4)
pub unsafe fn SetShaderValueMatrix(shader: Shader, locIndex: i32, mat: Matrix)
{
    if (locIndex > -1)
    {
        rlEnableShader(shader.id);
        rlSetUniformMatrix(locIndex, mat);
        //rlDisableShader();
    }
}

// Set shader uniform value for texture
pub unsafe fn SetShaderValueTexture(shader: Shader, locIndex: i32, texture: Texture2D)
{
    if (locIndex > -1)
    {
        rlEnableShader(shader.id);
        rlSetUniformSampler(locIndex, texture.id);
        //rlDisableShader();
    }
}

//----------------------------------------------------------------------------------
// Module Functions Definition: Screen-space Queries
//----------------------------------------------------------------------------------

// Get a ray trace from screen position (i.e mouse)
pub unsafe fn GetScreenToWorldRay(position: Vector2, camera: Camera) -> Ray
{
    let ray: Ray = GetScreenToWorldRayEx(position, camera, GetScreenWidth(), GetScreenHeight());

    return ray;
}

// Get a ray trace from the screen position (i.e mouse) within a specific section of the screen
pub unsafe fn GetScreenToWorldRayEx(position: Vector2, camera: Camera, width: i32, height: i32) -> Ray
{
    let mut ray: Ray = unsafe { std::mem::zeroed() };

    // Calculate normalized device coordinates
    // NOTE: y value is negative
    let x: f32 = (2.0*position.x)/(width as f32) - 1.0;
    let y: f32 = 1.0 - (2.0*position.y)/(height as f32);
    let z: f32 = 1.0;

    // Store values in a vector
    let deviceCoords: Vector3 = Vector3 { x: x, y: y, z: z };

    // Calculate view matrix from camera look at
    let matView: Matrix = Matrix::look_at(camera.position, camera.target, camera.up);

    let mut matProj: Matrix = Matrix::identity();

    if (camera.projection == CameraProjection::Perspective as i32)
    {
        // Calculate projection matrix from perspective
        matProj = Matrix::perspective((camera.fovy*DEG2RAD) as f64, ((width as f64)/(height as f64)), rlGetCullDistanceNear(), rlGetCullDistanceFar());
    }
    else if (camera.projection == CameraProjection::Orthographic as i32)
    {
        let aspect: f64 = (width as f64)/(height as f64);
        let top: f64 = camera.fovy as f64/2.0;
        let right: f64 = top*aspect as f64;

        // Calculate projection matrix from orthographic
        matProj = Matrix::ortho(-right, right, -top, top, rlGetCullDistanceNear(), rlGetCullDistanceFar());
    }

    // Unproject far/near points
    let nearPoint: Vector3 = Vector3Unproject(Vector3 { x: deviceCoords.x, y: deviceCoords.y, z: 0.0 }, matProj, matView);
    let farPoint: Vector3 = Vector3Unproject(Vector3 { x: deviceCoords.x, y: deviceCoords.y, z: 1.0 }, matProj, matView);

    // Unproject the mouse cursor in the near plane
    // It is needed as the source position because orthographic projects,
    // compared to perspective doesn't have a convergence point,
    // meaning that the "eye" of the camera is more like a plane than a point
    let cameraPlanePointerPos: Vector3 = Vector3Unproject(Vector3 { x: deviceCoords.x, y: deviceCoords.y, z: -1.0 }, matProj, matView);

    // Calculate normalized direction vector
    let direction: Vector3 = (farPoint - nearPoint).normalize_or_zero();

    if (camera.projection == CameraProjection::Perspective as i32) { ray.position = camera.position; }
    else if (camera.projection == CameraProjection::Orthographic as i32) { ray.position = cameraPlanePointerPos; }

    // Apply calculated vectors to ray
    ray.direction = direction;

    return ray;
}

// Get transform matrix for camera
pub fn GetCameraMatrix(camera: Camera) -> Matrix
{
    let mat: Matrix = Matrix::look_at(camera.position, camera.target, camera.up);

    return mat;
}

// Get camera 2d transform matrix
pub fn GetCameraMatrix2D(camera: Camera2D) -> Matrix
{
    let mut matTransform: Matrix = Matrix::ZERO;
    // The camera in world-space is set by
    //   1. Move it to target
    //   2. Rotate by -rotation and scale by (1/zoom)
    //      When setting higher scale, it's more intuitive for the world to become bigger (= camera become smaller),
    //      not for the camera getting bigger, hence the invert. Same deal with rotation
    //   3. Move it by (-offset);
    //      Offset defines target transform relative to screen, but since effectively "moving" screen (camera)
    //      it needs to be moved into opposite direction (inverse transform)

    // Having camera transform in world-space, inverse of it gives the modelview transform
    // Since (A*B*C)' = C'*B'*A', the modelview is
    //   1. Move to offset
    //   2. Rotate and Scale
    //   3. Move by -target
    let matOrigin: Matrix = Matrix::translate(-camera.target.x, -camera.target.y, 0.0);
    let matRotation: Matrix = Matrix::rotate(Vector3 { x: 0.0, y: 0.0, z: 1.0 }, camera.rotation*DEG2RAD);
    let matScale: Matrix = Matrix::scale(camera.zoom, camera.zoom, 1.0);
    let matTranslation: Matrix = Matrix::translate(camera.offset.x, camera.offset.y, 0.0);

    matTransform = Matrix::multiply(Matrix::multiply(matOrigin, Matrix::multiply(matScale, matRotation)), matTranslation);

    return matTransform;
}

// Get screen space position from a 3d world space position
pub unsafe fn GetWorldToScreen(position: Vector3, camera: Camera) -> Vector2
{
    let screenPosition: Vector2 = GetWorldToScreenEx(position, camera, GetScreenWidth(), GetScreenHeight());

    return screenPosition;
}

// Get sized screen space position for a 3d world space position (useful for texture drawing)
pub unsafe fn GetWorldToScreenEx(position: Vector3, camera: Camera, width: i32, height: i32) -> Vector2
{
    // Calculate projection matrix (from perspective instead of frustum
    let mut matProj: Matrix = Matrix::identity();

    if (camera.projection == CameraProjection::Perspective as i32)
    {
        // Calculate projection matrix from perspective
        matProj = Matrix::perspective((camera.fovy*DEG2RAD) as f64, ((width as f64)/(height as f64)), rlGetCullDistanceNear(), rlGetCullDistanceFar());
    }
    else if (camera.projection == CameraProjection::Orthographic as i32)
    {
        let aspect: f64 = (width as f64)/(height as f64);
        let top: f64 = camera.fovy as f64/2.0;
        let right: f64 = top*aspect as f64;

        // Calculate projection matrix from orthographic
        matProj = Matrix::ortho(-right, right, -top, top, rlGetCullDistanceNear(), rlGetCullDistanceFar());
    }

    // Calculate view matrix from camera look at (and transpose it)
    let matView: Matrix = Matrix::look_at(camera.position, camera.target, camera.up);

    // Convert world position vector to quaternion
    let mut worldPos: Quaternion = Quaternion::new(position.x, position.y, position.z, 1.0);

    // Transform world position to view
    worldPos = QuaternionTransform(worldPos, matView);

    // Transform result to projection (clip space position)
    worldPos = QuaternionTransform(worldPos, matProj);

    // Calculate normalized device coordinates (inverted y)
    let ndcPos: Vector3 = Vector3 { x: worldPos.x/worldPos.w, y: -worldPos.y/worldPos.w, z: worldPos.z/worldPos.w };

    // Calculate 2d screen position vector
    let screenPosition: Vector2 = Vector2 { x: (ndcPos.x + 1.0)/2.0*(width as f32), y: (ndcPos.y + 1.0)/2.0*(height as f32) };

    return screenPosition;
}

// Get screen space position for a 2d camera world space position
pub unsafe fn GetWorldToScreen2D(position: Vector2, camera: Camera2D) -> Vector2
{
    let matCamera: Matrix = GetCameraMatrix2D(camera);
    let transform: Vector3 = Vector3Transform(Vector3 { x: position.x, y: position.y, z: 0.0 }, matCamera);

    return Vector2 { x: transform.x, y: transform.y };
}

// Get world space position for a 2d camera screen space position
pub unsafe fn GetScreenToWorld2D(position: Vector2, camera: Camera2D) -> Vector2
{
    let invMatCamera: Matrix = Matrix::invert(GetCameraMatrix2D(camera));
    let transform: Vector3 = Vector3Transform(Vector3 { x: position.x, y: position.y, z: 0.0 }, invMatCamera);

    return Vector2 { x: transform.x, y: transform.y };
}

//----------------------------------------------------------------------------------
// Module Functions Definition: Timing
//----------------------------------------------------------------------------------

// NOTE: Functions with a platform-specific implementation on rcore_<platform>.c
//double GetTime(void)

// Set target FPS (maximum)
pub unsafe fn SetTargetFPS(fps: i32)
{
    if (fps < 1) { CORE.Time.target = 0.0; }
    else { CORE.Time.target = 1.0/(fps as f64); }

    info!("TIMER: Target time per frame: {:.3} milliseconds", (CORE.Time.target as f32)*1000.0);
}

// Get current FPS
// NOTE: Calculating an average framerate
pub unsafe fn GetFPS() -> i32
{
    let mut fps: i32 = 0;

#[cfg(not(feature = "SUPPORT_CUSTOM_FRAME_CONTROL"))]
{
    const FPS_CAPTURE_FRAMES_COUNT: usize = 30;      // 30 captures
    const FPS_AVERAGE_TIME_SECONDS: f32 = 0.5;     // 500 milliseconds
    const FPS_STEP: f32 = FPS_AVERAGE_TIME_SECONDS/FPS_CAPTURE_FRAMES_COUNT as f32;

    static mut index: usize = 0;
    static mut history: [f32; FPS_CAPTURE_FRAMES_COUNT] = [0.0; FPS_CAPTURE_FRAMES_COUNT];
    static mut average: f32 = 0.0;
    static mut last: f32 = 0.0;
    let fpsFrame: f32 = GetFrameTime();

    // If reseting the window, reset the FPS info
    if (CORE.Time.frameCounter == 0)
    {
        average = 0.0;
        last = 0.0;
        index = 0;

        for i in 0..FPS_CAPTURE_FRAMES_COUNT { history[(i) as usize] = 0.0; }
    }

    if (fpsFrame != 0.0)
    {
        if ((GetTime() - last as f64) > FPS_STEP as f64)
        {
            last = (GetTime() as f32);
            index = (index + 1)%FPS_CAPTURE_FRAMES_COUNT;
            average -= history[(index) as usize];
            history[(index) as usize] = fpsFrame/FPS_CAPTURE_FRAMES_COUNT as f32;
            average += history[(index) as usize];
        }

        fps = ((1.0/average).round() as i32);
    }
    else { fps = 0; }
}

    return fps;
}

// Get time in seconds for last frame drawn (delta time)
pub unsafe fn GetFrameTime() -> f32
{
    return (CORE.Time.frame as f32);
}

//----------------------------------------------------------------------------------
// Module Functions Definition: Custom frame control
//----------------------------------------------------------------------------------

// NOTE: Functions with a platform-specific implementation on rcore_<platform>.c
//void SwapScreenBuffer(void);
//void PollInputEvents(void);

// Wait for some time (stop program execution)
// NOTE: Sleep() granularity could be around 10 ms, it means, Sleep() could
// take longer than expected... for that reason a busy wait loop is used
// REF: http://stackoverflow.com/questions/43057578/c-programming-win32-games-sleep-taking-longer-than-expected
// REF: http://www.geisswerks.com/ryan/FAQS/timing.html --> All about timing on Win32!
pub unsafe fn WaitTime(seconds: f64)
{
    if (seconds < 0.0) { return; }    // Security check

#[cfg(any(feature = "SUPPORT_BUSY_WAIT_LOOP", feature = "SUPPORT_PARTIALBUSY_WAIT_LOOP"))]
    let mut destinationTime: f64 = GetTime() + seconds;

#[cfg(feature = "SUPPORT_BUSY_WAIT_LOOP")]
{
    while (GetTime() < destinationTime) { }
}
#[cfg(not(any(feature = "SUPPORT_BUSY_WAIT_LOOP")))]
{
    #[cfg(feature = "SUPPORT_PARTIALBUSY_WAIT_LOOP")]
        let mut sleepSeconds: f64 = seconds - seconds*0.05;  // NOTE: Reserve a percentage of the time for busy waiting
    #[cfg(not(feature = "SUPPORT_PARTIALBUSY_WAIT_LOOP"))]
        let sleepSeconds: f64 = seconds;

    // System halt functions
    #[cfg(target_os = "android")]
    {
        std::thread::sleep(std::time::Duration::from_secs_f64(sleepSeconds));
    }
    #[cfg(target_os = "windows")]
    {
        std::thread::sleep(std::time::Duration::from_secs_f64(sleepSeconds));
    }
    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd", target_os = "emscripten"))]
    {
        let mut req: libc::timespec = std::mem::zeroed();
        let sec: libc::time_t = sleepSeconds as libc::time_t;
        let nsec: libc::c_long = ((sleepSeconds - sec as f64)*1000000000.0) as libc::c_long;
        req.tv_sec = sec;
        req.tv_nsec = nsec;

        // NOTE: Use nanosleep() on Unix platforms... usleep() it's deprecated
        while (libc::nanosleep(&req, &mut req) == -1) { continue; }
    }
    #[cfg(target_vendor = "apple")]
    {
        libc::usleep((sleepSeconds*1000000.0) as u32);
    }

    #[cfg(feature = "SUPPORT_PARTIALBUSY_WAIT_LOOP")]
    {
        while (GetTime() < destinationTime) { }
    }
}
}

//----------------------------------------------------------------------------------
// Module Functions Definition: Misc
//----------------------------------------------------------------------------------

// NOTE: Functions with a platform-specific implementation on rcore_<platform>.c
//void OpenURL(const char *url)

// Set the seed for the random number generator

// Takes a screenshot of current screen
pub unsafe fn TakeScreenshot(fileName: &str)
{
#[cfg(feature = "SUPPORT_MODULE_RTEXTURES")]
{
    // Security check to (partially) avoid malicious code
    if (fileName.contains('\'')) { warn!("SYSTEM: Provided fileName could be potentially malicious, avoid [\'] character"); return; }

    // Apply content scaling if required
    let mut scale: Vector2 = Vector2 { x: 1.0, y: 1.0 };
    if (((CORE.Window.flags & ConfigFlags::FLAG_WINDOW_HIGHDPI as u32) == ConfigFlags::FLAG_WINDOW_HIGHDPI as u32)) { scale = GetWindowScaleDPI(); }

    let imgData = rlReadScreenPixels((((CORE.Window.render.x as f32)*scale.x) as i32), (((CORE.Window.render.y as f32)*scale.y) as i32));
    let image = Image { data: imgData.cast(), width: (CORE.Window.render.x*scale.x) as i32, height: (CORE.Window.render.y*scale.y) as i32, mipmaps: 1, format: PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32 };

    let mut path = String::new();
    if (!IsPathAbsolute(fileName)) { path = format!("{}/{}", CStr::from_ptr(CORE.Storage.basePath).to_string_lossy(), fileName); }
    else { path = fileName.to_owned(); }

    ExportImage(image, &path); // WARNING: Module required: rtextures
    libc::free(imgData.cast());

    if (FileExists(&path)) { info!("SYSTEM: [{}] Screenshot taken successfully", path); }
    else { warn!("SYSTEM: [{}] Screenshot could not be saved", path); }
}
#[cfg(not(any(feature = "SUPPORT_MODULE_RTEXTURES")))]
{
    warn!("IMAGE: ExportImage() requires module: rtextures");
}
}

// Set up window configuration flags (view FLAGS)
// NOTE: This function is expected to be called before window creation,
// because it sets up some flags for the window creation process
// To configure window states after creation, use SetWindowState()
pub unsafe fn SetConfigFlags(flags: u32)
{
    if (CORE.Window.ready) { warn!("WINDOW: SetConfigFlags called after window initialization, Use \"SetWindowState\" to set flags instead"); }

    // Selected flags are set but not evaluated at this point,
    // flag evaluation happens at InitWindow() or SetWindowState()
    CORE.Window.flags |= flags;
}

//----------------------------------------------------------------------------------
// Module Functions Definition: Logging system
//----------------------------------------------------------------------------------
// Set the current threshold (minimum) log level
pub unsafe fn SetTraceLogLevel(logType: i32) { logTypeLevel = logType; }

// Show trace log messages (LOG_INFO, LOG_WARNING, LOG_ERROR, LOG_DEBUG)
pub unsafe fn TraceLog(logType: i32, text: std::fmt::Arguments<'_>)
{
    #[cfg(feature = "SUPPORT_TRACELOG")]
    {
        // Message has level below current threshold, don't emit
        if (logType < logTypeLevel) { return; }

        let args = text;

        if let Some(callback) = traceLog
        {
            callback(logType, args);
            return;
        }

        #[cfg(feature = "PLATFORM_ANDROID")]
        match logType
        {
            1 => trace!("{}", args),
            2 => debug!("{}", args),
            3 => info!("{}", args),
            4 => warn!("{}", args),
            5 | 6 => error!("{}", args),
            _ => {}
        }
        #[cfg(not(feature = "PLATFORM_ANDROID"))]
        {
            let mut buffer = String::with_capacity(MAX_TRACELOG_MSG_LENGTH);

            match logType
            {
                1 => buffer.push_str("TRACE: "),
                2 => buffer.push_str("DEBUG: "),
                3 => buffer.push_str("INFO: "),
                4 => buffer.push_str("WARNING: "),
                5 => buffer.push_str("ERROR: "),
                6 => buffer.push_str("FATAL: "),
                _ => {}
            }

            let text = args.to_string();
            let textLength = text.len();
            let mut textLength = if textLength < MAX_TRACELOG_MSG_LENGTH - 12 { textLength } else { MAX_TRACELOG_MSG_LENGTH - 12 };
            while !text.is_char_boundary(textLength) { textLength -= 1; }
            buffer.push_str(&text[..textLength]);
            buffer.push('\n');
            print!("{}", buffer);
            let _ = std::io::Write::flush(&mut std::io::stdout());
        }

        if (logType == TraceLogLevel::LOG_FATAL as i32) { std::process::exit(1); }  // If fatal logging, exit program
    }
}

// Set custom trace log
pub unsafe fn SetTraceLogCallback(callback: Option<TraceLogCallback>)
{
    traceLog = callback;
}

//----------------------------------------------------------------------------------
// Module Functions Definition: Memory management
//----------------------------------------------------------------------------------
// Internal memory allocator
// NOTE: Initializes to zero by default
pub fn MemAlloc(size: u32) -> *mut c_void
{
    let ptr = unsafe { libc::calloc(size as usize, 1) };
    return ptr;
}

// Internal memory reallocator
pub unsafe fn MemRealloc(ptr: *mut c_void, size: u32) -> *mut c_void
{
    let ret = libc::realloc(ptr, size as usize);
    return ret;
}

// Internal memory free
pub unsafe fn MemFree(ptr: *mut c_void)
{
    libc::free(ptr);
}

//----------------------------------------------------------------------------------
// Module Functions Definition: File System management
//----------------------------------------------------------------------------------
// Load data from file into a buffer
pub unsafe fn LoadFileData(fileName: &str, dataSize: &mut i32) -> *mut u8
{
    let mut data: *mut u8 = std::ptr::null_mut();
    *dataSize = 0;

    if let Ok(fileNameC) = CString::new(fileName)
    {
        if let Some(callback) = loadFileData { return callback(fileName, dataSize); }

        let file = libc::fopen(fileNameC.as_ptr(), c"rb".as_ptr());

        if (!file.is_null())
        {
            // WARNING: On binary streams SEEK_END could not be found,
            // using fseek() and ftell() could not work in some (rare) cases
            libc::fseek(file, 0, libc::SEEK_END);
            let size = libc::ftell(file) as i32;     // WARNING: ftell() returns 'long int', maximum size returned is INT_MAX (2147483647 bytes)
            libc::fseek(file, 0, libc::SEEK_SET);

            if (size > 0)
            {
                data = libc::calloc(size as usize, std::mem::size_of::<u8>()).cast();

                if (!data.is_null())
                {
                    // NOTE: fread() returns number of read elements instead of bytes, so reading [1 byte, size elements]
                    let count = libc::fread(data.cast(), std::mem::size_of::<u8>(), size as usize, file);

                    // WARNING: fread() returns a size_t value, usually 'unsigned int' (32bit compilation) and 'unsigned long long' (64bit compilation)
                    // dataSize is unified along raylib as a 'int' type, so, for file-sizes >INT_MAX (2147483647 bytes) there is a limitation
                    if (count > 2147483647)
                    {
                        warn!("FILEIO: [{}] File is bigger than 2147483647 bytes, avoid using LoadFileData()", fileName);

                        libc::free(data.cast());
                        data = std::ptr::null_mut();
                    }
                    else
                    {
                        *dataSize = count as i32;

                        if (*dataSize != size) { warn!("FILEIO: [{}] File partially loaded ({} bytes out of {})", fileName, *dataSize, size); }
                        else { info!("FILEIO: [{}] File loaded successfully", fileName); }
                    }
                }
                else { warn!("FILEIO: [{}] Failed to allocated memory for file reading", fileName); }
            }
            else { warn!("FILEIO: [{}] Failed to read file", fileName); }

            libc::fclose(file);
        }
        else { warn!("FILEIO: [{}] Failed to open file", fileName); }
    }
    else { warn!("FILEIO: File name provided is not valid"); }

    return data;
}

// Unload file data allocated by LoadFileData()
pub unsafe fn UnloadFileData(data: *mut u8)
{
    libc::free(data.cast());
}

// Save data to file from buffer
pub unsafe fn SaveFileData(fileName: &str, data: *const c_void, dataSize: i32) -> bool
{
    let mut result = false;

    if let Ok(fileNameC) = CString::new(fileName)
    {
        if let Some(callback) = saveFileData { return callback(fileName, data, dataSize); }

        let file = libc::fopen(fileNameC.as_ptr(), c"wb".as_ptr());

        if (!file.is_null())
        {
            // WARNING: fwrite() returns a size_t value, usually 'unsigned int' (32bit compilation) and 'unsigned long long' (64bit compilation)
            // and expects a size_t input value but as dataSize is limited to INT_MAX (2147483647 bytes), there shouldn't be a problem
            let count = libc::fwrite(data, std::mem::size_of::<u8>(), dataSize as usize, file) as i32;

            if (count == 0) { warn!("FILEIO: [{}] Failed to write file", fileName); }
            else if (count != dataSize) { warn!("FILEIO: [{}] File partially written", fileName); }
            else { info!("FILEIO: [{}] File saved successfully", fileName); }

            let closed = libc::fclose(file);
            if (closed == 0) { result = true; }
        }
        else { warn!("FILEIO: [{}] Failed to open file", fileName); }
    }
    else { warn!("FILEIO: File name provided is not valid"); }

    return result;
}

// Export data to code (.h), returns true on success
pub unsafe fn ExportDataAsCode(data: &[u8], dataSize: i32, fileName: &str) -> bool
{
    let mut result = false;

    const TEXT_BYTES_PER_LINE: i32 = 20;

    // NOTE: Text data buffer size is estimated considering raw data size in bytes
    // and requiring 6 char bytes for every byte: "0x00, "
    let mut txtData = String::with_capacity(dataSize as usize*6 + 2000);

    let mut byteCount = 0;
    txtData.push_str("////////////////////////////////////////////////////////////////////////////////////////\n");
    txtData.push_str("//                                                                                    //\n");
    txtData.push_str("// DataAsCode exporter v1.0 - Raw data exported as an array of bytes                  //\n");
    txtData.push_str("//                                                                                    //\n");
    txtData.push_str("// more info and bugs-report:  github.com/raysan5/raylib                              //\n");
    txtData.push_str("// feedback and support:       ray[at]raylib.com                                      //\n");
    txtData.push_str("//                                                                                    //\n");
    txtData.push_str("// Copyright (c) 2022-2026 Ramon Santamaria (@raysan5)                                //\n");
    txtData.push_str("//                                                                                    //\n");
    txtData.push_str("////////////////////////////////////////////////////////////////////////////////////////\n\n");

    // Get file name from path
    let mut varFileName = GetFileNameWithoutExt(fileName).into_bytes();
    for i in 0..varFileName.len()
    {
        // Convert variable name to uppercase
        if ((varFileName[i] >= b'a') && (varFileName[i] <= b'z')) { varFileName[i] = varFileName[i] - 32; }
        // Replace non valid character for C identifier with '_'
        else if (varFileName[i] == b'.' || varFileName[i] == b'-' || varFileName[i] == b'?' || varFileName[i] == b'!' || varFileName[i] == b'+') { varFileName[i] = b'_'; }
    }
    let varFileName = String::from_utf8(varFileName).unwrap();

    write!(txtData, "#define {}_DATA_SIZE     {}\n\n", varFileName, dataSize).unwrap();

    write!(txtData, "static unsigned char {}_DATA[{}_DATA_SIZE] = {{ ", varFileName, varFileName).unwrap();
    for i in 0..(dataSize - 1) { write!(txtData, "0x{:x},{}", data[i as usize], if i%TEXT_BYTES_PER_LINE == 0 { "\n" } else { " " }).unwrap(); }
    write!(txtData, "0x{:x} }};\n", data[(dataSize - 1) as usize]).unwrap();
    byteCount = txtData.len();

    // NOTE: Text data size exported is determined by '\0' (NULL) character
    result = SaveFileText(fileName, &txtData);

    drop(txtData);

    if (result) { info!("FILEIO: [{}] Data as code exported successfully", fileName); }
    else { warn!("FILEIO: [{}] Failed to export data as code", fileName); }

    return result;
}

// Load text data from file, returns a '\0' terminated string
// NOTE: text chars array should be freed manually
pub unsafe fn LoadFileText(fileName: &str) -> Option<String>
{
    let mut text = None;

    if let Ok(fileNameC) = CString::new(fileName)
    {
        if let Some(callback) = loadFileText { return callback(fileName); }

        let file = libc::fopen(fileNameC.as_ptr(), c"rt".as_ptr());

        if (!file.is_null())
        {
            // WARNING: When reading a file as 'text' file,
            // text mode causes carriage return-linefeed translation...
            // ...but using fseek() should return correct byte-offset
            libc::fseek(file, 0, libc::SEEK_END);
            let size = libc::ftell(file) as u32;
            libc::fseek(file, 0, libc::SEEK_SET);

            if (size > 0)
            {
                let mut buffer = Vec::<u8>::new();

                if (buffer.try_reserve_exact(size as usize + 1).is_ok())
                {
                    buffer.resize(size as usize + 1, 0);
                    let count = libc::fread(buffer.as_mut_ptr().cast(), std::mem::size_of::<c_char>(), size as usize, file) as u32;

                    // WARNING: \r\n is converted to \n on reading, so,
                    // read bytes count gets reduced by the number of lines
                    if (count < size) { buffer.truncate(count as usize + 1); }

                    // Zero-terminate the string
                    buffer[count as usize] = 0;
                    text = Some(String::from_utf8_lossy(&buffer[..count as usize]).into_owned());

                    info!("FILEIO: [{}] Text file loaded successfully", fileName);
                }
                else { warn!("FILEIO: [{}] Failed to allocated memory for file reading", fileName); }
            }
            else { warn!("FILEIO: [{}] Failed to read text file", fileName); }

            libc::fclose(file);
        }
        else { warn!("FILEIO: [{}] Failed to open text file", fileName); }
    }
    else { warn!("FILEIO: File name provided is not valid"); }

    return text;
}

// Unload file text data allocated by LoadFileText()
pub fn UnloadFileText(text: Option<String>)
{
    drop(text);
}

// Save text data to file (write), string must be '\0' terminated
pub unsafe fn SaveFileText(fileName: &str, text: &str) -> bool
{
    let mut result = false;

    if let Ok(fileNameC) = CString::new(fileName)
    {
        if let Some(callback) = saveFileText { return callback(fileName, text); }

        let file = libc::fopen(fileNameC.as_ptr(), c"wt".as_ptr());

        if (!file.is_null())
        {
            let text = CString::new(text).unwrap();
            let count = libc::fprintf(file, c"%s".as_ptr(), text.as_ptr());

            if (count < 0) { warn!("FILEIO: [{}] Failed to write text file", fileName); }
            else { info!("FILEIO: [{}] Text file saved successfully", fileName); }

            let closed = libc::fclose(file);
            if (closed == 0) { result = true; }
        }
        else { warn!("FILEIO: [{}] Failed to open text file", fileName); }
    }
    else { warn!("FILEIO: File name provided is not valid"); }

    return result;
}

// Check if the file exists
pub fn FileExists(fileName: &str) -> bool
{
    let mut result = false;

    if (std::fs::metadata(fileName).is_ok()) { result = true; }

    // NOTE: Alternatively, stat() can be used instead of access()
    //#include <sys/stat.h>
    //struct stat statbuf;
    //if (stat(filename, &statbuf) == 0) result = true;

    return result;
}

// Check file extension
pub fn IsFileExtension(fileName: &str, ext: &str) -> bool
{
    const MAX_FILE_EXTENSIONS: usize = 32;

    let mut result = false;
    let fileExt = GetFileExtension(fileName);

    // WARNING: fileExt points to last '.' on fileName string but it could happen
    // that fileName is not correct: "myfile.png more text following\n"

    if let Some(fileExt) = fileExt
    {
        let fileExtLength = fileExt.len();
        let mut fileExtLower = [0u8; 16];
        for i in 0..fileExtLength.min(15)
        {
            // Copy and convert to lower-case
            if ((fileExt.as_bytes()[i] >= b'A') && (fileExt.as_bytes()[i] <= b'Z')) { fileExtLower[i] = fileExt.as_bytes()[i] + 32; }
            else { fileExtLower[i] = fileExt.as_bytes()[i]; }
        }

        let mut extCount = 1;
        let extLength = ext.len();
        let mut extList = vec![0u8; extLength + 1];
        let mut extListPtrs = [0usize; MAX_FILE_EXTENSIONS];
        extList[..extLength].copy_from_slice(ext.as_bytes());
        extListPtrs[0] = 0;

        for i in 0..extLength
        {
            // Convert to lower-case if extension is upper-case
            if ((extList[i] >= b'A') && (extList[i] <= b'Z')) { extList[i] += 32; }

            // Get pointer to next extension and add null-terminator
            if (extList[i] == b';')
            {
                extList[i] = 0;

                if (extCount < MAX_FILE_EXTENSIONS)
                {
                    extListPtrs[extCount] = i + 1;
                    extCount += 1;
                }
            }
        }

        for i in 0..extCount
        {
            // Consider the case where extension provided
            // does not start with the '.'
            let mut fileExtLowerPtr = 0;
            if (extList[extListPtrs[i]] != b'.') { fileExtLowerPtr += 1; }

            let fileExtLowerEnd = fileExtLower.iter().position(|&ch| ch == 0).unwrap();
            let extEnd = extList[extListPtrs[i]..].iter().position(|&ch| ch == 0).unwrap() + extListPtrs[i];
            if (fileExtLower[fileExtLowerPtr..fileExtLowerEnd] == extList[extListPtrs[i]..extEnd])
            {
                result = true;
                break;
            }
        }

        drop(extList);
    }

    return result;
}

// Check if directory path exists
pub fn DirectoryExists(dirPath: &str) -> bool
{
    let mut result = false;
    let dir = std::fs::read_dir(dirPath);

    if let Ok(dir) = dir
    {
        result = true;
        drop(dir);
    }

    return result;
}

// Get pointer to extension for a filename string (includes the dot: .png)
// WARNING: Getting the pointer to the input string extension position (not a string copy)
pub fn GetFileExtension(fileName: &str) -> Option<&str>
{
    let dot = fileName.rfind('.');

    if (dot.is_none() || dot == Some(0)) { return None; }

    return Some(&fileName[dot.unwrap()..]);
}

// String pointer reverse break: returns right-most occurrence of charset in s
fn strprbrk<'a>(text: &'a str, charset: &str) -> Option<&'a str>
{
    let mut latestMatch = None;
    let mut text = text;

    while let Some(index) = text.find(|ch| charset.contains(ch))
    {
        latestMatch = Some(&text[index..]);
        text = &text[index + text[index..].chars().next().unwrap().len_utf8()..];
    }

    return latestMatch;
}

// Get pointer to filename for a path string
pub fn GetFileName(filePath: &str) -> &str
{
    let mut fileName = None;

    fileName = strprbrk(filePath, "\\/");

    if (fileName.is_none()) { return filePath; }

    return &fileName.unwrap()[1..];
}

// Get filename string without extension (uses static string)
pub fn GetFileNameWithoutExt(filePath: &str) -> String
{
    const MAX_FILENAME_LENGTH: usize = 256;

    let mut fileName = String::new();
    fileName.clear();

    {
        fileName = GetFileName(filePath).to_owned(); // Get filename.ext without path
        let mut fileNameLength = fileName.len().min(MAX_FILENAME_LENGTH - 1); // Get size in bytes
        while !fileName.is_char_boundary(fileNameLength) { fileNameLength -= 1; }
        fileName.truncate(fileNameLength);

        for i in (1..fileNameLength).rev() // Reverse search '.'
        {
            if (fileName.as_bytes()[i] == b'.')
            {
                // NOTE: Break on first '.' found
                fileName.truncate(i);
                break;
            }
        }
    }

    return fileName;
}

// Get directory for a provided filePath
pub fn GetDirectoryPath(filePath: &str) -> String
{
    /*
    // NOTE: Directory separator is different in Windows and other platforms,
    // fortunately, Windows also support the '/' separator, that's the one should be used
    #if defined(_WIN32)
        char separator = '\\';
    #else
        char separator = '/';
    #endif
    */
    let mut lastSlash = None;
    let mut dirPath = String::new();
    dirPath.clear();

    // In case provided path does not contain a root drive letter (C:\, D:\)
    // nor leading path separator (\, /), add the current directory path to dirPath
    if (filePath.as_bytes().get(1) != Some(&b':') && !filePath.starts_with('\\') && !filePath.starts_with('/'))
    {
        // For security, set starting path to current directory,
        // obtained path will be concatenated to this
        dirPath.push('.');
        dirPath.push('/');
    }

    lastSlash = strprbrk(filePath, "\\/");
    if let Some(lastSlash) = lastSlash
    {
        if (lastSlash.len() == filePath.len())
        {
            // The last and only slash is the leading one: path is in a root directory
            dirPath.clear();
            dirPath.push(filePath.chars().next().unwrap());
        }
        else
        {
            let dirPathPtr = dirPath.len(); // Skip drive letter, "C:"
            dirPath.push_str(&filePath[..filePath.len() - lastSlash.len() + 1]);
            dirPath.truncate(filePath.len() - lastSlash.len() + dirPathPtr);  // Add '\0' manually
        }
    }

    return dirPath;
}

// Get current working directory
pub fn GetWorkingDirectory() -> String
{
    let mut currentDir = String::new();
    currentDir.clear();

    let path = std::env::current_dir().map(|path| path.to_string_lossy().into_owned()).unwrap_or_default();

    return path;
}

// Check if provided path point to a file
pub fn IsPathFile(path: &str) -> bool
{
    let mut result = false;

    let info = std::fs::metadata(path);

    if (info.map(|info| info.is_file()).unwrap_or(false)) { result = true; }

    return result;
}

// Check if provided path point to a directory
pub fn IsPathDirectory(path: &str) -> bool
{
    let mut result = false;

    if (!IsPathFile(path)) { result = true; }

    return result;
}

// Check if provided path is an absolute path
pub fn IsPathAbsolute(path: &str) -> bool
{
    let mut result = false;

    if (!path.is_empty())
    {
        #[cfg(target_os = "windows")]
        {
            // Check UNC path (\\server\share)
            if (path.starts_with("\\\\")) { result = true; }
            // Check path starts with a drive letter (e.g. C:\ or D:/)
            else if (path.as_bytes()[0].is_ascii_alphabetic() && path.as_bytes().get(1) == Some(&b':') && matches!(path.as_bytes().get(2), Some(b'\\' | b'/'))) { result = true; }
        }
        #[cfg(not(target_os = "windows"))]
        {
            // Check POSIX path, must start with /
            if (path.starts_with('/')) { result = true; }
        }
    }

    return result;
}

// Check if file has been dropped into window
pub unsafe fn IsFileDropped() -> bool
{
    let mut result = false;

    if (CORE.Window.dropFileCount > 0) { result = true; }

    return result;
}

//----------------------------------------------------------------------------------
// Module Functions Definition: Compression and Encoding
//----------------------------------------------------------------------------------

// Compress data (DEFLATE algorithm)
pub unsafe fn CompressData(data: *const u8, dataSize: i32, compDataSize: &mut i32) -> *mut u8
{
    const COMPRESSION_QUALITY_DEFLATE: i32 = 8;

    let compData = std::ptr::null_mut();

    #[cfg(feature = "SUPPORT_COMPRESSION_API")]
    {
        // Compress data and generate a valid DEFLATE stream
        let sdefl = libc::calloc(1, std::mem::size_of::<sdefl>()) as *mut sdefl;   // WARNING: Possible stack overflow, struct sdefl is almost 1MB
        let bounds = sdefl_bound(dataSize);
        compData = libc::calloc(bounds as usize, 1).cast();

        *compDataSize = sdeflate(sdefl, compData, data, dataSize, COMPRESSION_QUALITY_DEFLATE);   // Compression level 8, same as stbiw
        libc::free(sdefl.cast());

        info!("SYSTEM: Compress data: Original size: {} -> Comp. size: {}", dataSize, *compDataSize);
    }

    return compData;
}

// Decompress data (DEFLATE algorithm)
pub unsafe fn DecompressData(compData: *const u8, compDataSize: i32, dataSize: &mut i32) -> *mut u8
{
    let data: *mut u8 = std::ptr::null_mut();

    #[cfg(feature = "SUPPORT_COMPRESSION_API")]
    {
        // Decompress data from a valid DEFLATE stream
        let data0 = libc::calloc(MAX_DECOMPRESSION_SIZE*1024*1024, 1) as *mut u8;
        let size = sinflate(data0, (MAX_DECOMPRESSION_SIZE*1024*1024) as i32, compData, compDataSize);

        // WARNING: RL_REALLOC can make (and leave) data copies in memory,
        // that can be a security concern in case of compression of sensitive data
        // So, using a second buffer to copy data manually, wiping original buffer memory
        data = libc::calloc(size as usize, 1).cast();
        std::ptr::copy_nonoverlapping(data0, data, size as usize);
        std::ptr::write_bytes(data0, 0, MAX_DECOMPRESSION_SIZE*1024*1024); // Wipe memory, is memset() safe?
        libc::free(data0.cast());

        info!("SYSTEM: Decompress data: Comp. size: {} -> Original size: {}", compDataSize, size);

        *dataSize = size;
    }

    return data;
}
//----------------------------------------------------------------------------------
// Module Functions Definition: Automation Events Recording and Playing
//----------------------------------------------------------------------------------

// Load automation events list from file, NULL for empty list, capacity = MAX_AUTOMATION_EVENTS
pub unsafe fn LoadAutomationEventList(fileName: Option<&str>) -> AutomationEventList
{
    let list: AutomationEventList = std::mem::zeroed();

    #[cfg(feature = "SUPPORT_AUTOMATION_EVENTS")]
    {
        // Allocate and empty automation event list, ready to record new events
        list.events = libc::calloc(MAX_AUTOMATION_EVENTS, std::mem::size_of::<AutomationEvent>()).cast();
        list.capacity = MAX_AUTOMATION_EVENTS as u32;

        if (fileName.is_none()) { info!("AUTOMATION: New empty events list loaded successfully"); }
        else
        {
            let fileName = fileName.unwrap();
            // Load automation events file (binary)
            /*
            //int dataSize = 0;
            //unsigned char *data = LoadFileData(fileName, &dataSize);

            FILE *raeFile = fopen(fileName, "rb");
            unsigned char fileId[4] = { 0 };

            fread(fileId, 1, 4, raeFile);

            if ((fileId[0] == 'r') && (fileId[1] == 'A') && (fileId[2] == 'E') && (fileId[1] == ' '))
            {
                fread(&eventCount, sizeof(int), 1, raeFile);
                TRACELOG(LOG_WARNING, "Events loaded: %i\n", eventCount);
                fread(events, sizeof(AutomationEvent), eventCount, raeFile);
            }

            fclose(raeFile);
            */

            // Load events file (text)
            //unsigned char *buffer = LoadFileText(fileName);
            let fileNameC = CString::new(fileName).unwrap();
            let raeFile = libc::fopen(fileNameC.as_ptr(), c"rt".as_ptr());

            if (!raeFile.is_null())
            {
                let mut counter: u32 = 0;
                let mut buffer = [0 as c_char; 256];
                let mut eventDesc = [0 as c_char; 64];

                let mut result = libc::fgets(buffer.as_mut_ptr(), 256, raeFile);
                if (result != buffer.as_mut_ptr()) { warn!("AUTOMATION: [{}] Issue reading line to buffer", fileName); }

                while (libc::feof(raeFile) == 0)
                {
                    match buffer[0] as u8
                    {
                        b'c' => { libc::sscanf(buffer.as_ptr(), c"c %u".as_ptr(), &mut list.count); }
                        b'e' =>
                        {
                            if (counter < list.capacity)
                            {
                                let event = list.events.add(counter as usize);
                                libc::sscanf(buffer.as_ptr(), c"e %u %u %d %d %d %d %63[^\n]s".as_ptr(), &mut (*event).frame, &mut (*event).type_,
                                    &mut (*event).params[0], &mut (*event).params[1], &mut (*event).params[2], &mut (*event).params[3], eventDesc.as_mut_ptr());

                                counter += 1;
                            }
                            else { warn!("AUTOMATION: Event goes beyond automated list capacity (MAX: {}): {}", list.capacity, CStr::from_ptr(buffer.as_ptr()).to_string_lossy()); }
                        }
                        _ => {}
                    }

                    result = libc::fgets(buffer.as_mut_ptr(), 256, raeFile);
                    if (result != buffer.as_mut_ptr()) { warn!("AUTOMATION: [{}] Issue reading line to buffer", fileName); }
                }

                if (counter != list.count)
                {
                    warn!("AUTOMATION: Events read from file [{}] do not mach event count specified [{}]", counter, list.count);
                    list.count = counter;
                }

                libc::fclose(raeFile);

                info!("AUTOMATION: Events file loaded successfully");
            }

            info!("AUTOMATION: Events loaded from file: {}", list.count);
        }
    }
    return list;
}

// Unload automation events list from file
pub unsafe fn UnloadAutomationEventList(list: AutomationEventList)
{
    #[cfg(feature = "SUPPORT_AUTOMATION_EVENTS")]
    libc::free(list.events.cast());
}

// Export automation events list as text file
pub unsafe fn ExportAutomationEventList(list: AutomationEventList, fileName: &str) -> bool
{
    let result = false;

    #[cfg(feature = "SUPPORT_AUTOMATION_EVENTS")]
    {
        // Export events as binary file
        // NOTE: Code not used, only for reference if required in the future
        /*
        if (list.count > 0)
        {
            int binarySize = 4 + sizeof(int) + sizeof(AutomationEvent)*list.count;
            unsigned char *binBuffer = (unsigned char *)RL_CALLOC(binarySize, 1);
            int offset = 0;
            memcpy(binBuffer + offset, "rAE ", 4);
            offset += 4;
            memcpy(binBuffer + offset, &list.count, sizeof(int));
            offset += sizeof(int);
            memcpy(binBuffer + offset, list.events, sizeof(AutomationEvent)*list.count);
            offset += sizeof(AutomationEvent)*list.count;

            result = SaveFileData(TextFormat("%s.rae",fileName), binBuffer, binarySize);
            RL_FREE(binBuffer);
        }
        */

        // Export events as text
        // NOTE: Save to memory buffer and SaveFileText()
        let mut txtData = String::with_capacity(256*list.count as usize + 2048); // 256 characters per line plus some header

        let mut byteCount = 0;
        txtData.push_str("#\n");
        txtData.push_str("# Automation events exporter v1.0 - raylib automation events list\n");
        txtData.push_str("#\n");
        txtData.push_str("#    c <events_count>\n");
        txtData.push_str("#    e <frame> <event_type> <param0> <param1> <param2> <param3> // <event_type_name>\n");
        txtData.push_str("#\n");
        txtData.push_str("# more info and bugs-report:  github.com/raysan5/raylib\n");
        txtData.push_str("# feedback and support:       ray[at]raylib.com\n");
        txtData.push_str("#\n");
        txtData.push_str("# Copyright (c) 2023-2026 Ramon Santamaria (@raysan5)\n");
        txtData.push_str("#\n\n");

        // Add events data
        write!(txtData, "c {}\n", list.count).unwrap();
        for i in 0..list.count
        {
            write!(txtData, "e {} {} {} {} {} {} // Event: {}\n", (*list.events.add(i as usize)).frame, (*list.events.add(i as usize)).type_,
                (*list.events.add(i as usize)).params[0], (*list.events.add(i as usize)).params[1], (*list.events.add(i as usize)).params[2], (*list.events.add(i as usize)).params[3], autoEventTypeName[(*list.events.add(i as usize)).type_ as usize]).unwrap();
        }
        byteCount = txtData.len();

        // NOTE: Text data size exported is determined by '\0' (NULL) character
        result = SaveFileText(fileName, &txtData);

        drop(txtData);
    }

    return result;
}

// Setup automation event list to record to
pub unsafe fn SetAutomationEventList(list: *mut AutomationEventList)
{
    #[cfg(feature = "SUPPORT_AUTOMATION_EVENTS")]
    { currentEventList = list; }
}

// Set automation event internal base frame to start recording
pub unsafe fn SetAutomationEventBaseFrame(frame: i32)
{
    CORE.Time.frameCounter = frame as u32;
}

// Start recording automation events (AutomationEventList must be set)
pub unsafe fn StartAutomationEventRecording()
{
    #[cfg(feature = "SUPPORT_AUTOMATION_EVENTS")]
    { automationEventRecording = true; }
}

// Stop recording automation events
pub unsafe fn StopAutomationEventRecording()
{
    #[cfg(feature = "SUPPORT_AUTOMATION_EVENTS")]
    { automationEventRecording = false; }
}
// Play a recorded automation event
pub unsafe fn PlayAutomationEvent(event: AutomationEvent)
{
#[cfg(feature = "SUPPORT_AUTOMATION_EVENTS")]
{
    // WARNING: When should event be played? After/before/replace PollInputEvents()? -> Up to the user!

    if (!automationEventRecording)
    {
        match event.type_
        {
            // Input event
            x if x == AutomationEventType::INPUT_KEY_UP as u32 => { CORE.Input.Keyboard.currentKeyState[event.params[0] as usize] = 0; }             // param[0]: key
            x if x == AutomationEventType::INPUT_KEY_DOWN as u32 => {                                                                              // param[0]: key
                CORE.Input.Keyboard.currentKeyState[event.params[0] as usize] = 1;

                if (CORE.Input.Keyboard.previousKeyState[event.params[0] as usize] == 0)
                {
                    if (CORE.Input.Keyboard.keyPressedQueueCount < MAX_KEY_PRESSED_QUEUE as i32)
                    {
                        // Add character to the queue
                        CORE.Input.Keyboard.keyPressedQueue[(CORE.Input.Keyboard.keyPressedQueueCount) as usize] = event.params[0];
                        CORE.Input.Keyboard.keyPressedQueueCount += 1;
                    }
                }
            }
            x if x == AutomationEventType::INPUT_MOUSE_BUTTON_UP as u32 => { CORE.Input.Mouse.currentButtonState[event.params[0] as usize] = 0; }    // param[0]: key
            x if x == AutomationEventType::INPUT_MOUSE_BUTTON_DOWN as u32 => { CORE.Input.Mouse.currentButtonState[event.params[0] as usize] = 1; }   // param[0]: key
            x if x == AutomationEventType::INPUT_MOUSE_POSITION as u32 =>      // param[0]: x, param[1]: y
            {
                CORE.Input.Mouse.currentPosition.x = (event.params[0] as f32);
                CORE.Input.Mouse.currentPosition.y = (event.params[1] as f32);
            }
            x if x == AutomationEventType::INPUT_MOUSE_WHEEL_MOTION as u32 =>  // param[0]: x delta, param[1]: y delta
            {
                CORE.Input.Mouse.currentWheelMove.x = (event.params[0] as f32);
                CORE.Input.Mouse.currentWheelMove.y = (event.params[1] as f32);
            }
            x if x == AutomationEventType::INPUT_TOUCH_UP as u32 => { CORE.Input.Touch.currentTouchState[event.params[0] as usize] = 0; }            // param[0]: id
            x if x == AutomationEventType::INPUT_TOUCH_DOWN as u32 => { CORE.Input.Touch.currentTouchState[event.params[0] as usize] = 1; }           // param[0]: id
            x if x == AutomationEventType::INPUT_TOUCH_POSITION as u32 =>      // param[0]: id, param[1]: x, param[2]: y
            {
                CORE.Input.Touch.position[event.params[0] as usize].x = (event.params[1] as f32);
                CORE.Input.Touch.position[event.params[0] as usize].y = (event.params[2] as f32);
            }
            x if x == AutomationEventType::INPUT_GAMEPAD_CONNECT as u32 => { CORE.Input.Gamepad.ready[event.params[0] as usize] = true; }                // param[0]: gamepad
            x if x == AutomationEventType::INPUT_GAMEPAD_DISCONNECT as u32 => { CORE.Input.Gamepad.ready[event.params[0] as usize] = false; }            // param[0]: gamepad
            x if x == AutomationEventType::INPUT_GAMEPAD_BUTTON_UP as u32 => { CORE.Input.Gamepad.currentButtonState[event.params[0] as usize][event.params[1] as usize] = 0; }    // param[0]: gamepad, param[1]: button
            x if x == AutomationEventType::INPUT_GAMEPAD_BUTTON_DOWN as u32 => { CORE.Input.Gamepad.currentButtonState[event.params[0] as usize][event.params[1] as usize] = 1; }   // param[0]: gamepad, param[1]: button
            x if x == AutomationEventType::INPUT_GAMEPAD_AXIS_MOTION as u32 => // param[0]: gamepad, param[1]: axis, param[2]: delta
            {
                CORE.Input.Gamepad.axisState[event.params[0] as usize][event.params[1] as usize] = ((event.params[2] as f32)/32768.0);
            }
    #[cfg(feature = "SUPPORT_GESTURES_SYSTEM")]
            x if x == AutomationEventType::INPUT_GESTURE as u32 => { GESTURES.current = event.params[0]; }     // param[0]: gesture (enum Gesture) -> rgestures.h: GESTURES.current
            // Window event
            x if x == AutomationEventType::WINDOW_CLOSE as u32 => { CORE.Window.shouldClose = true; }
            x if x == AutomationEventType::WINDOW_MAXIMIZE as u32 => { MaximizeWindow(); }
            x if x == AutomationEventType::WINDOW_MINIMIZE as u32 => { MinimizeWindow(); }
            x if x == AutomationEventType::WINDOW_RESIZE as u32 => { SetWindowSize(event.params[0], event.params[1]); }
            // Custom event
    #[cfg(feature = "SUPPORT_SCREEN_CAPTURE")]
            x if x == AutomationEventType::ACTION_TAKE_SCREENSHOT as u32 =>
            {
                TakeScreenshot(&format!("screenshot{:03}.png", screenshotCounter));
                screenshotCounter += 1;
            }
            x if x == AutomationEventType::ACTION_SETTARGETFPS as u32 => { SetTargetFPS(event.params[0]); }
            _ => {}
        }

        info!("AUTOMATION PLAY: Frame: {} | Event type: {} | Event parameters: {}, {}, {}", event.frame, event.type_, event.params[0], event.params[1], event.params[2]);
    }
}
}

//----------------------------------------------------------------------------------
// Module Functions Definition: Input Handling: Keyboard
//----------------------------------------------------------------------------------

// Check if key has been pressed once
pub unsafe fn IsKeyPressed(key: i32) -> bool
{
    let mut pressed: bool = false;

    if ((key > 0) && (key < MAX_KEYBOARD_KEYS as i32))
    {
        if ((CORE.Input.Keyboard.previousKeyState[(key) as usize] == 0) && (CORE.Input.Keyboard.currentKeyState[(key) as usize] == 1)) { pressed = true; }
    }

    return pressed;
}

// Check if key has been pressed again
pub unsafe fn IsKeyPressedRepeat(key: i32) -> bool
{
    let mut repeat: bool = false;

    if ((key > 0) && (key < MAX_KEYBOARD_KEYS as i32))
    {
        if (CORE.Input.Keyboard.keyRepeatInFrame[(key) as usize] == 1) { repeat = true; }
    }

    return repeat;
}

// Check if key is being pressed (key held down)
pub unsafe fn IsKeyDown(key: i32) -> bool
{
    let mut down: bool = false;

    if ((key > 0) && (key < MAX_KEYBOARD_KEYS as i32))
    {
        if (CORE.Input.Keyboard.currentKeyState[(key) as usize] == 1) { down = true; }
    }

    return down;
}

// Check if key has been released once
pub unsafe fn IsKeyReleased(key: i32) -> bool
{
    let mut released: bool = false;

    if ((key > 0) && (key < MAX_KEYBOARD_KEYS as i32))
    {
        if ((CORE.Input.Keyboard.previousKeyState[(key) as usize] == 1) && (CORE.Input.Keyboard.currentKeyState[(key) as usize] == 0)) { released = true; }
    }

    return released;
}

// Check if key is NOT being pressed (key not held down)
pub unsafe fn IsKeyUp(key: i32) -> bool
{
    let mut up: bool = false;

    if ((key > 0) && (key < MAX_KEYBOARD_KEYS as i32))
    {
        if (CORE.Input.Keyboard.currentKeyState[(key) as usize] == 0) { up = true; }
    }

    return up;
}

// Get the last key pressed
pub unsafe fn GetKeyPressed() -> i32
{
    let mut value: i32 = 0;

    if (CORE.Input.Keyboard.keyPressedQueueCount > 0)
    {
        // Get character from the queue head
        value = CORE.Input.Keyboard.keyPressedQueue[0];

        // Shift elements 1 step toward the head
        for i in 0..(CORE.Input.Keyboard.keyPressedQueueCount - 1)
            { CORE.Input.Keyboard.keyPressedQueue[(i) as usize] = CORE.Input.Keyboard.keyPressedQueue[(i + 1) as usize]; }

        // Reset last character in the queue
        CORE.Input.Keyboard.keyPressedQueue[(CORE.Input.Keyboard.keyPressedQueueCount - 1) as usize] = 0;
        CORE.Input.Keyboard.keyPressedQueueCount -= 1;
    }

    return value;
}

// Get the last char pressed
pub unsafe fn GetCharPressed() -> i32
{
    let mut value: i32 = 0;

    if (CORE.Input.Keyboard.charPressedQueueCount > 0)
    {
        // Get character from the queue head
        value = CORE.Input.Keyboard.charPressedQueue[0];

        // Shift elements 1 step toward the head
        for i in 0..(CORE.Input.Keyboard.charPressedQueueCount - 1)
            { CORE.Input.Keyboard.charPressedQueue[(i) as usize] = CORE.Input.Keyboard.charPressedQueue[(i + 1) as usize]; }

        // Reset last character in the queue
        CORE.Input.Keyboard.charPressedQueue[(CORE.Input.Keyboard.charPressedQueueCount - 1) as usize] = 0;
        CORE.Input.Keyboard.charPressedQueueCount -= 1;
    }

    return value;
}

// Set a custom key to exit program
// NOTE: default exitKey is set to ESCAPE
pub unsafe fn SetExitKey(key: i32)
{
    CORE.Input.Keyboard.exitKey = key;
}

//----------------------------------------------------------------------------------
// Module Functions Definition: Input Handling: Gamepad
//----------------------------------------------------------------------------------

// NOTE: Functions with a platform-specific implementation on rcore_<platform>.c
//int SetGamepadMappings(const char *mappings)

// Check if gamepad is available
pub unsafe fn IsGamepadAvailable(gamepad: i32) -> bool
{
    let mut result: bool = false;

    if ((gamepad >= 0) && (gamepad < MAX_GAMEPADS as i32) && CORE.Input.Gamepad.ready[(gamepad) as usize]) { result = true; }

    return result;
}

// Get gamepad internal name id
pub unsafe fn GetGamepadName(gamepad: i32) -> Option<String>
{
    let mut name: Option<String> = None;

    if ((gamepad >= 0) && (gamepad < MAX_GAMEPADS as i32)) { name = Some(CStr::from_ptr(CORE.Input.Gamepad.name[gamepad as usize].as_ptr().cast()).to_string_lossy().into_owned()); }

    return name;
}

// Check if gamepad button has been pressed once
pub unsafe fn IsGamepadButtonPressed(gamepad: i32, button: i32) -> bool
{
    let mut pressed: bool = false;

    if ((gamepad >= 0) && (gamepad < MAX_GAMEPADS as i32) && CORE.Input.Gamepad.ready[(gamepad) as usize] && (button < MAX_GAMEPAD_BUTTONS as i32))
    {
        if ((CORE.Input.Gamepad.previousButtonState[(gamepad) as usize][(button) as usize] == 0) && (CORE.Input.Gamepad.currentButtonState[(gamepad) as usize][(button) as usize] == 1)) { pressed = true; }
    }

    return pressed;
}

// Check if gamepad button is being pressed
pub unsafe fn IsGamepadButtonDown(gamepad: i32, button: i32) -> bool
{
    let mut down: bool = false;

    if ((gamepad >= 0) && (gamepad < MAX_GAMEPADS as i32) && CORE.Input.Gamepad.ready[(gamepad) as usize] && (button < MAX_GAMEPAD_BUTTONS as i32))
    {
        if (CORE.Input.Gamepad.currentButtonState[(gamepad) as usize][(button) as usize] == 1) { down = true; }
    }

    return down;
}

// Check if gamepad button has NOT been pressed once
pub unsafe fn IsGamepadButtonReleased(gamepad: i32, button: i32) -> bool
{
    let mut released: bool = false;

    if ((gamepad >= 0) && (gamepad < MAX_GAMEPADS as i32) && CORE.Input.Gamepad.ready[(gamepad) as usize] && (button < MAX_GAMEPAD_BUTTONS as i32))
    {
        if ((CORE.Input.Gamepad.previousButtonState[(gamepad) as usize][(button) as usize] == 1) && (CORE.Input.Gamepad.currentButtonState[(gamepad) as usize][(button) as usize] == 0)) { released = true; }
    }

    return released;
}

// Check if gamepad button is NOT being pressed
pub unsafe fn IsGamepadButtonUp(gamepad: i32, button: i32) -> bool
{
    let mut up: bool = false;

    if ((gamepad >= 0) && (gamepad < MAX_GAMEPADS as i32) && CORE.Input.Gamepad.ready[(gamepad) as usize] && (button < MAX_GAMEPAD_BUTTONS as i32))
    {
        if (CORE.Input.Gamepad.currentButtonState[(gamepad) as usize][(button) as usize] == 0) { up = true; }
    }

    return up;
}

// Get the last gamepad button pressed
// NOTE: Returns last gamepad button down, down->up change not considered
pub unsafe fn GetGamepadButtonPressed() -> i32
{
    return CORE.Input.Gamepad.lastButtonPressed;
}

// Get gamepad axis count
pub unsafe fn GetGamepadAxisCount(gamepad: i32) -> i32
{
    let mut result: i32 = 0;

    if ((gamepad >= 0) && (gamepad < MAX_GAMEPADS as i32)) { result = CORE.Input.Gamepad.axisCount[(gamepad) as usize]; }

    return result;
}

// Get axis movement vector for a gamepad
pub unsafe fn GetGamepadAxisMovement(gamepad: i32, axis: i32) -> f32
{
    let mut value: f32 = if ((axis == GamepadAxis::GAMEPAD_AXIS_LEFT_TRIGGER as i32) || (axis == GamepadAxis::GAMEPAD_AXIS_RIGHT_TRIGGER as i32)) { -1.0 } else { 0.0 };

    if ((gamepad >= 0) && (gamepad < MAX_GAMEPADS as i32) && CORE.Input.Gamepad.ready[(gamepad) as usize] && (axis < MAX_GAMEPAD_AXES as i32))
    {
        let movement: f32 = if (value < 0.0) { CORE.Input.Gamepad.axisState[gamepad as usize][axis as usize] } else { CORE.Input.Gamepad.axisState[gamepad as usize][axis as usize].abs() };

        if (movement > value) { value = CORE.Input.Gamepad.axisState[(gamepad) as usize][(axis) as usize]; }
    }

    return value;
}

//----------------------------------------------------------------------------------
// Module Functions Definition: Input Handling: Mouse
//----------------------------------------------------------------------------------

// NOTE: Functions with a platform-specific implementation on rcore_<platform>.c
//void SetMousePosition(int x, int y)
//void SetMouseCursor(int cursor)

// Check if mouse button has been pressed once
pub unsafe fn IsMouseButtonPressed(button: i32) -> bool
{
    let mut pressed: bool = false;

    if ((button >= 0) && (button <= MouseButton::MOUSE_BUTTON_BACK as i32))
    {
        if ((CORE.Input.Mouse.currentButtonState[(button) as usize] == 1) && (CORE.Input.Mouse.previousButtonState[(button) as usize] == 0)) { pressed = true; }

        // Map touches to mouse buttons checking
        if ((CORE.Input.Touch.currentTouchState[(button) as usize] == 1) && (CORE.Input.Touch.previousTouchState[(button) as usize] == 0)) { pressed = true; }
    }

    return pressed;
}

// Check if mouse button is being pressed
pub unsafe fn IsMouseButtonDown(button: i32) -> bool
{
    let mut down: bool = false;

    if ((button >= 0) && (button <= MouseButton::MOUSE_BUTTON_BACK as i32))
    {
        if (CORE.Input.Mouse.currentButtonState[(button) as usize] == 1) { down = true; }

        // NOTE: Touches are considered like mouse buttons
        if (CORE.Input.Touch.currentTouchState[(button) as usize] == 1) { down = true; }
    }

    return down;
}

// Check if mouse button has been released once
pub unsafe fn IsMouseButtonReleased(button: i32) -> bool
{
    let mut released: bool = false;

    if ((button >= 0) && (button <= MouseButton::MOUSE_BUTTON_BACK as i32))
    {
        if ((CORE.Input.Mouse.currentButtonState[(button) as usize] == 0) && (CORE.Input.Mouse.previousButtonState[(button) as usize] == 1)) { released = true; }

        // Map touches to mouse buttons checking
        if ((CORE.Input.Touch.currentTouchState[(button) as usize] == 0) && (CORE.Input.Touch.previousTouchState[(button) as usize] == 1)) { released = true; }
    }

    return released;
}

// Check if mouse button is NOT being pressed
pub unsafe fn IsMouseButtonUp(button: i32) -> bool
{
    let mut up: bool = false;

    if ((button >= 0) && (button <= MouseButton::MOUSE_BUTTON_BACK as i32))
    {
        if (CORE.Input.Mouse.currentButtonState[(button) as usize] == 0) { up = true; }

        // NOTE: Touches are considered like mouse buttons
        if (CORE.Input.Touch.currentTouchState[(button) as usize] == 0) { up = true; }
    }

    return up;
}

// Get mouse position X
pub unsafe fn GetMouseX() -> i32
{
    let mouseX: i32 = (((CORE.Input.Mouse.currentPosition.x + CORE.Input.Mouse.offset.x)*CORE.Input.Mouse.scale.x) as i32);

    return mouseX;
}

// Get mouse position Y
pub unsafe fn GetMouseY() -> i32
{
    let mouseY: i32 = (((CORE.Input.Mouse.currentPosition.y + CORE.Input.Mouse.offset.y)*CORE.Input.Mouse.scale.y) as i32);

    return mouseY;
}

// Get mouse position XY
pub unsafe fn GetMousePosition() -> Vector2
{
    let mut position: Vector2 = Vector2::ZERO;

    position.x = (CORE.Input.Mouse.currentPosition.x + CORE.Input.Mouse.offset.x)*CORE.Input.Mouse.scale.x;
    position.y = (CORE.Input.Mouse.currentPosition.y + CORE.Input.Mouse.offset.y)*CORE.Input.Mouse.scale.y;

    return position;
}

// Get mouse delta between frames
pub unsafe fn GetMouseDelta() -> Vector2
{
    let mut delta: Vector2 = Vector2::ZERO;

    delta.x = (CORE.Input.Mouse.currentPosition.x - CORE.Input.Mouse.previousPosition.x)*CORE.Input.Mouse.scale.x;
    delta.y = (CORE.Input.Mouse.currentPosition.y - CORE.Input.Mouse.previousPosition.y)*CORE.Input.Mouse.scale.y;

    return delta;
}

// Set mouse offset
// NOTE: Useful when rendering to different size targets
pub unsafe fn SetMouseOffset(offsetX: i32, offsetY: i32)
{
    CORE.Input.Mouse.offset = Vector2 { x: (offsetX as f32), y: (offsetY as f32) };
}

// Set mouse scaling
// NOTE: Useful when rendering to different size targets
pub unsafe fn SetMouseScale(scaleX: f32, scaleY: f32)
{
    CORE.Input.Mouse.scale = Vector2 { x: scaleX, y: scaleY };
}

// Get mouse wheel movement Y
pub unsafe fn GetMouseWheelMove() -> f32
{
    let mut result: f32 = 0.0;

    if (CORE.Input.Mouse.currentWheelMove.x.abs() > CORE.Input.Mouse.currentWheelMove.y.abs()) { result = (CORE.Input.Mouse.currentWheelMove.x as f32); }
    else { result = (CORE.Input.Mouse.currentWheelMove.y as f32); }

    return result;
}

// Get mouse wheel movement X/Y as a vector
pub unsafe fn GetMouseWheelMoveV() -> Vector2
{
    let mut result: Vector2 = Vector2::ZERO;

    result = CORE.Input.Mouse.currentWheelMove;

    return result;
}

//----------------------------------------------------------------------------------
// Module Functions Definition: Input Handling: Touch
//----------------------------------------------------------------------------------

// Get touch position X for touch point 0 (relative to screen size)
pub unsafe fn GetTouchX() -> i32
{
    let touchX: i32 = (CORE.Input.Touch.position[0].x as i32);
    return touchX;
}

// Get touch position Y for touch point 0 (relative to screen size)
pub unsafe fn GetTouchY() -> i32
{
    let touchY: i32 = (CORE.Input.Touch.position[0].y as i32);
    return touchY;
}

// Get touch position XY for a touch point index (relative to screen size)
pub unsafe fn GetTouchPosition(index: i32) -> Vector2
{
    let mut position: Vector2 = Vector2 { x: -1.0, y: -1.0 };

    if (index < MAX_TOUCH_POINTS as i32) { position = CORE.Input.Touch.position[(index) as usize]; }
    else { warn!("INPUT: Required touch point out of range (Max touch points: {})", MAX_TOUCH_POINTS); }

    return position;
}

// Get touch point identifier for provided index
pub unsafe fn GetTouchPointId(index: i32) -> i32
{
    let mut id: i32 = -1;

    if (index < MAX_TOUCH_POINTS as i32) { id = CORE.Input.Touch.pointId[(index) as usize]; }

    return id;
}

// Get number of touch points
pub unsafe fn GetTouchPointCount() -> i32
{
    return CORE.Input.Touch.pointCount;
}

//----------------------------------------------------------------------------------
// Module Internal Functions Definition
//----------------------------------------------------------------------------------

// NOTE: Functions with a platform-specific implementation on rcore_<platform>.c
//int InitPlatform(void)
//void ClosePlatform(void)

// Initialize hi-resolution timer
pub unsafe fn InitTimer()
{
    // Setting a higher resolution can improve the accuracy of time-out intervals in wait functions
    // However, it can also reduce overall system performance, because the thread scheduler switches tasks more often
    // High resolutions can also prevent the CPU power management system from entering power-saving modes
    // Setting a higher resolution does not improve the accuracy of the high-resolution performance counter
#[cfg(all(target_os = "windows", feature = "SUPPORT_WINMM_HIGHRES_TIMER", not(feature = "SUPPORT_BUSY_WAIT_LOOP"), not(feature = "PLATFORM_DESKTOP_SDL")))]
{
    timeBeginPeriod(1); // Setup high-resolution timer to 1ms (granularity of 1-2 ms)
}

#[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "openbsd", target_os = "emscripten"))]
{
    let mut now: libc::timespec = std::mem::zeroed();

    if (libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut now) == 0) { // Success
    {
        CORE.Time.base = (now.tv_sec as u64)*1000000000 + (now.tv_nsec as u64); }
    }
    else { warn!("TIMER: Hi-resolution timer not available"); }
}

    CORE.Time.previous = GetTime(); // Get time as double
}

// Set viewport for a provided width and height
pub unsafe fn SetupViewport(width: i32, height: i32)
{
    CORE.Window.render.x = width as f32;
    CORE.Window.render.y = height as f32;

    // Set viewport width and height
    rlViewport(CORE.Window.renderOffset.x as i32/2, CORE.Window.renderOffset.y as i32/2, CORE.Window.render.x as i32, CORE.Window.render.y as i32);

    rlMatrixMode(RL_PROJECTION);        // Switch to projection matrix
    rlLoadIdentity();                   // Reset current matrix (projection)

    // Set orthographic projection to current framebuffer size
    // NOTE: Configured top-left corner as (0, 0)
    rlOrtho(0.0, CORE.Window.render.x as f64, CORE.Window.render.y as f64, 0.0, 0.0, 1.0);

    rlMatrixMode(RL_MODELVIEW);         // Switch back to modelview matrix
    rlLoadIdentity();                   // Reset current matrix (modelview)
}
#[cfg(feature = "SUPPORT_AUTOMATION_EVENTS")]
// Automation event recording
// Checking events in current frame and save them into currentEventList
// NOTE: Recording is by default done at EndDrawing(), before PollInputEvents()
pub unsafe fn RecordAutomationEvent()
{
    if ((*currentEventList).count == (*currentEventList).capacity) { return; }

    // Keyboard input events recording
    //-------------------------------------------------------------------------------------
    for key in 0..MAX_KEYBOARD_KEYS as i32
    {
        // Event type: INPUT_KEY_UP (only saved once)
        if ((CORE.Input.Keyboard.previousKeyState[(key) as usize] != 0) && (CORE.Input.Keyboard.currentKeyState[(key) as usize] == 0))
        {
            (*(*currentEventList).events.add((*currentEventList).count as usize)).frame = CORE.Time.frameCounter;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).type_ = AutomationEventType::INPUT_KEY_UP as u32;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0] = key;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1] = 0;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2] = 0;

            info!("AUTOMATION: Frame: {} | Event type: INPUT_KEY_UP | Event parameters: {}, {}, {}", (*(*currentEventList).events.add((*currentEventList).count as usize)).frame, (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2]);
            (*currentEventList).count += 1;
        }

        if ((*currentEventList).count == (*currentEventList).capacity) { return; }    // Security check

        // Event type: INPUT_KEY_DOWN
        if ((CORE.Input.Keyboard.currentKeyState[(key) as usize] != 0))
        {
            (*(*currentEventList).events.add((*currentEventList).count as usize)).frame = CORE.Time.frameCounter;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).type_ = AutomationEventType::INPUT_KEY_DOWN as u32;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0] = key;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1] = 0;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2] = 0;

            info!("AUTOMATION: Frame: {} | Event type: INPUT_KEY_DOWN | Event parameters: {}, {}, {}", (*(*currentEventList).events.add((*currentEventList).count as usize)).frame, (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2]);
            (*currentEventList).count += 1;
        }

        if ((*currentEventList).count == (*currentEventList).capacity) { return; }    // Security check
    }
    //-------------------------------------------------------------------------------------

    // Mouse input currentEventList->events recording
    //-------------------------------------------------------------------------------------
    for button in 0..MAX_MOUSE_BUTTONS as i32
    {
        // Event type: INPUT_MOUSE_BUTTON_UP
        if ((CORE.Input.Mouse.previousButtonState[(button) as usize] != 0) && (CORE.Input.Mouse.currentButtonState[(button) as usize] == 0))
        {
            (*(*currentEventList).events.add((*currentEventList).count as usize)).frame = CORE.Time.frameCounter;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).type_ = AutomationEventType::INPUT_MOUSE_BUTTON_UP as u32;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0] = button;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1] = 0;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2] = 0;

            info!("AUTOMATION: Frame: {} | Event type: INPUT_MOUSE_BUTTON_UP | Event parameters: {}, {}, {}", (*(*currentEventList).events.add((*currentEventList).count as usize)).frame, (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2]);
            (*currentEventList).count += 1;
        }

        if ((*currentEventList).count == (*currentEventList).capacity) { return; }    // Security check

        // Event type: INPUT_MOUSE_BUTTON_DOWN
        if ((CORE.Input.Mouse.currentButtonState[(button) as usize] != 0))
        {
            (*(*currentEventList).events.add((*currentEventList).count as usize)).frame = CORE.Time.frameCounter;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).type_ = AutomationEventType::INPUT_MOUSE_BUTTON_DOWN as u32;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0] = button;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1] = 0;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2] = 0;

            info!("AUTOMATION: Frame: {} | Event type: INPUT_MOUSE_BUTTON_DOWN | Event parameters: {}, {}, {}", (*(*currentEventList).events.add((*currentEventList).count as usize)).frame, (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2]);
            (*currentEventList).count += 1;
        }

        if ((*currentEventList).count == (*currentEventList).capacity) { return; }    // Security check
    }

    // Event type: INPUT_MOUSE_POSITION (only saved if changed)
    if (((CORE.Input.Mouse.currentPosition.x as i32) != (CORE.Input.Mouse.previousPosition.x as i32)) ||
        ((CORE.Input.Mouse.currentPosition.y as i32) != (CORE.Input.Mouse.previousPosition.y as i32)))
    {
        (*(*currentEventList).events.add((*currentEventList).count as usize)).frame = CORE.Time.frameCounter;
        (*(*currentEventList).events.add((*currentEventList).count as usize)).type_ = AutomationEventType::INPUT_MOUSE_POSITION as u32;
        (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0] = (CORE.Input.Mouse.currentPosition.x as i32);
        (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1] = (CORE.Input.Mouse.currentPosition.y as i32);
        (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2] = 0;

        info!("AUTOMATION: Frame: {} | Event type: INPUT_MOUSE_POSITION | Event parameters: {}, {}, {}", (*(*currentEventList).events.add((*currentEventList).count as usize)).frame, (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2]);
        (*currentEventList).count += 1;

        if ((*currentEventList).count == (*currentEventList).capacity) { return; }    // Security check
    }

    // Event type: INPUT_MOUSE_WHEEL_MOTION
    if (((CORE.Input.Mouse.currentWheelMove.x as i32) != (CORE.Input.Mouse.previousWheelMove.x as i32)) ||
        ((CORE.Input.Mouse.currentWheelMove.y as i32) != (CORE.Input.Mouse.previousWheelMove.y as i32)))
    {
        (*(*currentEventList).events.add((*currentEventList).count as usize)).frame = CORE.Time.frameCounter;
        (*(*currentEventList).events.add((*currentEventList).count as usize)).type_ = AutomationEventType::INPUT_MOUSE_WHEEL_MOTION as u32;
        (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0] = (CORE.Input.Mouse.currentWheelMove.x as i32);
        (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1] = (CORE.Input.Mouse.currentWheelMove.y as i32);
        (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2] = 0;

        info!("AUTOMATION: Frame: {} | Event type: INPUT_MOUSE_WHEEL_MOTION | Event parameters: {}, {}, {}", (*(*currentEventList).events.add((*currentEventList).count as usize)).frame, (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2]);
        (*currentEventList).count += 1;

        if ((*currentEventList).count == (*currentEventList).capacity) { return; }    // Security check
    }
    //-------------------------------------------------------------------------------------

    // Touch input currentEventList->events recording
    //-------------------------------------------------------------------------------------
    for id in 0..MAX_TOUCH_POINTS as i32
    {
        // Event type: INPUT_TOUCH_UP
        if ((CORE.Input.Touch.previousTouchState[(id) as usize] != 0) && (CORE.Input.Touch.currentTouchState[(id) as usize] == 0))
        {
            (*(*currentEventList).events.add((*currentEventList).count as usize)).frame = CORE.Time.frameCounter;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).type_ = AutomationEventType::INPUT_TOUCH_UP as u32;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0] = id;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1] = 0;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2] = 0;

            info!("AUTOMATION: Frame: {} | Event type: INPUT_TOUCH_UP | Event parameters: {}, {}, {}", (*(*currentEventList).events.add((*currentEventList).count as usize)).frame, (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2]);
            (*currentEventList).count += 1;
        }

        if ((*currentEventList).count == (*currentEventList).capacity) { return; }    // Security check

        // Event type: INPUT_TOUCH_DOWN
        if ((CORE.Input.Touch.currentTouchState[(id) as usize] != 0))
        {
            (*(*currentEventList).events.add((*currentEventList).count as usize)).frame = CORE.Time.frameCounter;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).type_ = AutomationEventType::INPUT_TOUCH_DOWN as u32;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0] = id;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1] = 0;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2] = 0;

            info!("AUTOMATION: Frame: {} | Event type: INPUT_TOUCH_DOWN | Event parameters: {}, {}, {}", (*(*currentEventList).events.add((*currentEventList).count as usize)).frame, (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2]);
            (*currentEventList).count += 1;
        }

        if ((*currentEventList).count == (*currentEventList).capacity) { return; }    // Security check

        // Event type: INPUT_TOUCH_POSITION
        if (((CORE.Input.Touch.position[(id) as usize].x as i32) != (CORE.Input.Touch.previousPosition[(id) as usize].x as i32)) ||
            ((CORE.Input.Touch.position[(id) as usize].y as i32) != (CORE.Input.Touch.previousPosition[(id) as usize].y as i32)))
        {
            (*(*currentEventList).events.add((*currentEventList).count as usize)).frame = CORE.Time.frameCounter;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).type_ = AutomationEventType::INPUT_TOUCH_POSITION as u32;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0] = id;
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1] = (CORE.Input.Touch.position[(id) as usize].x as i32);
            (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2] = (CORE.Input.Touch.position[(id) as usize].y as i32);

            info!("AUTOMATION: Frame: {} | Event type: INPUT_TOUCH_POSITION | Event parameters: {}, {}, {}", (*(*currentEventList).events.add((*currentEventList).count as usize)).frame, (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2]);
            (*currentEventList).count += 1;
        }


        if ((*currentEventList).count == (*currentEventList).capacity) { return; }    // Security check
    }
    //-------------------------------------------------------------------------------------

    // Gamepad input currentEventList->events recording
    //-------------------------------------------------------------------------------------
    for gamepad in 0..MAX_GAMEPADS as i32
    {
        // Event type: INPUT_GAMEPAD_CONNECT
        /*
        if ((CORE.Input.Gamepad.currentState[gamepad] != CORE.Input.Gamepad.previousState[gamepad]) &&
            (CORE.Input.Gamepad.currentState[gamepad])) // Check if changed to ready
        {
            // TODO: Save gamepad connect event
        }
        */

        // Event type: INPUT_GAMEPAD_DISCONNECT
        /*
        if ((CORE.Input.Gamepad.currentState[gamepad] != CORE.Input.Gamepad.previousState[gamepad]) &&
            (!CORE.Input.Gamepad.currentState[gamepad])) // Check if changed to not-ready
        {
            // TODO: Save gamepad disconnect event
        }
        */

        for button in 0..MAX_GAMEPAD_BUTTONS as i32
        {
            // Event type: INPUT_GAMEPAD_BUTTON_UP
            if ((CORE.Input.Gamepad.previousButtonState[gamepad as usize][button as usize] != 0) && (CORE.Input.Gamepad.currentButtonState[gamepad as usize][button as usize] == 0))
            {
                (*(*currentEventList).events.add((*currentEventList).count as usize)).frame = CORE.Time.frameCounter;
                (*(*currentEventList).events.add((*currentEventList).count as usize)).type_ = AutomationEventType::INPUT_GAMEPAD_BUTTON_UP as u32;
                (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0] = gamepad;
                (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1] = button;
                (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2] = 0;

                info!("AUTOMATION: Frame: {} | Event type: INPUT_GAMEPAD_BUTTON_UP | Event parameters: {}, {}, {}", (*(*currentEventList).events.add((*currentEventList).count as usize)).frame, (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2]);
                (*currentEventList).count += 1;
            }

            if ((*currentEventList).count == (*currentEventList).capacity) { return; }    // Security check

            // Event type: INPUT_GAMEPAD_BUTTON_DOWN
            if (CORE.Input.Gamepad.currentButtonState[gamepad as usize][button as usize] != 0)
            {
                (*(*currentEventList).events.add((*currentEventList).count as usize)).frame = CORE.Time.frameCounter;
                (*(*currentEventList).events.add((*currentEventList).count as usize)).type_ = AutomationEventType::INPUT_GAMEPAD_BUTTON_DOWN as u32;
                (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0] = gamepad;
                (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1] = button;
                (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2] = 0;

                info!("AUTOMATION: Frame: {} | Event type: INPUT_GAMEPAD_BUTTON_DOWN | Event parameters: {}, {}, {}", (*(*currentEventList).events.add((*currentEventList).count as usize)).frame, (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2]);
                (*currentEventList).count += 1;
            }

            if ((*currentEventList).count == (*currentEventList).capacity) { return; }    // Security check
        }

        for axis in 0..MAX_GAMEPAD_AXES as i32
        {
            // Event type: INPUT_GAMEPAD_AXIS_MOTION
            let mut defaultMovement: f32 = if ((axis == GamepadAxis::GAMEPAD_AXIS_LEFT_TRIGGER as i32) || (axis == GamepadAxis::GAMEPAD_AXIS_RIGHT_TRIGGER as i32)) { -1.0 } else { 0.0 };
            if (GetGamepadAxisMovement(gamepad, axis) != defaultMovement)
            {
                (*(*currentEventList).events.add((*currentEventList).count as usize)).frame = CORE.Time.frameCounter;
                (*(*currentEventList).events.add((*currentEventList).count as usize)).type_ = AutomationEventType::INPUT_GAMEPAD_AXIS_MOTION as u32;
                (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0] = gamepad;
                (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1] = axis;
                (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2] = ((CORE.Input.Gamepad.axisState[(gamepad) as usize][(axis) as usize]*32768.0) as i32);

                info!("AUTOMATION: Frame: {} | Event type: INPUT_GAMEPAD_AXIS_MOTION | Event parameters: {}, {}, {}", (*(*currentEventList).events.add((*currentEventList).count as usize)).frame, (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2]);
                (*currentEventList).count += 1;
            }

            if ((*currentEventList).count == (*currentEventList).capacity) { return; }    // Security check
        }
    }
    //-------------------------------------------------------------------------------------

#[cfg(feature = "SUPPORT_GESTURES_SYSTEM")]
{
    // Gestures input currentEventList->events recording
    //-------------------------------------------------------------------------------------
    if (GESTURES.current != Gesture::GESTURE_NONE as i32)
    {
        // Event type: INPUT_GESTURE
        (*(*currentEventList).events.add((*currentEventList).count as usize)).frame = CORE.Time.frameCounter;
        (*(*currentEventList).events.add((*currentEventList).count as usize)).type_ = AutomationEventType::INPUT_GESTURE as u32;
        (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0] = GESTURES.current;
        (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1] = 0;
        (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2] = 0;

        info!("AUTOMATION: Frame: {} | Event type: INPUT_GESTURE | Event parameters: {}, {}, {}", (*(*currentEventList).events.add((*currentEventList).count as usize)).frame, (*(*currentEventList).events.add((*currentEventList).count as usize)).params[0], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[1], (*(*currentEventList).events.add((*currentEventList).count as usize)).params[2]);
        (*currentEventList).count += 1;

        if ((*currentEventList).count == (*currentEventList).capacity) { return; }    // Security check
    }
    //-------------------------------------------------------------------------------------
}
}


#[cfg(not(feature = "SUPPORT_MODULE_RTEXT"))]
// Formatting of text with variables to 'embed'
// WARNING: String returned will expire after this function is called MAX_TEXTFORMAT_BUFFERS times
pub fn TextFormat(text: std::fmt::Arguments<'_>) -> String
{
    const MAX_TEXTFORMAT_BUFFERS: usize = 4;        // Maximum number of static buffers for text formatting
    const MAX_TEXT_BUFFER_LENGTH: usize = 1024;     // Maximum size of static text buffer

    // Define an array of buffers, so strings don't expire until MAX_TEXTFORMAT_BUFFERS invocations
    // Rust returns an owned string, so a rotating static buffer is unnecessary.
    let mut currentBuffer = String::new();
    currentBuffer.clear();   // Clear buffer before using

    {
        let args = text;
        write!(currentBuffer, "{}", args).unwrap();
        let requiredByteCount = currentBuffer.len();

        // If requiredByteCount is larger than the MAX_TEXT_BUFFER_LENGTH, then overflow occurred
        if (requiredByteCount >= MAX_TEXT_BUFFER_LENGTH)
        {
            // Inserting "..." at the end of the string to mark as truncated
            let mut truncBuffer = MAX_TEXT_BUFFER_LENGTH - 4; // Adding 4 bytes = "...\0"
            while !currentBuffer.is_char_boundary(truncBuffer) { truncBuffer -= 1; }
            currentBuffer.truncate(truncBuffer);
            currentBuffer.push_str("...");
        }

        // Move to next buffer for next function call
        // Owned strings need no buffer index.
    }

    return currentBuffer;
}
