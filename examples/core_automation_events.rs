use log::info;
use raylib::core::{
    begin_drawing, begin_mode_2d, clear_background, close_window, end_drawing, end_mode_2d,
    export_automation_event_list, get_mouse_wheel_move, get_world_to_screen_2d, init_window,
    is_file_dropped, is_key_down, is_key_pressed, load_automation_event_list,
    play_automation_event, set_automation_event_base_frame, set_automation_event_list,
    set_target_fps, start_automation_event_recording, stop_automation_event_recording,
};
use raylib::rcolors::{
    BLACK, BLUE, DARKGRAY, DARKGREEN, GRAY, LIGHTGRAY, LIME, MAROON, RED, SKYBLUE,
};
use raylib::sdl::window_should_close;
use raylib::shapes::{
    draw_circle, draw_rectangle, draw_rectangle_lines, draw_rectangle_rec, draw_triangle,
};
use raylib::text::draw_text;
use raylib::textures::fade;
use raylib::types::{Camera2D, Color, KeyboardKey, Rectangle, Vector2};

const GRAVITY: f32 = 400.0;
const PLAYER_JUMP_SPD: f32 = 350.0;
const PLAYER_HOR_SPD: f32 = 200.0;

const MAX_ENVIRONMENT_ELEMENTS: usize = 5;

#[derive(Clone, Copy, Default)]
struct Player {
    position: Vector2,
    speed: f32,
    can_jump: bool,
}

#[derive(Clone, Copy)]
struct EnvElement {
    rect: Rectangle,
    blocking: i32,
    color: Color,
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default().filter_or("MY_LOG_LEVEL", "info"));
    start();
}

fn start() {
    // Initialization
    let screen_width: i32 = 800;
    let screen_height: i32 = 450;

    init_window(
        screen_width,
        screen_height,
        "raylib [core] example - automation events",
    );

    // Define player
    let mut player = Player::default();
    player.position = Vector2 { x: 400.0, y: 280.0 };

    // Define environment elements (platforms)
    let env_elements: [EnvElement; MAX_ENVIRONMENT_ELEMENTS] = [
        EnvElement {
            rect: Rectangle {
                x: 0.0,
                y: 0.0,
                width: 1000.0,
                height: 400.0,
            },
            blocking: 0,
            color: LIGHTGRAY,
        },
        EnvElement {
            rect: Rectangle {
                x: 0.0,
                y: 400.0,
                width: 1000.0,
                height: 200.0,
            },
            blocking: 1,
            color: GRAY,
        },
        EnvElement {
            rect: Rectangle {
                x: 300.0,
                y: 200.0,
                width: 400.0,
                height: 10.0,
            },
            blocking: 1,
            color: GRAY,
        },
        EnvElement {
            rect: Rectangle {
                x: 250.0,
                y: 300.0,
                width: 100.0,
                height: 10.0,
            },
            blocking: 1,
            color: GRAY,
        },
        EnvElement {
            rect: Rectangle {
                x: 650.0,
                y: 300.0,
                width: 100.0,
                height: 10.0,
            },
            blocking: 1,
            color: GRAY,
        },
    ];

    // Define camera
    let mut camera = Camera2D::default();
    camera.target = player.position;
    camera.offset = Vector2 {
        x: screen_width as f32 / 2.0,
        y: screen_height as f32 / 2.0,
    };
    camera.rotation = 0.0;
    camera.zoom = 1.0;

    // Automation events
    // Initialize list of automation events to record new events
    let mut aelist = load_automation_event_list(None::<String>);
    set_automation_event_list(&mut aelist);
    let mut event_recording = false;
    let mut event_playing = false;

    let mut frame_counter: u32 = 0;
    let mut play_frame_counter: u32 = 0;
    let mut current_play_frame: usize = 0;

    set_target_fps(60);

    // Main game loop
    while !window_should_close() {
        // Update
        let delta_time: f32 = 0.015; // get_frame_time();

        // Dropped files logic
        // Dropped files logic
        if is_file_dropped() {
            // TODO: load the dropped .rae file to replay it. That needs LoadDroppedFiles(),
            // which this port does not implement yet.
        }

        // Update player
        if is_key_down(KeyboardKey::KEY_LEFT) {
            player.position.x -= PLAYER_HOR_SPD * delta_time;
        }
        if is_key_down(KeyboardKey::KEY_RIGHT) {
            player.position.x += PLAYER_HOR_SPD * delta_time;
        }
        if is_key_down(KeyboardKey::KEY_SPACE) && player.can_jump {
            player.speed = -PLAYER_JUMP_SPD;
            player.can_jump = false;
        }

        let mut hit_obstacle = 0;
        for i in 0..MAX_ENVIRONMENT_ELEMENTS {
            let element = &env_elements[i];
            if element.blocking != 0
                && element.rect.x <= player.position.x
                && element.rect.x + element.rect.width >= player.position.x
                && element.rect.y >= player.position.y
                && element.rect.y <= player.position.y + player.speed * delta_time
            {
                hit_obstacle = 1;
                player.speed = 0.0;
                player.position.y = element.rect.y;
            }
        }

        if hit_obstacle == 0 {
            player.position.y += player.speed * delta_time;
            player.speed += GRAVITY * delta_time;
            player.can_jump = false;
        } else {
            player.can_jump = true;
        }

        if is_key_pressed(KeyboardKey::KEY_R) {
            // Reset game state
            player.position = Vector2 { x: 400.0, y: 280.0 };
            player.speed = 0.0;
            player.can_jump = false;

            camera.target = player.position;
            camera.offset = Vector2 {
                x: screen_width as f32 / 2.0,
                y: screen_height as f32 / 2.0,
            };
            camera.rotation = 0.0;
            camera.zoom = 1.0;
        }

        // Events playing
        // NOTE: Logic must be before Camera update because it depends on mouse-wheel value,
        // that can be set by the played event... but some other inputs could be affected
        if event_playing {
            // NOTE: Multiple events could be executed in a single frame
            while play_frame_counter == aelist.events[current_play_frame].frame {
                play_automation_event(aelist.events[current_play_frame]);
                current_play_frame += 1;

                if current_play_frame == aelist.count as usize {
                    event_playing = false;
                    current_play_frame = 0;
                    play_frame_counter = 0;

                    info!("FINISH PLAYING!");
                    break;
                }
            }

            play_frame_counter += 1;
        }

        // Update camera
        camera.target = player.position;
        camera.offset = Vector2 {
            x: screen_width as f32 / 2.0,
            y: screen_height as f32 / 2.0,
        };
        let mut min_x: f32 = 1000.0;
        let mut min_y: f32 = 1000.0;
        let mut max_x: f32 = -1000.0;
        let mut max_y: f32 = -1000.0;

        // WARNING: On event replay, mouse-wheel internal value is set
        camera.zoom += get_mouse_wheel_move() * 0.05;
        if camera.zoom > 3.0 {
            camera.zoom = 3.0;
        } else if camera.zoom < 0.25 {
            camera.zoom = 0.25;
        }

        for i in 0..MAX_ENVIRONMENT_ELEMENTS {
            let element = &env_elements[i];
            min_x = f32::min(element.rect.x, min_x);
            max_x = f32::max(element.rect.x + element.rect.width, max_x);
            min_y = f32::min(element.rect.y, min_y);
            max_y = f32::max(element.rect.y + element.rect.height, max_y);
        }

        let max = get_world_to_screen_2d(Vector2 { x: max_x, y: max_y }, camera);
        let min = get_world_to_screen_2d(Vector2 { x: min_x, y: min_y }, camera);

        if max.x < screen_width as f32 {
            camera.offset.x = screen_width as f32 - (max.x - screen_width as f32 / 2.0);
        }
        if max.y < screen_height as f32 {
            camera.offset.y = screen_height as f32 - (max.y - screen_height as f32 / 2.0);
        }
        if min.x > 0.0 {
            camera.offset.x = screen_width as f32 / 2.0 - min.x;
        }
        if min.y > 0.0 {
            camera.offset.y = screen_height as f32 / 2.0 - min.y;
        }

        // Events management
        if is_key_pressed(KeyboardKey::KEY_S) {
            // Toggle events recording
            if !event_playing {
                if event_recording {
                    stop_automation_event_recording();
                    event_recording = false;

                    export_automation_event_list(aelist.clone(), "automation.rae");

                    info!("{}", format!("RECORDED FRAMES: {}", aelist.count));
                } else {
                    set_automation_event_base_frame(180);
                    start_automation_event_recording();
                    event_recording = true;
                }
            }
        } else if is_key_pressed(KeyboardKey::KEY_A) {
            // Toggle events playing (WARNING: Starts next frame)
            if !event_recording && (aelist.count > 0) {
                // Reset scene state to play
                event_playing = true;
                play_frame_counter = 0;
                current_play_frame = 0;

                player.position = Vector2 { x: 400.0, y: 280.0 };
                player.speed = 0.0;
                player.can_jump = false;

                camera.target = player.position;
                camera.offset = Vector2 {
                    x: screen_width as f32 / 2.0,
                    y: screen_height as f32 / 2.0,
                };
                camera.rotation = 0.0;
                camera.zoom = 1.0;
            }
        }

        if event_recording || event_playing {
            frame_counter += 1;
        } else {
            frame_counter = 0;
        }

        // Draw
        begin_drawing();

        clear_background(LIGHTGRAY);

        begin_mode_2d(camera);

        // Draw environment elements
        for i in 0..MAX_ENVIRONMENT_ELEMENTS {
            draw_rectangle_rec(env_elements[i].rect, env_elements[i].color);
        }

        // Draw player rectangle
        draw_rectangle_rec(
            Rectangle {
                x: player.position.x - 20.0,
                y: player.position.y - 40.0,
                width: 40.0,
                height: 40.0,
            },
            RED,
        );

        end_mode_2d();

        // Draw game controls
        draw_rectangle(10, 10, 290, 145, fade(SKYBLUE, 0.5));
        draw_rectangle_lines(10, 10, 290, 145, fade(BLUE, 0.8));

        draw_text("Controls:", 20, 20, 10, BLACK);
        draw_text("- RIGHT | LEFT: Player movement", 30, 40, 10, DARKGRAY);
        draw_text("- SPACE: Player jump", 30, 60, 10, DARKGRAY);
        draw_text("- R: Reset game state", 30, 80, 10, DARKGRAY);

        draw_text("- S: START/STOP RECORDING INPUT EVENTS", 30, 110, 10, BLACK);
        draw_text("- A: REPLAY LAST RECORDED INPUT EVENTS", 30, 130, 10, BLACK);

        // Draw automation events recording indicator
        if event_recording {
            draw_rectangle(10, 160, 290, 30, fade(RED, 0.3));
            draw_rectangle_lines(10, 160, 290, 30, fade(MAROON, 0.8));
            draw_circle(30, 175, 10.0, MAROON);

            if ((frame_counter / 15) % 2) == 1 {
                draw_text(
                    &format!("RECORDING EVENTS... [{}]", aelist.count),
                    50,
                    170,
                    10,
                    MAROON,
                );
            }
        } else if event_playing {
            draw_rectangle(10, 160, 290, 30, fade(LIME, 0.3));
            draw_rectangle_lines(10, 160, 290, 30, fade(DARKGREEN, 0.8));
            draw_triangle(
                Vector2 { x: 20.0, y: 165.0 },
                Vector2 { x: 20.0, y: 185.0 },
                Vector2 { x: 40.0, y: 175.0 },
                DARKGREEN,
            );

            if ((frame_counter / 15) % 2) == 1 {
                draw_text(
                    &format!("PLAYING RECORDED EVENTS... [{}]", current_play_frame),
                    50,
                    170,
                    10,
                    DARKGREEN,
                );
            }
        }

        end_drawing();
    }

    // De-Initialization
    close_window(); // Close window and OpenGL context
}
