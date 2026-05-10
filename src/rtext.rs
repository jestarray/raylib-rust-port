use crate::types::{Color, Font, GlyphInfo, Rectangle, Vector2, Texture, Image};
use crate::rtextures;
use std::fs;

pub fn load_font(file_name: &str) -> Font {
    unsafe {
        let ttf_data = fs::read(file_name).unwrap_or_else(|_| Vec::new());
        if ttf_data.is_empty() {
            return Font {
                base_size: 10,
                glyph_count: 0,
                glyph_padding: 0,
                texture: Texture { id: 0, width: 0, height: 0, mipmaps: 0, format: 0 },
                recs: std::ptr::null_mut(),
                glyphs: std::ptr::null_mut(),
            };
        }

        let mut cdata: Vec<crate::external::stbtt_bakedchar> = Vec::with_capacity(96);
        cdata.set_len(96);
        
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

        let texture = rtextures::load_texture_from_image(&image);

        let recs = libc::malloc(std::mem::size_of::<Rectangle>() * 96) as *mut Rectangle;
        let glyphs = libc::malloc(std::mem::size_of::<GlyphInfo>() * 96) as *mut GlyphInfo;

        for i in 0..96 {
            let baked = cdata[i];
            let rec = Rectangle::new(
                baked.x0 as f32,
                baked.y0 as f32,
                (baked.x1 - baked.x0) as f32,
                (baked.y1 - baked.y0) as f32,
            );
            *recs.add(i) = rec;
            
            let glyph = GlyphInfo {
                value: i as i32 + 32,
                offset_x: baked.xoff as i32,
                offset_y: baked.yoff as i32,
                advance_x: baked.xadvance as i32,
                image: Image::default(),
            };
            *glyphs.add(i) = glyph;
        }

        Font {
            base_size: 32,
            glyph_count: 96,
            glyph_padding: 0,
            texture,
            recs,
            glyphs,
        }
    }
}

pub fn unload_font(font: &mut Font) {
    unsafe {
        rtextures::unload_texture(&mut font.texture);
        if !font.recs.is_null() {
            libc::free(font.recs as *mut std::ffi::c_void);
            font.recs = std::ptr::null_mut();
        }
        if !font.glyphs.is_null() {
            libc::free(font.glyphs as *mut std::ffi::c_void);
            font.glyphs = std::ptr::null_mut();
        }
    }
}

pub fn draw_text_ex(font: &Font, text: &str, position: Vector2, font_size: f32, spacing: f32, tint: Color) {
    if font.texture.id == 0 || font.glyphs.is_null() {
        return;
    }

    let scale_factor = font_size / (font.base_size as f32);
    let mut text_offset_x = 0.0;

    for c in text.chars() {
        let mut index = c as i32 - 32;
        if index < 0 || index >= font.glyph_count {
            index = 63; // '?' or default character
        }

        unsafe {
            let rec = *font.recs.add(index as usize);
            let glyph = *font.glyphs.add(index as usize);

            if rec.width > 0.0 && rec.height > 0.0 {
                let dest = Rectangle::new(
                    position.x + text_offset_x + (glyph.offset_x as f32) * scale_factor,
                    position.y + (glyph.offset_y as f32) * scale_factor,
                    rec.width * scale_factor,
                    rec.height * scale_factor,
                );

                rtextures::draw_texture_pro(
                    &font.texture,
                    rec,
                    dest,
                    Vector2::new(0.0, 0.0),
                    0.0,
                    tint,
                );
            }

            text_offset_x += (glyph.advance_x as f32) * scale_factor + spacing;
        }
    }
}
