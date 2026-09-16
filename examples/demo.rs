use raylib_rs::rcore::{BeginDrawing, ClearBackground, CloseWindow, EndDrawing, InitWindow};
use raylib_rs::rcore_desktop_sdl::WindowShouldClose;
use raylib_rs::rlgl;
use raylib_rs::rshapes::*;
use raylib_rs::types::{BLUE, Color, GREEN, RED};

fn main() {
    unsafe {
        env_logger::init_from_env(env_logger::Env::default().filter_or("MY_LOG_LEVEL", "trace"));
        InitWindow(640, 480, "raylib-rs - MVP Demo");
        rlgl::rlCheckErrors();
        while !WindowShouldClose() {
            BeginDrawing();
            ClearBackground(Color::RED);
            DrawRectangle(100, 100, 200, 150, RED);
            DrawCircle(400, 225, 50.0, BLUE);
            DrawLine(0, 0, 800, 450, GREEN);
            EndDrawing();
        }

        CloseWindow();
    }
}
