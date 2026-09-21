use raylib::core::{
    begin_drawing, clear_background, close_window, end_drawing, get_frame_time, get_mouse_position,
    get_random_value, get_screen_height, get_screen_width, get_touch_point_count, init_window,
    is_key_pressed, is_mouse_button_down, set_target_fps,
};
use raylib::rcolors::{BLACK, GREEN, MAROON, RAYWHITE};
use raylib::sdl::window_should_close;
use raylib::shapes::draw_rectangle;
use raylib::text::{draw_fps, draw_text};
use raylib::textures::{draw_texture, load_texture, unload_texture};
use raylib::types::{Color, KeyboardKey, MouseButton, Texture2D, Vector2};

// This is the maximum amount of elements (quads) per batch
// NOTE: This value is defined in [rlgl] module and can be changed there
const MAX_BATCH_ELEMENTS: i32 = 8192;

#[derive(Clone, Copy, Default)]
struct Bunny {
    position: Vector2,
    speed: Vector2,
    color: Color,
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default().filter_or("MY_LOG_LEVEL", "info"));

    // Initialization
    let screen_width: i32 = 800;
    let screen_height: i32 = 450;

    init_window(
        screen_width,
        screen_height,
        "raylib [textures] example - bunnymark",
    );

    // Load bunny texture (support both local desktop path and Android APK assets)
    let mut tex_bunny: Texture2D = load_texture("resources/raybunny.png");

    let mut bunnies: Vec<Bunny> = Vec::new(); // Bunnies array
    let mut paused = false;

    set_target_fps(60); // Set our game to run at 60 frames-per-second

    // Main game loop
    while !window_should_close() {
        // Update
        let is_touching = get_touch_point_count() > 0;
        if is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) || is_touching {
            // Create more bunnies
            for _ in 0..100 {
                bunnies.push(Bunny {
                    position: get_mouse_position(),
                    speed: Vector2 {
                        x: get_random_value(-250, 250) as f32,
                        y: get_random_value(-250, 250) as f32,
                    },
                    color: Color {
                        r: get_random_value(50, 240) as u8,
                        g: get_random_value(80, 240) as u8,
                        b: get_random_value(100, 240) as u8,
                        a: 255,
                    },
                });
            }
        }

        if is_key_pressed(KeyboardKey::KEY_P) {
            paused = !paused;
        }

        if !paused {
            // Update bunnies
            for bunny in &mut bunnies {
                bunny.position.x += bunny.speed.x * get_frame_time();
                bunny.position.y += bunny.speed.y * get_frame_time();

                if (bunny.position.x + tex_bunny.width as f32 / 2.0) > get_screen_width() as f32
                    || (bunny.position.x + tex_bunny.width as f32 / 2.0) < 0.0
                {
                    bunny.speed.x *= -1.0;
                }
                if (bunny.position.y + tex_bunny.height as f32 / 2.0) > get_screen_height() as f32
                    || (bunny.position.y + tex_bunny.height as f32 / 2.0 - 40.0) < 0.0
                {
                    bunny.speed.y *= -1.0;
                }
            }
        }

        // Draw
        // NOTE: When the internal batch buffer limit is reached (MAX_BATCH_ELEMENTS) a draw call is
        // launched and the buffer starts filling again; pushing past it costs a stall per frame.
        begin_drawing();

        clear_background(RAYWHITE);

        for bunny in &bunnies {
            draw_texture(
                &tex_bunny,
                bunny.position.x as i32,
                bunny.position.y as i32,
                bunny.color,
            );
        }

        draw_rectangle(0, 0, get_screen_width(), 40, BLACK);
        draw_text(
            &format!("bunnies: {}", bunnies.len() as i32),
            120,
            10,
            20,
            GREEN,
        );
        draw_text(
            &format!(
                "batched draw calls!: {}",
                1 + bunnies.len() as i32 / MAX_BATCH_ELEMENTS
            ),
            320,
            10,
            20,
            MAROON,
        );

        draw_fps(10, 10);

        end_drawing();
    }

    // De-Initialization
    unload_texture(&mut tex_bunny); // Unload bunny texture

    close_window(); // Close window and OpenGL context
}
