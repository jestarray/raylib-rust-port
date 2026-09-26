//! Safe wrappers for the raw `rtextures` bindings.
//!
//! Global texture functions use `snake_case` names and can be called without `unsafe`.
//! Methods on images, textures, and pixel formats live in [`crate::types`].
//! Path-like parameters are generic over [`AsRef<Path>`](AsRef).

use std::path::Path;

use crate::rtextures::*;
use crate::safe::core::path_to_str;
use crate::types::{
    Color, Image, NPatchInfo, PixelFormat, Rectangle, RenderTexture2D, Texture, Vector2,
};

pub fn load_image<P: AsRef<Path>>(file_path: P) -> Image {
    LoadImage(file_path)
}

/// The data size is derived from `file_data.len()`.
pub fn load_image_from_memory(file_ext: &str, file_data: &[u8]) -> Image {
    LoadImageFromMemory(file_ext, file_data, file_data.len() as i32)
}

pub fn load_render_texture(width: i32, height: i32) -> RenderTexture2D {
    return unsafe { LoadRenderTexture(width, height) };
}

pub fn load_render_texture_ex(width: i32, height: i32, format: PixelFormat) -> RenderTexture2D {
    return unsafe { LoadRenderTextureEx(width, height, format as i32) };
}

pub fn load_image_from_screen() -> Image {
    unsafe { LoadImageFromScreen() }
}

/// Falls back to the "missing texture" checkerboard when the path is not valid UTF-8.
pub fn load_texture<P: AsRef<Path>>(file_name: P) -> Texture {
    match path_to_str(file_name.as_ref()) {
        Some(file_name) => LoadTexture(file_name),
        None => LoadTextureFromImage(&Image::default()),
    }
}

pub fn draw_texture(texture: &Texture, pos_x: i32, pos_y: i32, tint: Color) {
    DrawTexture(texture, pos_x, pos_y, tint)
}

pub fn draw_texture_v(texture: &Texture, pos: Vector2, tint: Color) {
    DrawTextureV(texture, pos, tint)
}

pub fn draw_texture_ex(
    texture: &Texture,
    position: Vector2,
    rotation: f32,
    scale: f32,
    tint: Color,
) {
    DrawTextureEx(texture, position, rotation, scale, tint)
}

pub fn draw_texture_rec(texture: &Texture, source: Rectangle, position: Vector2, tint: Color) {
    DrawTextureRec(texture, source, position, tint)
}

pub fn draw_texture_pro(
    texture: &Texture,
    source: Rectangle,
    dest: Rectangle,
    origin: Vector2,
    rotation: f32,
    tint: Color,
) {
    DrawTexturePro(texture, source, dest, origin, rotation, tint)
}

pub fn draw_texture_n_patch(
    texture: &Texture,
    n_patch_info: NPatchInfo,
    dest: Rectangle,
    origin: Vector2,
    rotation: f32,
    tint: Color,
) {
    unsafe { DrawTextureNPatch(texture, n_patch_info, dest, origin, rotation, tint) }
}

pub fn init_missing_texture() {
    unsafe { crate::rtextures::init_missing_texture() }
}
