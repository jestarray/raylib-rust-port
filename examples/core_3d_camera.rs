use raylib_rs::rcore::*;
use raylib_rs::rmodels::*;
use raylib_rs::rshapes::*;
use raylib_rs::rtext::*;
use raylib_rs::types::*;

fn main() {
    unsafe {
        InitWindow(800, 450, "raylib-rust [core] example - 3d camera mode");

        let mut camera = Camera {
            position: Vector3::new(0.0, 10.0, 10.0),
            target: Vector3::new(0.0, 0.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            fovy: 45.0,
            projection: CameraProjection::Perspective as i32,
        };

        SetTargetFPS(60);

        while !WindowShouldClose() {
            // Update
            // ... (camera control could be added here)

            // Draw
            BeginDrawing();
            ClearBackground(Color::RAYWHITE);

            BeginMode3D(camera);
            DrawCube(Vector3::new(0.0, 0.0, 0.0), 2.0, 2.0, 2.0, Color::RED);
            DrawCubeWires(Vector3::new(0.0, 0.0, 0.0), 2.0, 2.0, 2.0, Color::MAROON);
            DrawGrid(10, 1.0);
            EndMode3D();

            //draw_text(
            //    "Welcome to the third dimension!",
            //    10,
            //    40,
            //    20,
            //    Color::DARKGRAY,
            //);
            draw_fps(10, 10);
            EndDrawing();
        }

        CloseWindow();
    }
}

fn draw_fps(x: i32, y: i32) {
    // Stub or implementation
    //draw_text("60 FPS", x, y, 20, Color::LIME);
}
