use raylib_rs::rcore::*;
use raylib_rs::rcore_desktop_sdl::WindowShouldClose;
use raylib_rs::rshapes::DrawCircle;
use raylib_rs::rshapes::DrawRectangle;
use raylib_rs::rtext::*;
use raylib_rs::rtextures::*;
use raylib_rs::types::Color;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().filter_or("MY_LOG_LEVEL", "info"));
    let screen_width = 800;
    let screen_height = 450;

    unsafe {
        InitWindow(
            screen_width,
            screen_height,
            "raylib-rs [textures] example - texture loading and drawing",
        );

        // Load texture from image
        let texture = load_texture("robot.png");

        SetTargetFPS(60);

        while !WindowShouldClose() {
            BeginDrawing();
            ClearBackground(Color::BLUE);

            // Draw texture
            unsafe {
                //DrawRectangle(0, 0, 100, 100, Color::RED);
                DrawCircle(20, 20, 30.0, Color::YELLOW);
            }
            draw_texture(
                &texture,
                screen_width / 2 - texture.width / 2,
                screen_height / 2 - texture.height / 2,
                Color::WHITE,
            );

            //DrawText("this is a texture!", 300, 380, 20, Color::GRAY);

            EndDrawing();
        }

        CloseWindow();
    }
}
