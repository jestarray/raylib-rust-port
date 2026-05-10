use raylib_rs::core::*;
use raylib_rs::rtextures::*;
use raylib_rs::rtext::*;
use raylib_rs::types::Color;

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    init_window(screen_width, screen_height, "raylib-rs [textures] example - texture loading and drawing");

    // Load texture from image
    let texture = load_texture("robot.png");

    set_target_fps(60);

    while !window_should_close() {
        begin_drawing();
        clear_background(Color::RAYWHITE);

        // Draw texture
        draw_texture(&texture, screen_width / 2 - texture.width / 2, screen_height / 2 - texture.height / 2, Color::WHITE);

        draw_text("this is a texture!", 300, 380, 20, Color::GRAY);

        end_drawing();
    }

    close_window();
}
