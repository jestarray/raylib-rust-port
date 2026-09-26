use raylib::types::{
    Color, Image, PixelFormat, RaylibTexture2D, Rectangle, Shader, ShaderUniformDataType, Texture,
};

#[test]
fn image_and_color_methods_provide_safe_operations() {
    let image = Image::gen_image_color(2, 2, Color::RED);
    assert!(image.is_image_valid());
    assert_eq!(image.load_image_colors(), vec![Color::RED; 4]);

    let crop = image.image_from_image(Rectangle::new(0.0, 0.0, 1.0, 1.0));
    assert_eq!(crop.load_image_colors(), vec![Color::RED]);
    assert_eq!(Color::RED.fade(1.0), Color::RED);
    assert_eq!(PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8.get_pixel_data_size(2, 2), 16);
}

#[test]
fn resource_query_methods_and_texture_mipmap_count() {
    assert!(!Shader::default().is_shader_valid());
    let texture = Texture { id: 0, width: 8, height: 8, mipmaps: 3, format: 0 };
    assert!(!texture.is_texture_valid());
    assert_eq!(RaylibTexture2D::mipmaps(&texture), 3);
}

#[test]
#[should_panic(expected = "shader uniform type does not match the value")]
fn shader_uniform_method_rejects_mismatched_type() {
    let mut shader = Shader::default();
    shader.set_shader_value(0, 1.0_f32, ShaderUniformDataType::SHADER_UNIFORM_VEC4);
}
