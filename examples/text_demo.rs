use raylib::core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window, set_target_fps,
};
use raylib::rcolors::{DARKGRAY, LIGHTGRAY, RAYWHITE};
use raylib::sdl::window_should_close;
use raylib::text::draw_text;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().filter_or("MY_LOG_LEVEL", "info"));
    let screen_width = 800;
    let screen_height = 450;

    init_window(
        screen_width,
        screen_height,
        "raylib-rs [text] example - default font",
    );

    set_target_fps(60);

    while !window_should_close() {
        begin_drawing();
        clear_background(RAYWHITE);

        draw_text(
            "Congratulations! You created your first window!",
            190,
            200,
            20,
            LIGHTGRAY,
        );
        draw_text(
            "This text is drawn with the default font!",
            240,
            240,
            20,
            DARKGRAY,
        );

        end_drawing();
    }

    close_window();
}
