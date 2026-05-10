use std::ffi::CString;
use crate::types::Color;
use gl;

pub struct CoreData {
    pub sdl_context: Option<sdl3::Sdl>,
    pub video_subsystem: Option<sdl3::VideoSubsystem>,
    pub window: Option<sdl3::video::Window>,
    pub gl_context: Option<sdl3::video::GLContext>,
    pub event_pump: Option<sdl3::EventPump>,
    pub window_should_close: bool,
}

pub static mut CORE: CoreData = CoreData {
    sdl_context: None,
    video_subsystem: None,
    window: None,
    gl_context: None,
    event_pump: None,
    window_should_close: false,
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

        gl::load_with(|name| {
            if let Some(vs) = unsafe { &CORE.video_subsystem } {
                vs.gl_get_proc_address(name).map(|f| f as *const std::ffi::c_void).unwrap_or(std::ptr::null())
            } else {
                std::ptr::null()
            }
        });
        crate::rlgl::rlgl_init(width, height);
    }
}

pub fn window_should_close() -> bool {
    unsafe {
        if let Some(event_pump) = &mut CORE.event_pump {
            for event in event_pump.poll_iter() {
                match event {
                    sdl3::event::Event::Quit { .. } => {
                        CORE.window_should_close = true;
                    }
                    sdl3::event::Event::KeyDown { keycode: Some(sdl3::keyboard::Keycode::Escape), .. } => {
                        CORE.window_should_close = true;
                    }
                    _ => {}
                }
            }
        }
        CORE.window_should_close
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
        crate::rlgl::rl_matrix_mode(crate::rlgl::RL_MODELVIEW);
        crate::rlgl::rl_load_identity();
        if let Some(f) = crate::external::glad_glClearColor {
            f(1.0, 1.0, 1.0, 1.0); // RAYWHITE
            crate::external::glad_glClear.unwrap()(crate::external::GL_COLOR_BUFFER_BIT | crate::external::GL_DEPTH_BUFFER_BIT);
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
    // Stub for now, in a real port we would track time
}

pub fn end_drawing() {
    unsafe {
        // rlgl logic: draw batches
        if let Some(batch_ptr) = crate::rlgl::RLGL.current_batch {
            crate::rlgl::rl_draw_render_batch(batch_ptr);
        }

        if let Some(window) = &CORE.window {
            window.gl_swap_window();
        }
    }
}
