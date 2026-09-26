//! Safe wrappers for the raw `rtext` bindings.
//!
//! Global text functions use `snake_case` names and can be called without `unsafe`.
//! Font methods live in [`crate::types`]. Font paths are generic over
//! [`AsRef<Path>`](AsRef) and codepoint parameters that name a font type use
//! [`FontType`].
//!

use std::path::Path;

use crate::rtext::*;
use crate::safe::core::path_to_str;
use crate::types::{Color, Font, FontType, GlyphInfo, Vector2};

pub fn load_font<P: AsRef<Path>>(file_name: P) -> Font {
    match path_to_str(file_name.as_ref()) {
        Some(file_name) => unsafe { LoadFont(file_name) },
        None => get_font_default(),
    }
}

pub fn load_font_ex<P: AsRef<Path>>(
    file_name: P,
    font_size: i32,
    codepoints: Option<&[i32]>,
) -> Font {
    match path_to_str(file_name.as_ref()) {
        Some(file_name) => unsafe { LoadFontEx(file_name, font_size, codepoints) },
        None => get_font_default(),
    }
}

pub fn load_font_from_memory(
    file_type: &str,
    file_data: &[u8],
    font_size: i32,
    codepoints: Option<&[i32]>,
) -> Font {
    unsafe { LoadFontFromMemory(file_type, file_data, font_size, codepoints) }
}

pub fn load_font_data(
    file_data: &[u8],
    font_size: i32,
    codepoints: Option<&[i32]>,
    font_type: FontType,
) -> Vec<GlyphInfo> {
    LoadFontData(file_data, font_size, codepoints, font_type)
}

pub fn load_font_default() {
    unsafe { LoadFontDefault() }
}

pub fn unload_font_default() {
    UnloadFontDefault()
}

/// Returns a copy of the default font's metrics and glyphs. The texture is shared;
/// [`Font::unload_font`](crate::types::Font::unload_font) leaves the default texture alive.
pub fn get_font_default() -> Font {
    unsafe { (&*GetFontDefault()).clone() }
}

pub fn draw_fps(pos_x: i32, pos_y: i32) {
    DrawFPS(pos_x, pos_y)
}

pub fn draw_text(text: &str, x: i32, y: i32, font_size: i32, color: Color) {
    DrawText(text, x, y, font_size, color)
}

pub fn set_text_line_spacing(spacing: i32) {
    SetTextLineSpacing(spacing)
}

pub fn get_codepoint_next(text: &str, codepoint_size: &mut i32) -> i32 {
    GetCodepointNext(text, codepoint_size)
}

pub fn draw_text_codepoint(
    font: &Font,
    codepoint: i32,
    position: Vector2,
    font_size: f32,
    tint: Color,
) {
    DrawTextCodepoint(font, codepoint, position, font_size, tint)
}

pub fn measure_text(text: &str, font_size: i32) -> i32 {
    MeasureText(text, font_size)
}

#[allow(clippy::too_many_arguments)]
pub fn draw_text_pro(
    font: &Font,
    text: &str,
    position: Vector2,
    origin: Vector2,
    rotation: f32,
    font_size: f32,
    spacing: f32,
    tint: Color,
) {
    DrawTextPro(
        font, text, position, origin, rotation, font_size, spacing, tint,
    )
}

pub fn draw_text_ex(
    font: &Font,
    text: &str,
    position: Vector2,
    font_size: f32,
    spacing: f32,
    tint: Color,
) {
    DrawTextEx(font, text, position, font_size, spacing, tint)
}
