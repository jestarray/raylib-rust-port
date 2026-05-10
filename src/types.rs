use glam::{Vec2, Vec3, Vec4, Mat4};
use std::ffi::c_void;

pub type Vector2 = Vec2;
pub type Vector3 = Vec3;
pub type Vector4 = Vec4;
pub type Quaternion = Vec4;
pub type Matrix = Mat4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    
    pub const LIGHTGRAY: Color  = Color::new(200, 200, 200, 255);
    pub const GRAY: Color       = Color::new(130, 130, 130, 255);
    pub const DARKGRAY: Color   = Color::new(80, 80, 80, 255);
    pub const YELLOW: Color     = Color::new(253, 249, 0, 255);
    pub const GOLD: Color       = Color::new(255, 203, 0, 255);
    pub const ORANGE: Color     = Color::new(255, 161, 0, 255);
    pub const PINK: Color       = Color::new(255, 109, 194, 255);
    pub const RED: Color        = Color::new(230, 41, 55, 255);
    pub const MAROON: Color     = Color::new(190, 33, 55, 255);
    pub const GREEN: Color      = Color::new(0, 228, 48, 255);
    pub const LIME: Color       = Color::new(0, 158, 47, 255);
    pub const DARKGREEN: Color  = Color::new(0, 117, 44, 255);
    pub const SKYBLUE: Color    = Color::new(102, 191, 255, 255);
    pub const BLUE: Color       = Color::new(0, 121, 241, 255);
    pub const DARKBLUE: Color   = Color::new(0, 82, 172, 255);
    pub const PURPLE: Color     = Color::new(200, 122, 255, 255);
    pub const VIOLET: Color     = Color::new(135, 60, 190, 255);
    pub const DARKPURPLE: Color = Color::new(112, 31, 126, 255);
    pub const BEIGE: Color      = Color::new(211, 176, 131, 255);
    pub const BROWN: Color      = Color::new(127, 106, 79, 255);
    pub const DARKBROWN: Color  = Color::new(76, 63, 47, 255);
    pub const WHITE: Color      = Color::new(255, 255, 255, 255);
    pub const BLACK: Color      = Color::new(0, 0, 0, 255);
    pub const BLANK: Color      = Color::new(0, 0, 0, 0);
    pub const MAGENTA: Color    = Color::new(255, 0, 255, 255);
    pub const RAYWHITE: Color   = Color::new(245, 245, 245, 255);
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Image {
    pub data: *mut c_void,
    pub width: i32,
    pub height: i32,
    pub mipmaps: i32,
    pub format: i32,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            data: std::ptr::null_mut(),
            width: 0,
            height: 0,
            mipmaps: 1,
            format: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Texture {
    pub id: u32,
    pub width: i32,
    pub height: i32,
    pub mipmaps: i32,
    pub format: i32,
}
pub type Texture2D = Texture;
pub type TextureCubemap = Texture;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RenderTexture {
    pub id: u32,
    pub texture: Texture,
    pub depth: Texture,
}
pub type RenderTexture2D = RenderTexture;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Camera3D {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fovy: f32,
    pub projection: i32,
}
pub type Camera = Camera3D;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Camera2D {
    pub offset: Vector2,
    pub target: Vector2,
    pub rotation: f32,
    pub zoom: f32,
}

// Some Basic Colors
pub const LIGHTGRAY: Color  = Color { r: 200, g: 200, b: 200, a: 255 };
pub const GRAY: Color       = Color { r: 130, g: 130, b: 130, a: 255 };
pub const DARKGRAY: Color   = Color { r: 80, g: 80, b: 80, a: 255 };
pub const YELLOW: Color     = Color { r: 253, g: 249, b: 0, a: 255 };
pub const GOLD: Color       = Color { r: 255, g: 203, b: 0, a: 255 };
pub const ORANGE: Color     = Color { r: 255, g: 161, b: 0, a: 255 };
pub const PINK: Color       = Color { r: 255, g: 109, b: 194, a: 255 };
pub const RED: Color        = Color { r: 230, g: 41, b: 55, a: 255 };
pub const MAROON: Color     = Color { r: 190, g: 33, b: 55, a: 255 };
pub const GREEN: Color      = Color { r: 0, g: 228, b: 48, a: 255 };
pub const LIME: Color       = Color { r: 0, g: 158, b: 47, a: 255 };
pub const DARKGREEN: Color  = Color { r: 0, g: 117, b: 44, a: 255 };
pub const SKYBLUE: Color    = Color { r: 102, g: 191, b: 255, a: 255 };
pub const BLUE: Color       = Color { r: 0, g: 121, b: 241, a: 255 };
pub const DARKBLUE: Color   = Color { r: 0, g: 82, b: 172, a: 255 };
pub const PURPLE: Color     = Color { r: 200, g: 122, b: 255, a: 255 };
pub const VIOLET: Color     = Color { r: 135, g: 60, b: 190, a: 255 };
pub const DARKPURPLE: Color = Color { r: 112, g: 31, b: 126, a: 255 };
pub const BEIGE: Color      = Color { r: 211, g: 176, b: 131, a: 255 };
pub const BROWN: Color      = Color { r: 127, g: 106, b: 79, a: 255 };
pub const DARKBROWN: Color  = Color { r: 76, g: 63, b: 47, a: 255 };
pub const WHITE: Color      = Color { r: 255, g: 255, b: 255, a: 255 };
pub const BLACK: Color      = Color { r: 0, g: 0, b: 0, a: 255 };
pub const BLANK: Color      = Color { r: 0, g: 0, b: 0, a: 0 };
pub const MAGENTA: Color    = Color { r: 255, g: 0, b: 255, a: 255 };
pub const RAYWHITE: Color   = Color { r: 245, g: 245, b: 245, a: 255 };

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NPatchLayout {
    NinePatch = 0,          // Npatch layout: 3x3 tiles
    ThreePatchVertical,    // Npatch layout: 1x3 tiles
    ThreePatchHorizontal   // Npatch layout: 3x1 tiles
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NPatchInfo {
    pub source: Rectangle,       // Texture source rectangle
    pub left: i32,               // Left border offset
    pub top: i32,                // Top border offset
    pub right: i32,              // Right border offset
    pub bottom: i32,             // Bottom border offset
    pub layout: i32,             // Layout of the n-patch: 3x3, 1x3 or 3x1
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GlyphInfo {
    pub value: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub advance_x: i32,
    pub image: Image,
}

#[repr(C)]
#[derive(Debug)]
pub struct Font {
    pub base_size: i32,
    pub glyph_count: i32,
    pub glyph_padding: i32,
    pub texture: Texture,
    pub recs: *mut Rectangle,
    pub glyphs: *mut GlyphInfo,
}
