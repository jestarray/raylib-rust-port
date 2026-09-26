use raylib::core;
use raylib::text;
use raylib::types::{Color, Image, PixelFormat, Rectangle, Texture};

#[test]
fn generated_image_has_exactly_four_bytes_per_pixel() {
    let image = Image::gen_image_color(2, 3, Color::RED);
    assert_eq!(image.data.len(), 2 * 3 * 4);
}

#[test]
fn malformed_image_does_not_read_past_its_buffer() {
    let image = Image::new(
        vec![255],
        2,
        2,
        1,
        PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32,
    );
    assert!(image.load_image_colors().is_empty());
    assert!(!image.is_image_valid());
    assert!(image.image_from_image(Rectangle::new(0.0, 0.0, 1.0, 1.0)).data.is_empty());
    assert_eq!(image.load_texture_from_image().id, 0);
}

#[test]
fn short_texture_updates_return_without_calling_gl() {
    let texture = Texture {
        id: 1,
        width: 2,
        height: 2,
        mipmaps: 1,
        format: PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32,
    };
    texture.update_texture(&[0]);
    texture.update_texture_rec(Rectangle::new(0.0, 0.0, 1.0, 1.0), &[0]);
    texture.update_texture_rec(Rectangle::new(1.0, 1.0, 2.0, 2.0), &[0; 16]);
}

#[test]
fn missing_font_file_returns_invalid_font_without_panicking() {
    let missing = std::env::temp_dir().join(format!("raylib-missing-font-{}.ttf", std::process::id()));
    let mut font = text::load_font_ex(missing, 32, None);
    assert!(!font.is_font_valid());
    font.unload_font();
}

#[test]
fn empty_data_cannot_be_exported_as_c_array() {
    let output = std::env::temp_dir().join(format!("raylib-empty-data-{}.h", std::process::id()));
    assert!(!core::export_data_as_code(&[], output));
}
