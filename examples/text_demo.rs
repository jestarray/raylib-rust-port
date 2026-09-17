use raylib::rcore::{
    BeginDrawing, ClearBackground, CloseWindow, EndDrawing, InitWindow, SetTargetFPS,
};
use raylib::rcore_desktop_sdl::*;
use raylib::rtext::*;
use raylib::types::Color;

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    unsafe {
        InitWindow(
            screen_width,
            screen_height,
            "raylib-rs [text] example - default font",
        );

        SetTargetFPS(60);

        while !WindowShouldClose() {
            BeginDrawing();
            ClearBackground(Color::RAYWHITE);

            DrawText(
                "Congratulations! You created your first window!",
                190,
                200,
                20,
                Color::LIGHTGRAY,
            );
            DrawText(
                "This text is drawn with the default font!",
                240,
                240,
                20,
                Color::DARKGRAY,
            );

            EndDrawing();
        }

        CloseWindow();
    }
}
