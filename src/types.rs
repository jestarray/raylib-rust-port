use glam::{Mat4, Vec2, Vec3, Vec4};
use image::ColorType;

pub type Vector2 = Vec2;
pub type Vector3 = Vec3;
pub type Vector4 = Vec4;
pub type Quaternion = Vec4;
/// Raylib's matrix type.
///
/// Raylib and glam both use column-major, right-handed 4x4 matrices. OpenGL
/// projection constructors must use glam's `camera::rh::proj::opengl` module.
/// Note that glam defaults a matrix to `IDENTITY`; use `Matrix::ZERO` when a
/// zero-initialized matrix is required.
pub type Matrix = Mat4;

pub const RAYLIB_VERSION: &str = "6.1-dev";
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const ZERO: Self = Self::new(0, 0, 0, 0);
    pub const TRANS: Self = Self::ZERO;
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Image {
    pub data: Vec<u8>,
    pub width: i32,
    pub height: i32,
    pub mipmaps: i32,
    pub format: i32,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            width: 0,
            height: 0,
            mipmaps: 1,
            format: 0,
        }
    }
}

impl Image {
    #[must_use]
    pub fn new(data: Vec<u8>, width: i32, height: i32, mipmaps: i32, format: i32) -> Self {
        Self {
            data,
            width,
            height,
            mipmaps,
            format,
        }
    }
    #[must_use]
    pub fn is_data_null(&self) -> bool {
        self.data.is_empty()
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
    CAMERA_PERSPECTIVE = 0,
    CAMERA_ORTHOGRAPHIC,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum CameraMode {
    CAMERA_CUSTOM = 0,
    CAMERA_FREE,
    CAMERA_ORBITAL,
    CAMERA_FIRST_PERSON,
    CAMERA_THIRD_PERSON,
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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Camera2D {
    pub offset: Vector2,
    pub target: Vector2,
    pub rotation: f32,
    pub zoom: f32,
}

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
#[derive(Debug, Clone, Default)]
pub struct GlyphInfo {
    pub value: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub advance_x: i32,
    pub image: Image,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Font {
    pub baseSize: i32,
    pub glyphCount: i32,
    pub glyphPadding: i32,
    pub texture: Texture,
    pub recs: Vec<Rectangle>,
    pub glyphs: Vec<GlyphInfo>,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[rustfmt::skip]
pub enum ConfigFlags {
    FLAG_FULLSCREEN_MODE            = 0x00000002,   // Set to run program in fullscreen
    FLAG_WINDOW_RESIZABLE            = 0x00000004,   // Set to allow resizable window
    FLAG_WINDOW_UNDECORATED          = 0x00000008,   // Set to disable window decoration (frame and buttons)
    FLAG_WINDOW_TRANSPARENT          = 0x00000010,   // Set to allow transparent framebuffer
    FLAG_MSAA_4X_HINT                = 0x00000020,   // Set to try enabling MSAA 4X
    FLAG_VSYNC_HINT                  = 0x00000040,   // Set to try enabling V-Sync on GPU
    FLAG_WINDOW_HIDDEN               = 0x00000080,   // Set to hide window
    FLAG_WINDOW_ALWAYS_RUN            = 0x00000100,   // Set to allow windows running while minimized
    FLAG_WINDOW_MINIMIZED             = 0x00000200,   // Set to minimize window (iconify)
    FLAG_WINDOW_MAXIMIZED             = 0x00000400,   // Set to maximize window (expanded to monitor)
    FLAG_WINDOW_UNFOCUSED             = 0x00000800,   // Set to window non focused
    FLAG_WINDOW_TOPMOST               = 0x00001000,   // Set to window always on top
    FLAG_WINDOW_HIGHDPI               = 0x00002000,   // Set to support HighDPI
    FLAG_WINDOW_MOUSE_PASSTHROUGH    = 0x00004000,   // Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
    FLAG_BORDERLESS_WINDOWED_MODE     = 0x00008000,   // Set to run program in borderless windowed mode
    FLAG_INTERLACED_HINT              = 0x00010000,   // Set to try enabling interlaced video format (for V3D)
    FLAG_GL_CONTEXT_DEBUG                 = 0x00020000,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TraceLogLevel {
    LOG_ALL = 0,
    LOG_TRACE = 1,
    LOG_DEBUG = 2,
    LOG_INFO = 3,
    LOG_WARNING = 4,
    LOG_ERROR = 5,
    LOG_FATAL = 6,
    LOG_NONE = 7,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum KeyboardKey {
    KEY_NULL = 0,
    KEY_APOSTROPHE = 39,
    KEY_COMMA = 44,
    KEY_MINUS = 45,
    KEY_PERIOD = 46,
    KEY_SLASH = 47,
    KEY_ZERO = 48,
    KEY_ONE = 49,
    KEY_TWO = 50,
    KEY_THREE = 51,
    KEY_FOUR = 52,
    KEY_FIVE = 53,
    KEY_SIX = 54,
    KEY_SEVEN = 55,
    KEY_EIGHT = 56,
    KEY_NINE = 57,
    KEY_SEMICOLON = 59,
    KEY_EQUAL = 61,
    KEY_A = 65,
    KEY_B = 66,
    KEY_C = 67,
    KEY_D = 68,
    KEY_E = 69,
    KEY_F = 70,
    KEY_G = 71,
    KEY_H = 72,
    KEY_I = 73,
    KEY_J = 74,
    KEY_K = 75,
    KEY_L = 76,
    KEY_M = 77,
    KEY_N = 78,
    KEY_O = 79,
    KEY_P = 80,
    KEY_Q = 81,
    KEY_R = 82,
    KEY_S = 83,
    KEY_T = 84,
    KEY_U = 85,
    KEY_V = 86,
    KEY_W = 87,
    KEY_X = 88,
    KEY_Y = 89,
    KEY_Z = 90,
    KEY_LEFT_BRACKET = 91,
    KEY_BACKSLASH = 92,
    KEY_RIGHT_BRACKET = 93,
    KEY_GRAVE = 96,
    KEY_SPACE = 32,
    KEY_ESCAPE = 256,
    KEY_ENTER = 257,
    KEY_TAB = 258,
    KEY_BACKSPACE = 259,
    KEY_INSERT = 260,
    KEY_DELETE = 261,
    KEY_RIGHT = 262,
    KEY_LEFT = 263,
    KEY_DOWN = 264,
    KEY_UP = 265,
    KEY_PAGE_UP = 266,
    KEY_PAGE_DOWN = 267,
    KEY_HOME = 268,
    KEY_END = 269,
    KEY_CAPS_LOCK = 280,
    KEY_SCROLL_LOCK = 281,
    KEY_NUM_LOCK = 282,
    KEY_PRINT_SCREEN = 283,
    KEY_PAUSE = 284,
    KEY_F1 = 290,
    KEY_F2 = 291,
    KEY_F3 = 292,
    KEY_F4 = 293,
    KEY_F5 = 294,
    KEY_F6 = 295,
    KEY_F7 = 296,
    KEY_F8 = 297,
    KEY_F9 = 298,
    KEY_F10 = 299,
    KEY_F11 = 300,
    KEY_F12 = 301,
    KEY_LEFT_SHIFT = 340,
    KEY_LEFT_CONTROL = 341,
    KEY_LEFT_ALT = 342,
    KEY_LEFT_SUPER = 343,
    KEY_RIGHT_SHIFT = 344,
    KEY_RIGHT_CONTROL = 345,
    KEY_RIGHT_ALT = 346,
    KEY_RIGHT_SUPER = 347,
    KEY_KB_MENU = 348,
    KEY_KP_0 = 320,
    KEY_KP_1 = 321,
    KEY_KP_2 = 322,
    KEY_KP_3 = 323,
    KEY_KP_4 = 324,
    KEY_KP_5 = 325,
    KEY_KP_6 = 326,
    KEY_KP_7 = 327,
    KEY_KP_8 = 328,
    KEY_KP_9 = 329,
    KEY_KP_DECIMAL = 330,
    KEY_KP_DIVIDE = 331,
    KEY_KP_MULTIPLY = 332,
    KEY_KP_SUBTRACT = 333,
    KEY_KP_ADD = 334,
    KEY_KP_ENTER = 335,
    KEY_KP_EQUAL = 336,
    KEY_BACK = 4,
    KEY_MENU = 5,
    KEY_VOLUME_UP = 24,
    KEY_VOLUME_DOWN = 25,
}
impl From<KeyboardKey> for i32 {
    fn from(val: KeyboardKey) -> Self {
        val as i32
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MouseButton {
    MOUSE_BUTTON_LEFT = 0,
    MOUSE_BUTTON_RIGHT = 1,
    MOUSE_BUTTON_MIDDLE = 2,
    MOUSE_BUTTON_SIDE = 3,
    MOUSE_BUTTON_EXTRA = 4,
    MOUSE_BUTTON_FORWARD = 5,
    MOUSE_BUTTON_BACK = 6,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MouseCursor {
    MOUSE_CURSOR_DEFAULT = 0,
    MOUSE_CURSOR_ARROW = 1,
    MOUSE_CURSOR_IBEAM = 2,
    MOUSE_CURSOR_CROSSHAIR = 3,
    MOUSE_CURSOR_POINTING_HAND = 4,
    MOUSE_CURSOR_RESIZE_EW = 5,
    MOUSE_CURSOR_RESIZE_NS = 6,
    MOUSE_CURSOR_RESIZE_NWSE = 7,
    MOUSE_CURSOR_RESIZE_NESW = 8,
    MOUSE_CURSOR_RESIZE_ALL = 9,
    MOUSE_CURSOR_NOT_ALLOWED = 10,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GamepadButton {
    GAMEPAD_BUTTON_UNKNOWN = 0,      // Unknown button, for error checking
    GAMEPAD_BUTTON_LEFT_FACE_UP,     // Gamepad left DPAD up button
    GAMEPAD_BUTTON_LEFT_FACE_RIGHT,  // Gamepad left DPAD right button
    GAMEPAD_BUTTON_LEFT_FACE_DOWN,   // Gamepad left DPAD down button
    GAMEPAD_BUTTON_LEFT_FACE_LEFT,   // Gamepad left DPAD left button
    GAMEPAD_BUTTON_RIGHT_FACE_UP,    // Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
    GAMEPAD_BUTTON_RIGHT_FACE_RIGHT, // Gamepad right button right (i.e. PS3: Circle, Xbox: B)
    GAMEPAD_BUTTON_RIGHT_FACE_DOWN,  // Gamepad right button down (i.e. PS3: Cross, Xbox: A)
    GAMEPAD_BUTTON_RIGHT_FACE_LEFT,  // Gamepad right button left (i.e. PS3: Square, Xbox: X)
    GAMEPAD_BUTTON_LEFT_TRIGGER_1, // Gamepad top/back trigger left (first), it could be a trailing button
    GAMEPAD_BUTTON_LEFT_TRIGGER_2, // Gamepad top/back trigger left (second), it could be a trailing button
    GAMEPAD_BUTTON_RIGHT_TRIGGER_1, // Gamepad top/back trigger right (first), it could be a trailing button
    GAMEPAD_BUTTON_RIGHT_TRIGGER_2, // Gamepad top/back trigger right (second), it could be a trailing button
    GAMEPAD_BUTTON_MIDDLE_LEFT,     // Gamepad center buttons, left one (i.e. PS3: Select)
    GAMEPAD_BUTTON_MIDDLE,          // Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
    GAMEPAD_BUTTON_MIDDLE_RIGHT,    // Gamepad center buttons, right one (i.e. PS3: Start)
    GAMEPAD_BUTTON_LEFT_THUMB,      // Gamepad joystick pressed button left
    GAMEPAD_BUTTON_RIGHT_THUMB,     // Gamepad joystick pressed button right
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GamepadAxis {
    GAMEPAD_AXIS_LEFT_X = 0,
    GAMEPAD_AXIS_LEFT_Y = 1,
    GAMEPAD_AXIS_RIGHT_X = 2,
    GAMEPAD_AXIS_RIGHT_Y = 3,
    GAMEPAD_AXIS_LEFT_TRIGGER = 4,
    GAMEPAD_AXIS_RIGHT_TRIGGER = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MaterialMapIndex {
    MATERIAL_MAP_ALBEDO = 0,
    MATERIAL_MAP_METALNESS = 1,
    MATERIAL_MAP_NORMAL = 2,
    MATERIAL_MAP_ROUGHNESS = 3,
    MATERIAL_MAP_OCCLUSION = 4,
    MATERIAL_MAP_EMISSION = 5,
    MATERIAL_MAP_HEIGHT = 6,
    MATERIAL_MAP_CUBEMAP = 7,
    MATERIAL_MAP_IRRADIANCE = 8,
    MATERIAL_MAP_PREFILTER = 9,
    MATERIAL_MAP_BRDF = 10,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ShaderLocationIndex {
    SHADER_LOC_VERTEX_POSITION = 0,
    SHADER_LOC_VERTEX_TEXCOORD01 = 1,
    SHADER_LOC_VERTEX_TEXCOORD02 = 2,
    SHADER_LOC_VERTEX_NORMAL = 3,
    SHADER_LOC_VERTEX_TANGENT = 4,
    SHADER_LOC_VERTEX_COLOR = 5,
    SHADER_LOC_MATRIX_MVP = 6,
    SHADER_LOC_MATRIX_VIEW = 7,
    SHADER_LOC_MATRIX_PROJECTION = 8,
    SHADER_LOC_MATRIX_MODEL = 9,
    SHADER_LOC_MATRIX_NORMAL = 10,
    SHADER_LOC_VECTOR_VIEW = 11,
    SHADER_LOC_COLOR_DIFFUSE = 12,
    SHADER_LOC_COLOR_SPECULAR = 13,
    SHADER_LOC_COLOR_AMBIENT = 14,
    SHADER_LOC_MAP_ALBEDO = 15,
    SHADER_LOC_MAP_METALNESS = 16,
    SHADER_LOC_MAP_NORMAL = 17,
    SHADER_LOC_MAP_ROUGHNESS = 18,
    SHADER_LOC_MAP_OCCLUSION = 19,
    SHADER_LOC_MAP_EMISSION = 20,
    SHADER_LOC_MAP_HEIGHT = 21,
    SHADER_LOC_MAP_CUBEMAP = 22,
    SHADER_LOC_MAP_IRRADIANCE = 23,
    SHADER_LOC_MAP_PREFILTER = 24,
    SHADER_LOC_MAP_BRDF = 25,
    SHADER_LOC_VERTEX_BONEIDS = 26,
    SHADER_LOC_VERTEX_BONEWEIGHTS = 27,
    SHADER_LOC_MATRIX_BONETRANSFORMS = 28,
    SHADER_LOC_VERTEX_INSTANCETRANSFORM = 29,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ShaderUniformDataType {
    SHADER_UNIFORM_FLOAT = 0,
    SHADER_UNIFORM_VEC2 = 1,
    SHADER_UNIFORM_VEC3 = 2,
    SHADER_UNIFORM_VEC4 = 3,
    SHADER_UNIFORM_INT = 4,
    SHADER_UNIFORM_IVEC2 = 5,
    SHADER_UNIFORM_IVEC3 = 6,
    SHADER_UNIFORM_IVEC4 = 7,
    SHADER_UNIFORM_UINT = 8,
    SHADER_UNIFORM_UIVEC2 = 9,
    SHADER_UNIFORM_UIVEC3 = 10,
    SHADER_UNIFORM_UIVEC4 = 11,
    SHADER_UNIFORM_SAMPLER2D = 12,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ShaderAttributeDataType {
    SHADER_ATTRIB_FLOAT = 0,
    SHADER_ATTRIB_VEC2 = 1,
    SHADER_ATTRIB_VEC3 = 2,
    SHADER_ATTRIB_VEC4 = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PixelFormat {
    PIXELFORMAT_UNCOMPRESSED_GRAYSCALE = 1,
    PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA = 2,
    PIXELFORMAT_UNCOMPRESSED_R5G6B5 = 3,
    PIXELFORMAT_UNCOMPRESSED_R8G8B8 = 4,
    PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 = 5,
    PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 = 6,
    PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 = 7,
    PIXELFORMAT_UNCOMPRESSED_R32 = 8,
    PIXELFORMAT_UNCOMPRESSED_R32G32B32 = 9,
    PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 = 10,
    PIXELFORMAT_UNCOMPRESSED_R16 = 11,
    PIXELFORMAT_UNCOMPRESSED_R16G16B16 = 12,
    PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 = 13,
    PIXELFORMAT_COMPRESSED_DXT1_RGB = 14,
    PIXELFORMAT_COMPRESSED_DXT1_RGBA = 15,
    PIXELFORMAT_COMPRESSED_DXT3_RGBA = 16,
    PIXELFORMAT_COMPRESSED_DXT5_RGBA = 17,
    PIXELFORMAT_COMPRESSED_ETC1_RGB = 18,
    PIXELFORMAT_COMPRESSED_ETC2_RGB = 19,
    PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA = 20,
    PIXELFORMAT_COMPRESSED_PVRT_RGB = 21,
    PIXELFORMAT_COMPRESSED_PVRT_RGBA = 22,
    PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA = 23,
    PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA = 24,
}
impl PixelFormat {
    #[must_use]
    pub fn to_color_type(self) -> Option<ColorType> {
        match self {
            Self::PIXELFORMAT_UNCOMPRESSED_GRAYSCALE => Some(ColorType::L8),
            Self::PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA => Some(ColorType::La8),
            Self::PIXELFORMAT_UNCOMPRESSED_R8G8B8 => Some(ColorType::Rgb8),
            Self::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 => Some(ColorType::Rgba8),
            Self::PIXELFORMAT_UNCOMPRESSED_R16 => Some(ColorType::L16),
            Self::PIXELFORMAT_UNCOMPRESSED_R16G16B16 => Some(ColorType::Rgb16),
            Self::PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 => Some(ColorType::Rgba16),
            Self::PIXELFORMAT_UNCOMPRESSED_R32G32B32 => Some(ColorType::Rgb32F),
            Self::PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 => Some(ColorType::Rgba32F),

            _ => None,
        }
    }
    #[must_use]
    pub fn from_color_type(color_type: ColorType) -> Option<PixelFormat> {
        match color_type {
            ColorType::L8 => Some(Self::PIXELFORMAT_UNCOMPRESSED_GRAYSCALE),
            ColorType::La8 => Some(Self::PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA),
            ColorType::Rgb8 => Some(Self::PIXELFORMAT_UNCOMPRESSED_R8G8B8),
            ColorType::Rgba8 => Some(Self::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8),
            ColorType::L16 => Some(Self::PIXELFORMAT_UNCOMPRESSED_R16),
            ColorType::Rgb16 => Some(Self::PIXELFORMAT_UNCOMPRESSED_R16G16B16),
            ColorType::Rgba16 => Some(Self::PIXELFORMAT_UNCOMPRESSED_R16G16B16A16),
            ColorType::Rgb32F => Some(Self::PIXELFORMAT_UNCOMPRESSED_R32G32B32),
            ColorType::Rgba32F => Some(Self::PIXELFORMAT_UNCOMPRESSED_R32G32B32A32),
            _ => None,
        }
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextureFilter {
    TEXTURE_FILTER_POINT = 0,
    TEXTURE_FILTER_BILINEAR = 1,
    TEXTURE_FILTER_TRILINEAR = 2,
    TEXTURE_FILTER_ANISOTROPIC_4X = 3,
    TEXTURE_FILTER_ANISOTROPIC_8X = 4,
    TEXTURE_FILTER_ANISOTROPIC_16X = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextureWrap {
    TEXTURE_WRAP_REPEAT = 0,
    TEXTURE_WRAP_CLAMP = 1,
    TEXTURE_WRAP_MIRROR_REPEAT = 2,
    TEXTURE_WRAP_MIRROR_CLAMP = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CubemapLayout {
    CUBEMAP_LAYOUT_AUTO_DETECT = 0,
    CUBEMAP_LAYOUT_LINE_VERTICAL = 1,
    CUBEMAP_LAYOUT_LINE_HORIZONTAL = 2,
    CUBEMAP_LAYOUT_CROSS_THREE_BY_FOUR = 3,
    CUBEMAP_LAYOUT_CROSS_FOUR_BY_THREE = 4,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FontType {
    FONT_DEFAULT = 0,
    FONT_BITMAP = 1,
    FONT_SDF = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BlendMode {
    BLEND_ALPHA = 0,
    BLEND_ADDITIVE = 1,
    BLEND_MULTIPLIED = 2,
    BLEND_ADD_COLORS = 3,
    BLEND_SUBTRACT_COLORS = 4,
    BLEND_ALPHA_PREMULTIPLY = 5,
    BLEND_CUSTOM = 6,
    BLEND_CUSTOM_SEPARATE = 7,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Gesture {
    GESTURE_NONE = 0,
    GESTURE_TAP = 1,
    GESTURE_DOUBLETAP = 2,
    GESTURE_HOLD = 4,
    GESTURE_DRAG = 8,
    GESTURE_SWIPE_RIGHT = 16,
    GESTURE_SWIPE_LEFT = 32,
    GESTURE_SWIPE_UP = 64,
    GESTURE_SWIPE_DOWN = 128,
    GESTURE_PINCH_IN = 256,
    GESTURE_PINCH_OUT = 512,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Shader {
    pub id: u32,
    pub locs: Vec<i32>,
}

// use crate::Matrix; // or import the matching C-compatible Matrix type

// VrDeviceInfo, Head-Mounted-Display device parameters
#[repr(C)]
#[allow(non_snake_case)]
pub struct VrDeviceInfo {
    pub hResolution: i32,               // Horizontal resolution in pixels
    pub vResolution: i32,               // Vertical resolution in pixels
    pub hScreenSize: f32,               // Horizontal size in meters
    pub vScreenSize: f32,               // Vertical size in meters
    pub eyeToScreenDistance: f32,       // Distance between eye and display in meters
    pub lensSeparationDistance: f32,    // Lens separation distance in meters
    pub interpupillaryDistance: f32,    // IPD (distance between pupils) in meters
    pub lensDistortionValues: [f32; 4], // Lens distortion constant parameters
    pub chromaAbCorrection: [f32; 4],   // Chromatic aberration correction parameters
}

// VrStereoConfig, VR stereo rendering configuration for simulator
#[repr(C)]
#[allow(non_snake_case)]
pub struct VrStereoConfig {
    pub projection: [Matrix; 2],     // VR projection matrices (per eye)
    pub viewOffset: [Matrix; 2],     // VR view offset matrices (per eye)
    pub leftLensCenter: [f32; 2],    // VR left lens center
    pub rightLensCenter: [f32; 2],   // VR right lens center
    pub leftScreenCenter: [f32; 2],  // VR left screen center
    pub rightScreenCenter: [f32; 2], // VR right screen center
    pub scale: [f32; 2],             // VR distortion scale
    pub scaleIn: [f32; 2],           // VR distortion scale in
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct FilePathList {
    pub count: u32,                        // Filepaths entries count
    pub paths: *mut *mut std::ffi::c_char, // Filepaths entries
}

// Automation event
#[repr(C)]
#[allow(non_snake_case)]
#[derive(Copy, Clone, Default)]
pub struct AutomationEvent {
    pub frame: u32,       // Event frame
    pub type_: u32,       // Event type (AutomationEventType)
    pub params: [i32; 4], // Event parameters (if required)
}

// Automation event list
#[repr(C)]
#[allow(non_snake_case)]
#[derive(Clone)]
pub struct AutomationEventList {
    pub capacity: u32,                // Events max entries (MAX_AUTOMATION_EVENTS)
    pub count: u32,                   // Events entries count
    pub events: Vec<AutomationEvent>, // Events entries
}
impl AutomationEventList {
    pub fn with_capacity(cap: u32) -> Self {
        Self {
            capacity: cap,
            count: 0,
            events: vec![AutomationEvent::default(); cap as usize],
        }
    }
}

#[repr(C)]
pub struct Ray {
    pub position: Vector3,  // Ray position (origin)
    pub direction: Vector3, // Ray direction (normalized)
}
