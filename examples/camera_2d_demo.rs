use raylib_rs::core::*;
use raylib_rs::rshapes::*;
use raylib_rs::rtext::*;
use raylib_rs::types::{Camera2D, Color, Rectangle, Vector2};

const MAX_BUILDINGS: usize = 100;

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    InitWindow(
        screen_width,
        screen_height,
        "raylib-rs [core] example - 2d camera",
    );

    let mut player = Rectangle::new(400.0, 280.0, 40.0, 40.0);
    let mut buildings = [Rectangle::default(); MAX_BUILDINGS];
    let mut build_colors = [Color::default(); MAX_BUILDINGS];

    let mut spacing = 0;

    for i in 0..MAX_BUILDINGS {
        buildings[i].width = get_random_value(50, 200) as f32;
        buildings[i].height = get_random_value(100, 800) as f32;
        buildings[i].y = screen_height as f32 - 130.0 - buildings[i].height;
        buildings[i].x = -6000.0 + spacing as f32;

        spacing += buildings[i].width as i32;

        build_colors[i] = Color::new(
            get_random_value(200, 240) as u8,
            get_random_value(200, 240) as u8,
            get_random_value(200, 250) as u8,
            255,
        );
    }

    let mut camera = Camera2D {
        target: Vector2::new(player.x + 20.0, player.y + 20.0),
        offset: Vector2::new(screen_width as f32 / 2.0, screen_height as f32 / 2.0),
        rotation: 0.0,
        zoom: 1.0,
    };

    set_target_fps(60);

    while !WindowShouldClose() {
        // Update
        if IsKeyDown(KEY_RIGHT) {
            player.x += 2.0;
        } else if IsKeyDown(KEY_LEFT) {
            player.x -= 2.0;
        }

        camera.target = Vector2::new(player.x + 20.0, player.y + 20.0);

        if IsKeyDown(KEY_A) {
            camera.rotation -= 1.0;
        } else if IsKeyDown(KEY_S) {
            camera.rotation += 1.0;
        }

        if camera.rotation > 40.0 {
            camera.rotation = 40.0;
        } else if camera.rotation < -40.0 {
            camera.rotation = -40.0;
        }

        camera.zoom += get_mouse_wheel_move() * 0.05;

        if camera.zoom > 3.0 {
            camera.zoom = 3.0;
        } else if camera.zoom < 0.1 {
            camera.zoom = 0.1;
        }

        if is_key_pressed(KEY_R) {
            camera.zoom = 1.0;
            camera.rotation = 0.0;
        }

        // Draw
        begin_drawing();
        clear_background(Color::RAYWHITE);

        begin_mode_2d(camera);
        draw_rectangle(-6000, 320, 13000, 8000, Color::DARKGRAY);

        for i in 0..MAX_BUILDINGS {
            draw_rectangle_rec(buildings[i], build_colors[i]);
        }

        draw_rectangle_rec(player, Color::RED);

        draw_line(
            camera.target.x as i32,
            -screen_height * 10,
            camera.target.x as i32,
            screen_height * 10,
            Color::GREEN,
        );
        draw_line(
            -screen_width * 10,
            camera.target.y as i32,
            screen_width * 10,
            camera.target.y as i32,
            Color::GREEN,
        );
        end_mode_2d();

        draw_text("SCREEN AREA", 640, 10, 20, Color::RED);

        draw_rectangle(0, 0, screen_width, 5, Color::RED);
        draw_rectangle(0, 5, 5, screen_height - 10, Color::RED);
        draw_rectangle(screen_width - 5, 5, 5, screen_height - 10, Color::RED);
        draw_rectangle(0, screen_height - 5, screen_width, 5, Color::RED);

        draw_rectangle(10, 10, 250, 113, fade(Color::SKYBLUE, 0.5));
        draw_rectangle_lines(10, 10, 250, 113, Color::BLUE);

        draw_text("Free 2D camera controls:", 20, 20, 10, Color::BLACK);
        draw_text("- Right/Left to move player", 40, 40, 10, Color::DARKGRAY);
        draw_text("- Mouse Wheel to Zoom in-out", 40, 60, 10, Color::DARKGRAY);
        draw_text("- A / S to Rotate", 40, 80, 10, Color::DARKGRAY);
        draw_text(
            "- R to reset Zoom and Rotation",
            40,
            100,
            10,
            Color::DARKGRAY,
        );

        EndDrawing();
    }

    close_window();
}
