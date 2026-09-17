#![allow(unused_variables)]
#![allow(unsafe_op_in_unsafe_fn)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens, non_snake_case, static_mut_refs)]
#![allow(
    clippy::too_many_arguments,
    clippy::needless_return,
    clippy::manual_range_contains,
    clippy::field_reassign_with_default,
    clippy::manual_map,
    clippy::match_like_matches_macro,
    clippy::upper_case_acronyms,
    clippy::let_and_return,
    clippy::double_parens
)]
use log::{info, warn};

use crate::rcore::LoadFileData;
use crate::rlgl::{self, rlPopMatrix, rlPushMatrix, rlRotatef, rlTranslatef};
use crate::rtextures::{self, DrawTexturePro, ImageFromImage};
use crate::types::{Color, Font, FontType, GlyphInfo, Image, PixelFormat, Rectangle, Texture, Vector2};
/// Load a font into GPU memory. TTF/OTF fonts use a 32-pixel height and the
/// default 95 codepoints; image fonts use magenta borders and start at codepoint 32.
///
/// # Safety
/// TTF/OTF files and any configured file-loading callback must satisfy the
/// requirements of `LoadFontEx`. Call on the rendering thread with an initialized
/// graphics context.
pub unsafe fn LoadFont(fileName: &str) -> Font {
    const FONT_TTF_DEFAULT_SIZE: i32 = 32;
    const FONT_TTF_DEFAULT_FIRST_CHAR: i32 = 32;

    let font = if crate::rcore::IsFileExtension(fileName, ".ttf")
        || crate::rcore::IsFileExtension(fileName, ".otf")
    {
        LoadFontEx(fileName, FONT_TTF_DEFAULT_SIZE, None)
    } else {
        let mut image = rtextures::load_image(fileName);
        let font = if !image.data.is_null() {
            LoadFontFromImage(&image, Color::MAGENTA, FONT_TTF_DEFAULT_FIRST_CHAR)
        } else {
            (&*GetFontDefault()).clone()
        };
        rtextures::UnloadImage(&mut image);
        font
    };

    if font.texture.id == 0 {
        warn!("FONT: [{}] Failed to load font texture", fileName);
    } else {
        // Apply the TEXTURE_FILTER_POINT behavior, including existing mipmaps.
        let min_filter = if font.texture.mipmaps > 1 {
            rlgl::RL_TEXTURE_FILTER_MIP_NEAREST
        } else {
            rlgl::RL_TEXTURE_FILTER_NEAREST
        };
        rlgl::rlTextureParameters(font.texture.id, rlgl::RL_TEXTURE_MIN_FILTER as i32, min_filter as i32);
        rlgl::rlTextureParameters(font.texture.id, rlgl::RL_TEXTURE_MAG_FILTER as i32, rlgl::RL_TEXTURE_FILTER_NEAREST as i32);
        info!("FONT: Data loaded successfully ({} pixel size | {} glyphs)", font.baseSize, font.glyphCount);
    }

    font
}

/// Load a font file at the requested pixel height and with the selected codepoints.
/// `None` selects the default character set (32..126).
/// A file-read failure returns an empty font, matching the C implementation.
///
/// # Safety
/// TTF/OTF files must contain complete, valid font data. Any configured file-loading
/// callback must return a readable buffer of its reported length, compatible with
/// `UnloadFileData`. Call on the rendering thread with an initialized graphics context.
pub unsafe fn LoadFontEx(fileName: &str, fontSize: i32, codepoints: Option<&[i32]>) -> Font {
    let mut font = Font {
        baseSize: 0,
        glyphCount: 0,
        glyphPadding: 0,
        texture: Texture { id: 0, width: 0, height: 0, mipmaps: 0, format: 0 },
        recs: Vec::new(),
        glyphs: Vec::new(),
    };

    let file_data = LoadFileData(fileName).unwrap();
    let data_size = file_data.len();
    if data_size > 0 {
        let data = std::slice::from_raw_parts(file_data.as_ptr(), data_size as usize);
        let extension = crate::rcore::GetFileExtension(fileName).unwrap_or("");
            font = LoadFontFromMemory(extension, data, fontSize, codepoints);
    }

    font
}

/// Load a TTF or OTF font from memory. The extension is matched ignoring ASCII case.
/// `None` selects the default 95 codepoints; a slice selects exactly its entries.
/// Unsupported types or missing glyphs return a clone of the default font,
/// sharing its image and texture data.
///
/// # Safety
/// For TTF/OTF input, `fileData` must contain a complete, valid font at offset zero;
/// stb_truetype does not bounds-check font data. Call on the rendering thread
/// with an initialized graphics context.
pub unsafe fn LoadFontFromMemory(
    fileType: &str,
    fileData: &[u8],
    fontSize: i32,
    codepoints: Option<&[i32]>,
) -> Font {
    const FONT_TTF_DEFAULT_CHARS_PADDING: i32 = 4;

    let mut glyphs = if fileType.eq_ignore_ascii_case(".ttf") || fileType.eq_ignore_ascii_case(".otf") {
        LoadFontData(fileData, fontSize, codepoints, FontType::FONT_DEFAULT)
    } else {
        Vec::new()
    };

    if glyphs.is_empty() {
        warn!("FONT: Font type is unsupported or no glyphs were found, reverted to default font");
        return (&*GetFontDefault()).clone();
    }

    let padding = FONT_TTF_DEFAULT_CHARS_PADDING;
    let Some((mut atlas, recs)) = GenImageFontAtlas(&glyphs, fontSize, padding) else {
        for glyph in &mut glyphs {
            rtextures::UnloadImage(&mut glyph.image);
        }
        warn!("FONT: Failed to generate font atlas, reverted to default font");
        return (&*GetFontDefault()).clone();
    };

    let texture = rtextures::LoadTextureFromImage(&atlas);

    // Replace grayscale glyph images with gray-alpha copies for image text drawing.
    for (glyph, &rec) in glyphs.iter_mut().zip(&recs) {
        rtextures::UnloadImage(&mut glyph.image);
        glyph.image = ImageFromImage(atlas, rec);
    }
    rtextures::UnloadImage(&mut atlas);

    info!("FONT: Data loaded successfully ({} pixel size | {} glyphs)", fontSize, glyphs.len());
    Font {
        baseSize: fontSize,
        glyphCount: glyphs.len() as i32,
        glyphPadding: padding,
        texture,
        recs,
        glyphs,
    }
}

// Basic (packMethod == 0) atlas generation used by LoadFontFromMemory.
// Glyph images must contain readable grayscale data. The returned Image owns
// its allocation and must be released with UnloadImage.
unsafe fn GenImageFontAtlas(glyphs: &[GlyphInfo], font_size: i32, padding: i32) -> Option<(Image, Vec<Rectangle>)> {
    if glyphs.is_empty() || font_size <= 0 || padding < 0 {
        return None;
    }

    let padding = padding as usize;
    let double_padding = padding.checked_mul(2)?;
    let padded_font_size = (font_size as usize).checked_add(double_padding)?;
    let mut total_width = 0usize;
    let mut max_glyph_width = 0usize;
    for glyph in glyphs {
        let width = usize::try_from(glyph.image.width).ok()?;
        let height = usize::try_from(glyph.image.height).ok()?;
        if width > 0 && height > 0 && (glyph.image.data.is_null()
            || glyph.image.format != PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAYSCALE as i32)
        {
            return None;
        }
        total_width = total_width.checked_add(width.checked_add(double_padding)?)?;
        max_glyph_width = max_glyph_width.max(width);
    }

    let total_area = total_width.checked_mul(padded_font_size)? as f64 * 1.2;
    let estimated_size = (total_area.sqrt().ceil() as usize).max(1).checked_next_power_of_two()?;
    // Ensure even a single unusually wide glyph fits with the packing margins.
    let minimum_width = max_glyph_width.checked_add(padding.checked_mul(3)?)?.checked_add(1)?;
    let width = estimated_size.max(minimum_width.checked_next_power_of_two()?);
    let mut height = if total_area < (width as f64 * width as f64 / 2.0) {
        (width / 2).max(1)
    } else {
        width
    };

    // Keep image sizes within the i32 indexing used by the image helpers.
    let atlas_byte_count = |height: usize| -> Option<usize> {
        if width > i32::MAX as usize || height > i32::MAX as usize {
            return None;
        }
        let count = width.checked_mul(height)?.checked_mul(2)?;
        if count > i32::MAX as usize { None } else { Some(count) }
    };
    let mut pixels = vec![0u8; atlas_byte_count(height)?];
    let mut recs = Vec::with_capacity(glyphs.len());
    let mut offset_x = padding;
    let mut offset_y = padding;
    let mut row_height = font_size as usize;

    for glyph in glyphs {
        let glyph_width = glyph.image.width as usize;
        let glyph_height = glyph.image.height as usize;
        if offset_x >= width - glyph_width - double_padding {
            offset_x = padding;
            offset_y = offset_y.checked_add(row_height.checked_add(double_padding)?)?;
            row_height = font_size as usize;
        }
        row_height = row_height.max(glyph_height);
        let required_height = offset_y.checked_add(row_height)?.checked_add(padding)?;
        while required_height > height {
            height = height.checked_mul(2)?;
            pixels.resize(atlas_byte_count(height)?, 0);
        }

        if glyph_width > 0 && glyph_height > 0 {
            let glyph_size = glyph_width.checked_mul(glyph_height)?;
            if glyph_size > isize::MAX as usize { return None; }
            let source = std::slice::from_raw_parts(glyph.image.data.cast::<u8>(), glyph_size);
            for y in 0..glyph_height {
                for x in 0..glyph_width {
                    pixels[((offset_y + y) * width + offset_x + x) * 2 + 1] = source[y * glyph_width + x];
                }
            }
        }
        recs.push(Rectangle::new(offset_x as f32, offset_y as f32, glyph_width as f32, glyph_height as f32));
        offset_x += glyph_width + double_padding;
    }

    // Gray is white everywhere; the glyph coverage is stored in alpha.
    for pixel in pixels.chunks_exact_mut(2) {
        pixel[0] = 255;
    }
    // Reserve the white 3x3 atlas corner used for drawing shapes.
    if width >= 3 && height >= 3 {
        for y in height - 3..height {
            for x in width - 3..width {
                pixels[(y * width + x) * 2 + 1] = 255;
            }
        }
    }

    let data = libc::malloc(pixels.len());
    if data.is_null() { return None; }
    std::ptr::copy_nonoverlapping(pixels.as_ptr(), data.cast::<u8>(), pixels.len());
    Some((Image {
        data,
        width: width as i32,
        height: height as i32,
        mipmaps: 1,
        format: PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA as i32,
    }, recs))
}

/// Load an XNA-style image font with key-colored borders between glyphs.
/// The source image is borrowed; the resulting glyph images are independent copies.
/// Invalid input returns a clone of the default font, sharing its image and texture data.
///
/// # Safety
/// `image.data` must cover the image dimensions in its declared pixel format.
/// Call on the rendering thread with an initialized graphics context.
pub unsafe fn LoadFontFromImage(image: &Image, key: Color, firstChar: i32) -> Font {
    const MAX_GLYPHS_FROM_IMAGE: usize = 256;

    let default_font = || (&*GetFontDefault()).clone();
    let mut pixels = rtextures::LoadImageColors(image);
    let Some(first_pixel) = pixels.iter().position(|&pixel| pixel != key) else {
        return default_font();
    };

    let width = image.width as usize;
    let height = image.height as usize;
    let char_spacing = first_pixel % width;
    let line_spacing = first_pixel / width;
    if char_spacing == 0 || line_spacing == 0 {
        return default_font();
    }

    let mut char_height = 0;
    while line_spacing + char_height < height
        && pixels[(line_spacing + char_height) * width + char_spacing] != key
    {
        char_height += 1;
    }

    let mut recs = Vec::with_capacity(MAX_GLYPHS_FROM_IMAGE);
    let mut y = line_spacing;
    while y + char_height <= height && recs.len() < MAX_GLYPHS_FROM_IMAGE {
        let mut x = char_spacing;
        while x < width && pixels[y * width + x] != key && recs.len() < MAX_GLYPHS_FROM_IMAGE {
            let mut char_width = 0;
            while x + char_width < width && pixels[y * width + x + char_width] != key {
                char_width += 1;
            }
            recs.push(Rectangle::new(x as f32, y as f32, char_width as f32, char_height as f32));
            x += char_width + char_spacing;
        }
        y += char_height + line_spacing;
    }

    if recs.is_empty() || firstChar.checked_add(recs.len() as i32 - 1).is_none() {
        return default_font();
    }

    // Clear the key color to prevent borders bleeding into scaled glyphs.
    for pixel in &mut pixels {
        if *pixel == key {
            *pixel = Color::BLANK;
        }
    }
    let font_clear = Image {
        data: pixels.as_mut_ptr().cast(),
        width: image.width,
        height: image.height,
        mipmaps: 1,
        format: PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32,
    };

    let texture = rtextures::LoadTextureFromImage(&font_clear);
    let glyphs = recs.iter().enumerate().map(|(index, &rec)| GlyphInfo {
        value: firstChar + index as i32,
        offset_x: 0,
        offset_y: 0,
        advance_x: 0,
        image: ImageFromImage(font_clear, rec),
    }).collect();

    // `pixels` owns font_clear.data and is dropped after uploading and copying.
    Font {
        baseSize: char_height as i32,
        glyphCount: recs.len() as i32,
        glyphPadding: 0,
        texture,
        recs,
        glyphs,
    }
}

/// Load glyph metrics and grayscale images from TTF data.
/// `None` selects codepoints 32..127; an explicit slice selects exactly its entries.
/// The returned vector's length is the number of glyphs found in the font.
/// Release each glyph's image with `rtextures::UnloadImage` before dropping the vector.
///
/// # Safety
/// `fileData` must contain a complete, valid font at offset zero. stb_truetype
/// does not bounds-check font data against the slice length.
pub unsafe fn LoadFontData(
    fileData: &[u8],
    fontSize: i32,
    codepoints: Option<&[i32]>,
    font_type: FontType,
) -> Vec<GlyphInfo> {
    use crate::external;

    const FONT_SDF_CHAR_PADDING: i32 = 4;
    const FONT_SDF_ON_EDGE_VALUE: u8 = 128;
    const FONT_SDF_PIXEL_DIST_SCALE: f32 = 64.0;
    const FONT_BITMAP_ALPHA_THRESHOLD: u8 = 80;

    if fileData.is_empty() || fontSize <= 0 {
        return Vec::new();
    }

    let default_codepoints: [i32; 95] = std::array::from_fn(|i| i as i32 + 32);
    let required_codepoints = codepoints.unwrap_or(&default_codepoints);
    if required_codepoints.is_empty() {
        return Vec::new();
    }

    let mut font_info: external::stbtt_fontinfo = std::mem::zeroed();
    if external::stbtt_InitFont(&mut font_info, fileData.as_ptr(), 0) == 0 {
        warn!("FONT: Failed to process TTF font data");
        return Vec::new();
    }

    let scale_factor = external::stbtt_ScaleForPixelHeight(&font_info, fontSize as f32);
    let mut ascent = 0;
    let mut descent = 0;
    let mut line_gap = 0;
    external::stbtt_GetFontVMetrics(&font_info, &mut ascent, &mut descent, &mut line_gap);

    let glyph_count = required_codepoints.iter().filter(|&&cp| {
        external::stbtt_FindGlyphIndex(&font_info, cp) > 0
    }).count();
    let mut glyphs = Vec::with_capacity(glyph_count);

    for &cp in required_codepoints {
        if external::stbtt_FindGlyphIndex(&font_info, cp) == 0 {
            continue;
        }

        let mut glyph = GlyphInfo { value: cp, ..GlyphInfo::default() };
        let mut cp_width = 0;
        let mut cp_height = 0;

        glyph.image.data = match font_type {
            FontType::FONT_DEFAULT | FontType::FONT_BITMAP => {
                external::stbtt_GetCodepointBitmap(
                    &font_info, scale_factor, scale_factor, cp,
                    &mut cp_width, &mut cp_height, &mut glyph.offset_x, &mut glyph.offset_y,
                ).cast()
            }
            FontType::FONT_SDF if cp != 32 => {
                external::stbtt_GetCodepointSDF(
                    &font_info, scale_factor, cp,
                    FONT_SDF_CHAR_PADDING, FONT_SDF_ON_EDGE_VALUE, FONT_SDF_PIXEL_DIST_SCALE,
                    &mut cp_width, &mut cp_height, &mut glyph.offset_x, &mut glyph.offset_y,
                ).cast()
            }
            FontType::FONT_SDF => std::ptr::null_mut(),
        };

        if !glyph.image.data.is_null() {
            external::stbtt_GetCodepointHMetrics(&font_info, cp, &mut glyph.advance_x, std::ptr::null_mut());
            glyph.advance_x = (glyph.advance_x as f32 * scale_factor) as i32;

            if font_type != FontType::FONT_SDF && cp_height > fontSize {
                warn!("FONT: [0x{:04x}] Glyph height is bigger than requested font size: {} > {}", cp, cp_height, fontSize);
            }

            glyph.image.width = cp_width;
            glyph.image.height = cp_height;
            glyph.image.mipmaps = 1;
            glyph.image.format = PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAYSCALE as i32;
            glyph.offset_y += (ascent as f32 * scale_factor) as i32;
        }

        // Spaces need a blank image with their advance width for atlas generation.
        if cp == 0x20 || cp == 0x3000 {
            external::stbtt_GetCodepointHMetrics(&font_info, cp, &mut glyph.advance_x, std::ptr::null_mut());
            glyph.advance_x = (glyph.advance_x as f32 * scale_factor) as i32;

            // Release any rendered bitmap before replacing it with a blank image.
            if !glyph.image.data.is_null() {
                if font_type == FontType::FONT_SDF {
                    external::stbtt_FreeSDF(glyph.image.data.cast(), font_info.userdata);
                } else {
                    external::stbtt_FreeBitmap(glyph.image.data.cast(), font_info.userdata);
                }
            }

            let width = glyph.advance_x;
            let data = if width > 0 {
                // calloc checks the multiplication for allocation-size overflow.
                libc::calloc(width as usize, fontSize as usize)
            } else {
                glyph.advance_x = 0;
                std::ptr::null_mut()
            };
            glyph.image = Image {
                data,
                width,
                height: fontSize,
                mipmaps: 1,
                format: PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAYSCALE as i32,
            };
        }

        if font_type == FontType::FONT_BITMAP && !glyph.image.data.is_null() {
            // Use the final image dimensions, including any replacement space image.
            let pixel_count = glyph.image.width as usize * glyph.image.height as usize;
            let pixels = std::slice::from_raw_parts_mut(glyph.image.data.cast::<u8>(), pixel_count);
            for pixel in pixels {
                *pixel = if *pixel < FONT_BITMAP_ALPHA_THRESHOLD { 0 } else { 255 };
            }
        }

        glyphs.push(glyph);
    }

    if glyphs.len() < required_codepoints.len() {
        warn!("FONT: Requested codepoints glyphs found: [{}/{}]", glyphs.len(), required_codepoints.len());
    }
    glyphs
}

pub fn UnloadFont(font: &mut Font) {
    unsafe {
        rtextures::UnloadTexture(&mut font.texture);
        font.glyphs.clear()
    }
}

static mut DEFAULT_FONT: Font = Font {
    baseSize: 0,
    glyphCount: 0,
    glyphPadding: 0,
    texture: Texture {
        id: 0,
        width: 0,
        height: 0,
        mipmaps: 0,
        format: 0,
    },
    recs: Vec::new(),
    glyphs: Vec::new(),
};


// Load raylib default font.
pub fn LoadFontDefault() {
    unsafe {
        if !DEFAULT_FONT.glyphs.is_empty() {
            return;
        }

        DEFAULT_FONT.glyphCount = 224;
        DEFAULT_FONT.glyphPadding = 0;

    // Default font is directly defined here (data generated from a sprite font image)
    // This way, reconstructing Font without creating large global variables
    // This data is automatically allocated to Stack and automatically deallocated at the end of this function
    let defaultFontData  = [
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00200020, 0x0001b000, 0x00000000, 0x00000000, 0x8ef92520, 0x00020a00, 0x7dbe8000, 0x1f7df45f,
        0x4a2bf2a0, 0x0852091e, 0x41224000, 0x10041450, 0x2e292020, 0x08220812, 0x41222000, 0x10041450, 0x10f92020, 0x3efa084c, 0x7d22103c, 0x107df7de,
        0xe8a12020, 0x08220832, 0x05220800, 0x10450410, 0xa4a3f000, 0x08520832, 0x05220400, 0x10450410, 0xe2f92020, 0x0002085e, 0x7d3e0281, 0x107df41f,
        0x00200000, 0x8001b000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0xc0000fbe, 0xfbf7e00f, 0x5fbf7e7d, 0x0050bee8, 0x440808a2, 0x0a142fe8, 0x50810285, 0x0050a048,
        0x49e428a2, 0x0a142828, 0x40810284, 0x0048a048, 0x10020fbe, 0x09f7ebaf, 0xd89f3e84, 0x0047a04f, 0x09e48822, 0x0a142aa1, 0x50810284, 0x0048a048,
        0x04082822, 0x0a142fa0, 0x50810285, 0x0050a248, 0x00008fbe, 0xfbf42021, 0x5f817e7d, 0x07d09ce8, 0x00008000, 0x00000fe0, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x000c0180,
        0xdfbf4282, 0x0bfbf7ef, 0x42850505, 0x004804bf, 0x50a142c6, 0x08401428, 0x42852505, 0x00a808a0, 0x50a146aa, 0x08401428, 0x42852505, 0x00081090,
        0x5fa14a92, 0x0843f7e8, 0x7e792505, 0x00082088, 0x40a15282, 0x08420128, 0x40852489, 0x00084084, 0x40a16282, 0x0842022a, 0x40852451, 0x00088082,
        0xc0bf4282, 0xf843f42f, 0x7e85fc21, 0x3e0900bf, 0x00000000, 0x00000004, 0x00000000, 0x000c0180, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x04000402, 0x41482000, 0x00000000, 0x00000800,
        0x04000404, 0x4100203c, 0x00000000, 0x00000800, 0xf7df7df0, 0x514bef85, 0xbefbefbe, 0x04513bef, 0x14414500, 0x494a2885, 0xa28a28aa, 0x04510820,
        0xf44145f0, 0x474a289d, 0xa28a28aa, 0x04510be0, 0x14414510, 0x494a2884, 0xa28a28aa, 0x02910a00, 0xf7df7df0, 0xd14a2f85, 0xbefbe8aa, 0x011f7be0,
        0x00000000, 0x00400804, 0x20080000, 0x00000000, 0x00000000, 0x00600f84, 0x20080000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0xac000000, 0x00000f01, 0x00000000, 0x00000000, 0x24000000, 0x00000f01, 0x00000000, 0x06000000, 0x24000000, 0x00000f01, 0x00000000, 0x09108000,
        0x24fa28a2, 0x00000f01, 0x00000000, 0x013e0000, 0x2242252a, 0x00000f52, 0x00000000, 0x038a8000, 0x2422222a, 0x00000f29, 0x00000000, 0x010a8000,
        0x2412252a, 0x00000f01, 0x00000000, 0x010a8000, 0x24fbe8be, 0x00000f01, 0x00000000, 0x0ebe8000, 0xac020000, 0x00000f01, 0x00000000, 0x00048000,
        0x0003e000, 0x00000f00, 0x00000000, 0x00008000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000038, 0x8443b80e, 0x00203a03,
        0x02bea080, 0xf0000020, 0xc452208a, 0x04202b02, 0xf8029122, 0x07f0003b, 0xe44b388e, 0x02203a02, 0x081e8a1c, 0x0411e92a, 0xf4420be0, 0x01248202,
        0xe8140414, 0x05d104ba, 0xe7c3b880, 0x00893a0a, 0x283c0e1c, 0x04500902, 0xc4400080, 0x00448002, 0xe8208422, 0x04500002, 0x80400000, 0x05200002,
        0x083e8e00, 0x04100002, 0x804003e0, 0x07000042, 0xf8008400, 0x07f00003, 0x80400000, 0x04000022, 0x00000000, 0x00000000, 0x80400000, 0x04000002,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00800702, 0x1848a0c2, 0x84010000, 0x02920921, 0x01042642, 0x00005121, 0x42023f7f, 0x00291002,
        0xefc01422, 0x7efdfbf7, 0xefdfa109, 0x03bbbbf7, 0x28440f12, 0x42850a14, 0x20408109, 0x01111010, 0x28440408, 0x42850a14, 0x2040817f, 0x01111010,
        0xefc78204, 0x7efdfbf7, 0xe7cf8109, 0x011111f3, 0x2850a932, 0x42850a14, 0x2040a109, 0x01111010, 0x2850b840, 0x42850a14, 0xefdfbf79, 0x03bbbbf7,
        0x001fa020, 0x00000000, 0x00001000, 0x00000000, 0x00002070, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x08022800, 0x00012283, 0x02430802, 0x01010001, 0x8404147c, 0x20000144, 0x80048404, 0x00823f08, 0xdfbf4284, 0x7e03f7ef, 0x142850a1, 0x0000210a,
        0x50a14684, 0x528a1428, 0x142850a1, 0x03efa17a, 0x50a14a9e, 0x52521428, 0x142850a1, 0x02081f4a, 0x50a15284, 0x4a221428, 0xf42850a1, 0x03efa14b,
        0x50a16284, 0x4a521428, 0x042850a1, 0x0228a17a, 0xdfbf427c, 0x7e8bf7ef, 0xf7efdfbf, 0x03efbd0b, 0x00000000, 0x04000000, 0x00000000, 0x00000008,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00200508, 0x00840400, 0x11458122, 0x00014210,
        0x00514294, 0x51420800, 0x20a22a94, 0x0050a508, 0x00200000, 0x00000000, 0x00050000, 0x08000000, 0xfefbefbe, 0xfbefbefb, 0xfbeb9114, 0x00fbefbe,
        0x20820820, 0x8a28a20a, 0x8a289114, 0x3e8a28a2, 0xfefbefbe, 0xfbefbe0b, 0x8a289114, 0x008a28a2, 0x228a28a2, 0x08208208, 0x8a289114, 0x088a28a2,
        0xfefbefbe, 0xfbefbefb, 0xfa2f9114, 0x00fbefbe, 0x00000000, 0x00000040, 0x00000000, 0x00000000, 0x00000000, 0x00000020, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00210100, 0x00000004, 0x00000000, 0x00000000, 0x14508200, 0x00001402, 0x00000000, 0x00000000,
        0x00000010, 0x00000020, 0x00000000, 0x00000000, 0xa28a28be, 0x00002228, 0x00000000, 0x00000000, 0xa28a28aa, 0x000022e8, 0x00000000, 0x00000000,
        0xa28a28aa, 0x000022a8, 0x00000000, 0x00000000, 0xa28a28aa, 0x000022e8, 0x00000000, 0x00000000, 0xbefbefbe, 0x00003e2f, 0x00000000, 0x00000000,
        0x00000004, 0x00002028, 0x00000000, 0x00000000, 0x80000000, 0x00003e0f, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000 ];

        let charsHeight = 10;
        let charsDivisor = 1;

        let charsWidth  = [
            3, 1, 4, 6, 5, 7, 6, 2, 3, 3, 5, 5, 2, 4, 1, 7, 5, 2, 5, 5, 5, 5, 5, 5, 5, 5, 1, 1, 3, 4, 3, 6,
                            7, 6, 6, 6, 6, 6, 6, 6, 6, 3, 5, 6, 5, 7, 6, 6, 6, 6, 6, 6, 7, 6, 7, 7, 6, 6, 6, 2, 7, 2, 3, 5,
                            2, 5, 5, 5, 5, 5, 4, 5, 5, 1, 2, 5, 2, 5, 5, 5, 5, 5, 5, 5, 4, 5, 5, 5, 5, 5, 5, 3, 1, 3, 4, 4,
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                            1, 1, 5, 5, 5, 7, 1, 5, 3, 7, 3, 5, 4, 1, 7, 4, 3, 5, 3, 3, 2, 5, 6, 1, 2, 2, 3, 5, 6, 6, 6, 6,
                            6, 6, 6, 6, 6, 6, 7, 6, 6, 6, 6, 6, 3, 3, 3, 3, 7, 6, 6, 6, 6, 6, 6, 5, 6, 6, 6, 6, 6, 6, 4, 6,
                            5, 5, 5, 5, 5, 5, 9, 5, 5, 5, 5, 5, 2, 2, 3, 3, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 3, 5
        ];

        // Keep the Vec alive while the Image borrows its gray/alpha pixels.
        let imFont = Image {
            data: libc::calloc(128 * 128, 2),
            width: 128,
            height: 128,
            mipmaps: 1,
            format: crate::types::PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA as i32,
        };
/*         for (word_index, &word) in default_font_data.iter().enumerate() {
            for bit in 0..32 {
                let pixel = (word_index * 32 + bit) * 2;
                pixels[pixel] = 0xff;
                pixels[pixel + 1] = if word & (1u32 << bit) != 0 { 0xff } else { 0 };
            }
        } */
       // Fill image.data with defaultFontData (convert from bit to pixel!)
        for (counter, i) in (0..(imFont.width*imFont.height) as usize).step_by(32).enumerate()
        {
            for j in (0..32).rev()
            {
                unsafe {
                    if (defaultFontData[counter] & (1u32 << j)) != 0
                    {
                        // NOTE: Unreferencing data as short, so,
                        // considering data as little-endian (alpha + gray)
                        *(imFont.data as *mut u16).add(i + j) = 0xffff;
                    }
                    else
                    {
                        *(imFont.data as *mut u8).add((i + j)*std::mem::size_of::<u16>()) = 0xff;
                        *(imFont.data as *mut u8).add((i + j)*std::mem::size_of::<u16>() + 1) = 0x00;
                    }
                }
            }
        }

        DEFAULT_FONT.texture = rtextures::LoadTextureFromImage(&imFont);

        DEFAULT_FONT.glyphs = vec![GlyphInfo::default(); DEFAULT_FONT.glyphCount as usize];
        DEFAULT_FONT.recs = vec![Rectangle::default(); DEFAULT_FONT.glyphCount as usize];
        let mut currentLine = 0;
        let mut currentPosX = charsDivisor;
        let mut testPosX = charsDivisor;

        for i in 0..DEFAULT_FONT.glyphCount as usize
        {
            DEFAULT_FONT.glyphs[i].value = 32 + i as i32;  // First char is 32
        
            DEFAULT_FONT.recs[i].x = currentPosX as f32;
            DEFAULT_FONT.recs[i].y = (charsDivisor + currentLine*(charsHeight + charsDivisor)) as f32;
            DEFAULT_FONT.recs[i].width = charsWidth[i] as f32;
            DEFAULT_FONT.recs[i].height = charsHeight as f32;
        
            testPosX += (DEFAULT_FONT.recs[i].width + charsDivisor as f32) as i32;
        
            if testPosX >= imFont.width
            {
                currentLine += 1;
                currentPosX = 2*charsDivisor + charsWidth[i];
                testPosX = currentPosX;
            
                DEFAULT_FONT.recs[i].x = charsDivisor as f32;
                DEFAULT_FONT.recs[i].y = (charsDivisor + currentLine*(charsHeight + charsDivisor)) as f32;
            }
            else { currentPosX = testPosX; }
        
            // NOTE: On default font character offsets and xAdvance are not required
            DEFAULT_FONT.glyphs[i].offset_x = 0;
            DEFAULT_FONT.glyphs[i].offset_y = 0;
            DEFAULT_FONT.glyphs[i].advance_x = 0;
        
            // Fill character image data from fontClear data
            DEFAULT_FONT.glyphs[i].image = unsafe { ImageFromImage(imFont, DEFAULT_FONT.recs[i]) };
        }

        DEFAULT_FONT.baseSize = DEFAULT_FONT.recs[0].height as i32;
        info!("FONT: Default font loaded successfully ({} glyphs)", DEFAULT_FONT.glyphCount);
    }
}

// Unload raylib default font
pub fn UnloadFontDefault()
{
    //for i in defaultFont.glyphCount { UnloadImage(defaultFont.glyphs[i].image); }
    //UnloadTexture(defaultFont.texture);
    //RL_FREE(defaultFont.glyphs);
    //RL_FREE(defaultFont.recs);
    //defaultFont.glyphCount = 0;
    //defaultFont.glyphs = NULL;
    //defaultFont.recs = NULL;
}

pub fn GetFontDefault() -> *mut Font {
    unsafe {
        if DEFAULT_FONT.glyphCount == 0 {
            LoadFontDefault();
        }
        &raw mut DEFAULT_FONT
    }
}
pub fn DrawFPS(posX: i32, posY: i32)
{
    let mut color = Color::LIME;                         // Good FPS
    let fps = unsafe { crate::rcore::GetFPS() };

    if ((fps < 30) && (fps >= 15)) { color = Color::ORANGE; }  // Warning FPS
    else if (fps < 15) { color = Color::RED; }             // Low FPS

    DrawText(&format!("{} FPS", fps), posX, posY, 20, color);
}

/// Draw text (using default font)
/// NOTE: fontSize work like in any drawing program but if fontSize is lower than font-base-size, then font-base-size is used
/// NOTE: chars spacing is proportional to fontSize
pub fn DrawText(text: &str, x: i32, y: i32, font_size: i32, color: Color) {
    let font = unsafe { GetFontDefault().as_ref_unchecked() };
    DrawTextEx(
        font,
        text,
        Vector2::new(x as f32, y as f32),
        font_size as f32,
        2.0,
        color,
    );
}

static TEXT_LINE_SPACING: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(2);

// Set vertical line spacing when drawing with line breaks.
pub fn SetTextLineSpacing(spacing: i32) {
    TEXT_LINE_SPACING.store(spacing, std::sync::atomic::Ordering::Relaxed);
}

// Validate the metrics and vector lengths before indexing glyph data.
// As in the C version, the GPU texture is not checked here.
pub fn IsFontValid(font: &Font) -> bool {
    font.baseSize > 0
        && font.glyphCount > 0
        && font.recs.len() >= font.glyphCount as usize
        && font.glyphs.len() >= font.glyphCount as usize
}

// Get the index of a Unicode codepoint, falling back to '?' or index zero.
pub fn GetGlyphIndex(font: &Font, codepoint: i32) -> usize {
    if !IsFontValid(font) {
        return 0;
    }

    let mut fallback_index = 0;
    for (index, glyph) in font.glyphs.iter().take(font.glyphCount as usize).enumerate() {
        if glyph.value == '?' as i32 {
            fallback_index = index;
        }
        if glyph.value == codepoint {
            return index;
        }
    }
    fallback_index
}

// Get glyph info for a codepoint, falling back to '?' or index zero.
pub fn GetGlyphInfo(font: &Font, codepoint: i32) -> GlyphInfo {
    font.glyphs[GetGlyphIndex(font, codepoint)]
}

// Get a glyph's atlas rectangle, falling back to '?' or index zero.
pub fn GetGlyphAtlasRec(font: &Font, codepoint: i32) -> Rectangle {
    font.recs[GetGlyphIndex(font, codepoint)]
}

// Decode the first Unicode codepoint and report its UTF-8 byte length.
// An empty string behaves like the C string terminator: codepoint 0, size 1.
pub fn GetCodepointNext(text: &str, codepointSize: &mut i32) -> i32 {
    match text.chars().next() {
        Some(codepoint) => {
            *codepointSize = codepoint.len_utf8() as i32;
            codepoint as i32
        }
        None => {
            *codepointSize = 1;
            0
        }
    }
}

// Draw a single codepoint, including glyph offsets and atlas padding.
pub fn DrawTextCodepoint(font: &Font, codepoint: i32, position: Vector2, fontSize: f32, tint: Color) {
    if !IsFontValid(font) {
        return;
    }

    let index = GetGlyphIndex(font, codepoint);
    let glyph = &font.glyphs[index];
    let rec = font.recs[index];
    let scale_factor = fontSize / font.baseSize as f32;
    let padding = font.glyphPadding as f32;

    let dst_rec = Rectangle::new(
        position.x + glyph.offset_x as f32 * scale_factor - padding * scale_factor,
        position.y + glyph.offset_y as f32 * scale_factor - padding * scale_factor,
        (rec.width + 2.0 * padding) * scale_factor,
        (rec.height + 2.0 * padding) * scale_factor,
    );
    let src_rec = Rectangle::new(
        rec.x - padding,
        rec.y - padding,
        rec.width + 2.0 * padding,
        rec.height + 2.0 * padding,
    );

    DrawTexturePro(&font.texture, src_rec, dst_rec, Vector2::new(0.0, 0.0), 0.0, tint);
}

// Measure text width using the default font.
pub fn MeasureText(text: &str, fontSize: i32) -> i32 {
    let font = unsafe { &*GetFontDefault() };
    if font.texture.id == 0 {
        return 0;
    }

    let default_font_size = 10;
    let font_size = fontSize.max(default_font_size);
    let spacing = font_size / default_font_size;

    MeasureTextEx(font, text, font_size as f32, spacing as f32).x as i32
}

// Measure UTF-8 text using Font, including character and line spacing.
pub fn MeasureTextEx(font: &Font, text: &str, fontSize: f32, spacing: f32) -> Vector2 {
    if font.texture.id == 0 || text.is_empty() || !IsFontValid(font) {
        return Vector2::new(0.0, 0.0);
    }

    let mut max_char_count = 0usize;
    let mut char_count = 0usize;
    let mut text_width = 0.0;
    let mut max_text_width = 0.0;
    let mut text_height = fontSize;
    let scale_factor = fontSize / font.baseSize as f32;
    let line_spacing = TEXT_LINE_SPACING.load(std::sync::atomic::Ordering::Relaxed) as f32;

    for codepoint in text.chars() {
        char_count += 1;

        if codepoint != '\n' {
            let index = GetGlyphIndex(font, codepoint as i32);
            let glyph = &font.glyphs[index];
            if glyph.advance_x > 0 {
                text_width += glyph.advance_x as f32;
            } else {
                text_width += font.recs[index].width + glyph.offset_x as f32;
            }
        } else {
            if max_text_width < text_width {
                max_text_width = text_width;
            }
            char_count = 0;
            text_width = 0.0;
            text_height += fontSize + line_spacing;
        }

        if max_char_count < char_count {
            max_char_count = char_count;
        }
    }

    if max_text_width < text_width {
        max_text_width = text_width;
    }

    // Match C: the maximum glyph width and character count are tracked separately.
    Vector2::new(
        max_text_width * scale_factor + (max_char_count as f32 - 1.0) * spacing,
        text_height,
    )
}

// Measure an array of Unicode codepoints. The slice supplies the length.
pub fn MeasureTextCodepoints(font: &Font, codepoints: &[i32], fontSize: f32, spacing: f32) -> Vector2 {
    if font.texture.id == 0 || codepoints.is_empty() || !IsFontValid(font) {
        return Vector2::new(0.0, 0.0);
    }

    let mut text_width = 0.0;
    let mut max_text_width = 0.0;
    let mut max_glyph_count = 0usize;
    let mut glyph_count = 0usize;
    let mut text_height = fontSize;
    let scale_factor = fontSize / font.baseSize as f32;
    let line_spacing = TEXT_LINE_SPACING.load(std::sync::atomic::Ordering::Relaxed) as f32;

    for &codepoint in codepoints {
        if codepoint != '\n' as i32 {
            glyph_count += 1;

            let index = GetGlyphIndex(font, codepoint);
            let glyph = &font.glyphs[index];
            if glyph.advance_x > 0 {
                text_width += glyph.advance_x as f32;
            } else {
                text_width += font.recs[index].width + glyph.offset_x as f32;
            }
        } else {
            if max_text_width < text_width {
                max_text_width = text_width;
            }
            text_width = 0.0;
            glyph_count = 0;
            text_height += fontSize + line_spacing;
        }

        if max_glyph_count < glyph_count {
            max_glyph_count = glyph_count;
        }
    }

    if max_text_width < text_width {
        max_text_width = text_width;
    }

    // Match C: the maximum glyph width and glyph count are tracked separately.
    Vector2::new(
        max_text_width * scale_factor + (max_glyph_count as f32 - 1.0) * spacing,
        text_height,
    )
}


/// Draw text using Font and pro parameters (rotation)
pub fn DrawTextPro(
    font: &Font,
    text: &str,
    position: Vector2,
    origin: Vector2,
    rotation: f32,
    fontSize: f32,
    spacing: f32,
    tint: Color,
) {
    unsafe {
        rlPushMatrix();

        rlTranslatef(position.x, position.y, 0.0);
        rlRotatef(rotation, 0.0, 0.0, 1.0);
        rlTranslatef(-origin.x, -origin.y, 0.0);

        DrawTextEx(font, text, Vector2::new(0.0, 0.0), fontSize, spacing, tint);

        rlPopMatrix();
    }
}

/// Draw UTF-8 text using Font. Character spacing is independent of font size.
/// Draw text using Font
/// NOTE: chars spacing is NOT proportional to fontSize
pub fn DrawTextEx(font: &Font, text: &str, position: Vector2, fontSize: f32, spacing: f32, tint: Color) {
    let font = if font.texture.id == 0 {
        // GetFontDefault initializes the module's default font when needed.
        unsafe { &*GetFontDefault() }
    } else {
        font
    };
    if !IsFontValid(font) {
        return;
    }

    let mut text_offset_y = 0.0;
    let mut text_offset_x = 0.0;
    let scale_factor = fontSize / font.baseSize as f32;
    let line_spacing = TEXT_LINE_SPACING.load(std::sync::atomic::Ordering::Relaxed) as f32;

    for codepoint in text.chars() {
        if codepoint == '\n' {
            text_offset_y += fontSize + line_spacing;
            text_offset_x = 0.0;
        } else {
            let index = GetGlyphIndex(font, codepoint as i32);
            if codepoint != ' ' && codepoint != '\t' {
                DrawTextCodepoint(
                    font,
                    codepoint as i32,
                    Vector2::new(position.x + text_offset_x, position.y + text_offset_y),
                    fontSize,
                    tint,
                );
            }

            let advance = if font.glyphs[index].advance_x == 0 {
                font.recs[index].width
            } else {
                font.glyphs[index].advance_x as f32
            };
            text_offset_x += advance * scale_factor + spacing;
        }
    }
}
