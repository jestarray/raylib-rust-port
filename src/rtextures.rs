#![allow(unused_variables)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(clippy::missing_safety_doc, unused_parens, static_mut_refs)]
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
use crate::rcore::LoadFileData;
use crate::rlgl::{RL_QUADS, rlBegin, rlColor4ub, rlEnd, rlLoadTexture, rlPopMatrix, rlPushMatrix, rlRotatef, rlSetTexture, rlTexCoord2f, rlTranslatef, rlUnloadTexture, rlVertex2f};
use crate::types::{Color, Image, NPatchInfo, NPatchLayout, PixelFormat, Rectangle, Texture, Vector2};
use std::path::Path;
use image::ImageReader;
use log::{warn};
use crate::types::PixelFormat::*;

#[allow(non_snake_case)]
pub fn GetPixelDataSize(width: i32, height: i32, format: i32) -> i32
{
    let mut dataSize = 0;       // Size in bytes
    let mut bpp = 0;            // Bits per pixel

    match format
    {
        f if f == PIXELFORMAT_UNCOMPRESSED_GRAYSCALE as i32 => bpp = 8,
        f if f == PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA as i32 ||
             f == PIXELFORMAT_UNCOMPRESSED_R5G6B5 as i32 ||
             f == PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 as i32 ||
             f == PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 as i32 => bpp = 16,
        f if f == PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32 => bpp = 32,
        f if f == PIXELFORMAT_UNCOMPRESSED_R8G8B8 as i32 => bpp = 24,
        f if f == PIXELFORMAT_UNCOMPRESSED_R32 as i32 => bpp = 32,
        f if f == PIXELFORMAT_UNCOMPRESSED_R32G32B32 as i32 => bpp = 32*3,
        f if f == PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 as i32 => bpp = 32*4,
        f if f == PIXELFORMAT_UNCOMPRESSED_R16 as i32 => bpp = 16,
        f if f == PIXELFORMAT_UNCOMPRESSED_R16G16B16 as i32 => bpp = 16*3,
        f if f == PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 as i32 => bpp = 16*4,
        f if f == PIXELFORMAT_COMPRESSED_DXT1_RGB as i32 ||
             f == PIXELFORMAT_COMPRESSED_DXT1_RGBA as i32 ||
             f == PIXELFORMAT_COMPRESSED_ETC1_RGB as i32 ||
             f == PIXELFORMAT_COMPRESSED_ETC2_RGB as i32 ||
             f == PIXELFORMAT_COMPRESSED_PVRT_RGB as i32 ||
             f == PIXELFORMAT_COMPRESSED_PVRT_RGBA as i32 => bpp = 4,
        f if f == PIXELFORMAT_COMPRESSED_DXT3_RGBA as i32 ||
             f == PIXELFORMAT_COMPRESSED_DXT5_RGBA as i32 ||
             f == PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA as i32 ||
             f == PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA as i32 => bpp = 8,
        f if f == PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA as i32 => bpp = 2,
        _ => {}
    }

    let dataSizeBytes = (width as u64).wrapping_mul(height as u64).wrapping_mul(bpp as u64) >> 3;  // Get size in bytes (dividing by 8)

    if dataSizeBytes < i32::MAX as u64
    {
        dataSize = dataSizeBytes as i32;

        // Most compressed formats works on 4x4 blocks,
        // if texture is smaller, minimum dataSize is 8 or 16
        if (width < 4) && (height < 4)
        {
            if (format >= PIXELFORMAT_COMPRESSED_DXT1_RGB as i32) && (format < PIXELFORMAT_COMPRESSED_DXT3_RGBA as i32) { dataSize = 8; }
            else if (format >= PIXELFORMAT_COMPRESSED_DXT3_RGBA as i32) && (format < PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA as i32) { dataSize = 16; }
        }
    }

    // NOTE: In case required image data larger than 2GB, no memory allocated at all (NULL)
    if dataSize == 0 { eprintln!("Requested image size is larger than 2GB, it can not be allocated"); }

    return dataSize;
}

/// Create an independently owned image from a rectangle in another image.
/// Release the result with [`UnloadImage()`].
///
/// # Safety
/// For a supported format, `image.data` must point to readable pixel storage
/// covering `image.width * image.height` pixels for the duration of this call.
#[allow(non_snake_case, unused_parens)]
pub unsafe fn ImageFromImage(image: &Image, rec: Rectangle) -> Image
{
    let mut result = Image::default();

    // Security check to avoid program crash
    if (image.is_data_null() || (image.width == 0) || (image.height == 0)) { return result; }

    if (image.format < PIXELFORMAT_COMPRESSED_DXT1_RGB as i32)
    {
        // Basic rectangle validation: size smaller than image size
        if ((rec.x >= 0.0) && (rec.y >= 0.0) && (rec.width > 0.0) && (rec.height > 0.0) &&
            ((rec.x as i32 + rec.width as i32) <= image.width) &&
            ((rec.y as i32 + rec.height as i32) <= image.height))
        {
            let bytesPerPixel = GetPixelDataSize(1, 1, image.format) as usize;

            result.width = rec.width as i32;
            result.height = rec.height as i32;
            result.data = vec![0; (rec.width *rec.height *bytesPerPixel as f32) as usize];
            result.format = image.format;
            result.mipmaps = 1;

            for y in 0..rec.height as i32
            {
                unsafe {
                    let src = &image.data[((y + rec.y as i32)*image.width + rec.x as i32) as usize*bytesPerPixel];
                    let res = &mut result.data[(y*rec.width as i32) as usize*bytesPerPixel];
                    std::ptr::copy_nonoverlapping(
                        src,
                        res,
                        rec.width as i32 as usize*bytesPerPixel);
                }
            }
        }
        else { warn!("IMAGE: ImageToImage(), rectangle provided not valid"); }
    }
    else { warn!("IMAGE: Image manipulation not supported for compressed formats"); }

    return result;
}

/// Copy an uncompressed image into RGBA colors. Drop the vector to release it.
/// Invalid dimensions, null data, and unsupported formats return an empty vector.
///
/// # Safety
/// `image.data` must point to readable storage for the specified dimensions and
/// pixel format for the duration of this call.
pub unsafe fn LoadImageColors(image: &Image) -> Vec<Color> {
    if image.is_data_null() || image.width <= 0 || image.height <= 0 {
        return Vec::new();
    }

    let stride = match image.format {
        f if f == PIXELFORMAT_UNCOMPRESSED_GRAYSCALE as i32 => 1,
        f if f == PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA as i32
            || f == PIXELFORMAT_UNCOMPRESSED_R5G6B5 as i32
            || f == PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 as i32
            || f == PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 as i32
            || f == PIXELFORMAT_UNCOMPRESSED_R16 as i32 => 2,
        f if f == PIXELFORMAT_UNCOMPRESSED_R8G8B8 as i32 => 3,
        f if f == PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32
            || f == PIXELFORMAT_UNCOMPRESSED_R32 as i32 => 4,
        f if f == PIXELFORMAT_UNCOMPRESSED_R16G16B16 as i32 => 6,
        f if f == PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 as i32 => 8,
        f if f == PIXELFORMAT_UNCOMPRESSED_R32G32B32 as i32 => 12,
        f if f == PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 as i32 => 16,
        _ => {
            warn!("IMAGE: Pixel data retrieval not supported for this image format");
            return Vec::new();
        }
    };
    let Some(pixel_count) = (image.width as usize).checked_mul(image.height as usize) else {
        return Vec::new();
    };
    let Some(byte_count) = pixel_count.checked_mul(stride) else {
        return Vec::new();
    };
    if byte_count > isize::MAX as usize || pixel_count > isize::MAX as usize / std::mem::size_of::<Color>() {
        return Vec::new();
    }

    // Decode half floats without requiring aligned input or a separate dependency.
    let half_to_float = |bits: u16| -> f32 {
        let sign = ((bits & 0x8000) as u32) << 16;
        let exponent = (bits >> 10) & 0x1f;
        let mantissa = (bits & 0x03ff) as u32;
        match exponent {
            0 => {
                let value = mantissa as f32 * (1.0 / 16_777_216.0);
                if sign == 0 { value } else { -value }
            }
            31 => f32::from_bits(sign | 0x7f800000 | (mantissa << 13)),
            _ => f32::from_bits(sign | (((exponent as u32) + 112) << 23) | (mantissa << 13)),
        }
    };

    let bytes = std::slice::from_raw_parts(image.data.as_ptr(), byte_count);
    let mut colors = Vec::with_capacity(pixel_count);
    for pixel in bytes.chunks_exact(stride) {
        let color = match image.format {
            f if f == PIXELFORMAT_UNCOMPRESSED_GRAYSCALE as i32 => Color::new(pixel[0], pixel[0], pixel[0], 255),
            f if f == PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA as i32 => Color::new(pixel[0], pixel[0], pixel[0], pixel[1]),
            f if f == PIXELFORMAT_UNCOMPRESSED_R8G8B8 as i32 => Color::new(pixel[0], pixel[1], pixel[2], 255),
            f if f == PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32 => Color::new(pixel[0], pixel[1], pixel[2], pixel[3]),
            f if f == PIXELFORMAT_UNCOMPRESSED_R5G6B5 as i32 => {
                let value = u16::from_ne_bytes([pixel[0], pixel[1]]);
                // Preserve the C helper's integer channel scaling.
                Color::new(((value >> 11) * (255 / 31)) as u8,
                    (((value >> 5) & 63) * (255 / 63)) as u8, ((value & 31) * (255 / 31)) as u8, 255)
            }
            f if f == PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 as i32 => {
                let value = u16::from_ne_bytes([pixel[0], pixel[1]]);
                Color::new(((value >> 11) * (255 / 31)) as u8,
                    (((value >> 6) & 31) * (255 / 31)) as u8,
                    (((value >> 1) & 31) * (255 / 31)) as u8, ((value & 1) * 255) as u8)
            }
            f if f == PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 as i32 => {
                let value = u16::from_ne_bytes([pixel[0], pixel[1]]);
                Color::new(((value >> 12) * 17) as u8, (((value >> 8) & 15) * 17) as u8,
                    (((value >> 4) & 15) * 17) as u8, ((value & 15) * 17) as u8)
            }
            _ => {
                let mut channels = [0, 0, 0, 255];
                if image.format >= PIXELFORMAT_UNCOMPRESSED_R16 as i32 {
                    for (channel, bytes) in channels.iter_mut().zip(pixel.chunks_exact(2)) {
                        *channel = (half_to_float(u16::from_ne_bytes([bytes[0], bytes[1]])) * 255.0) as u8;
                    }
                } else {
                    for (channel, bytes) in channels.iter_mut().zip(pixel.chunks_exact(4)) {
                        *channel = (f32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) * 255.0) as u8;
                    }
                }
                Color::new(channels[0], channels[1], channels[2], channels[3])
            }
        };
        colors.push(color);
    }
    colors
}

pub fn LoadImage<P: AsRef<Path>>(file_path: P) -> Image {
    let path = file_path.as_ref();
    let Some(ext) = path.extension() else {
        warn!("Bad filename, could not parse extension");
        return Image::default();
    };
    let try_load = LoadFileData(&file_path);
    let Ok(file_data) = try_load else { 
        if let Err(err) = try_load {
            warn!("{}! Could not LoadImage", err);
        }
        return Image::default();
    };
    let data_size = file_data.len() as i32;
    let image = LoadImageFromMemory(ext.to_str().unwrap(), &file_data, data_size);
    return image;
}

pub fn LoadImageFromMemory(file_ext: &str, file_data: &[u8], data_size: i32) -> Image {
    let mut res = Image::default();
    use std::io::Cursor;

    let reader = ImageReader::new(Cursor::new(file_data))
    .with_guessed_format()
    .expect("Cursor io never fails");
    let try_decode = reader.decode();
    if let Err(err) = try_decode {
        warn!("Failed to decode image: {}", err);
        return res;
    }
    let decode = try_decode.unwrap(); // safe because above
    let width = decode.width() as i32;
    let height = decode.height() as i32;
    let color = decode.color();
    let pixels = decode.into_bytes();
    res.data = pixels;
    res.width = width;
    res.height = height;
    res.mipmaps = 1;
    if let Some(format) = PixelFormat::from_color_type(color) {
        res.format = format as i32;
    } else {
        warn!("Could not map PixelFormat for {}!", file_ext);
    }
    return res;
}

// Check if an image is ready
pub fn IsImageValid(image: &Image) -> bool
{
    let mut result = false;

    result =
        ((!image.is_data_null()) &&     // Validate pixel data available
        (image.width > 0) &&        // Validate image width
        (image.height > 0) &&       // Validate image height
        (image.format > 0) &&       // Validate image format
        (image.mipmaps > 0)); // Validate image mipmaps (at least 1 for basic mipmap level)

    return result;
}

pub fn UnloadImage(image: &mut Image) {
    unsafe {
        if !image.is_data_null() {
            image.data = Vec::new();
        }
    }
}

pub fn ExportImage(image: &Image, path: &str) {
    use image::*;
    let width = image.width as u32;
    let height= image.height as u32;
    let try_buffer: Option<ImageBuffer<Rgba<u8>, &[u8]>> = 
        ImageBuffer::from_raw(width, height, image.data.as_slice());
    if try_buffer.is_none() {
        warn!("Could not create buffer?");
    }
    let buffer = try_buffer.unwrap();
    if let Err(err) = buffer.save(path) {
        warn!("Could not write image {}", err);
    }
}

pub fn LoadTexture(file_name: &str) -> Texture {
    let mut image = LoadImage(file_name);
    let texture = LoadTextureFromImage(&image);
    UnloadImage(&mut image);
    texture
}

pub fn LoadTextureFromImage(image: &Image) -> Texture {
    if image.is_data_null() || image.width <= 0 || image.height <= 0 {
        warn!("IMAGE: Data is not valid to load texture");
        let width = if image.width > 0 { image.width } else { unsafe { MISSING_TEXTURE.width } };
        let height = if image.height > 0 { image.width } else { unsafe { MISSING_TEXTURE.height } };
        return unsafe { Texture { id: MISSING_TEXTURE.id, width, height, mipmaps: 1, format: 0 } };
    }
    unsafe {
        let id = rlLoadTexture(
            &image.data,
            image.width,
            image.height,
            image.format,
            image.mipmaps,
        );

        Texture {
            id,
            width: image.width,
            height: image.height,
            mipmaps: image.mipmaps,
            format: image.format,
        }
    }
}

pub fn UnloadTexture(texture: &mut Texture) {
    unsafe {
        rlUnloadTexture(texture.id);
        texture.id = 0;
    }
}

// Draw a texture
pub fn DrawTexture(texture: &Texture, pos_x: i32, pos_y: i32, tint: Color) {
    DrawTextureEx(texture, Vector2::new(pos_x as f32, pos_y as f32), 0.0, 1.0, tint);
}

// Draw a texture with position defined as Vector2
pub fn DrawTextureV(texture: &Texture, pos: Vector2,  tint: Color) {
    DrawTextureEx(texture, pos, 0.0, 1.0, tint);
}

// Draw a texture with rotation and scale
pub fn DrawTextureEx(texture: &Texture, position: Vector2, rotation: f32, scale: f32, tint: Color) {
    let source = Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32);
    let dest = Rectangle::new(
        position.x,
        position.y,
        texture.width as f32 * scale,
        texture.height as f32 * scale,
    );
    let origin = Vector2::new(0.0, 0.0);
    DrawTexturePro(texture, source, dest, origin, rotation, tint);
}

// Draw a part of a texture (defined by a rectangle)
pub fn DrawTextureRec(texture: &Texture, source: Rectangle, position: Vector2, tint: Color) {
    let dest = Rectangle::new(position.x, position.y, source.width.abs(), source.height.abs());
    let origin = Vector2::new(0.0, 0.0);
    DrawTexturePro(texture, source, dest, origin, 0.0, tint);
}

// Draw a part of a texture (defined by a rectangle) with 'pro' parameters
// NOTE: origin is relative to destination rectangle size
pub fn DrawTexturePro(
    texture: &Texture,
    source: Rectangle,
    dest: Rectangle,
    origin: Vector2,
    rotation: f32,
    tint: Color,
) {
    if texture.id == 0 {
        return;
    }

    let width = texture.width as f32;
    let height = texture.height as f32;

    let mut flip_x = false;
    let mut flip_y = false;
    let mut src = source;

    if source.width < 0.0 {
        flip_x = true;
        src.width *= -1.0;
    }
    if source.height < 0.0 {
        flip_y = true;
        src.height *= -1.0;
    }

    let mut top_left = Vector2::new(0.0, 0.0);
    let mut top_right = Vector2::new(0.0, 0.0);
    let mut bottom_left = Vector2::new(0.0, 0.0);
    let mut bottom_right = Vector2::new(0.0, 0.0);

    if rotation == 0.0 {
        let x = dest.x - origin.x;
        let y = dest.y - origin.y;
        top_left = Vector2::new(x, y);
        top_right = Vector2::new(x + dest.width, y);
        bottom_left = Vector2::new(x, y + dest.height);
        bottom_right = Vector2::new(x + dest.width, y + dest.height);
    } else {
        let sin = (rotation * crate::math::DEG2RAD).sin();
        let cos = (rotation * crate::math::DEG2RAD).cos();

        let dx = -origin.x;
        let dy = -origin.y;

        top_left.x = dest.x + dx * cos - dy * sin;
        top_left.y = dest.y + dx * sin + dy * cos;

        top_right.x = dest.x + (dx + dest.width) * cos - dy * sin;
        top_right.y = dest.y + (dx + dest.width) * sin + dy * cos;

        bottom_left.x = dest.x + dx * cos - (dy + dest.height) * sin;
        bottom_left.y = dest.y + dx * sin + (dy + dest.height) * cos;

        bottom_right.x = dest.x + (dx + dest.width) * cos - (dy + dest.height) * sin;
        bottom_right.y = dest.y + (dx + dest.width) * sin + (dy + dest.height) * cos;
    }

    unsafe {
        rlSetTexture(texture.id);
        rlBegin(RL_QUADS);
        rlColor4ub(tint.r, tint.g, tint.b, tint.a);

        // Bottom-left corner
        rlTexCoord2f(
            if flip_x { (src.x + src.width) / width } else { src.x / width },
            if flip_y { src.y / height } else { (src.y + src.height) / height },
        );
        rlVertex2f(bottom_left.x, bottom_left.y);

        // Bottom-right corner
        rlTexCoord2f(
            if flip_x { src.x / width } else { (src.x + src.width) / width },
            if flip_y { src.y / height } else { (src.y + src.height) / height },
        );
        rlVertex2f(bottom_right.x, bottom_right.y);

        // Top-right corner
        rlTexCoord2f(
            if flip_x { src.x / width } else { (src.x + src.width) / width },
            if flip_y { (src.y + src.height) / height } else { src.y / height },
        );
        rlVertex2f(top_right.x, top_right.y);

        // Top-left corner
        rlTexCoord2f(
            if flip_x { (src.x + src.width) / width } else { src.x / width },
            if flip_y { (src.y + src.height) / height } else { src.y / height },
        );
        rlVertex2f(top_left.x, top_left.y);

        rlEnd();
    }
}

pub fn DrawTextureNPatch(
    texture: &Texture,
    n_patch_info: NPatchInfo,
    dest: Rectangle,
    origin: Vector2,
    rotation: f32,
    tint: Color,
) {
    if texture.id > 0 {
        let width = texture.width as f32;
        let height = texture.height as f32;

        let mut patch_width = if dest.width <= 0.0 { 0.0 } else { dest.width };
        let mut patch_height = if dest.height <= 0.0 { 0.0 } else { dest.height };

        let mut source = n_patch_info.source;
        if source.width < 0.0 {
            source.x -= source.width;
        }
        if source.height < 0.0 {
            source.y -= source.height;
        }

        if n_patch_info.layout == NPatchLayout::ThreePatchHorizontal as i32 {
            patch_height = source.height;
        }
        if n_patch_info.layout == NPatchLayout::ThreePatchVertical as i32 {
            patch_width = source.width;
        }

        let mut draw_center = true;
        let mut draw_middle = true;
        let mut left_border = n_patch_info.left as f32;
        let mut top_border = n_patch_info.top as f32;
        let mut right_border = n_patch_info.right as f32;
        let mut bottom_border = n_patch_info.bottom as f32;

        // Adjust lateral border widths
        if patch_width <= (left_border + right_border)
            && n_patch_info.layout != NPatchLayout::ThreePatchVertical as i32
        {
            draw_center = false;
            left_border = (left_border / (left_border + right_border)) * patch_width;
            right_border = patch_width - left_border;
        }

        // Adjust lateral border heights
        if patch_height <= (top_border + bottom_border)
            && n_patch_info.layout != NPatchLayout::ThreePatchHorizontal as i32
        {
            draw_middle = false;
            top_border = (top_border / (top_border + bottom_border)) * patch_height;
            bottom_border = patch_height - top_border;
        }

        let vert_a = Vector2::new(0.0, 0.0);
        let vert_b = Vector2::new(left_border, top_border);
        let vert_c = Vector2::new(patch_width - right_border, patch_height - bottom_border);
        let vert_d = Vector2::new(patch_width, patch_height);

        let coord_a = Vector2::new(source.x / width, source.y / height);
        let coord_b = Vector2::new((source.x + left_border) / width, (source.y + top_border) / height);
        let coord_c = Vector2::new(
            (source.x + source.width - right_border) / width,
            (source.y + source.height - bottom_border) / height,
        );
        let coord_d = Vector2::new((source.x + source.width) / width, (source.y + source.height) / height);

        unsafe {
            rlSetTexture(texture.id);
            rlPushMatrix();
            rlTranslatef(dest.x, dest.y, 0.0);
            rlRotatef(rotation, 0.0, 0.0, 1.0);
            rlTranslatef(-origin.x, -origin.y, 0.0);

            rlBegin(RL_QUADS);
            rlColor4ub(tint.r, tint.g, tint.b, tint.a);

            if n_patch_info.layout == NPatchLayout::NinePatch as i32 {
                // TOP-LEFT QUAD
                rlTexCoord2f(coord_a.x, coord_b.y); rlVertex2f(vert_a.x, vert_b.y);
                rlTexCoord2f(coord_b.x, coord_b.y); rlVertex2f(vert_b.x, vert_b.y);
                rlTexCoord2f(coord_b.x, coord_a.y); rlVertex2f(vert_b.x, vert_a.y);
                rlTexCoord2f(coord_a.x, coord_a.y); rlVertex2f(vert_a.x, vert_a.y);

                if draw_center {
                    // TOP-CENTER QUAD
                    rlTexCoord2f(coord_b.x, coord_b.y); rlVertex2f(vert_b.x, vert_b.y);
                    rlTexCoord2f(coord_c.x, coord_b.y); rlVertex2f(vert_c.x, vert_b.y);
                    rlTexCoord2f(coord_c.x, coord_a.y); rlVertex2f(vert_c.x, vert_a.y);
                    rlTexCoord2f(coord_b.x, coord_a.y); rlVertex2f(vert_b.x, vert_a.y);
                }

                // TOP-RIGHT QUAD
                rlTexCoord2f(coord_c.x, coord_b.y); rlVertex2f(vert_c.x, vert_b.y);
                rlTexCoord2f(coord_d.x, coord_b.y); rlVertex2f(vert_d.x, vert_b.y);
                rlTexCoord2f(coord_d.x, coord_a.y); rlVertex2f(vert_d.x, vert_a.y);
                rlTexCoord2f(coord_c.x, coord_a.y); rlVertex2f(vert_c.x, vert_a.y);

                if draw_middle {
                    // MIDDLE-LEFT QUAD
                    rlTexCoord2f(coord_a.x, coord_c.y); rlVertex2f(vert_a.x, vert_c.y);
                    rlTexCoord2f(coord_b.x, coord_c.y); rlVertex2f(vert_b.x, vert_c.y);
                    rlTexCoord2f(coord_b.x, coord_b.y); rlVertex2f(vert_b.x, vert_b.y);
                    rlTexCoord2f(coord_a.x, coord_b.y); rlVertex2f(vert_a.x, vert_b.y);

                    if draw_center {
                        // MIDDLE-CENTER QUAD
                        rlTexCoord2f(coord_b.x, coord_c.y); rlVertex2f(vert_b.x, vert_c.y);
                        rlTexCoord2f(coord_c.x, coord_c.y); rlVertex2f(vert_c.x, vert_c.y);
                        rlTexCoord2f(coord_c.x, coord_b.y); rlVertex2f(vert_c.x, vert_b.y);
                        rlTexCoord2f(coord_b.x, coord_b.y); rlVertex2f(vert_b.x, vert_b.y);
                    }

                    // MIDDLE-RIGHT QUAD
                    rlTexCoord2f(coord_c.x, coord_c.y); rlVertex2f(vert_c.x, vert_c.y);
                    rlTexCoord2f(coord_d.x, coord_c.y); rlVertex2f(vert_d.x, vert_c.y);
                    rlTexCoord2f(coord_d.x, coord_b.y); rlVertex2f(vert_d.x, vert_b.y);
                    rlTexCoord2f(coord_c.x, coord_b.y); rlVertex2f(vert_c.x, vert_b.y);
                }

                // BOTTOM-LEFT QUAD
                rlTexCoord2f(coord_a.x, coord_d.y); rlVertex2f(vert_a.x, vert_d.y);
                rlTexCoord2f(coord_b.x, coord_d.y); rlVertex2f(vert_b.x, vert_d.y);
                rlTexCoord2f(coord_b.x, coord_c.y); rlVertex2f(vert_b.x, vert_c.y);
                rlTexCoord2f(coord_a.x, coord_c.y); rlVertex2f(vert_a.x, vert_c.y);

                if draw_center {
                    // BOTTOM-CENTER QUAD
                    rlTexCoord2f(coord_b.x, coord_d.y); rlVertex2f(vert_b.x, vert_d.y);
                    rlTexCoord2f(coord_c.x, coord_d.y); rlVertex2f(vert_c.x, vert_d.y);
                    rlTexCoord2f(coord_c.x, coord_c.y); rlVertex2f(vert_c.x, vert_c.y);
                    rlTexCoord2f(coord_b.x, coord_c.y); rlVertex2f(vert_b.x, vert_c.y);
                }

                // BOTTOM-RIGHT QUAD
                rlTexCoord2f(coord_c.x, coord_d.y); rlVertex2f(vert_c.x, vert_d.y);
                rlTexCoord2f(coord_d.x, coord_d.y); rlVertex2f(vert_d.x, vert_d.y);
                rlTexCoord2f(coord_d.x, coord_c.y); rlVertex2f(vert_d.x, vert_c.y);
                rlTexCoord2f(coord_c.x, coord_c.y); rlVertex2f(vert_c.x, vert_c.y);
            } else if n_patch_info.layout == NPatchLayout::ThreePatchVertical as i32 {
                // TOP QUAD
                rlTexCoord2f(coord_a.x, coord_b.y); rlVertex2f(vert_a.x, vert_b.y);
                rlTexCoord2f(coord_d.x, coord_b.y); rlVertex2f(vert_d.x, vert_b.y);
                rlTexCoord2f(coord_d.x, coord_a.y); rlVertex2f(vert_d.x, vert_a.y);
                rlTexCoord2f(coord_a.x, coord_a.y); rlVertex2f(vert_a.x, vert_a.y);

                if draw_center {
                    // MIDDLE QUAD
                    rlTexCoord2f(coord_a.x, coord_c.y); rlVertex2f(vert_a.x, vert_c.y);
                    rlTexCoord2f(coord_d.x, coord_c.y); rlVertex2f(vert_d.x, vert_c.y);
                    rlTexCoord2f(coord_d.x, coord_b.y); rlVertex2f(vert_d.x, vert_b.y);
                    rlTexCoord2f(coord_a.x, coord_b.y); rlVertex2f(vert_a.x, vert_b.y);
                }

                // BOTTOM QUAD
                rlTexCoord2f(coord_a.x, coord_d.y); rlVertex2f(vert_a.x, vert_d.y);
                rlTexCoord2f(coord_d.x, coord_d.y); rlVertex2f(vert_d.x, vert_d.y);
                rlTexCoord2f(coord_d.x, coord_c.y); rlVertex2f(vert_d.x, vert_c.y);
                rlTexCoord2f(coord_a.x, coord_c.y); rlVertex2f(vert_a.x, vert_c.y);
            } else if n_patch_info.layout == NPatchLayout::ThreePatchHorizontal as i32 {
                // LEFT QUAD
                rlTexCoord2f(coord_a.x, coord_d.y); rlVertex2f(vert_a.x, vert_d.y);
                rlTexCoord2f(coord_b.x, coord_d.y); rlVertex2f(vert_b.x, vert_d.y);
                rlTexCoord2f(coord_b.x, coord_a.y); rlVertex2f(vert_b.x, vert_a.y);
                rlTexCoord2f(coord_a.x, coord_a.y); rlVertex2f(vert_a.x, vert_a.y);

                if draw_center {
                    // CENTER QUAD
                    rlTexCoord2f(coord_b.x, coord_d.y); rlVertex2f(vert_b.x, vert_d.y);
                    rlTexCoord2f(coord_c.x, coord_d.y); rlVertex2f(vert_c.x, vert_d.y);
                    rlTexCoord2f(coord_c.x, coord_a.y); rlVertex2f(vert_c.x, vert_a.y);
                    rlTexCoord2f(coord_b.x, coord_a.y); rlVertex2f(vert_b.x, vert_a.y);
                }

                // RIGHT QUAD
                rlTexCoord2f(coord_c.x, coord_d.y); rlVertex2f(vert_c.x, vert_d.y);
                rlTexCoord2f(coord_d.x, coord_d.y); rlVertex2f(vert_d.x, vert_d.y);
                rlTexCoord2f(coord_d.x, coord_a.y); rlVertex2f(vert_d.x, vert_a.y);
                rlTexCoord2f(coord_c.x, coord_a.y); rlVertex2f(vert_c.x, vert_a.y);
            }

            rlEnd();
            rlPopMatrix();
        }
    }
}

// Get color with alpha applied, alpha goes from 0.0 to 1.0
pub fn Fade(color: Color, alpha: f32) -> Color {
    let mut result = color;

    // Clamp alpha between 0.0 and 1.0
    let alpha = alpha.clamp(0.0, 1.0);

    result.a = (255.0 * alpha) as u8;

    result
}

pub static mut MISSING_TEXTURE: Texture = Texture { id: 0, width: 0, height: 0, mipmaps: 0, format: 0 };
/// Creates a 2x2 magenta/black checkerboard fallback texture.
/// 
/// # Safety
/// Calls unsafe OpenGL FFI functions. An active OpenGL context must be bound on the calling thread.
pub unsafe fn init_missing_texture() {
    let magenta: [u8; 4] = [255, 0, 255, 255];
    let black: [u8; 4]   = [0, 0, 0, 255];

    // 2x2 pixel grid (RGBA):
    // [ Magenta, Black   ]
    // [ Black,   Magenta ]
    let mut pixels = [0u8; 2 * 2 * 4];
    pixels[0..4].copy_from_slice(&magenta);
    pixels[4..8].copy_from_slice(&black);
    pixels[8..12].copy_from_slice(&black);
    pixels[12..16].copy_from_slice(&magenta);

    let mut texture_id: gl::types::GLuint = 0;
    gl::GenTextures(1, &mut texture_id);
    gl::BindTexture(gl::TEXTURE_2D, texture_id);

    // Upload 2x2 RGBA8 pixel data to GPU
    gl::TexImage2D(
        gl::TEXTURE_2D,
        0, // Base mipmap level
        gl::RGBA8 as gl::types::GLint,
        2, // Width
        2, // Height
        0, // Border (must be 0)
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        pixels.as_ptr() as *const std::ffi::c_void,
    );

    // GL_NEAREST keeps the checkerboard grid edges crisp without blur
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

    // GL_REPEAT lets UVs scale and tile the pattern seamlessly
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

    // Unbind texture
    gl::BindTexture(gl::TEXTURE_2D, 0);

    MISSING_TEXTURE = Texture { id: texture_id, width: 32, height: 32, format: 0, mipmaps: 1 };
}