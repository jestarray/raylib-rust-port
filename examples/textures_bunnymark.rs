#![allow(unused_variables)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(missing_safety_doc, unused_parens, static_mut_refs)]
#![allow(
    clippy::too_many_arguments,
    clippy::needless_return,
    clippy::manual_range_contains,
    clippy::field_reassign_with_default,
    clippy::manual_map,
    clippy::match_like_matches_macro,
    clippy::upper_case_acronyms,
    clippy::let_and_return,
    clippy::double_parens,
)]
use raylib::rcore::*;
use raylib::rcore_desktop_sdl::*;
use raylib::rshapes::*;
use raylib::rtext::DrawText;
use raylib::rtext::*;
use raylib::rtextures::DrawTexture;
use raylib::rtextures::Fade;
use raylib::rtextures::LoadTexture;
use raylib::rtextures::UnloadTexture;
use raylib::rtextures::*;
use raylib::types::ConfigFlags::FLAG_MSAA_4X_HINT;
use raylib::types::KeyboardKey::*;
use raylib::types::{Camera2D, Color, Rectangle, Vector2};
use raylib::types::*;
use raylib::types::GamepadButton::*;
use raylib::types::GamepadAxis::*;
use raylib::types::MouseButton::*;
const MAX_BUNNIES: usize = 80000; // 80K bunnies limit

// This is the maximum amount of elements (quads) per batch
// NOTE: This value is defined in [rlgl] module and can be changed there
const MAX_BATCH_ELEMENTS: i32 = 8192;

//----------------------------------------------------------------------------------
// Types and Structures Definition
//----------------------------------------------------------------------------------
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Bunny {
    pub position: Vector2,
    pub speed: Vector2,
    pub color: Color,
}

//------------------------------------------------------------------------------------
// Program main entry point
//------------------------------------------------------------------------------------

pub fn main() {
    unsafe { start(); }
}
unsafe fn start() {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screenWidth: i32 = 800;
    let screenHeight: i32 = 450;

    InitWindow(screenWidth, screenHeight, "raylib [textures] example - bunnymark");

    // Load bunny texture
    let mut texBunny: Texture2D = LoadTexture("resources/raybunny.png");

    let mut bunnies: Vec<Bunny> = Vec::with_capacity(MAX_BUNNIES); // Bunnies array

    let mut paused: bool = false;

    SetTargetFPS(60);               // Set our game to run at 60 frames-per-second
    //--------------------------------------------------------------------------------------

    // Main game loop
    while !WindowShouldClose() // Detect window close button or ESC key
    {
        // Update
        //----------------------------------------------------------------------------------
        if IsMouseButtonDown(MOUSE_BUTTON_LEFT as i32) {
            // Create more bunnies
            for _ in 0..100 {
                if bunnies.len() < MAX_BUNNIES {
                    bunnies.push(Bunny {
                        position: GetMousePosition(),
                        speed: Vector2 {
                            x: GetRandomValue(-250, 250) as f32,
                            y: GetRandomValue(-250, 250) as f32,
                        },
                        color: Color {
                            r: GetRandomValue(50, 240) as u8,
                            g: GetRandomValue(80, 240) as u8,
                            b: GetRandomValue(100, 240) as u8,
                            a: 255,
                        },
                    });
                }
            }
        }

        if IsKeyPressed(KEY_P) {
            paused = !paused;
        }

        if !paused {
            // Update bunnies
            for bunny in &mut bunnies {
                bunny.position.x += bunny.speed.x * GetFrameTime();
                bunny.position.y += bunny.speed.y * GetFrameTime();

                if ((bunny.position.x + texBunny.width as f32 / 2.0) > GetScreenWidth() as f32) ||
                   ((bunny.position.x + texBunny.width as f32 / 2.0) < 0.0) {
                    bunny.speed.x *= -1.0;
                }
                if ((bunny.position.y + texBunny.height as f32 / 2.0) > GetScreenHeight() as f32) ||
                   ((bunny.position.y + texBunny.height as f32 / 2.0 - 40.0) < 0.0) {
                    bunny.speed.y *= -1.0;
                }
            }
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        BeginDrawing();

            ClearBackground(Color::RAYWHITE);

            for bunny in &bunnies {
                // NOTE: When internal batch buffer limit is reached (MAX_BATCH_ELEMENTS),
                // a draw call is launched and buffer starts being filled again;
                // before issuing a draw call, updated vertex data from internal CPU buffer is send to GPU...
                // Process of sending data is costly and it could happen that GPU data has not been completely
                // processed for drawing while new data is tried to be sent (updating current in-use buffers)
                // it could generates a stall and consequently a frame drop, limiting the number of drawn bunnies
                DrawTexture(&texBunny, bunny.position.x as i32, bunny.position.y as i32, bunny.color);
            }

            DrawRectangle(0, 0, screenWidth, 40, Color::BLACK);
            DrawText(&format!("bunnies: {}", bunnies.len() as i32), 120, 10, 20, Color::GREEN);
            DrawText(&format!("batched draw calls: {}", 1 + bunnies.len() as i32 / MAX_BATCH_ELEMENTS), 320, 10, 20, Color::MAROON);

            DrawFPS(10, 10);

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    UnloadTexture(&mut texBunny); // Unload bunny texture

    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------
}