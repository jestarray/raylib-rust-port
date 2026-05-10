use crate::types::{Color, Vector2};
use gl;
use sdl3;

pub struct CoreData {
    pub sdl_context: Option<sdl3::Sdl>,
    pub video_subsystem: Option<sdl3::VideoSubsystem>,
    pub window: Option<sdl3::video::Window>,
    pub gl_context: Option<sdl3::video::GLContext>,
    pub event_pump: Option<sdl3::EventPump>,
    pub window_should_close: bool,
    pub key_state: [bool; 512],
    pub prev_key_state: [bool; 512],
    pub mouse_button_state: [bool; 5],
    pub prev_mouse_button_state: [bool; 5],
    pub mouse_position: Vector2,
    pub mouse_delta: Vector2,
    pub mouse_wheel_move: f32,
    pub frame_time: f32,
    pub start_time: std::time::Instant,
}

pub static mut CORE: CoreData = CoreData {
    sdl_context: None,
    video_subsystem: None,
    window: None,
    gl_context: None,
    event_pump: None,
    window_should_close: false,
    key_state: [false; 512],
    prev_key_state: [false; 512],
    mouse_button_state: [false; 5],
    prev_mouse_button_state: [false; 5],
    mouse_position: Vector2::ZERO,
    mouse_delta: Vector2::ZERO,
    mouse_wheel_move: 0.0,
    frame_time: 0.016,
    start_time: unsafe { std::mem::transmute([0u8; std::mem::size_of::<std::time::Instant>()]) },
};

pub fn init_window(width: i32, height: i32, title: &str) {
    unsafe {
        let sdl_context = sdl3::init().expect("Failed to initialize SDL3");
        let video_subsystem = sdl_context.video().expect("Failed to initialize video subsystem");

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl3::video::GLProfile::Compatibility);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem.window(title, width as u32, height as u32)
            .position_centered()
            .opengl()
            .build()
            .expect("Failed to create window");

        let gl_context = window.gl_create_context().expect("Failed to create GL context");
        window.gl_make_current(&gl_context).unwrap();

        let event_pump = sdl_context.event_pump().expect("Failed to create event pump");

        CORE.sdl_context = Some(sdl_context);
        CORE.video_subsystem = Some(video_subsystem);
        CORE.window = Some(window);
        CORE.gl_context = Some(gl_context);
        CORE.event_pump = Some(event_pump);
        CORE.window_should_close = false;
        CORE.start_time = std::time::Instant::now();

        gl::load_with(|name| {
            if let Some(vs) = &CORE.video_subsystem {
                vs.gl_get_proc_address(name).map(|f| f as *const std::ffi::c_void).unwrap_or(std::ptr::null())
            } else {
                std::ptr::null()
            }
        });
        crate::rlgl::rlglInit(width, height);
        crate::rtext::load_font_default();
    }
}

pub fn window_should_close() -> bool {
    unsafe {
        CORE.prev_key_state = CORE.key_state;
        CORE.prev_mouse_button_state = CORE.mouse_button_state;
        CORE.mouse_wheel_move = 0.0;
        
        let mut mouse_x = 0;
        let mut mouse_y = 0;
        
        if let Some(event_pump) = &mut CORE.event_pump {
            for event in event_pump.poll_iter() {
                match event {
                    sdl3::event::Event::Quit { .. } => {
                        CORE.window_should_close = true;
                    }
                    sdl3::event::Event::KeyDown { keycode: Some(k), .. } => {
                        let code = k as i32;
                        let index = map_sdl_key(k);
                        if index > 0 && index < 512 { CORE.key_state[index as usize] = true; }
                        if k == sdl3::keyboard::Keycode::Escape {
                            CORE.window_should_close = true;
                        }
                    }
                    sdl3::event::Event::KeyUp { keycode: Some(k), .. } => {
                        let index = map_sdl_key(k);
                        if index > 0 && index < 512 { CORE.key_state[index as usize] = false; }
                    }
                    sdl3::event::Event::MouseButtonDown { mouse_btn, .. } => {
                        let btn = match mouse_btn {
                            sdl3::mouse::MouseButton::Left => 0,
                            sdl3::mouse::MouseButton::Right => 1,
                            sdl3::mouse::MouseButton::Middle => 2,
                            _ => 3,
                        };
                        CORE.mouse_button_state[btn] = true;
                    }
                    sdl3::event::Event::MouseButtonUp { mouse_btn, .. } => {
                        let btn = match mouse_btn {
                            sdl3::mouse::MouseButton::Left => 0,
                            sdl3::mouse::MouseButton::Right => 1,
                            sdl3::mouse::MouseButton::Middle => 2,
                            _ => 3,
                        };
                        CORE.mouse_button_state[btn] = false;
                    }
                    sdl3::event::Event::MouseWheel { y, .. } => {
                        CORE.mouse_wheel_move = y as f32;
                    }
                    sdl3::event::Event::MouseMotion { x, y, .. } => {
                        mouse_x = x as i32;
                        mouse_y = y as i32;
                    }
                    _ => {}
                }
            }
        }
        
        let new_mouse_pos = Vector2::new(mouse_x as f32, mouse_y as f32);
        CORE.mouse_delta = new_mouse_pos - CORE.mouse_position;
        CORE.mouse_position = new_mouse_pos;
        
        CORE.window_should_close
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
        A => 65, B => 66, C => 67, D => 68, E => 69, F => 70, G => 71, H => 72, I => 73, J => 74, K => 75, L => 76, M => 77,
        N => 78, O => 79, P => 80, Q => 81, R => 82, S => 83, T => 84, U => 85, V => 86, W => 87, X => 88, Y => 89, Z => 90,
        Space => 32,
        _ => k as i32,
    }
}

pub fn close_window() {
    unsafe {
        CORE.gl_context = None;
        CORE.window = None;
        CORE.event_pump = None;
        CORE.video_subsystem = None;
        CORE.sdl_context = None;
    }
}

pub fn begin_drawing() {
    unsafe {
        crate::rlgl::rlMatrixMode(crate::rlgl::RL_PROJECTION);
        crate::rlgl::rlLoadIdentity();
        crate::rlgl::rlOrtho(0.0, crate::rlgl::RLGL.State.framebufferWidth as f64, crate::rlgl::RLGL.State.framebufferHeight as f64, 0.0, -1.0, 1.0);
        
        crate::rlgl::rlMatrixMode(crate::rlgl::RL_MODELVIEW);
        crate::rlgl::rlLoadIdentity();
    }
}

pub fn end_drawing() {
    unsafe {
        crate::rlgl::rlDrawRenderBatchActive();
        if let Some(window) = &CORE.window {
            window.gl_swap_window();
        }
    }
}

pub fn clear_background(color: Color) {
    unsafe {
        gl::ClearColor(color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0, color.a as f32 / 255.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }
}

pub fn set_target_fps(_fps: i32) {
    // Stub
}

pub fn get_frame_time() -> f32 {
    0.016 // Stub
}

pub fn is_key_down(key: i32) -> bool {
    unsafe { if key < 512 { CORE.key_state[key as usize] } else { false } }
}

pub fn is_key_pressed(key: i32) -> bool {
    unsafe { if key < 512 { CORE.key_state[key as usize] && !CORE.prev_key_state[key as usize] } else { false } }
}

pub fn is_mouse_button_down(button: i32) -> bool {
    unsafe { if button < 5 { CORE.mouse_button_state[button as usize] } else { false } }
}

pub fn get_mouse_position() -> Vector2 {
    unsafe { CORE.mouse_position }
}

pub fn get_mouse_wheel_move() -> f32 {
    unsafe { CORE.mouse_wheel_move }
}

pub fn begin_mode_2d(camera: crate::types::Camera2D) {
    unsafe {
        crate::rlgl::rlDrawRenderBatchActive();

        crate::rlgl::rlMatrixMode(crate::rlgl::RL_PROJECTION);
        crate::rlgl::rlPushMatrix();
        crate::rlgl::rlLoadIdentity();

        crate::rlgl::rlOrtho(0.0, crate::rlgl::RLGL.State.framebufferWidth as f64, crate::rlgl::RLGL.State.framebufferHeight as f64, 0.0, -1.0, 1.0);

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
    unsafe {
        crate::rlgl::rlDrawRenderBatchActive();
        
        crate::rlgl::rlMatrixMode(crate::rlgl::RL_PROJECTION);
        crate::rlgl::rlPushMatrix();
        crate::rlgl::rlLoadIdentity();
        
        let aspect = crate::rlgl::RLGL.State.framebufferWidth as f32 / crate::rlgl::RLGL.State.framebufferHeight as f32;
        let proj = crate::rcamera::get_camera_projection_matrix(&camera, aspect);
        crate::rlgl::rlMultMatrixf(proj.to_cols_array().as_ptr());
        
        crate::rlgl::rlMatrixMode(crate::rlgl::RL_MODELVIEW);
        crate::rlgl::rlPushMatrix();
        crate::rlgl::rlLoadIdentity();
        
        let view = crate::rcamera::get_camera_view_matrix(&camera);
        crate::rlgl::rlMultMatrixf(view.to_cols_array().as_ptr());
        
        crate::rlgl::rlEnableDepthTest();
    }
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
    if a < 0.0 { a = 0.0; }
    if a > 1.0 { a = 1.0; }
    result.a = (color.a as f32 * a) as u8;
    result
}

pub fn get_random_value(min: i32, max: i32) -> i32 {
    unsafe {
        libc::rand() % (max - min + 1) + min
    }
}
