use raylib::core::{
    begin_drawing, begin_shader_mode, clear_background, close_window, end_drawing, end_shader_mode,
    get_frame_time, get_screen_height, get_screen_width, init_window,
    load_shader, set_target_fps,
};
use raylib::rcolors::{RAYWHITE, WHITE};
use raylib::sdl::window_should_close;
use raylib::textures::{draw_texture, load_texture};
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

    let seconds_loc = shader.get_shader_location("seconds");
    let freq_x_loc = shader.get_shader_location("freqX");
    let freq_y_loc = shader.get_shader_location("freqY");
    let amp_x_loc = shader.get_shader_location("ampX");
    let amp_y_loc = shader.get_shader_location("ampY");
    let speed_x_loc = shader.get_shader_location("speedX");
    let speed_y_loc = shader.get_shader_location("speedY");

    // Shader uniform values that can be updated at any time
    let freq_x: f32 = 25.0;
    let freq_y: f32 = 25.0;
    let amp_x: f32 = 5.0;
    let amp_y: f32 = 5.0;
    let speed_x: f32 = 8.0;
    let speed_y: f32 = 8.0;

    let screen_size: [f32; 2] = [get_screen_width() as f32, get_screen_height() as f32];
    let size_loc = shader.get_shader_location("size");

    // Shader uniform methods accept values whose types match the declared uniform type.
    shader.set_shader_value(
        size_loc,
        screen_size,
        ShaderUniformDataType::SHADER_UNIFORM_VEC2,
    );
    shader.set_shader_value(
        freq_x_loc,
        freq_x,
        ShaderUniformDataType::SHADER_UNIFORM_FLOAT,
    );
    shader.set_shader_value(
        freq_y_loc,
        freq_y,
        ShaderUniformDataType::SHADER_UNIFORM_FLOAT,
    );
    shader.set_shader_value(
        amp_x_loc,
        amp_x,
        ShaderUniformDataType::SHADER_UNIFORM_FLOAT,
    );
    shader.set_shader_value(
        amp_y_loc,
        amp_y,
        ShaderUniformDataType::SHADER_UNIFORM_FLOAT,
    );
    shader.set_shader_value(
        speed_x_loc,
        speed_x,
        ShaderUniformDataType::SHADER_UNIFORM_FLOAT,
    );
    shader.set_shader_value(
        speed_y_loc,
        speed_y,
        ShaderUniformDataType::SHADER_UNIFORM_FLOAT,
    );

    let mut seconds: f32 = 0.0;

    set_target_fps(60);

    // Main game loop
    while !window_should_close() {
        // Update
        seconds += get_frame_time();

        shader.set_shader_value(
            seconds_loc,
            seconds,
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
    shader.unload_shader();
    texture.unload_texture();

    close_window();
}
