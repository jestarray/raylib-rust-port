use crate::types::{Color, Image, Rectangle, Texture, Vector2};
use crate::rlgl::{self, RL_QUADS};
use std::ffi::CString;

pub fn load_image(file_name: &str) -> Image {
    unsafe {
        let c_file_name = CString::new(file_name).unwrap();
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut channels: i32 = 0;
        
        let data = crate::external::stbi_load(
            c_file_name.as_ptr(),
            &mut width,
            &mut height,
            &mut channels,
            0,
        );

        if data.is_null() {
            println!("ERROR: Failed to load image: {}", file_name);
            return Image {
                data: std::ptr::null_mut(),
                width: 0,
                height: 0,
                mipmaps: 0,
                format: 0,
            };
        }

        let format = if channels == 1 {
            1 // UNCOMPRESSED_GRAYSCALE
        } else if channels == 2 {
            2 // UNCOMPRESSED_GRAY_ALPHA
        } else if channels == 3 {
            4 // UNCOMPRESSED_R8G8B8
        } else if channels == 4 {
            7 // UNCOMPRESSED_R8G8B8A8
        } else {
            0
        };

        Image {
            data: data as *mut std::ffi::c_void,
            width,
            height,
            mipmaps: 1,
            format,
        }
    }
}

pub fn unload_image(image: &mut Image) {
    unsafe {
        if !image.data.is_null() {
            crate::external::stbi_image_free(image.data);
            image.data = std::ptr::null_mut();
        }
    }
}

pub fn load_texture(file_name: &str) -> Texture {
    let mut image = load_image(file_name);
    let texture = load_texture_from_image(&image);
    unload_image(&mut image);
    texture
}

pub fn load_texture_from_image(image: &Image) -> Texture {
    unsafe {
        let id = rlgl::rlLoadTexture(
            image.data,
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

pub fn unload_texture(texture: &mut Texture) {
    unsafe {
        rlgl::rlUnloadTexture(texture.id);
        texture.id = 0;
    }
}

pub fn draw_texture(texture: &Texture, pos_x: i32, pos_y: i32, tint: Color) {
    draw_texture_ex(texture, Vector2::new(pos_x as f32, pos_y as f32), 0.0, 1.0, tint);
}

pub fn draw_texture_ex(texture: &Texture, position: Vector2, rotation: f32, scale: f32, tint: Color) {
    let source = Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32);
    let dest = Rectangle::new(
        position.x,
        position.y,
        texture.width as f32 * scale,
        texture.height as f32 * scale,
    );
    let origin = Vector2::new(0.0, 0.0);
    draw_texture_pro(texture, source, dest, origin, rotation, tint);
}

pub fn draw_texture_rec(texture: &Texture, source: Rectangle, position: Vector2, tint: Color) {
    let dest = Rectangle::new(position.x, position.y, source.width.abs(), source.height.abs());
    let origin = Vector2::new(0.0, 0.0);
    draw_texture_pro(texture, source, dest, origin, 0.0, tint);
}

pub fn draw_texture_pro(
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
        rlgl::rlSetTexture(texture.id);
        rlgl::rlBegin(RL_QUADS);
        rlgl::rlColor4ub(tint.r, tint.g, tint.b, tint.a);

        // Bottom-left corner
        rlgl::rlTexCoord2f(
            if flip_x { (src.x + src.width) / width } else { src.x / width },
            if flip_y { src.y / height } else { (src.y + src.height) / height },
        );
        rlgl::rlVertex2f(bottom_left.x, bottom_left.y);

        // Bottom-right corner
        rlgl::rlTexCoord2f(
            if flip_x { src.x / width } else { (src.x + src.width) / width },
            if flip_y { src.y / height } else { (src.y + src.height) / height },
        );
        rlgl::rlVertex2f(bottom_right.x, bottom_right.y);

        // Top-right corner
        rlgl::rlTexCoord2f(
            if flip_x { src.x / width } else { (src.x + src.width) / width },
            if flip_y { (src.y + src.height) / height } else { src.y / height },
        );
        rlgl::rlVertex2f(top_right.x, top_right.y);

        // Top-left corner
        rlgl::rlTexCoord2f(
            if flip_x { (src.x + src.width) / width } else { src.x / width },
            if flip_y { (src.y + src.height) / height } else { src.y / height },
        );
        rlgl::rlVertex2f(top_left.x, top_left.y);

        rlgl::rlEnd();
    }
}

pub fn draw_texture_n_patch(
    texture: &Texture,
    n_patch_info: crate::types::NPatchInfo,
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

        if n_patch_info.layout == crate::types::NPatchLayout::ThreePatchHorizontal as i32 {
            patch_height = source.height;
        }
        if n_patch_info.layout == crate::types::NPatchLayout::ThreePatchVertical as i32 {
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
            && n_patch_info.layout != crate::types::NPatchLayout::ThreePatchVertical as i32
        {
            draw_center = false;
            left_border = (left_border / (left_border + right_border)) * patch_width;
            right_border = patch_width - left_border;
        }

        // Adjust lateral border heights
        if patch_height <= (top_border + bottom_border)
            && n_patch_info.layout != crate::types::NPatchLayout::ThreePatchHorizontal as i32
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
            rlgl::rlSetTexture(texture.id);
            rlgl::rlPushMatrix();
            rlgl::rlTranslatef(dest.x, dest.y, 0.0);
            rlgl::rlRotatef(rotation, 0.0, 0.0, 1.0);
            rlgl::rlTranslatef(-origin.x, -origin.y, 0.0);

            rlgl::rlBegin(rlgl::RL_QUADS);
            rlgl::rlColor4ub(tint.r, tint.g, tint.b, tint.a);

            if n_patch_info.layout == crate::types::NPatchLayout::NinePatch as i32 {
                // TOP-LEFT QUAD
                rlgl::rlTexCoord2f(coord_a.x, coord_b.y); rlgl::rlVertex2f(vert_a.x, vert_b.y);
                rlgl::rlTexCoord2f(coord_b.x, coord_b.y); rlgl::rlVertex2f(vert_b.x, vert_b.y);
                rlgl::rlTexCoord2f(coord_b.x, coord_a.y); rlgl::rlVertex2f(vert_b.x, vert_a.y);
                rlgl::rlTexCoord2f(coord_a.x, coord_a.y); rlgl::rlVertex2f(vert_a.x, vert_a.y);

                if draw_center {
                    // TOP-CENTER QUAD
                    rlgl::rlTexCoord2f(coord_b.x, coord_b.y); rlgl::rlVertex2f(vert_b.x, vert_b.y);
                    rlgl::rlTexCoord2f(coord_c.x, coord_b.y); rlgl::rlVertex2f(vert_c.x, vert_b.y);
                    rlgl::rlTexCoord2f(coord_c.x, coord_a.y); rlgl::rlVertex2f(vert_c.x, vert_a.y);
                    rlgl::rlTexCoord2f(coord_b.x, coord_a.y); rlgl::rlVertex2f(vert_b.x, vert_a.y);
                }

                // TOP-RIGHT QUAD
                rlgl::rlTexCoord2f(coord_c.x, coord_b.y); rlgl::rlVertex2f(vert_c.x, vert_b.y);
                rlgl::rlTexCoord2f(coord_d.x, coord_b.y); rlgl::rlVertex2f(vert_d.x, vert_b.y);
                rlgl::rlTexCoord2f(coord_d.x, coord_a.y); rlgl::rlVertex2f(vert_d.x, vert_a.y);
                rlgl::rlTexCoord2f(coord_c.x, coord_a.y); rlgl::rlVertex2f(vert_c.x, vert_a.y);

                if draw_middle {
                    // MIDDLE-LEFT QUAD
                    rlgl::rlTexCoord2f(coord_a.x, coord_c.y); rlgl::rlVertex2f(vert_a.x, vert_c.y);
                    rlgl::rlTexCoord2f(coord_b.x, coord_c.y); rlgl::rlVertex2f(vert_b.x, vert_c.y);
                    rlgl::rlTexCoord2f(coord_b.x, coord_b.y); rlgl::rlVertex2f(vert_b.x, vert_b.y);
                    rlgl::rlTexCoord2f(coord_a.x, coord_b.y); rlgl::rlVertex2f(vert_a.x, vert_b.y);

                    if draw_center {
                        // MIDDLE-CENTER QUAD
                        rlgl::rlTexCoord2f(coord_b.x, coord_c.y); rlgl::rlVertex2f(vert_b.x, vert_c.y);
                        rlgl::rlTexCoord2f(coord_c.x, coord_c.y); rlgl::rlVertex2f(vert_c.x, vert_c.y);
                        rlgl::rlTexCoord2f(coord_c.x, coord_b.y); rlgl::rlVertex2f(vert_c.x, vert_b.y);
                        rlgl::rlTexCoord2f(coord_b.x, coord_b.y); rlgl::rlVertex2f(vert_b.x, vert_b.y);
                    }

                    // MIDDLE-RIGHT QUAD
                    rlgl::rlTexCoord2f(coord_c.x, coord_c.y); rlgl::rlVertex2f(vert_c.x, vert_c.y);
                    rlgl::rlTexCoord2f(coord_d.x, coord_c.y); rlgl::rlVertex2f(vert_d.x, vert_c.y);
                    rlgl::rlTexCoord2f(coord_d.x, coord_b.y); rlgl::rlVertex2f(vert_d.x, vert_b.y);
                    rlgl::rlTexCoord2f(coord_c.x, coord_b.y); rlgl::rlVertex2f(vert_c.x, vert_b.y);
                }

                // BOTTOM-LEFT QUAD
                rlgl::rlTexCoord2f(coord_a.x, coord_d.y); rlgl::rlVertex2f(vert_a.x, vert_d.y);
                rlgl::rlTexCoord2f(coord_b.x, coord_d.y); rlgl::rlVertex2f(vert_b.x, vert_d.y);
                rlgl::rlTexCoord2f(coord_b.x, coord_c.y); rlgl::rlVertex2f(vert_b.x, vert_c.y);
                rlgl::rlTexCoord2f(coord_a.x, coord_c.y); rlgl::rlVertex2f(vert_a.x, vert_c.y);

                if draw_center {
                    // BOTTOM-CENTER QUAD
                    rlgl::rlTexCoord2f(coord_b.x, coord_d.y); rlgl::rlVertex2f(vert_b.x, vert_d.y);
                    rlgl::rlTexCoord2f(coord_c.x, coord_d.y); rlgl::rlVertex2f(vert_c.x, vert_d.y);
                    rlgl::rlTexCoord2f(coord_c.x, coord_c.y); rlgl::rlVertex2f(vert_c.x, vert_c.y);
                    rlgl::rlTexCoord2f(coord_b.x, coord_c.y); rlgl::rlVertex2f(vert_b.x, vert_c.y);
                }

                // BOTTOM-RIGHT QUAD
                rlgl::rlTexCoord2f(coord_c.x, coord_d.y); rlgl::rlVertex2f(vert_c.x, vert_d.y);
                rlgl::rlTexCoord2f(coord_d.x, coord_d.y); rlgl::rlVertex2f(vert_d.x, vert_d.y);
                rlgl::rlTexCoord2f(coord_d.x, coord_c.y); rlgl::rlVertex2f(vert_d.x, vert_c.y);
                rlgl::rlTexCoord2f(coord_c.x, coord_c.y); rlgl::rlVertex2f(vert_c.x, vert_c.y);
            } else if n_patch_info.layout == crate::types::NPatchLayout::ThreePatchVertical as i32 {
                // TOP QUAD
                rlgl::rlTexCoord2f(coord_a.x, coord_b.y); rlgl::rlVertex2f(vert_a.x, vert_b.y);
                rlgl::rlTexCoord2f(coord_d.x, coord_b.y); rlgl::rlVertex2f(vert_d.x, vert_b.y);
                rlgl::rlTexCoord2f(coord_d.x, coord_a.y); rlgl::rlVertex2f(vert_d.x, vert_a.y);
                rlgl::rlTexCoord2f(coord_a.x, coord_a.y); rlgl::rlVertex2f(vert_a.x, vert_a.y);

                if draw_center {
                    // MIDDLE QUAD
                    rlgl::rlTexCoord2f(coord_a.x, coord_c.y); rlgl::rlVertex2f(vert_a.x, vert_c.y);
                    rlgl::rlTexCoord2f(coord_d.x, coord_c.y); rlgl::rlVertex2f(vert_d.x, vert_c.y);
                    rlgl::rlTexCoord2f(coord_d.x, coord_b.y); rlgl::rlVertex2f(vert_d.x, vert_b.y);
                    rlgl::rlTexCoord2f(coord_a.x, coord_b.y); rlgl::rlVertex2f(vert_a.x, vert_b.y);
                }

                // BOTTOM QUAD
                rlgl::rlTexCoord2f(coord_a.x, coord_d.y); rlgl::rlVertex2f(vert_a.x, vert_d.y);
                rlgl::rlTexCoord2f(coord_d.x, coord_d.y); rlgl::rlVertex2f(vert_d.x, vert_d.y);
                rlgl::rlTexCoord2f(coord_d.x, coord_c.y); rlgl::rlVertex2f(vert_d.x, vert_c.y);
                rlgl::rlTexCoord2f(coord_a.x, coord_c.y); rlgl::rlVertex2f(vert_a.x, vert_c.y);
            } else if n_patch_info.layout == crate::types::NPatchLayout::ThreePatchHorizontal as i32 {
                // LEFT QUAD
                rlgl::rlTexCoord2f(coord_a.x, coord_d.y); rlgl::rlVertex2f(vert_a.x, vert_d.y);
                rlgl::rlTexCoord2f(coord_b.x, coord_d.y); rlgl::rlVertex2f(vert_b.x, vert_d.y);
                rlgl::rlTexCoord2f(coord_b.x, coord_a.y); rlgl::rlVertex2f(vert_b.x, vert_a.y);
                rlgl::rlTexCoord2f(coord_a.x, coord_a.y); rlgl::rlVertex2f(vert_a.x, vert_a.y);

                if draw_center {
                    // CENTER QUAD
                    rlgl::rlTexCoord2f(coord_b.x, coord_d.y); rlgl::rlVertex2f(vert_b.x, vert_d.y);
                    rlgl::rlTexCoord2f(coord_c.x, coord_d.y); rlgl::rlVertex2f(vert_c.x, vert_d.y);
                    rlgl::rlTexCoord2f(coord_c.x, coord_a.y); rlgl::rlVertex2f(vert_c.x, vert_a.y);
                    rlgl::rlTexCoord2f(coord_b.x, coord_a.y); rlgl::rlVertex2f(vert_b.x, vert_a.y);
                }

                // RIGHT QUAD
                rlgl::rlTexCoord2f(coord_c.x, coord_d.y); rlgl::rlVertex2f(vert_c.x, vert_d.y);
                rlgl::rlTexCoord2f(coord_d.x, coord_d.y); rlgl::rlVertex2f(vert_d.x, vert_d.y);
                rlgl::rlTexCoord2f(coord_d.x, coord_a.y); rlgl::rlVertex2f(vert_d.x, vert_a.y);
                rlgl::rlTexCoord2f(coord_c.x, coord_a.y); rlgl::rlVertex2f(vert_c.x, vert_a.y);
            }

            rlgl::rlEnd();
            rlgl::rlPopMatrix();
        }
    }
}
