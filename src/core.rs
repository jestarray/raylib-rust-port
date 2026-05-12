use std::ffi::{c_char, CString};

use crate::{
    rlgl::{rlGetVersion, rlGlVersion},
    types::{Color, Matrix, Vector2},
};
use gl;
use sdl3::{self, event::EventType, sys::video, video::WindowFlags};

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

static mut CORE: CoreData = CoreData {
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

#[derive(Default)]
pub struct PlatformData {
    window: Option<sdl3::video::Window>,
    gl_context: Option<sdl3::video::GLContext>,
    video_subsystem: Option<sdl3::VideoSubsystem>, // new
    event_pump: Option<sdl3::EventPump>,           // new

    gamepad: Option<sdl3::gamepad::Gamepad>,
    gamepadId: [Option<sdl3::joystick::JoystickId>; MAX_GAMEPADS],
    cursor: Option<sdl3::mouse::Cursor>,
}

pub static mut PLATFORM: PlatformData = PlatformData {
    window: None,
    gl_context: None,
    video_subsystem: None,
    event_pump: None,
    gamepad: None,
    gamepadId: [None; MAX_GAMEPADS],
    cursor: None,
};

pub fn InitPlatform() {
    let sdl_context = sdl3::init().expect("Failed to initialize SDL3");
    let video_subsystem = sdl_context
        .video()
        .expect("Failed to initialize video subsystem");

    let mut flags = 0;
    flags |= WindowFlags::INPUT_FOCUS.bits();
    flags |= WindowFlags::MOUSE_FOCUS.bits();
    flags |= WindowFlags::MOUSE_CAPTURE.bits();

    let gl_version = rlGetVersion();

    if gl_version != rlGlVersion::RL_OPENGL_SOFTWARE {
        flags |= WindowFlags::OPENGL.bits();
    }
    let gl_attr = video_subsystem.gl_attr();
    match gl_version {
        rlGlVersion::RL_OPENGL_SOFTWARE => {}
        rlGlVersion::RL_OPENGL_11 => todo!(),
        rlGlVersion::RL_OPENGL_21 => todo!(),
        rlGlVersion::RL_OPENGL_33 => {
            gl_attr.set_context_major_version(3);
            gl_attr.set_context_minor_version(3);
            gl_attr.set_context_profile(sdl3::video::GLProfile::Core);
        }
        rlGlVersion::RL_OPENGL_43 => {
            gl_attr.set_context_major_version(4);
            gl_attr.set_context_minor_version(3);
            gl_attr.set_context_profile(sdl3::video::GLProfile::Core);
            #[cfg(feature = "RLGL_ENABLE_OPENGL_DEBUG_CONTEXT")]
            {
                gl_attr.set_context_flags().debug().set();
            }
        }
        rlGlVersion::RL_OPENGL_ES_20 => {}
        rlGlVersion::RL_OPENGL_ES_30 => {}
    }

    unsafe {
        let window = video_subsystem
            .window(
                "test",
                CORE.Window.screen.x as u32,
                CORE.Window.screen.y as u32,
            )
            .position_centered()
            .opengl()
            .build()
            .expect("Failed to create window");

        let gl_context = window
            .gl_create_context()
            .expect("Failed to create GL context");
        window.gl_make_current(&gl_context).unwrap();

        let event_pump = sdl_context
            .event_pump()
            .expect("Failed to create event pump");

        PLATFORM.window = Some(window);
        PLATFORM.video_subsystem = Some(video_subsystem);
        PLATFORM.gl_context = Some(gl_context);
        PLATFORM.event_pump = Some(event_pump);

        if let (Some(window), Some(glContext)) = (&PLATFORM.window, &PLATFORM.gl_context) {
            CORE.Window.ready = true;
            let display = window.get_display().unwrap();
            let displayMode = display.get_mode().unwrap();
            CORE.Window.display.x = displayMode.w as f32;
            CORE.Window.display.y = displayMode.h as f32;

            CORE.Window.render.x = CORE.Window.screen.x;
            CORE.Window.render.y = CORE.Window.screen.y;
            CORE.Window.currentFbo.x = CORE.Window.render.x;
            CORE.Window.currentFbo.y = CORE.Window.render.y;

            gl::load_with(|name| {
                if let Some(vs) = &PLATFORM.video_subsystem {
                    vs.gl_get_proc_address(name)
                        .map(|f| f as *const std::ffi::c_void)
                        .unwrap_or(std::ptr::null())
                } else {
                    std::ptr::null()
                }
            });
            crate::rlgl::rlLoadExtensions(std::ptr::null_mut());
        }

        //if let Some(video_subsystem) = &CORE.video_subsystem {
        //    let v = sdl3::sys::video::SDL_GL_GetProcAddress;
        //    crate::rlgl::rlLoadExtensions(v as *mut std::ffi::c_void);
        //}
    }
}

pub fn PollInputEvents() {
    unsafe {
        if let Some(event_pump) = &mut PLATFORM.event_pump {
            for event in event_pump.poll_iter() {
                match event {
                    sdl3::event::Event::Quit { .. } => {
                        CORE.Window.shouldClose = true;
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn InitWindow(width: i32, height: i32, title: &str) {
    unsafe {
        CORE.Window.screen.x = width as f32;
        CORE.Window.screen.y = height as f32;

        CORE.Window.eventWaiting = false;
        CORE.Window.screenScale = Matrix::IDENTITY;

        // Initialize global input state
        CORE.Input.Keyboard.exitKey = KeyboardKey::KEY_ESCAPE as i32;
        CORE.Input.Mouse.scale = Vector2::new(1.0, 1.0);
        CORE.Input.Mouse.cursor = MouseCursor::MOUSE_CURSOR_ARROW as i32;
        CORE.Input.Gamepad.lastButtonPressed = GamepadButton::GAMEPAD_BUTTON_UNKNOWN as i32;

        InitPlatform();

        crate::rlgl::rlglInit(width, height);
        crate::rtext::load_font_default();
    }
}

pub fn WindowShouldClose() -> bool {
    unsafe {
        if CORE.Window.ready {
            return CORE.Window.shouldClose;
        } else {
            return true;
        }
    }
}

pub const DEFAULT_VSHADER: &str = "#version 330
    in vec3 vertexPosition;
    in vec2 vertexTexCoord;
    in vec4 vertexColor;
    out vec2 fragTexCoord;
    out vec4 fragColor;
    uniform mat4 mvp;
    void main() {
        fragTexCoord = vertexTexCoord;
        fragColor = vertexColor;
        gl_Position = mvp * vec4(vertexPosition, 1.0);
    }";

pub const DEFAULT_FSHADER: &str = "#version 330
    in vec2 fragTexCoord;
    in vec4 fragColor;
    out vec4 finalColor;
    uniform sampler2D texture0;
    void main() {
        finalColor = fragColor * texture(texture0, fragTexCoord);
    }";

fn map_sdl_key(k: sdl3::keyboard::Keycode) -> i32 {
    use sdl3::keyboard::Keycode::*;
    match k {
        Right => 262,
        Left => 263,
        Down => 264,
        Up => 265,
        A => 65,
        B => 66,
        C => 67,
        D => 68,
        E => 69,
        F => 70,
        G => 71,
        H => 72,
        I => 73,
        J => 74,
        K => 75,
        L => 76,
        M => 77,
        N => 78,
        O => 79,
        P => 80,
        Q => 81,
        R => 82,
        S => 83,
        T => 84,
        U => 85,
        V => 86,
        W => 87,
        X => 88,
        Y => 89,
        Z => 90,
        Space => 32,
        _ => k as i32,
    }
}

pub fn close_window() {}

pub fn begin_drawing() {
    unsafe {
        // WARNING: Previously to BeginDrawing() other render textures drawing could happen,
        // consequently the measure for update vs draw is not accurate (only the total frame time is accurate)

        //CORE.Time.current = GetTime(); // Number of elapsed seconds since InitTimer()
        //CORE.Time.update = CORE.Time.current - CORE.Time.previous;
        //CORE.Time.previous = CORE.Time.current;

        crate::rlgl::rlLoadIdentity();
        crate::rlgl::rlMultMatrixf(&Matrix::IDENTITY.to_array());
    }
}

pub fn EndDrawing() {
    unsafe {
        crate::rlgl::rlDrawRenderBatchActive();
        if let Some(window) = &PLATFORM.window {
            window.gl_swap_window();
        }
        PollInputEvents();
    }
}

pub fn clear_background(color: Color) {
    unsafe {
        gl::ClearColor(
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
            color.a as f32 / 255.0,
        );
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

pub fn set_target_fps(_fps: i32) {
    // Stub
}

pub fn get_frame_time() -> f32 {
    0.016 // Stub
}

pub fn IsKeyDown(key: i32) -> bool {
    unsafe {
        let mut down = false;
        if key > 0 && key < MAX_KEYBOARD_KEYS as i32 {
            if CORE.Input.Keyboard.currentKeyState[key as usize] == 1 {
                down = true;
            }
        }
        return down;
    }
}

pub fn is_key_pressed(key: i32) -> bool {
    unsafe {
        let mut released = false;
        if key > 0 && key < MAX_KEYBOARD_KEYS as i32 {
            if CORE.Input.Keyboard.currentKeyState[key as usize] == 1
                && CORE.Input.Keyboard.previousKeyState[key as usize] == 0
            {
                released = true;
            }
        }
        return released;
    }
}

// Check if a mouse button is being pressed
pub fn IsMouseButtonDown(button: i32) -> bool {
    let mut down = false;

    unsafe {
        if ((button >= 0) && (button <= MouseButton::MOUSE_BUTTON_BACK as i32)) {
            if (CORE.Input.Mouse.currentButtonState[button as usize] == 1) {
                down = true;
            }

            // NOTE: Touches are considered like mouse buttons
            if (CORE.Input.Touch.currentTouchState[button as usize] == 1) {
                down = true;
            }
        }
    }

    return down;
}

pub fn begin_mode_2d(camera: crate::types::Camera2D) {
    unsafe {
        crate::rlgl::rlDrawRenderBatchActive();

        crate::rlgl::rlMatrixMode(crate::rlgl::RL_PROJECTION);
        crate::rlgl::rlPushMatrix();
        crate::rlgl::rlLoadIdentity();

        crate::rlgl::rlOrtho(
            0.0,
            crate::rlgl::RLGL.State.framebufferWidth as f64,
            crate::rlgl::RLGL.State.framebufferHeight as f64,
            0.0,
            -1.0,
            1.0,
        );

        crate::rlgl::rlMatrixMode(crate::rlgl::RL_MODELVIEW);
        crate::rlgl::rlPushMatrix();
        crate::rlgl::rlLoadIdentity();

        crate::rlgl::rlTranslatef(camera.offset.x, camera.offset.y, 0.0);
        crate::rlgl::rlRotatef(camera.rotation, 0.0, 0.0, 1.0);
        crate::rlgl::rlScalef(camera.zoom, camera.zoom, 1.0);
        crate::rlgl::rlTranslatef(-camera.target.x, -camera.target.y, 0.0);
    }
}

pub fn end_mode_2d() {
    unsafe {
        crate::rlgl::rlDrawRenderBatchActive();

        crate::rlgl::rlMatrixMode(crate::rlgl::RL_MODELVIEW);
        crate::rlgl::rlPopMatrix();

        crate::rlgl::rlMatrixMode(crate::rlgl::RL_PROJECTION);
        crate::rlgl::rlPopMatrix();
    }
}

pub fn begin_mode_3d(camera: crate::types::Camera) {
    //unsafe {
    //    crate::rlgl::rlDrawRenderBatchActive();
    //
    //    crate::rlgl::rlMatrixMode(crate::rlgl::RL_PROJECTION);
    //    crate::rlgl::rlPushMatrix();
    //    crate::rlgl::rlLoadIdentity();
    //
    //    let aspect = crate::rlgl::RLGL.State.framebufferWidth as f32 / crate::rlgl::RLGL.State.framebufferHeight as f32;
    //    let proj = crate::rcamera::get_camera_projection_matrix(&camera, aspect);
    //    crate::rlgl::rlMultMatrixf(proj.to_cols_array().as_ptr());
    //
    //    crate::rlgl::rlMatrixMode(crate::rlgl::RL_MODELVIEW);
    //    crate::rlgl::rlPushMatrix();
    //    crate::rlgl::rlLoadIdentity();
    //
    //    let view = crate::rcamera::get_camera_view_matrix(&camera);
    //    crate::rlgl::rlMultMatrixf(view.to_cols_array().as_ptr());
    //
    //    crate::rlgl::rlEnableDepthTest();
    //}
}

pub fn end_mode_3d() {
    unsafe {
        crate::rlgl::rlDrawRenderBatchActive();

        crate::rlgl::rlMatrixMode(crate::rlgl::RL_MODELVIEW);
        crate::rlgl::rlPopMatrix();

        crate::rlgl::rlMatrixMode(crate::rlgl::RL_PROJECTION);
        crate::rlgl::rlPopMatrix();

        crate::rlgl::rlDisableDepthTest();
    }
}

pub const KEY_RIGHT: i32 = 262;
pub const KEY_LEFT: i32 = 263;
pub const KEY_DOWN: i32 = 264;
pub const KEY_UP: i32 = 265;
pub const KEY_A: i32 = 65;
pub const KEY_S: i32 = 83;
pub const KEY_R: i32 = 82;

pub fn fade(color: Color, alpha: f32) -> Color {
    let mut result = color;
    let mut a = alpha;
    if a < 0.0 {
        a = 0.0;
    }
    if a > 1.0 {
        a = 1.0;
    }
    result.a = (color.a as f32 * a) as u8;
    result
}

pub fn get_random_value(min: i32, max: i32) -> i32 {
    unsafe { libc::rand() % (max - min + 1) + min }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KeyboardKey {
    KEY_NULL = 0,
    KEY_APOSTROPHE = 39,
    KEY_COMMA = 44,
    KEY_MINUS = 45,
    KEY_PERIOD = 46,
    KEY_SLASH = 47,
    KEY_ZERO = 48,
    KEY_ONE = 49,
    KEY_TWO = 50,
    KEY_THREE = 51,
    KEY_FOUR = 52,
    KEY_FIVE = 53,
    KEY_SIX = 54,
    KEY_SEVEN = 55,
    KEY_EIGHT = 56,
    KEY_NINE = 57,
    KEY_SEMICOLON = 59,
    KEY_EQUAL = 61,
    KEY_A = 65,
    KEY_B = 66,
    KEY_C = 67,
    KEY_D = 68,
    KEY_E = 69,
    KEY_F = 70,
    KEY_G = 71,
    KEY_H = 72,
    KEY_I = 73,
    KEY_J = 74,
    KEY_K = 75,
    KEY_L = 76,
    KEY_M = 77,
    KEY_N = 78,
    KEY_O = 79,
    KEY_P = 80,
    KEY_Q = 81,
    KEY_R = 82,
    KEY_S = 83,
    KEY_T = 84,
    KEY_U = 85,
    KEY_V = 86,
    KEY_W = 87,
    KEY_X = 88,
    KEY_Y = 89,
    KEY_Z = 90,
    KEY_LEFT_BRACKET = 91,
    KEY_BACKSLASH = 92,
    KEY_RIGHT_BRACKET = 93,
    KEY_GRAVE = 96,
    KEY_SPACE = 32,
    KEY_ESCAPE = 256,
    KEY_ENTER = 257,
    KEY_TAB = 258,
    KEY_BACKSPACE = 259,
    KEY_INSERT = 260,
    KEY_DELETE = 261,
    KEY_RIGHT = 262,
    KEY_LEFT = 263,
    KEY_DOWN = 264,
    KEY_UP = 265,
    KEY_PAGE_UP = 266,
    KEY_PAGE_DOWN = 267,
    KEY_HOME = 268,
    KEY_END = 269,
    KEY_CAPS_LOCK = 280,
    KEY_SCROLL_LOCK = 281,
    KEY_NUM_LOCK = 282,
    KEY_PRINT_SCREEN = 283,
    KEY_PAUSE = 284,
    KEY_F1 = 290,
    KEY_F2 = 291,
    KEY_F3 = 292,
    KEY_F4 = 293,
    KEY_F5 = 294,
    KEY_F6 = 295,
    KEY_F7 = 296,
    KEY_F8 = 297,
    KEY_F9 = 298,
    KEY_F10 = 299,
    KEY_F11 = 300,
    KEY_F12 = 301,
    KEY_LEFT_SHIFT = 340,
    KEY_LEFT_CONTROL = 341,
    KEY_LEFT_ALT = 342,
    KEY_LEFT_SUPER = 343,
    KEY_RIGHT_SHIFT = 344,
    KEY_RIGHT_CONTROL = 345,
    KEY_RIGHT_ALT = 346,
    KEY_RIGHT_SUPER = 347,
    KEY_KB_MENU = 348,
    KEY_KP_0 = 320,
    KEY_KP_1 = 321,
    KEY_KP_2 = 322,
    KEY_KP_3 = 323,
    KEY_KP_4 = 324,
    KEY_KP_5 = 325,
    KEY_KP_6 = 326,
    KEY_KP_7 = 327,
    KEY_KP_8 = 328,
    KEY_KP_9 = 329,
    KEY_KP_DECIMAL = 330,
    KEY_KP_DIVIDE = 331,
    KEY_KP_MULTIPLY = 332,
    KEY_KP_SUBTRACT = 333,
    KEY_KP_ADD = 334,
    KEY_KP_ENTER = 335,
    KEY_KP_EQUAL = 336,
    KEY_BACK = 4,
    KEY_MENU = 5,
    KEY_VOLUME_UP = 24,
    KEY_VOLUME_DOWN = 25,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MouseButton {
    MOUSE_BUTTON_LEFT = 0,
    MOUSE_BUTTON_RIGHT = 1,
    MOUSE_BUTTON_MIDDLE = 2,
    MOUSE_BUTTON_SIDE = 3,
    MOUSE_BUTTON_EXTRA = 4,
    MOUSE_BUTTON_FORWARD = 5,
    MOUSE_BUTTON_BACK = 6,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MouseCursor {
    MOUSE_CURSOR_DEFAULT = 0,
    MOUSE_CURSOR_ARROW = 1,
    MOUSE_CURSOR_IBEAM = 2,
    MOUSE_CURSOR_CROSSHAIR = 3,
    MOUSE_CURSOR_POINTING_HAND = 4,
    MOUSE_CURSOR_RESIZE_EW = 5,
    MOUSE_CURSOR_RESIZE_NS = 6,
    MOUSE_CURSOR_RESIZE_NWSE = 7,
    MOUSE_CURSOR_RESIZE_NESW = 8,
    MOUSE_CURSOR_RESIZE_ALL = 9,
    MOUSE_CURSOR_NOT_ALLOWED = 10,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GamepadButton {
    GAMEPAD_BUTTON_UNKNOWN = 0,
    GAMEPAD_BUTTON_LEFT_FACE_UP = 1,
    GAMEPAD_BUTTON_LEFT_FACE_RIGHT = 2,
    GAMEPAD_BUTTON_LEFT_FACE_DOWN = 3,
    GAMEPAD_BUTTON_LEFT_FACE_LEFT = 4,
    GAMEPAD_BUTTON_RIGHT_FACE_UP = 5,
    GAMEPAD_BUTTON_RIGHT_FACE_RIGHT = 6,
    GAMEPAD_BUTTON_RIGHT_FACE_DOWN = 7,
    GAMEPAD_BUTTON_RIGHT_FACE_LEFT = 8,
    GAMEPAD_BUTTON_LEFT_TRIGGER_1 = 9,
    GAMEPAD_BUTTON_LEFT_TRIGGER_2 = 10,
    GAMEPAD_BUTTON_RIGHT_TRIGGER_1 = 11,
    GAMEPAD_BUTTON_RIGHT_TRIGGER_2 = 12,
    GAMEPAD_BUTTON_MIDDLE_LEFT = 13,
    GAMEPAD_BUTTON_MIDDLE = 14,
    GAMEPAD_BUTTON_MIDDLE_RIGHT = 15,
    GAMEPAD_BUTTON_LEFT_THUMB = 16,
    GAMEPAD_BUTTON_RIGHT_THUMB = 17,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GamepadAxis {
    GAMEPAD_AXIS_LEFT_X = 0,
    GAMEPAD_AXIS_LEFT_Y = 1,
    GAMEPAD_AXIS_RIGHT_X = 2,
    GAMEPAD_AXIS_RIGHT_Y = 3,
    GAMEPAD_AXIS_LEFT_TRIGGER = 4,
    GAMEPAD_AXIS_RIGHT_TRIGGER = 5,
}
