use raylib::core::{
    begin_drawing, clear_background, close_window, end_drawing, get_gamepad_axis_count,
    get_gamepad_axis_movement, get_gamepad_button_pressed, get_gamepad_name, get_mouse_position,
    init_window, is_gamepad_available, is_gamepad_button_down, is_key_pressed,
    is_mouse_button_pressed, set_config_flags, set_target_fps,
};
use raylib::rcolors::{
    BLACK, BLUE, DARKGRAY, GOLD, GRAY, GREEN, LIGHTGRAY, LIME, MAROON, PINK, RAYWHITE, RED,
    SKYBLUE, VIOLET,
};
use raylib::sdl::{set_gamepad_vibration, window_should_close};
use raylib::shapes::{
    check_collision_point_rec, draw_circle, draw_rectangle, draw_rectangle_rec,
    draw_rectangle_rounded, draw_triangle,
};
use raylib::text::draw_text;
use raylib::textures::{draw_texture, load_texture};
use raylib::types::{
    ConfigFlags, GamepadAxis, GamepadButton, KeyboardKey, MouseButton, Rectangle, Vector2,
};

const XBOX_ALIAS_1: &str = "xbox";
const XBOX_ALIAS_2: &str = "x-box";
const PS_ALIAS_1: &str = "playstation";
const PS_ALIAS_2: &str = "sony";

fn main() {
    env_logger::init_from_env(env_logger::Env::default().filter_or("MY_LOG_LEVEL", "info"));

    // Initialization
    let screen_width: i32 = 800;
    let screen_height: i32 = 450;

    set_config_flags(ConfigFlags::FLAG_MSAA_4X_HINT as u32);

    init_window(
        screen_width,
        screen_height,
        "raylib [core] example - input gamepad",
    );

    let mut tex_ps3_pad = load_texture("resources/ps3.png");
    let mut tex_xbox_pad = load_texture("resources/xbox.png");

    // Set axis deadzones
    let left_stick_deadzone_x: f32 = 0.1;
    let left_stick_deadzone_y: f32 = 0.1;
    let right_stick_deadzone_x: f32 = 0.1;
    let right_stick_deadzone_y: f32 = 0.1;
    let left_trigger_deadzone: f32 = -0.9;
    let right_trigger_deadzone: f32 = -0.9;

    set_target_fps(60);

    let mut gamepad: i32 = 0;

    // Main game loop
    while !window_should_close() {
        // Update
        if is_key_pressed(KeyboardKey::KEY_LEFT) && gamepad > 0 {
            gamepad -= 1;
        }
        if is_key_pressed(KeyboardKey::KEY_RIGHT) {
            gamepad += 1;
        }
        let mouse_position = get_mouse_position();

        let vibrate_button = Rectangle {
            x: 10.0,
            y: 70.0 + 20.0 * get_gamepad_axis_count(gamepad) as f32 + 20.0,
            width: 75.0,
            height: 24.0,
        };
        if is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
            && check_collision_point_rec(mouse_position, vibrate_button)
        {
            set_gamepad_vibration(gamepad, 1.0, 1.0, 1.0);
        }

        // Draw
        begin_drawing();

        clear_background(RAYWHITE);

        if is_gamepad_available(gamepad) {
            draw_text(
                &format!(
                    "GP{}: {}",
                    gamepad,
                    get_gamepad_name(gamepad).unwrap_or_default()
                ),
                10,
                10,
                10,
                BLACK,
            );

            // Get axis values
            let mut left_stick_x =
                get_gamepad_axis_movement(gamepad, GamepadAxis::GAMEPAD_AXIS_LEFT_X);
            let mut left_stick_y =
                get_gamepad_axis_movement(gamepad, GamepadAxis::GAMEPAD_AXIS_LEFT_Y);
            let mut right_stick_x =
                get_gamepad_axis_movement(gamepad, GamepadAxis::GAMEPAD_AXIS_RIGHT_X);
            let mut right_stick_y =
                get_gamepad_axis_movement(gamepad, GamepadAxis::GAMEPAD_AXIS_RIGHT_Y);
            let mut left_trigger =
                get_gamepad_axis_movement(gamepad, GamepadAxis::GAMEPAD_AXIS_LEFT_TRIGGER);
            let mut right_trigger =
                get_gamepad_axis_movement(gamepad, GamepadAxis::GAMEPAD_AXIS_RIGHT_TRIGGER);

            // Calculate deadzones
            if left_stick_x > -left_stick_deadzone_x && left_stick_x < left_stick_deadzone_x {
                left_stick_x = 0.0;
            }
            if left_stick_y > -left_stick_deadzone_y && left_stick_y < left_stick_deadzone_y {
                left_stick_y = 0.0;
            }
            if right_stick_x > -right_stick_deadzone_x && right_stick_x < right_stick_deadzone_x {
                right_stick_x = 0.0;
            }
            if right_stick_y > -right_stick_deadzone_y && right_stick_y < right_stick_deadzone_y {
                right_stick_y = 0.0;
            }
            if left_trigger < left_trigger_deadzone {
                left_trigger = -1.0;
            }
            if right_trigger < right_trigger_deadzone {
                right_trigger = -1.0;
            }

            let gamepad_name = get_gamepad_name(gamepad)
                .unwrap_or_default()
                .to_ascii_lowercase();

            if gamepad_name.contains(XBOX_ALIAS_1) || gamepad_name.contains(XBOX_ALIAS_2) {
                draw_texture(&tex_xbox_pad, 0, 0, DARKGRAY);

                // Draw buttons: xbox home
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_MIDDLE) {
                    draw_circle(394, 89, 19.0, RED);
                }

                // Draw buttons: basic
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT) {
                    draw_circle(436, 150, 9.0, RED);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT) {
                    draw_circle(352, 150, 9.0, RED);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT) {
                    draw_circle(501, 151, 15.0, BLUE);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN) {
                    draw_circle(536, 187, 15.0, LIME);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT) {
                    draw_circle(572, 151, 15.0, MAROON);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP) {
                    draw_circle(536, 115, 15.0, GOLD);
                }

                // Draw buttons: d-pad
                draw_rectangle(317, 202, 19, 71, BLACK);
                draw_rectangle(293, 228, 69, 19, BLACK);
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP) {
                    draw_rectangle(317, 202, 19, 26, RED);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN) {
                    draw_rectangle(317, 202 + 45, 19, 26, RED);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT) {
                    draw_rectangle(292, 228, 25, 19, RED);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT) {
                    draw_rectangle(292 + 44, 228, 26, 19, RED);
                }

                // Draw buttons: left-right back
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1) {
                    draw_circle(259, 61, 20.0, RED);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1) {
                    draw_circle(536, 61, 20.0, RED);
                }

                // Draw axis: left joystick
                let mut left_gamepad_color = BLACK;
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_THUMB) {
                    left_gamepad_color = RED;
                }
                draw_circle(259, 152, 39.0, BLACK);
                draw_circle(259, 152, 34.0, LIGHTGRAY);
                draw_circle(
                    259 + (left_stick_x * 20.0) as i32,
                    152 + (left_stick_y * 20.0) as i32,
                    25.0,
                    left_gamepad_color,
                );

                // Draw axis: right joystick
                let mut right_gamepad_color = BLACK;
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_THUMB) {
                    right_gamepad_color = RED;
                }
                draw_circle(461, 237, 38.0, BLACK);
                draw_circle(461, 237, 33.0, LIGHTGRAY);
                draw_circle(
                    461 + (right_stick_x * 20.0) as i32,
                    237 + (right_stick_y * 20.0) as i32,
                    25.0,
                    right_gamepad_color,
                );

                // Draw axis: left-right triggers
                draw_rectangle(170, 30, 15, 70, GRAY);
                draw_rectangle(604, 30, 15, 70, GRAY);
                draw_rectangle(
                    170,
                    30,
                    15,
                    (((1.0 + left_trigger) / 2.0) * 70.0) as i32,
                    RED,
                );
                draw_rectangle(
                    604,
                    30,
                    15,
                    (((1.0 + right_trigger) / 2.0) * 70.0) as i32,
                    RED,
                );
            } else if gamepad_name.contains(PS_ALIAS_1) || gamepad_name.contains(PS_ALIAS_2) {
                draw_texture(&tex_ps3_pad, 0, 0, DARKGRAY);

                // Draw buttons: ps
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_MIDDLE) {
                    draw_circle(396, 222, 13.0, RED);
                }

                // Draw buttons: basic
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT) {
                    draw_rectangle(328, 170, 32, 13, RED);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT) {
                    draw_triangle(
                        Vector2 { x: 436.0, y: 168.0 },
                        Vector2 { x: 436.0, y: 185.0 },
                        Vector2 { x: 464.0, y: 177.0 },
                        RED,
                    );
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP) {
                    draw_circle(557, 144, 13.0, LIME);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT) {
                    draw_circle(586, 173, 13.0, RED);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN) {
                    draw_circle(557, 203, 13.0, VIOLET);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT) {
                    draw_circle(527, 173, 13.0, PINK);
                }

                // Draw buttons: d-pad
                draw_rectangle(225, 132, 24, 84, BLACK);
                draw_rectangle(195, 161, 84, 25, BLACK);
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP) {
                    draw_rectangle(225, 132, 24, 29, RED);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN) {
                    draw_rectangle(225, 132 + 54, 24, 30, RED);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT) {
                    draw_rectangle(195, 161, 30, 25, RED);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT) {
                    draw_rectangle(195 + 54, 161, 30, 25, RED);
                }

                // Draw buttons: left-right back buttons
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1) {
                    draw_circle(239, 82, 20.0, RED);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1) {
                    draw_circle(557, 82, 20.0, RED);
                }

                // Draw axis: left joystick
                let mut left_gamepad_color = BLACK;
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_THUMB) {
                    left_gamepad_color = RED;
                }
                draw_circle(319, 255, 35.0, BLACK);
                draw_circle(319, 255, 31.0, LIGHTGRAY);
                draw_circle(
                    319 + (left_stick_x * 20.0) as i32,
                    255 + (left_stick_y * 20.0) as i32,
                    25.0,
                    left_gamepad_color,
                );

                // Draw axis: right joystick
                let mut right_gamepad_color = BLACK;
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_THUMB) {
                    right_gamepad_color = RED;
                }
                draw_circle(475, 255, 35.0, BLACK);
                draw_circle(475, 255, 31.0, LIGHTGRAY);
                draw_circle(
                    475 + (right_stick_x * 20.0) as i32,
                    255 + (right_stick_y * 20.0) as i32,
                    25.0,
                    right_gamepad_color,
                );

                // Draw axis: left-right triggers
                draw_rectangle(169, 48, 15, 70, GRAY);
                draw_rectangle(611, 48, 15, 70, GRAY);
                draw_rectangle(
                    169,
                    48,
                    15,
                    (((1.0 + left_trigger) / 2.0) * 70.0) as i32,
                    RED,
                );
                draw_rectangle(
                    611,
                    48,
                    15,
                    (((1.0 + right_trigger) / 2.0) * 70.0) as i32,
                    RED,
                );
            } else {
                // Draw background: generic
                draw_rectangle_rounded(
                    Rectangle {
                        x: 175.0,
                        y: 110.0,
                        width: 460.0,
                        height: 220.0,
                    },
                    0.3,
                    16,
                    DARKGRAY,
                );

                // Draw buttons: basic
                draw_circle(365, 170, 12.0, RAYWHITE);
                draw_circle(405, 170, 12.0, RAYWHITE);
                draw_circle(445, 170, 12.0, RAYWHITE);
                draw_circle(516, 191, 17.0, RAYWHITE);
                draw_circle(551, 227, 17.0, RAYWHITE);
                draw_circle(587, 191, 17.0, RAYWHITE);
                draw_circle(551, 155, 17.0, RAYWHITE);
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT) {
                    draw_circle(365, 170, 10.0, RED);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_MIDDLE) {
                    draw_circle(405, 170, 10.0, GREEN);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT) {
                    draw_circle(445, 170, 10.0, BLUE);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT) {
                    draw_circle(516, 191, 15.0, GOLD);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN) {
                    draw_circle(551, 227, 15.0, BLUE);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT) {
                    draw_circle(587, 191, 15.0, GREEN);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP) {
                    draw_circle(551, 155, 15.0, RED);
                }

                // Draw buttons: d-pad
                draw_rectangle(245, 145, 28, 88, RAYWHITE);
                draw_rectangle(215, 174, 88, 29, RAYWHITE);
                draw_rectangle(247, 147, 24, 84, BLACK);
                draw_rectangle(217, 176, 84, 25, BLACK);
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP) {
                    draw_rectangle(247, 147, 24, 29, RED);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN) {
                    draw_rectangle(247, 147 + 54, 24, 30, RED);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT) {
                    draw_rectangle(217, 176, 30, 25, RED);
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT) {
                    draw_rectangle(217 + 54, 176, 30, 25, RED);
                }

                // Draw buttons: left-right back
                draw_rectangle_rounded(
                    Rectangle {
                        x: 215.0,
                        y: 98.0,
                        width: 100.0,
                        height: 10.0,
                    },
                    0.5,
                    16,
                    DARKGRAY,
                );
                draw_rectangle_rounded(
                    Rectangle {
                        x: 495.0,
                        y: 98.0,
                        width: 100.0,
                        height: 10.0,
                    },
                    0.5,
                    16,
                    DARKGRAY,
                );
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1) {
                    draw_rectangle_rounded(
                        Rectangle {
                            x: 215.0,
                            y: 98.0,
                            width: 100.0,
                            height: 10.0,
                        },
                        0.5,
                        16,
                        RED,
                    );
                }
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1) {
                    draw_rectangle_rounded(
                        Rectangle {
                            x: 495.0,
                            y: 98.0,
                            width: 100.0,
                            height: 10.0,
                        },
                        0.5,
                        16,
                        RED,
                    );
                }

                // Draw axis: left joystick
                let mut left_gamepad_color = BLACK;
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_LEFT_THUMB) {
                    left_gamepad_color = RED;
                }
                draw_circle(345, 260, 40.0, BLACK);
                draw_circle(345, 260, 35.0, LIGHTGRAY);
                draw_circle(
                    345 + (left_stick_x * 20.0) as i32,
                    260 + (left_stick_y * 20.0) as i32,
                    25.0,
                    left_gamepad_color,
                );

                // Draw axis: right joystick
                let mut right_gamepad_color = BLACK;
                if is_gamepad_button_down(gamepad, GamepadButton::GAMEPAD_BUTTON_RIGHT_THUMB) {
                    right_gamepad_color = RED;
                }
                draw_circle(465, 260, 40.0, BLACK);
                draw_circle(465, 260, 35.0, LIGHTGRAY);
                draw_circle(
                    465 + (right_stick_x * 20.0) as i32,
                    260 + (right_stick_y * 20.0) as i32,
                    25.0,
                    right_gamepad_color,
                );

                // Draw axis: left-right triggers
                draw_rectangle(151, 110, 15, 70, GRAY);
                draw_rectangle(644, 110, 15, 70, GRAY);
                draw_rectangle(
                    151,
                    110,
                    15,
                    (((1.0 + left_trigger) / 2.0) * 70.0) as i32,
                    RED,
                );
                draw_rectangle(
                    644,
                    110,
                    15,
                    (((1.0 + right_trigger) / 2.0) * 70.0) as i32,
                    RED,
                );
            }

            draw_text(
                &format!("DETECTED AXIS [{}]:", get_gamepad_axis_count(gamepad)),
                10,
                50,
                10,
                MAROON,
            );

            // Draw vibrate button
            draw_rectangle_rec(vibrate_button, SKYBLUE);
            draw_text(
                "VIBRATE",
                (vibrate_button.x + 14.0) as i32,
                (vibrate_button.y + 1.0) as i32,
                10,
                DARKGRAY,
            );

            if get_gamepad_button_pressed() != GamepadButton::GAMEPAD_BUTTON_UNKNOWN as i32 {
                draw_text(
                    &format!("DETECTED BUTTON: {}", get_gamepad_button_pressed()),
                    10,
                    430,
                    10,
                    RED,
                );
            } else {
                draw_text("DETECTED BUTTON: NONE", 10, 430, 10, GRAY);
            }
        } else {
            draw_text(&format!("GP{}: NOT DETECTED", gamepad), 10, 10, 10, GRAY);
            draw_texture(&tex_xbox_pad, 0, 0, LIGHTGRAY);
        }

        end_drawing();
    }

    // De-Initialization
    tex_ps3_pad.unload_texture();
    tex_xbox_pad.unload_texture();

    close_window();
}
