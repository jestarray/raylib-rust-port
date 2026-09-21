//! Safe wrappers for the raw `rtextures` bindings.
//!
//! Every public function of `rtextures` is mirrored here with a `snake_case` name and a
//! signature that can be called without `unsafe`. Path-like parameters are generic over
//! [`AsRef<Path>`](AsRef), redundant slice lengths are derived from `slice::len()`, and
//! `i32` format parameters use the enums from [`crate::types`].
//!
//! The raw `rtextures` module is private, so this module is the only way to reach image and
//! texture functionality from outside the crate.

use std::path::Path;

use crate::rtextures::*;
use crate::safe::core::path_to_str;
use crate::types::{Color, Image, NPatchInfo, PixelFormat, Rectangle, Texture, Vector2};

pub fn get_pixel_data_size(width: i32, height: i32, format: PixelFormat) -> i32 {
    GetPixelDataSize(width, height, format as i32)
}

pub fn image_from_image(image: &Image, rec: Rectangle) -> Image {
    unsafe { ImageFromImage(image, rec) }
}

pub fn load_image_colors(image: &Image) -> Vec<Color> {
    unsafe { LoadImageColors(image) }
}

pub fn load_image<P: AsRef<Path>>(file_path: P) -> Image {
    LoadImage(file_path)
}

/// The data size is derived from `file_data.len()`.
pub fn load_image_from_memory(file_ext: &str, file_data: &[u8]) -> Image {
    LoadImageFromMemory(file_ext, file_data, file_data.len() as i32)
}

pub fn is_image_valid(image: &Image) -> bool {
    IsImageValid(image)
}

pub fn unload_image(image: &mut Image) {
    UnloadImage(image)
}

pub fn export_image<P: AsRef<Path>>(image: &Image, path: P) {
    let path = path.as_ref();
    if let Some(path) = path_to_str(path) {
        ExportImage(image, path);
    }
}

/// Falls back to the "missing texture" checkerboard when the path is not valid UTF-8.
pub fn load_texture<P: AsRef<Path>>(file_name: P) -> Texture {
    match path_to_str(file_name.as_ref()) {
        Some(file_name) => LoadTexture(file_name),
        None => LoadTextureFromImage(&Image::default()),
    }
}

pub fn load_texture_from_image(image: &Image) -> Texture {
    LoadTextureFromImage(image)
}

pub fn unload_texture(texture: &mut Texture) {
    UnloadTexture(texture)
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
    DrawTextureNPatch(texture, n_patch_info, dest, origin, rotation, tint)
}

pub fn fade(color: Color, alpha: f32) -> Color {
    Fade(color, alpha)
}

pub fn init_missing_texture() {
    unsafe { crate::rtextures::init_missing_texture() }
}
