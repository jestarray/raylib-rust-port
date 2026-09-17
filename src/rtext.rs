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
use log::info;

use crate::rlgl::{rlPopMatrix, rlPushMatrix, rlRotatef, rlTranslatef};
use crate::rtextures::{self, DrawTexturePro, ImageFromImage};
use crate::types::{Color, Font, GlyphInfo, Image, Rectangle, Texture, Vector2};
use std::fs;

pub fn load_font(file_name: &str) -> Font {
    unsafe {
        let ttf_data = fs::read(file_name).unwrap_or_else(|_| Vec::new());
        if ttf_data.is_empty() {
            return Font {
                baseSize: 10,
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
        }

        let mut cdata = Vec::with_capacity(96);

        let mut pixels: Vec<u8> = vec![0; 512 * 512];

        crate::external::stbtt_BakeFontBitmap(
            ttf_data.as_ptr(),
            0,
            32.0,
            pixels.as_mut_ptr(),
            512,
            512,
            32,
            96,
            cdata.as_mut_ptr(),
        );

        let image = Image {
            data: pixels.as_mut_ptr() as *mut std::ffi::c_void,
            width: 512,
            height: 512,
            mipmaps: 1,
            format: 1,
        };

        let texture = rtextures::LoadTextureFromImage(&image);

        let mut recs = Vec::with_capacity(96);
        let mut glyphs = vec![GlyphInfo::default(); 96];

        for i in 0..96 {
            let baked = cdata[i];
            let rec = Rectangle::new(
                baked.x0 as f32,
                baked.y0 as f32,
                (baked.x1 - baked.x0) as f32,
                (baked.y1 - baked.y0) as f32,
            );
            recs[i] = rec;

            let glyph = GlyphInfo {
                value: i as i32 + 32,
                offset_x: baked.xoff as i32,
                offset_y: baked.yoff as i32,
                advance_x: baked.xadvance as i32,
                image: Image::default(),
            };
            glyphs[i] = glyph;
        }

        Font {
            baseSize: 32,
            glyphCount: 96,
            glyphPadding: 0,
            texture,
            recs,
            glyphs,
        }
    }
}

pub fn unload_font(font: &mut Font) {
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
/// Draw text (using default font)
/// NOTE: fontSize work like in any drawing program but if fontSize is lower than font-base-size, then font-base-size is used
/// NOTE: chars spacing is proportional to fontSize
pub fn DrawText(text: &str, x: i32, y: i32, font_size: i32, color: Color) {
    let font = unsafe { GetFontDefault().as_ref_unchecked() };
    DrawTextEx(
        &font,
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
