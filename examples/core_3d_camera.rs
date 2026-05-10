use raylib_rs::core::*;
use raylib_rs::types::*;
use raylib_rs::rmodels::*;
use raylib_rs::rtext::*;

fn main() {
    init_window(800, 450, "raylib-rust [core] example - 3d camera mode");

    let mut camera = Camera {
        position: Vector3::new(0.0, 10.0, 10.0),
        target: Vector3::new(0.0, 0.0, 0.0),
        up: Vector3::new(0.0, 1.0, 0.0),
        fovy: 45.0,
        projection: CameraProjection::Perspective as i32,
    };

    set_target_fps(60);

    while !window_should_close() {
        // Update
        // ... (camera control could be added here)

        // Draw
        begin_drawing();
            clear_background(Color::RAYWHITE);

            begin_mode_3d(camera);
                draw_cube(Vector3::new(0.0, 0.0, 0.0), 2.0, 2.0, 2.0, Color::RED);
                draw_cube_wires(Vector3::new(0.0, 0.0, 0.0), 2.0, 2.0, 2.0, Color::MAROON);
                draw_grid(10, 1.0);
            end_mode_3d();

            draw_text("Welcome to the third dimension!", 10, 40, 20, Color::DARKGRAY);
            draw_fps(10, 10);
        end_drawing();
    }

    close_window();
}

fn draw_fps(x: i32, y: i32) {
    // Stub or implementation
    draw_text("60 FPS", x, y, 20, Color::LIME);
}
