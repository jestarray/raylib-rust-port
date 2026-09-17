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

use raylib::{
    rcore::{
        BeginDrawing, BeginShaderMode, ClearBackground, CloseWindow, EndDrawing, EndShaderMode, GetFrameTime, GetScreenHeight, GetScreenWidth, GetShaderLocation, InitWindow, LoadShader, SetShaderValue, SetTargetFPS, UnloadShader,
    }, rcore_desktop_sdl::WindowShouldClose, rtextures::{DrawTexture, LoadTexture, UnloadTexture}, types::{RAYWHITE, ShaderUniformDataType::*, WHITE},
};

#[cfg(target_os = "android")]
const GLSL_VERSION: i32 = 100;
#[cfg(not(target_os = "android"))]
const GLSL_VERSION: i32 = 330;

fn main() {
    unsafe {
        start();
    }
}
unsafe fn start() {
    // Initialization
    let screenWidth: i32 = 800;
    let screenHeight: i32 = 450;

    InitWindow(
        screenWidth,
        screenHeight,
        "raylib [shaders] example - texture waves",
    );

    // Load texture to apply shaders
    let mut texture = LoadTexture("resources/space.png");

    // Load shader and setup location points and values
    let mut shader = LoadShader(
        None,
        Some(&format!("resources/shaders/glsl{}/wave.fs", GLSL_VERSION)),
    );

    let secondsLoc = GetShaderLocation(&shader, "seconds");
    let freqXLoc = GetShaderLocation(&shader, "freqX");
    let freqYLoc = GetShaderLocation(&shader, "freqY");
    let ampXLoc = GetShaderLocation(&shader, "ampX");
    let ampYLoc = GetShaderLocation(&shader, "ampY");
    let speedXLoc = GetShaderLocation(&shader, "speedX");
    let speedYLoc = GetShaderLocation(&shader, "speedY");

    // Shader uniform values that can be updated at any time
    let mut freqX: f32 = 25.0;
    let mut freqY: f32 = 25.0;
    let mut ampX: f32 = 5.0;
    let mut ampY: f32 = 5.0;
    let mut speedX: f32 = 8.0;
    let mut speedY: f32 = 8.0;

    let screenSize: [f32; 2] = [GetScreenWidth() as f32, GetScreenHeight() as f32];
    let locIndex = GetShaderLocation(&shader, "size");
    SetShaderValue(
        &mut shader,
        locIndex,
        screenSize.as_ptr() as *const c_void,
        SHADER_UNIFORM_VEC2 as i32,
    );
    SetShaderValue(&mut shader, freqXLoc, &freqX as *const f32 as *const c_void, SHADER_UNIFORM_FLOAT as i32);
    SetShaderValue(&mut shader, freqYLoc, &freqY as *const f32 as *const c_void, SHADER_UNIFORM_FLOAT as i32);
    SetShaderValue(&mut shader, ampXLoc, &ampX as *const f32 as *const c_void, SHADER_UNIFORM_FLOAT as i32);
    SetShaderValue(&mut shader, ampYLoc, &ampY as *const f32 as *const c_void, SHADER_UNIFORM_FLOAT as i32);
    SetShaderValue(&mut shader, speedXLoc, &speedX as *const f32 as *const c_void, SHADER_UNIFORM_FLOAT as i32);
    SetShaderValue(&mut shader, speedYLoc, &speedY as *const f32 as *const c_void, SHADER_UNIFORM_FLOAT as i32);

    let mut seconds: f32 = 0.0;

    SetTargetFPS(60);

    // Main game loop
    while !WindowShouldClose() {
        // Update
        seconds += GetFrameTime();

        SetShaderValue(&mut shader, secondsLoc, &seconds as *const f32 as *const c_void, SHADER_UNIFORM_FLOAT as i32);

        // Draw
        BeginDrawing();

        ClearBackground(RAYWHITE);

        BeginShaderMode(&mut shader);

        DrawTexture(&texture, 0, 0, WHITE);
        DrawTexture(&texture, texture.width, 0, WHITE);

        EndShaderMode();

        EndDrawing();
    }

    // De-Initialization
    UnloadShader(&mut shader);
    UnloadTexture(&mut texture);

    CloseWindow();
}
