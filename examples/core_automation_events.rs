#![allow(unused_variables)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(missing_safety_doc, unused_parens, non_snake_case, static_mut_refs)]
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
use std::os::raw::c_void;

use log::info;
use raylib::{
    rcore::{
        BeginDrawing, BeginShaderMode, ClearBackground, CloseWindow, EndDrawing, EndShaderMode, GetFrameTime, GetScreenHeight, GetScreenWidth, GetShaderLocation, InitWindow, LoadShader, SetShaderValue, SetTargetFPS, UnloadShader,
    }, rcore_desktop_sdl::WindowShouldClose, rtextures::{DrawTexture, LoadTexture, UnloadTexture}, types::{RAYWHITE, ShaderUniformDataType::*, WHITE},
};
use raylib::types::*;
use raylib::rcore::*;
use raylib::rtext::*;
use raylib::rshapes::*;
use raylib::rtextures::*;
use raylib::types::KeyboardKey::*;

const GRAVITY: f32 = 400.0;
const PLAYER_JUMP_SPD: f32 = 350.0;
const PLAYER_HOR_SPD: f32 = 200.0;

const MAX_ENVIRONMENT_ELEMENTS: usize = 5;

//----------------------------------------------------------------------------------
// Types and Structures Definition
//----------------------------------------------------------------------------------
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Player {
    pub position: Vector2,
    pub speed: f32,
    pub canJump: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct EnvElement {
    pub rect: Rectangle,
    pub blocking: i32,
    pub color: Color,
}

//------------------------------------------------------------------------------------
// Program main entry point
//------------------------------------------------------------------------------------
fn main() {
    unsafe { start(); }
}
unsafe fn start() {
    // Initialization
    //--------------------------------------------------------------------------------------
    let screenWidth: i32 = 800;
    let screenHeight: i32 = 450;

    InitWindow(screenWidth, screenHeight, "raylib [core] example - automation events");

    // Define player
    let mut player = Player::default();
    player.position = Vector2 { x: 400.0, y: 280.0 };
    player.speed = 0.0;
    player.canJump = false;

    // Define environment elements (platforms)
    let mut envElements: [EnvElement; MAX_ENVIRONMENT_ELEMENTS] = [
        EnvElement { rect: Rectangle { x: 0.0, y: 0.0, width: 1000.0, height: 400.0 }, blocking: 0, color: LIGHTGRAY },
        EnvElement { rect: Rectangle { x: 0.0, y: 400.0, width: 1000.0, height: 200.0 }, blocking: 1, color: GRAY },
        EnvElement { rect: Rectangle { x: 300.0, y: 200.0, width: 400.0, height: 10.0 }, blocking: 1, color: GRAY },
        EnvElement { rect: Rectangle { x: 250.0, y: 300.0, width: 100.0, height: 10.0 }, blocking: 1, color: GRAY },
        EnvElement { rect: Rectangle { x: 650.0, y: 300.0, width: 100.0, height: 10.0 }, blocking: 1, color: GRAY },
    ];

    // Define camera
    let mut camera = Camera2D::default();
    camera.target = player.position;
    camera.offset = Vector2 { x: screenWidth as f32 / 2.0, y: screenHeight as f32 / 2.0 };
    camera.rotation = 0.0;
    camera.zoom = 1.0;

    // Automation events
    let mut aelist = LoadAutomationEventList(None as Option<String>); // Initialize list of automation events to record new events
    SetAutomationEventList(&mut aelist);
    let mut eventRecording = false;
    let mut eventPlaying = false;

    let mut frameCounter: u32 = 0;
    let mut playFrameCounter: u32 = 0;
    let mut currentPlayFrame: usize = 0;

    SetTargetFPS(60);
    //--------------------------------------------------------------------------------------

    // Main game loop
    while !WindowShouldClose() {
        // Update
        //----------------------------------------------------------------------------------
        let deltaTime: f32 = 0.015; //GetFrameTime();

        // Dropped files logic
        //----------------------------------------------------------------------------------
        if IsFileDropped() {
            // dragged and drop files
            //let droppedFiles = LoadDroppedFiles();

            // Supports loading .rgs style files (text or binary) and .png style palette images
            //if IsFileExtension(droppedFiles.paths[0], ".txt;.rae") {
            //    UnloadAutomationEventList(&mut aelist);
            //    aelist = LoadAutomationEventList(droppedFiles.paths[0]);
//
            //    eventRecording = false;
//
            //    // Reset scene state to play
            //    eventPlaying = true;
            //    playFrameCounter = 0;
            //    currentPlayFrame = 0;
//
            //    player.position = Vector2 { x: 400.0, y: 280.0 };
            //    player.speed = 0.0;
            //    player.canJump = false;
//
            //    camera.target = player.position;
            //    camera.offset = Vector2 { x: screenWidth as f32 / 2.0, y: screenHeight as f32 / 2.0 };
            //    camera.rotation = 0.0;
            //    camera.zoom = 1.0;
            //}
//
            //UnloadDroppedFiles(droppedFiles); // Unload filepaths from memory
        }
        //----------------------------------------------------------------------------------

        // Update player
        //----------------------------------------------------------------------------------
        if IsKeyDown(KEY_LEFT) { player.position.x -= PLAYER_HOR_SPD * deltaTime; }
        if IsKeyDown(KEY_RIGHT) { player.position.x += PLAYER_HOR_SPD * deltaTime; }
        if IsKeyDown(KEY_SPACE) && player.canJump {
            player.speed = -PLAYER_JUMP_SPD;
            player.canJump = false;
        }

        let mut hitObstacle = 0;
        for i in 0..MAX_ENVIRONMENT_ELEMENTS {
            let element = &mut envElements[i];
            let p = &mut player.position;
            if element.blocking != 0 &&
                element.rect.x <= p.x &&
                element.rect.x + element.rect.width >= p.x &&
                element.rect.y >= p.y &&
                element.rect.y <= p.y + player.speed * deltaTime
            {
                hitObstacle = 1;
                player.speed = 0.0;
                p.y = element.rect.y;
            }
        }

        if hitObstacle == 0 {
            player.position.y += player.speed * deltaTime;
            player.speed += GRAVITY * deltaTime;
            player.canJump = false;
        } else {
            player.canJump = true;
        }

        if IsKeyPressed(KEY_R) {
            // Reset game state
            player.position = Vector2 { x: 400.0, y: 280.0 };
            player.speed = 0.0;
            player.canJump = false;

            camera.target = player.position;
            camera.offset = Vector2 { x: screenWidth as f32 / 2.0, y: screenHeight as f32 / 2.0 };
            camera.rotation = 0.0;
            camera.zoom = 1.0;
        }
        //----------------------------------------------------------------------------------

        // Events playing
        // NOTE: Logic must be before Camera update because it depends on mouse-wheel value,
        // that can be set by the played event... but some other inputs could be affected
        //----------------------------------------------------------------------------------
        if eventPlaying {
            // NOTE: Multiple events could be executed in a single frame
            while playFrameCounter == aelist.events[currentPlayFrame].frame {
                PlayAutomationEvent(aelist.events[currentPlayFrame]);
                currentPlayFrame += 1;

                if currentPlayFrame == aelist.count as usize {
                    eventPlaying = false;
                    currentPlayFrame = 0;
                    playFrameCounter = 0;

                        info!("FINISH PLAYING!");
                    break;
                }
            }

            playFrameCounter += 1;
        }
        //----------------------------------------------------------------------------------

        // Update camera
        //----------------------------------------------------------------------------------
        camera.target = player.position;
        camera.offset = Vector2 { x: screenWidth as f32 / 2.0, y: screenHeight as f32 / 2.0 };
        let mut minX: f32 = 1000.0;
        let mut minY: f32 = 1000.0;
        let mut maxX: f32 = -1000.0;
        let mut maxY: f32 = -1000.0;

        // WARNING: On event replay, mouse-wheel internal value is set
        camera.zoom += GetMouseWheelMove() as f32 * 0.05;
        if camera.zoom > 3.0 {
            camera.zoom = 3.0;
        } else if camera.zoom < 0.25 {
            camera.zoom = 0.25;
        }

        for i in 0..MAX_ENVIRONMENT_ELEMENTS {
            let element = &envElements[i];
            minX = f32::min(element.rect.x, minX);
            maxX = f32::max(element.rect.x + element.rect.width, maxX);
            minY = f32::min(element.rect.y, minY);
            maxY = f32::max(element.rect.y + element.rect.height, maxY);
        }

        let max = GetWorldToScreen2D(Vector2 { x: maxX, y: maxY }, camera);
        let min = GetWorldToScreen2D(Vector2 { x: minX, y: minY }, camera);

        if max.x < screenWidth as f32 { camera.offset.x = screenWidth as f32 - (max.x - screenWidth as f32 / 2.0); }
        if max.y < screenHeight as f32 { camera.offset.y = screenHeight as f32 - (max.y - screenHeight as f32 / 2.0); }
        if min.x > 0.0 { camera.offset.x = screenWidth as f32 / 2.0 - min.x; }
        if min.y > 0.0 { camera.offset.y = screenHeight as f32 / 2.0 - min.y; }
        //----------------------------------------------------------------------------------

        // Events management
        if IsKeyPressed(KEY_S) { // Toggle events recording
            if !eventPlaying {
                if eventRecording {
                    StopAutomationEventRecording();
                    eventRecording = false;

                    ExportAutomationEventList(aelist.clone(), "automation.rae");

                    info!("{}", format!("RECORDED FRAMES: {}", aelist.count))
                } else {
                    SetAutomationEventBaseFrame(180);
                    StartAutomationEventRecording();
                    eventRecording = true;
                }
            }
        } else if IsKeyPressed(KEY_A) { // Toggle events playing (WARNING: Starts next frame)
            if !eventRecording && (aelist.count > 0) {
                // Reset scene state to play
                eventPlaying = true;
                playFrameCounter = 0;
                currentPlayFrame = 0;

                player.position = Vector2 { x: 400.0, y: 280.0 };
                player.speed = 0.0;
                player.canJump = false;

                camera.target = player.position;
                camera.offset = Vector2 { x: screenWidth as f32 / 2.0, y: screenHeight as f32 / 2.0 };
                camera.rotation = 0.0;
                camera.zoom = 1.0;
            }
        }

        if eventRecording || eventPlaying {
            frameCounter += 1;
        } else {
            frameCounter = 0;
        }
        //----------------------------------------------------------------------------------

        // Draw
        //----------------------------------------------------------------------------------
        BeginDrawing();

            ClearBackground(LIGHTGRAY);

            BeginMode2D(camera);

                // Draw environment elements
                for i in 0..MAX_ENVIRONMENT_ELEMENTS {
                    DrawRectangleRec(envElements[i].rect, envElements[i].color);
                }

                // Draw player rectangle
                DrawRectangleRec(Rectangle { x: player.position.x - 20.0, y: player.position.y - 40.0, width: 40.0, height: 40.0 }, RED);

            EndMode2D();

            // Draw game controls
            DrawRectangle(10, 10, 290, 145, Fade(SKYBLUE, 0.5));
            DrawRectangleLines(10, 10, 290, 145, Fade(BLUE, 0.8));

            DrawText("Controls:", 20, 20, 10, BLACK);
            DrawText("- RIGHT | LEFT: Player movement", 30, 40, 10, DARKGRAY);
            DrawText("- SPACE: Player jump", 30, 60, 10, DARKGRAY);
            DrawText("- R: Reset game state", 30, 80, 10, DARKGRAY);

            DrawText("- S: START/STOP RECORDING INPUT EVENTS", 30, 110, 10, BLACK);
            DrawText("- A: REPLAY LAST RECORDED INPUT EVENTS", 30, 130, 10, BLACK);

            // Draw automation events recording indicator
            if eventRecording {
                DrawRectangle(10, 160, 290, 30, Fade(RED, 0.3));
                DrawRectangleLines(10, 160, 290, 30, Fade(MAROON, 0.8));
                DrawCircle(30, 175, 10.0, MAROON);

                if ((frameCounter / 15) % 2) == 1 { DrawText(&format!("RECORDING EVENTS... [{}]", aelist.count), 50, 170, 10, MAROON); }
            } else if eventPlaying {
                DrawRectangle(10, 160, 290, 30, Fade(LIME, 0.3));
                DrawRectangleLines(10, 160, 290, 30, Fade(DARKGREEN, 0.8));
                DrawTriangle(Vector2 { x: 20.0, y: 155.0 + 10.0 }, Vector2 { x: 20.0, y: 155.0 + 30.0 }, Vector2 { x: 40.0, y: 155.0 + 20.0 }, DARKGREEN);

                if ((frameCounter / 15) % 2) == 1 { DrawText(&format!("PLAYING RECORDED EVENTS... [{}]", currentPlayFrame), 50, 170, 10, DARKGREEN); }
            }

        EndDrawing();
        //----------------------------------------------------------------------------------
    }

    // De-Initialization
    //--------------------------------------------------------------------------------------
    CloseWindow(); // Close window and OpenGL context
    //--------------------------------------------------------------------------------------
}