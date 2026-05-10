use raylib_rs::core::*;
use raylib_rs::rtext::*;
use raylib_rs::types::Color;

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    init_window(screen_width, screen_height, "raylib-rs [text] example - default font");

    set_target_fps(60);

    while !window_should_close() {
        begin_drawing();
        clear_background(Color::RAYWHITE);

        draw_text("Congratulations! You created your first window!", 190, 200, 20, Color::LIGHTGRAY);
        draw_text("This text is drawn with the default font!", 240, 240, 20, Color::DARKGRAY);

        end_drawing();
    }

    close_window();
}
