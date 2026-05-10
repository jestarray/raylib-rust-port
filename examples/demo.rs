use raylib_rs::core::{init_window, window_should_close, close_window, begin_drawing, end_drawing};
use raylib_rs::rshapes::{draw_rectangle, draw_circle, draw_line};
use raylib_rs::types::{RED, BLUE, GREEN};

fn main() {
    init_window(800, 450, "raylib-rs - MVP Demo");

    while !window_should_close() {
        begin_drawing();

        draw_rectangle(100, 100, 200, 150, RED);
        draw_circle(400, 225, 50.0, BLUE);
        draw_line(0, 0, 800, 450, GREEN);

        end_drawing();
    }

    close_window();
}
