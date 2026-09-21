use raylib::core::{
    begin_drawing, begin_mode_3d, clear_background, close_window, end_drawing, end_mode_3d,
    init_window, set_target_fps,
};
use raylib::rcolors::{MAROON, RAYWHITE, RED};
use raylib::rmodels::{draw_cube, draw_cube_wires, draw_grid};
use raylib::sdl::window_should_close;
use raylib::text::draw_fps;
use raylib::types::{Camera, CameraProjection, Vector3};

fn main() {
    env_logger::init_from_env(env_logger::Env::default().filter_or("MY_LOG_LEVEL", "info"));

    init_window(800, 450, "raylib-rust [core] example - 3d camera mode");

    let camera = Camera {
        position: Vector3::new(0.0, 10.0, 10.0),
        target: Vector3::new(0.0, 0.0, 0.0),
        up: Vector3::new(0.0, 1.0, 0.0),
        fovy: 45.0,
        projection: CameraProjection::CAMERA_PERSPECTIVE as i32,
    };

    set_target_fps(60);

    while !window_should_close() {
        // Update
        // ... (camera control could be added here)

        // Draw
        begin_drawing();
        clear_background(RAYWHITE);

        begin_mode_3d(camera);
        draw_cube(Vector3::new(0.0, 0.0, 0.0), 2.0, 2.0, 2.0, RED);
        draw_cube_wires(Vector3::new(0.0, 0.0, 0.0), 2.0, 2.0, 2.0, MAROON);
        draw_grid(10, 1.0);
        end_mode_3d();

        draw_fps(10, 10);
        end_drawing();
    }

    close_window();
}
