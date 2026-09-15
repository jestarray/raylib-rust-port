use raylib_rs::core::{
    begin_drawing, clear_background, close_window, EndDrawing, InitWindow, WindowShouldClose,
};
use raylib_rs::rlgl;
use raylib_rs::rshapes::{draw_circle, draw_line, draw_rectangle};
use raylib_rs::types::{Color, BLUE, GREEN, RED};

fn main() {
    env_logger::init_from_env(env_logger::Env::default().filter_or("MY_LOG_LEVEL", "trace"));
    InitWindow(640, 480, "raylib-rs - MVP Demo");
    unsafe {
        rlgl::rlCheckErrors();
    }
    while !WindowShouldClose() {
        begin_drawing();
        //clear_background(Color::RED);
        draw_rectangle(100, 100, 200, 150, RED);
        draw_circle(400, 225, 50.0, BLUE);
        draw_line(0, 0, 800, 450, GREEN);
        EndDrawing();
    }

    close_window();
}
