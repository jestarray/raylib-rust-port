use glam::{Mat4, Vec2, Vec3, Vec4};
use std::ffi::c_void;

pub type Vector2 = Vec2;
pub type Vector3 = Vec3;
pub type Vector4 = Vec4;
pub type Quaternion = Vec4;
//pub type Matrix = Mat4;

use std::ops::Mul;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Matrix {
    // First row
    pub m0: f32,
    pub m4: f32,
    pub m8: f32,
    pub m12: f32,

    // Second row
    pub m1: f32,
    pub m5: f32,
    pub m9: f32,
    pub m13: f32,

    // Third row
    pub m2: f32,
    pub m6: f32,
    pub m10: f32,
    pub m14: f32,

    // Fourth row
    pub m3: f32,
    pub m7: f32,
    pub m11: f32,
    pub m15: f32,
}

impl Matrix {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        m0: f32,
        m4: f32,
        m8: f32,
        m12: f32,
        m1: f32,
        m5: f32,
        m9: f32,
        m13: f32,
        m2: f32,
        m6: f32,
        m10: f32,
        m14: f32,
        m3: f32,
        m7: f32,
        m11: f32,
        m15: f32,
    ) -> Self {
        Self {
            m0,
            m4,
            m8,
            m12,
            m1,
            m5,
            m9,
            m13,
            m2,
            m6,
            m10,
            m14,
            m3,
            m7,
            m11,
            m15,
        }
    }
    pub const IDENTITY: Self = Self::identity();
    pub const fn identity() -> Self {
        Self {
            m0: 1.0,
            m4: 0.0,
            m8: 0.0,
            m12: 0.0,

            m1: 0.0,
            m5: 1.0,
            m9: 0.0,
            m13: 0.0,

            m2: 0.0,
            m6: 0.0,
            m10: 1.0,
            m14: 0.0,

            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }

    /// Explicit conversion to column-major memory layout
    pub fn to_array(self) -> [f32; 16] {
        [
            self.m0, self.m1, self.m2, self.m3, self.m4, self.m5, self.m6, self.m7, self.m8,
            self.m9, self.m10, self.m11, self.m12, self.m13, self.m14, self.m15,
        ]
    }

    pub fn transpose(self) -> Self {
        Self {
            m0: self.m0,
            m1: self.m4,
            m2: self.m8,
            m3: self.m12,

            m4: self.m1,
            m5: self.m5,
            m6: self.m9,
            m7: self.m13,

            m8: self.m2,
            m9: self.m6,
            m10: self.m10,
            m11: self.m14,

            m12: self.m3,
            m13: self.m7,
            m14: self.m11,
            m15: self.m15,
        }
    }

    pub fn invert(self) -> Self {
        let a00 = self.m0;
        let a01 = self.m1;
        let a02 = self.m2;
        let a03 = self.m3;

        let a10 = self.m4;
        let a11 = self.m5;
        let a12 = self.m6;
        let a13 = self.m7;

        let a20 = self.m8;
        let a21 = self.m9;
        let a22 = self.m10;
        let a23 = self.m11;

        let a30 = self.m12;
        let a31 = self.m13;
        let a32 = self.m14;
        let a33 = self.m15;

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        let inv_det = 1.0 / (b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06);

        Self {
            m0: (a11 * b11 - a12 * b10 + a13 * b09) * inv_det,
            m1: (-a01 * b11 + a02 * b10 - a03 * b09) * inv_det,
            m2: (a31 * b05 - a32 * b04 + a33 * b03) * inv_det,
            m3: (-a21 * b05 + a22 * b04 - a23 * b03) * inv_det,

            m4: (-a10 * b11 + a12 * b08 - a13 * b07) * inv_det,
            m5: (a00 * b11 - a02 * b08 + a03 * b07) * inv_det,
            m6: (-a30 * b05 + a32 * b02 - a33 * b01) * inv_det,
            m7: (a20 * b05 - a22 * b02 + a23 * b01) * inv_det,

            m8: (a10 * b10 - a11 * b08 + a13 * b06) * inv_det,
            m9: (-a00 * b10 + a01 * b08 - a03 * b06) * inv_det,
            m10: (a30 * b04 - a31 * b02 + a33 * b00) * inv_det,
            m11: (-a20 * b04 + a21 * b02 - a23 * b00) * inv_det,

            m12: (-a10 * b09 + a11 * b07 - a12 * b06) * inv_det,
            m13: (a00 * b09 - a01 * b07 + a02 * b06) * inv_det,
            m14: (-a30 * b03 + a31 * b01 - a32 * b00) * inv_det,
            m15: (a20 * b03 - a21 * b01 + a22 * b00) * inv_det,
        }
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            m0: self.m0 * rhs.m0 + self.m1 * rhs.m4 + self.m2 * rhs.m8 + self.m3 * rhs.m12,

            m1: self.m0 * rhs.m1 + self.m1 * rhs.m5 + self.m2 * rhs.m9 + self.m3 * rhs.m13,

            m2: self.m0 * rhs.m2 + self.m1 * rhs.m6 + self.m2 * rhs.m10 + self.m3 * rhs.m14,

            m3: self.m0 * rhs.m3 + self.m1 * rhs.m7 + self.m2 * rhs.m11 + self.m3 * rhs.m15,

            m4: self.m4 * rhs.m0 + self.m5 * rhs.m4 + self.m6 * rhs.m8 + self.m7 * rhs.m12,

            m5: self.m4 * rhs.m1 + self.m5 * rhs.m5 + self.m6 * rhs.m9 + self.m7 * rhs.m13,

            m6: self.m4 * rhs.m2 + self.m5 * rhs.m6 + self.m6 * rhs.m10 + self.m7 * rhs.m14,

            m7: self.m4 * rhs.m3 + self.m5 * rhs.m7 + self.m6 * rhs.m11 + self.m7 * rhs.m15,

            m8: self.m8 * rhs.m0 + self.m9 * rhs.m4 + self.m10 * rhs.m8 + self.m11 * rhs.m12,

            m9: self.m8 * rhs.m1 + self.m9 * rhs.m5 + self.m10 * rhs.m9 + self.m11 * rhs.m13,

            m10: self.m8 * rhs.m2 + self.m9 * rhs.m6 + self.m10 * rhs.m10 + self.m11 * rhs.m14,

            m11: self.m8 * rhs.m3 + self.m9 * rhs.m7 + self.m10 * rhs.m11 + self.m11 * rhs.m15,

            m12: self.m12 * rhs.m0 + self.m13 * rhs.m4 + self.m14 * rhs.m8 + self.m15 * rhs.m12,

            m13: self.m12 * rhs.m1 + self.m13 * rhs.m5 + self.m14 * rhs.m9 + self.m15 * rhs.m13,

            m14: self.m12 * rhs.m2 + self.m13 * rhs.m6 + self.m14 * rhs.m10 + self.m15 * rhs.m14,

            m15: self.m12 * rhs.m3 + self.m13 * rhs.m7 + self.m14 * rhs.m11 + self.m15 * rhs.m15,
        }
    }
}

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

    pub const LIGHTGRAY: Color = Color::new(200, 200, 200, 255);
    pub const GRAY: Color = Color::new(130, 130, 130, 255);
    pub const DARKGRAY: Color = Color::new(80, 80, 80, 255);
    pub const YELLOW: Color = Color::new(253, 249, 0, 255);
    pub const GOLD: Color = Color::new(255, 203, 0, 255);
    pub const ORANGE: Color = Color::new(255, 161, 0, 255);
    pub const PINK: Color = Color::new(255, 109, 194, 255);
    pub const RED: Color = Color::new(230, 41, 55, 255);
    pub const MAROON: Color = Color::new(190, 33, 55, 255);
    pub const GREEN: Color = Color::new(0, 228, 48, 255);
    pub const LIME: Color = Color::new(0, 158, 47, 255);
    pub const DARKGREEN: Color = Color::new(0, 117, 44, 255);
    pub const SKYBLUE: Color = Color::new(102, 191, 255, 255);
    pub const BLUE: Color = Color::new(0, 121, 241, 255);
    pub const DARKBLUE: Color = Color::new(0, 82, 172, 255);
    pub const PURPLE: Color = Color::new(200, 122, 255, 255);
    pub const VIOLET: Color = Color::new(135, 60, 190, 255);
    pub const DARKPURPLE: Color = Color::new(112, 31, 126, 255);
    pub const BEIGE: Color = Color::new(211, 176, 131, 255);
    pub const BROWN: Color = Color::new(127, 106, 79, 255);
    pub const DARKBROWN: Color = Color::new(76, 63, 47, 255);
    pub const WHITE: Color = Color::new(255, 255, 255, 255);
    pub const BLACK: Color = Color::new(0, 0, 0, 255);
    pub const BLANK: Color = Color::new(0, 0, 0, 0);
    pub const MAGENTA: Color = Color::new(255, 0, 255, 255);
    pub const RAYWHITE: Color = Color::new(245, 245, 245, 255);
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
        Self {
            x,
            y,
            width,
            height,
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum CameraProjection {
    Perspective = 0,
    Orthographic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum CameraMode {
    Custom = 0,
    Free,
    Orbital,
    FirstPerson,
    ThirdPerson,
}

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
pub const LIGHTGRAY: Color = Color {
    r: 200,
    g: 200,
    b: 200,
    a: 255,
};
pub const GRAY: Color = Color {
    r: 130,
    g: 130,
    b: 130,
    a: 255,
};
pub const DARKGRAY: Color = Color {
    r: 80,
    g: 80,
    b: 80,
    a: 255,
};
pub const YELLOW: Color = Color {
    r: 253,
    g: 249,
    b: 0,
    a: 255,
};
pub const GOLD: Color = Color {
    r: 255,
    g: 203,
    b: 0,
    a: 255,
};
pub const ORANGE: Color = Color {
    r: 255,
    g: 161,
    b: 0,
    a: 255,
};
pub const PINK: Color = Color {
    r: 255,
    g: 109,
    b: 194,
    a: 255,
};
pub const RED: Color = Color {
    r: 230,
    g: 41,
    b: 55,
    a: 255,
};
pub const MAROON: Color = Color {
    r: 190,
    g: 33,
    b: 55,
    a: 255,
};
pub const GREEN: Color = Color {
    r: 0,
    g: 228,
    b: 48,
    a: 255,
};
pub const LIME: Color = Color {
    r: 0,
    g: 158,
    b: 47,
    a: 255,
};
pub const DARKGREEN: Color = Color {
    r: 0,
    g: 117,
    b: 44,
    a: 255,
};
pub const SKYBLUE: Color = Color {
    r: 102,
    g: 191,
    b: 255,
    a: 255,
};
pub const BLUE: Color = Color {
    r: 0,
    g: 121,
    b: 241,
    a: 255,
};
pub const DARKBLUE: Color = Color {
    r: 0,
    g: 82,
    b: 172,
    a: 255,
};
pub const PURPLE: Color = Color {
    r: 200,
    g: 122,
    b: 255,
    a: 255,
};
pub const VIOLET: Color = Color {
    r: 135,
    g: 60,
    b: 190,
    a: 255,
};
pub const DARKPURPLE: Color = Color {
    r: 112,
    g: 31,
    b: 126,
    a: 255,
};
pub const BEIGE: Color = Color {
    r: 211,
    g: 176,
    b: 131,
    a: 255,
};
pub const BROWN: Color = Color {
    r: 127,
    g: 106,
    b: 79,
    a: 255,
};
pub const DARKBROWN: Color = Color {
    r: 76,
    g: 63,
    b: 47,
    a: 255,
};
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};
pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};
pub const BLANK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};
pub const MAGENTA: Color = Color {
    r: 255,
    g: 0,
    b: 255,
    a: 255,
};
pub const RAYWHITE: Color = Color {
    r: 245,
    g: 245,
    b: 245,
    a: 255,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NPatchLayout {
    NinePatch = 0,        // Npatch layout: 3x3 tiles
    ThreePatchVertical,   // Npatch layout: 1x3 tiles
    ThreePatchHorizontal, // Npatch layout: 3x1 tiles
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NPatchInfo {
    pub source: Rectangle, // Texture source rectangle
    pub left: i32,         // Left border offset
    pub top: i32,          // Top border offset
    pub right: i32,        // Right border offset
    pub bottom: i32,       // Bottom border offset
    pub layout: i32,       // Layout of the n-patch: 3x3, 1x3 or 3x1
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
#[derive(Debug, Clone, Copy)]
pub struct Font {
    pub base_size: i32,
    pub glyph_count: i32,
    pub glyph_padding: i32,
    pub texture: Texture,
    pub recs: *mut Rectangle,
    pub glyphs: *mut GlyphInfo,
}
