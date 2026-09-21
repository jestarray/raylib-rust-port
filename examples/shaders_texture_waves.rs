use raylib::core::{
    begin_drawing, begin_shader_mode, clear_background, close_window, end_drawing, end_shader_mode,
    get_frame_time, get_screen_height, get_screen_width, get_shader_location, init_window,
    load_shader, set_shader_value, set_target_fps, unload_shader,
};
use raylib::rcolors::{RAYWHITE, WHITE};
use raylib::sdl::window_should_close;
use raylib::textures::{draw_texture, load_texture, unload_texture};
use raylib::types::ShaderUniformDataType;

#[cfg(target_os = "android")]
const GLSL_VERSION: i32 = 100;
#[cfg(not(target_os = "android"))]
const GLSL_VERSION: i32 = 330;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().filter_or("MY_LOG_LEVEL", "info"));

    // Initialization
    let screen_width: i32 = 800;
    let screen_height: i32 = 450;

    init_window(
        screen_width,
        screen_height,
        "raylib [shaders] example - texture waves",
    );

    // Load texture to apply shaders
    let mut texture = load_texture("resources/space.png");

    // Load shader and setup location points and values
    let mut shader = load_shader(
        None::<&str>,
        Some(format!("resources/shaders/glsl{GLSL_VERSION}/wave.fs")),
    );

    let seconds_loc = get_shader_location(&shader, "seconds");
    let freq_x_loc = get_shader_location(&shader, "freqX");
    let freq_y_loc = get_shader_location(&shader, "freqY");
    let amp_x_loc = get_shader_location(&shader, "ampX");
    let amp_y_loc = get_shader_location(&shader, "ampY");
    let speed_x_loc = get_shader_location(&shader, "speedX");
    let speed_y_loc = get_shader_location(&shader, "speedY");

    // Shader uniform values that can be updated at any time
    let freq_x: f32 = 25.0;
    let freq_y: f32 = 25.0;
    let amp_x: f32 = 5.0;
    let amp_y: f32 = 5.0;
    let speed_x: f32 = 8.0;
    let speed_y: f32 = 8.0;

    let screen_size: [f32; 2] = [get_screen_width() as f32, get_screen_height() as f32];
    let size_loc = get_shader_location(&shader, "size");

    // The raw bindings took a `*const c_void` plus a uniform type; the safe wrappers take a
    // reference to the value, so no pointer casts are needed here.
    set_shader_value(
        &mut shader,
        size_loc,
        &screen_size,
        ShaderUniformDataType::SHADER_UNIFORM_VEC2,
    );
    set_shader_value(
        &mut shader,
        freq_x_loc,
        &freq_x,
        ShaderUniformDataType::SHADER_UNIFORM_FLOAT,
    );
    set_shader_value(
        &mut shader,
        freq_y_loc,
        &freq_y,
        ShaderUniformDataType::SHADER_UNIFORM_FLOAT,
    );
    set_shader_value(
        &mut shader,
        amp_x_loc,
        &amp_x,
        ShaderUniformDataType::SHADER_UNIFORM_FLOAT,
    );
    set_shader_value(
        &mut shader,
        amp_y_loc,
        &amp_y,
        ShaderUniformDataType::SHADER_UNIFORM_FLOAT,
    );
    set_shader_value(
        &mut shader,
        speed_x_loc,
        &speed_x,
        ShaderUniformDataType::SHADER_UNIFORM_FLOAT,
    );
    set_shader_value(
        &mut shader,
        speed_y_loc,
        &speed_y,
        ShaderUniformDataType::SHADER_UNIFORM_FLOAT,
    );

    let mut seconds: f32 = 0.0;

    set_target_fps(60);

    // Main game loop
    while !window_should_close() {
        // Update
        seconds += get_frame_time();

        set_shader_value(
            &mut shader,
            seconds_loc,
            &seconds,
            ShaderUniformDataType::SHADER_UNIFORM_FLOAT,
        );

        // Draw
        begin_drawing();

        clear_background(RAYWHITE);

        begin_shader_mode(&mut shader);

        draw_texture(&texture, 0, 0, WHITE);
        draw_texture(&texture, texture.width, 0, WHITE);

        end_shader_mode();

        end_drawing();
    }

    // De-Initialization
    unload_shader(&mut shader);
    unload_texture(&mut texture);

    close_window();
}
