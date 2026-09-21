use raylib::core::{
    begin_drawing, clear_background, close_window, end_drawing, init_window, set_target_fps,
};
use raylib::rcolors::{BLUE, WHITE, YELLOW};
use raylib::sdl::window_should_close;
use raylib::shapes::draw_circle;
use raylib::textures::{draw_texture, load_texture};

fn main() {
    env_logger::init_from_env(env_logger::Env::default().filter_or("MY_LOG_LEVEL", "info"));
    let screen_width = 800;
    let screen_height = 450;

    init_window(
        screen_width,
        screen_height,
        "raylib-rs [textures] example - texture loading and drawing",
    );

    // Load texture from image
    let texture = load_texture("robot.png");

    set_target_fps(60);

    while !window_should_close() {
        begin_drawing();
        clear_background(BLUE);

        draw_circle(20, 20, 30.0, YELLOW);
        draw_texture(
            &texture,
            screen_width / 2 - texture.width / 2,
            screen_height / 2 - texture.height / 2,
            WHITE,
        );

        end_drawing();
    }

    close_window();
}
