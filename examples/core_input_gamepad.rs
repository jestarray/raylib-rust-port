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
const XBOX_ALIAS_1: &str = "xbox";
const XBOX_ALIAS_2: &str = "x-box";
const PS_ALIAS_1: &str = "playstation";
const PS_ALIAS_2: &str = "sony";

fn main() {
    unsafe { start(); }
}
unsafe fn start() {
    // Initialization
    let screenWidth: i32 = 800;
    let screenHeight: i32 = 450;

    SetConfigFlags(FLAG_MSAA_4X_HINT as u32);

    InitWindow(screenWidth, screenHeight, "raylib [core] example - input gamepad");

    let mut texPs3Pad = LoadTexture("resources/ps3.png");
    let mut texXboxPad = LoadTexture("resources/xbox.png");

    // Set axis deadzones
    let leftStickDeadzoneX: f32 = 0.1;
    let leftStickDeadzoneY: f32 = 0.1;
    let rightStickDeadzoneX: f32 = 0.1;
    let rightStickDeadzoneY: f32 = 0.1;
    let leftTriggerDeadzone: f32 = -0.9;
    let rightTriggerDeadzone: f32 = -0.9;

    let mut vibrateButton = Rectangle { x: 0.0, y: 0.0, width: 0.0, height: 0.0 };

    SetTargetFPS(60);

    let mut gamepad: i32 = 0;

    // Main game loop
    while !WindowShouldClose() {
        // Update
        if IsKeyPressed(KEY_LEFT) && gamepad > 0 {
            gamepad -= 1;
        }
        if IsKeyPressed(KEY_RIGHT) {
            gamepad += 1;
        }
        let mousePosition = GetMousePosition();

        vibrateButton = Rectangle {
            x: 10.0,
            y: 70.0 + 20.0 * GetGamepadAxisCount(gamepad) as f32 + 20.0,
            width: 75.0,
            height: 24.0,
        };
        if IsMouseButtonPressed(MOUSE_BUTTON_LEFT) && CheckCollisionPointRec(mousePosition, vibrateButton) {
            SetGamepadVibration(gamepad, 1.0, 1.0, 1.0);
        }

        // Draw
        BeginDrawing();

        ClearBackground(RAYWHITE);

        if IsGamepadAvailable(gamepad) {
            DrawText(&format!("GP{}: {}", gamepad, GetGamepadName(gamepad).unwrap_or_default()), 10, 10, 10, BLACK);

            // Get axis values
            let mut leftStickX = GetGamepadAxisMovement(gamepad, GAMEPAD_AXIS_LEFT_X);
            let mut leftStickY = GetGamepadAxisMovement(gamepad, GAMEPAD_AXIS_LEFT_Y);
            let mut rightStickX = GetGamepadAxisMovement(gamepad, GAMEPAD_AXIS_RIGHT_X);
            let mut rightStickY = GetGamepadAxisMovement(gamepad, GAMEPAD_AXIS_RIGHT_Y);
            let mut leftTrigger = GetGamepadAxisMovement(gamepad, GAMEPAD_AXIS_LEFT_TRIGGER);
            let mut rightTrigger = GetGamepadAxisMovement(gamepad, GAMEPAD_AXIS_RIGHT_TRIGGER);

            // Calculate deadzones
            if leftStickX > -leftStickDeadzoneX && leftStickX < leftStickDeadzoneX {
                leftStickX = 0.0;
            }
            if leftStickY > -leftStickDeadzoneY && leftStickY < leftStickDeadzoneY {
                leftStickY = 0.0;
            }
            if rightStickX > -rightStickDeadzoneX && rightStickX < rightStickDeadzoneX {
                rightStickX = 0.0;
            }
            if rightStickY > -rightStickDeadzoneY && rightStickY < rightStickDeadzoneY {
                rightStickY = 0.0;
            }
            if leftTrigger < leftTriggerDeadzone {
                leftTrigger = -1.0;
            }
            if rightTrigger < rightTriggerDeadzone {
                rightTrigger = -1.0;
            }
            let gamepadname = GetGamepadName(gamepad).unwrap_or_default().to_ascii_lowercase();
            if (((gamepadname).contains( XBOX_ALIAS_1)))
                || (((gamepadname).contains( XBOX_ALIAS_2)))
            {
                DrawTexture(&texXboxPad, 0, 0, DARKGRAY);

                // Draw buttons: xbox home
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_MIDDLE) {
                    DrawCircle(394, 89, 19.0, RED);
                }

                // Draw buttons: basic
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_MIDDLE_RIGHT) {
                    DrawCircle(436, 150, 9.0, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_MIDDLE_LEFT) {
                    DrawCircle(352, 150, 9.0, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_LEFT) {
                    DrawCircle(501, 151, 15.0, BLUE);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_DOWN) {
                    DrawCircle(536, 187, 15.0, LIME);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_RIGHT) {
                    DrawCircle(572, 151, 15.0, MAROON);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_UP) {
                    DrawCircle(536, 115, 15.0, GOLD);
                }

                // Draw buttons: d-pad
                DrawRectangle(317, 202, 19, 71, BLACK);
                DrawRectangle(293, 228, 69, 19, BLACK);
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_FACE_UP) {
                    DrawRectangle(317, 202, 19, 26, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_FACE_DOWN) {
                    DrawRectangle(317, 202 + 45, 19, 26, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_FACE_LEFT) {
                    DrawRectangle(292, 228, 25, 19, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_FACE_RIGHT) {
                    DrawRectangle(292 + 44, 228, 26, 19, RED);
                }

                // Draw buttons: left-right back
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_TRIGGER_1) {
                    DrawCircle(259, 61, 20.0, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_TRIGGER_1) {
                    DrawCircle(536, 61, 20.0, RED);
                }

                // Draw axis: left joystick
                let mut leftGamepadColor = BLACK;
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_THUMB) {
                    leftGamepadColor = RED;
                }
                DrawCircle(259, 152, 39.0, BLACK);
                DrawCircle(259, 152, 34.0, LIGHTGRAY);
                DrawCircle(259 + (leftStickX * 20.0) as i32, 152 + (leftStickY * 20.0) as i32, 25.0, leftGamepadColor);

                // Draw axis: right joystick
                let mut rightGamepadColor = BLACK;
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_THUMB) {
                    rightGamepadColor = RED;
                }
                DrawCircle(461, 237, 38.0, BLACK);
                DrawCircle(461, 237, 33.0, LIGHTGRAY);
                DrawCircle(461 + (rightStickX * 20.0) as i32, 237 + (rightStickY * 20.0) as i32, 25.0, rightGamepadColor);

                // Draw axis: left-right triggers
                DrawRectangle(170, 30, 15, 70, GRAY);
                DrawRectangle(604, 30, 15, 70, GRAY);
                DrawRectangle(170, 30, 15, (((1.0 + leftTrigger) / 2.0) * 70.0) as i32, RED);
                DrawRectangle(604, 30, 15, (((1.0 + rightTrigger) / 2.0) * 70.0) as i32, RED);
            } else if (((gamepadname).contains( PS_ALIAS_1)))
                || (((gamepadname).contains( PS_ALIAS_2)))
            {
                DrawTexture(&texPs3Pad, 0, 0, DARKGRAY);

                // Draw buttons: ps
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_MIDDLE) {
                    DrawCircle(396, 222, 13.0, RED);
                }

                // Draw buttons: basic
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_MIDDLE_LEFT) {
                    DrawRectangle(328, 170, 32, 13, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_MIDDLE_RIGHT) {
                    DrawTriangle(Vector2 { x: 436.0, y: 168.0 }, Vector2 { x: 436.0, y: 185.0 }, Vector2 { x: 464.0, y: 177.0 }, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_UP) {
                    DrawCircle(557, 144, 13.0, LIME);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_RIGHT) {
                    DrawCircle(586, 173, 13.0, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_DOWN) {
                    DrawCircle(557, 203, 13.0, VIOLET);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_LEFT) {
                    DrawCircle(527, 173, 13.0, PINK);
                }

                // Draw buttons: d-pad
                DrawRectangle(225, 132, 24, 84, BLACK);
                DrawRectangle(195, 161, 84, 25, BLACK);
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_FACE_UP) {
                    DrawRectangle(225, 132, 24, 29, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_FACE_DOWN) {
                    DrawRectangle(225, 132 + 54, 24, 30, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_FACE_LEFT) {
                    DrawRectangle(195, 161, 30, 25, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_FACE_RIGHT) {
                    DrawRectangle(195 + 54, 161, 30, 25, RED);
                }

                // Draw buttons: left-right back buttons
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_TRIGGER_1) {
                    DrawCircle(239, 82, 20.0, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_TRIGGER_1) {
                    DrawCircle(557, 82, 20.0, RED);
                }

                // Draw axis: left joystick
                let mut leftGamepadColor = BLACK;
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_THUMB) {
                    leftGamepadColor = RED;
                }
                DrawCircle(319, 255, 35.0, BLACK);
                DrawCircle(319, 255, 31.0, LIGHTGRAY);
                DrawCircle(319 + (leftStickX * 20.0) as i32, 255 + (leftStickY * 20.0) as i32, 25.0, leftGamepadColor);

                // Draw axis: right joystick
                let mut rightGamepadColor = BLACK;
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_THUMB) {
                    rightGamepadColor = RED;
                }
                DrawCircle(475, 255, 35.0, BLACK);
                DrawCircle(475, 255, 31.0, LIGHTGRAY);
                DrawCircle(475 + (rightStickX * 20.0) as i32, 255 + (rightStickY * 20.0) as i32, 25.0, rightGamepadColor);

                // Draw axis: left-right triggers
                DrawRectangle(169, 48, 15, 70, GRAY);
                DrawRectangle(611, 48, 15, 70, GRAY);
                DrawRectangle(169, 48, 15, (((1.0 + leftTrigger) / 2.0) * 70.0) as i32, RED);
                DrawRectangle(611, 48, 15, (((1.0 + rightTrigger) / 2.0) * 70.0) as i32, RED);
            } else {
                // Draw background: generic
                DrawRectangleRounded(Rectangle { x: 175.0, y: 110.0, width: 460.0, height: 220.0 }, 0.3, 16, DARKGRAY);

                // Draw buttons: basic
                DrawCircle(365, 170, 12.0, RAYWHITE);
                DrawCircle(405, 170, 12.0, RAYWHITE);
                DrawCircle(445, 170, 12.0, RAYWHITE);
                DrawCircle(516, 191, 17.0, RAYWHITE);
                DrawCircle(551, 227, 17.0, RAYWHITE);
                DrawCircle(587, 191, 17.0, RAYWHITE);
                DrawCircle(551, 155, 17.0, RAYWHITE);
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_MIDDLE_LEFT) {
                    DrawCircle(365, 170, 10.0, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_MIDDLE) {
                    DrawCircle(405, 170, 10.0, GREEN);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_MIDDLE_RIGHT) {
                    DrawCircle(445, 170, 10.0, BLUE);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_LEFT) {
                    DrawCircle(516, 191, 15.0, GOLD);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_DOWN) {
                    DrawCircle(551, 227, 15.0, BLUE);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_RIGHT) {
                    DrawCircle(587, 191, 15.0, GREEN);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_FACE_UP) {
                    DrawCircle(551, 155, 15.0, RED);
                }

                // Draw buttons: d-pad
                DrawRectangle(245, 145, 28, 88, RAYWHITE);
                DrawRectangle(215, 174, 88, 29, RAYWHITE);
                DrawRectangle(247, 147, 24, 84, BLACK);
                DrawRectangle(217, 176, 84, 25, BLACK);
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_FACE_UP) {
                    DrawRectangle(247, 147, 24, 29, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_FACE_DOWN) {
                    DrawRectangle(247, 147 + 54, 24, 30, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_FACE_LEFT) {
                    DrawRectangle(217, 176, 30, 25, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_FACE_RIGHT) {
                    DrawRectangle(217 + 54, 176, 30, 25, RED);
                }

                // Draw buttons: left-right back
                DrawRectangleRounded(Rectangle { x: 215.0, y: 98.0, width: 100.0, height: 10.0 }, 0.5, 16, DARKGRAY);
                DrawRectangleRounded(Rectangle { x: 495.0, y: 98.0, width: 100.0, height: 10.0 }, 0.5, 16, DARKGRAY);
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_TRIGGER_1) {
                    DrawRectangleRounded(Rectangle { x: 215.0, y: 98.0, width: 100.0, height: 10.0 }, 0.5, 16, RED);
                }
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_TRIGGER_1) {
                    DrawRectangleRounded(Rectangle { x: 495.0, y: 98.0, width: 100.0, height: 10.0 }, 0.5, 16, RED);
                }

                // Draw axis: left joystick
                let mut leftGamepadColor = BLACK;
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_LEFT_THUMB) {
                    leftGamepadColor = RED;
                }
                DrawCircle(345, 260, 40.0, BLACK);
                DrawCircle(345, 260, 35.0, LIGHTGRAY);
                DrawCircle(345 + (leftStickX * 20.0) as i32, 260 + (leftStickY * 20.0) as i32, 25.0, leftGamepadColor);

                // Draw axis: right joystick
                let mut rightGamepadColor = BLACK;
                if IsGamepadButtonDown(gamepad, GAMEPAD_BUTTON_RIGHT_THUMB) {
                    rightGamepadColor = RED;
                }
                DrawCircle(465, 260, 40.0, BLACK);
                DrawCircle(465, 260, 35.0, LIGHTGRAY);
                DrawCircle(465 + (rightStickX * 20.0) as i32, 260 + (rightStickY * 20.0) as i32, 25.0, rightGamepadColor);

                // Draw axis: left-right triggers
                DrawRectangle(151, 110, 15, 70, GRAY);
                DrawRectangle(644, 110, 15, 70, GRAY);
                DrawRectangle(151, 110, 15, (((1.0 + leftTrigger) / 2.0) * 70.0) as i32, RED);
                DrawRectangle(644, 110, 15, (((1.0 + rightTrigger) / 2.0) * 70.0) as i32, RED);
            }

            DrawText(&format!("DETECTED AXIS [{}]:", GetGamepadAxisCount(gamepad)), 10, 50, 10, MAROON);

            for i in 0..GetGamepadAxisCount(gamepad) {
                //DrawText(&format!("AXIS {}: {}", i, GetGamepadAxisMovement(gamepad, i)), 20, 70 + 20 * i, 10, DARKGRAY);
            }

            // Draw vibrate button
            DrawRectangleRec(vibrateButton, SKYBLUE);
            DrawText("VIBRATE", (vibrateButton.x + 14.0) as i32, (vibrateButton.y + 1.0) as i32, 10, DARKGRAY);

            if GetGamepadButtonPressed() != GAMEPAD_BUTTON_UNKNOWN as i32 {
                DrawText(&format!("DETECTED BUTTON: {}", GetGamepadButtonPressed()), 10, 430, 10, RED);
            } else {
                DrawText("DETECTED BUTTON: NONE", 10, 430, 10, GRAY);
            }
        } else {
            DrawText(&format!("GP{}: NOT DETECTED", gamepad), 10, 10, 10, GRAY);
            DrawTexture(&texXboxPad, 0, 0, LIGHTGRAY);
        }

        EndDrawing();
    }

    // De-Initialization
    UnloadTexture(&mut texPs3Pad);
    UnloadTexture(&mut texXboxPad);

    CloseWindow();
}