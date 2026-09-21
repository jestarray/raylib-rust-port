use raylib::core::{begin_drawing, clear_background, close_window, end_drawing, init_window};
use raylib::rcolors::{BLUE, GREEN, RED};
use raylib::rlgl;
use raylib::sdl::window_should_close;
use raylib::shapes::{draw_circle, draw_line, draw_rectangle};

fn main() {
    env_logger::init_from_env(env_logger::Env::default().filter_or("MY_LOG_LEVEL", "info"));

    init_window(640, 480, "raylib-rs - MVP Demo");

    // rlgl is not wrapped by `raylib::safe` yet, so this single call stays unsafe.
    unsafe { rlgl::rlCheckErrors() };

    while !window_should_close() {
        begin_drawing();
        clear_background(RED);
        draw_rectangle(100, 100, 200, 150, RED);
        draw_circle(400, 225, 50.0, BLUE);
        draw_line(0, 0, 800, 450, GREEN);
        end_drawing();
    }

    close_window();
}
