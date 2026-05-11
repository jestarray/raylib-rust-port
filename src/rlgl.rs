#![allow(missing_safety_doc, unused_parens, non_snake_case, static_mut_refs)]
#![allow(
    clippy::too_many_arguments,
    clippy::needless_return,
    clippy::manual_range_contains,
    clippy::field_reassign_with_default,
    clippy::manual_map,
    clippy::match_like_matches_macro,
    clippy::upper_case_acronyms,
    clippy::let_and_return
)]
use std::ptr::null_mut;

use crate::types::{Color, Matrix, Rectangle, Vector2};
use gl;
use glam::{Mat4, Vec3};
use log::{debug, error, info, warn};

// --- Constants ---

pub const RL_DEFAULT_BATCH_BUFFER_ELEMENTS: i32 = 8192;
pub const RL_DEFAULT_BATCH_BUFFERS: i32 = 1;
pub const RL_DEFAULT_BATCH_DRAWCALLS: i32 = 256;
pub const RL_DEFAULT_BATCH_MAX_TEXTURE_UNITS: i32 = 4;
pub const RL_MAX_MATRIX_STACK_SIZE: usize = 32;
pub const RL_MAX_SHADER_LOCATIONS: usize = 32;
pub const RL_CULL_DISTANCE_NEAR: f64 = 0.05;
pub const RL_CULL_DISTANCE_FAR: f64 = 4000.0;

pub const RL_TEXTURE_WRAP_S: u32 = 0x2802;
pub const RL_TEXTURE_WRAP_T: u32 = 0x2803;
pub const RL_TEXTURE_MAG_FILTER: u32 = 0x2800;
pub const RL_TEXTURE_MIN_FILTER: u32 = 0x2801;

pub const RL_TEXTURE_FILTER_NEAREST: u32 = 0x2600;
pub const RL_TEXTURE_FILTER_LINEAR: u32 = 0x2601;
pub const RL_TEXTURE_FILTER_MIP_NEAREST: u32 = 0x2700;
pub const RL_TEXTURE_FILTER_NEAREST_MIP_LINEAR: u32 = 0x2702;
pub const RL_TEXTURE_FILTER_LINEAR_MIP_NEAREST: u32 = 0x2701;
pub const RL_TEXTURE_FILTER_MIP_LINEAR: u32 = 0x2703;
pub const RL_TEXTURE_FILTER_ANISOTROPIC: u32 = 0x3000;
pub const RL_TEXTURE_MIPMAP_BIAS_RATIO: u32 = 0x4000;

pub const RL_TEXTURE_WRAP_REPEAT: u32 = 0x2901;
pub const RL_TEXTURE_WRAP_CLAMP: u32 = 0x812F;
pub const RL_TEXTURE_WRAP_MIRROR_REPEAT: u32 = 0x8370;
pub const RL_TEXTURE_WRAP_MIRROR_CLAMP: u32 = 0x8742;

pub const RL_MODELVIEW: i32 = 0x1700;
pub const RL_PROJECTION: i32 = 0x1701;
pub const RL_TEXTURE: i32 = 0x1702;

pub const RL_LINES: i32 = 0x0001;
pub const RL_TRIANGLES: i32 = 0x0004;
pub const RL_QUADS: i32 = 0x0007;

pub const RL_UNSIGNED_BYTE: u32 = 0x1401;
pub const RL_FLOAT: u32 = 0x1406;

pub const RL_STREAM_DRAW: u32 = 0x88E0;
pub const RL_STREAM_READ: u32 = 0x88E1;
pub const RL_STREAM_COPY: u32 = 0x88E2;
pub const RL_STATIC_DRAW: u32 = 0x88E4;
pub const RL_STATIC_READ: u32 = 0x88E5;
pub const RL_STATIC_COPY: u32 = 0x88E6;
pub const RL_DYNAMIC_DRAW: u32 = 0x88E8;
pub const RL_DYNAMIC_READ: u32 = 0x88E9;
pub const RL_DYNAMIC_COPY: u32 = 0x88EA;

pub const RL_FRAGMENT_SHADER: u32 = 0x8B30;
pub const RL_VERTEX_SHADER: u32 = 0x8B31;
pub const RL_COMPUTE_SHADER: u32 = 0x91B9;

pub const RL_ZERO: i32 = 0;
pub const RL_ONE: i32 = 1;
pub const RL_SRC_COLOR: i32 = 0x0300;
pub const RL_ONE_MINUS_SRC_COLOR: i32 = 0x0301;
pub const RL_SRC_ALPHA: i32 = 0x0302;
pub const RL_ONE_MINUS_SRC_ALPHA: i32 = 0x0303;
pub const RL_DST_ALPHA: i32 = 0x0304;
pub const RL_ONE_MINUS_DST_ALPHA: i32 = 0x0305;
pub const RL_DST_COLOR: i32 = 0x0306;
pub const RL_ONE_MINUS_DST_COLOR: i32 = 0x0307;
pub const RL_SRC_ALPHA_SATURATE: i32 = 0x0308;
pub const RL_CONSTANT_COLOR: i32 = 0x8001;
pub const RL_ONE_MINUS_CONSTANT_COLOR: i32 = 0x8002;
pub const RL_CONSTANT_ALPHA: i32 = 0x8003;
pub const RL_ONE_MINUS_CONSTANT_ALPHA: i32 = 0x8004;

pub const RL_FUNC_ADD: i32 = 0x8006;
pub const RL_MIN: i32 = 0x8007;
pub const RL_MAX: i32 = 0x8008;
pub const RL_FUNC_SUBTRACT: i32 = 0x800A;
pub const RL_FUNC_REVERSE_SUBTRACT: i32 = 0x800B;
pub const RL_BLEND_EQUATION: i32 = 0x8009;
pub const RL_BLEND_EQUATION_RGB: i32 = 0x8009;
pub const RL_BLEND_EQUATION_ALPHA: i32 = 0x883D;
pub const RL_BLEND_DST_RGB: i32 = 0x80C8;
pub const RL_BLEND_SRC_RGB: i32 = 0x80C9;
pub const RL_BLEND_DST_ALPHA: i32 = 0x80CA;
pub const RL_BLEND_SRC_ALPHA: i32 = 0x80CB;
pub const RL_BLEND_COLOR: i32 = 0x8005;

pub const RL_READ_FRAMEBUFFER: u32 = 0x8CA8;
pub const RL_DRAW_FRAMEBUFFER: u32 = 0x8CA9;

pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_POSITION: u32 = 0;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD: u32 = 1;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_NORMAL: u32 = 2;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_COLOR: u32 = 3;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_TANGENT: u32 = 4;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD2: u32 = 5;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_INDICES: u32 = 6;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_BONEINDICES: u32 = 7;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_BONEWEIGHTS: u32 = 8;
pub const RL_DEFAULT_SHADER_ATTRIB_LOCATION_INSTANCETRANSFORM: u32 = 9;

pub const RL_CULL_FACE_FRONT: i32 = 0;
pub const RL_CULL_FACE_BACK: i32 = 1;

pub const RL_FRONT: u32 = 0x0404;
pub const RL_BACK: u32 = 0x0405;
pub const RL_FRONT_AND_BACK: u32 = 0x0408;

#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum rlGlVersion {
    RL_OPENGL_SOFTWARE = 0, // Software rendering
    RL_OPENGL_11,           // OpenGL 1.1
    RL_OPENGL_21,           // OpenGL 2.1 (GLSL 120)
    RL_OPENGL_33,           // OpenGL 3.3 (GLSL 330)
    RL_OPENGL_43,           // OpenGL 4.3 (using GLSL 330)
    RL_OPENGL_ES_20,        // OpenGL ES 2.0 (GLSL 100)
    RL_OPENGL_ES_30,        // OpenGL ES 3.0 (GLSL 300 es)
}

#[repr(i32)]
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

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum rlTextureFilter {
    RL_TEXTURE_FILTER_POINT = 0,       // No filter, pixel approximation
    RL_TEXTURE_FILTER_BILINEAR,        // Linear filtering
    RL_TEXTURE_FILTER_TRILINEAR,       // Trilinear filtering (linear with mipmaps)
    RL_TEXTURE_FILTER_ANISOTROPIC_4X,  // Anisotropic filtering 4x
    RL_TEXTURE_FILTER_ANISOTROPIC_8X,  // Anisotropic filtering 8x
    RL_TEXTURE_FILTER_ANISOTROPIC_16X, // Anisotropic filtering 16x
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum rlBlendMode {
    RL_BLEND_ALPHA = 0,         // Blend textures considering alpha (default)
    RL_BLEND_ADDITIVE,          // Blend textures adding colors
    RL_BLEND_MULTIPLIED,        // Blend textures multiplying colors
    RL_BLEND_ADD_COLORS,        // Blend textures adding colors (alternative)
    RL_BLEND_SUBTRACT_COLORS,   // Blend textures subtracting colors (alternative)
    RL_BLEND_ALPHA_PREMULTIPLY, // Blend premultiplied textures considering alpha
    RL_BLEND_CUSTOM, // Blend textures using custom src/dst factors (use rlSetBlendFactors())
    RL_BLEND_CUSTOM_SEPARATE, // Blend textures using custom src/dst factors (use rlSetBlendFactorsSeparate())
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum rlShaderLocationIndex {
    RL_SHADER_LOC_VERTEX_POSITION = 0, // Shader location: vertex attribute: position
    RL_SHADER_LOC_VERTEX_TEXCOORD01,   // Shader location: vertex attribute: texcoord01
    RL_SHADER_LOC_VERTEX_TEXCOORD02,   // Shader location: vertex attribute: texcoord02
    RL_SHADER_LOC_VERTEX_NORMAL,       // Shader location: vertex attribute: normal
    RL_SHADER_LOC_VERTEX_TANGENT,      // Shader location: vertex attribute: tangent
    RL_SHADER_LOC_VERTEX_COLOR,        // Shader location: vertex attribute: color
    RL_SHADER_LOC_MATRIX_MVP,          // Shader location: matrix uniform: model-view-projection
    RL_SHADER_LOC_MATRIX_VIEW,         // Shader location: matrix uniform: view (camera transform)
    RL_SHADER_LOC_MATRIX_PROJECTION,   // Shader location: matrix uniform: projection
    RL_SHADER_LOC_MATRIX_MODEL,        // Shader location: matrix uniform: model (transform)
    RL_SHADER_LOC_MATRIX_NORMAL,       // Shader location: matrix uniform: normal
    RL_SHADER_LOC_VECTOR_VIEW,         // Shader location: vector uniform: view
    RL_SHADER_LOC_COLOR_DIFFUSE,       // Shader location: vector uniform: diffuse color
    RL_SHADER_LOC_COLOR_SPECULAR,      // Shader location: vector uniform: specular color
    RL_SHADER_LOC_COLOR_AMBIENT,       // Shader location: vector uniform: ambient color
    RL_SHADER_LOC_MAP_ALBEDO, // Shader location: sampler2d texture: albedo (same as: RL_SHADER_LOC_MAP_DIFFUSE)
    RL_SHADER_LOC_MAP_METALNESS, // Shader location: sampler2d texture: metalness (same as: RL_SHADER_LOC_MAP_SPECULAR)
    RL_SHADER_LOC_MAP_NORMAL,    // Shader location: sampler2d texture: normal
    RL_SHADER_LOC_MAP_ROUGHNESS, // Shader location: sampler2d texture: roughness
    RL_SHADER_LOC_MAP_OCCLUSION, // Shader location: sampler2d texture: occlusion
    RL_SHADER_LOC_MAP_EMISSION,  // Shader location: sampler2d texture: emission
    RL_SHADER_LOC_MAP_HEIGHT,    // Shader location: sampler2d texture: height
    RL_SHADER_LOC_MAP_CUBEMAP,   // Shader location: samplerCube texture: cubemap
    RL_SHADER_LOC_MAP_IRRADIANCE, // Shader location: samplerCube texture: irradiance
    RL_SHADER_LOC_MAP_PREFILTER, // Shader location: samplerCube texture: prefilter
    RL_SHADER_LOC_MAP_BRDF,      // Shader location: sampler2d texture: brdf
}

pub const RL_SHADER_LOC_MAP_DIFFUSE: i32 = rlShaderLocationIndex::RL_SHADER_LOC_MAP_ALBEDO as i32;
pub const RL_SHADER_LOC_MAP_SPECULAR: i32 =
    rlShaderLocationIndex::RL_SHADER_LOC_MAP_METALNESS as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum rlShaderUniformDataType {
    RL_SHADER_UNIFORM_FLOAT = 0, // Shader uniform type: float
    RL_SHADER_UNIFORM_VEC2,      // Shader uniform type: vec2 (2 float)
    RL_SHADER_UNIFORM_VEC3,      // Shader uniform type: vec3 (3 float)
    RL_SHADER_UNIFORM_VEC4,      // Shader uniform type: vec4 (4 float)
    RL_SHADER_UNIFORM_INT,       // Shader uniform type: int
    RL_SHADER_UNIFORM_IVEC2,     // Shader uniform type: ivec2 (2 int)
    RL_SHADER_UNIFORM_IVEC3,     // Shader uniform type: ivec3 (3 int)
    RL_SHADER_UNIFORM_IVEC4,     // Shader uniform type: ivec4 (4 int)
    RL_SHADER_UNIFORM_UINT,      // Shader uniform type: unsigned int
    RL_SHADER_UNIFORM_UIVEC2,    // Shader uniform type: uivec2 (2 unsigned int)
    RL_SHADER_UNIFORM_UIVEC3,    // Shader uniform type: uivec3 (3 unsigned int)
    RL_SHADER_UNIFORM_UIVEC4,    // Shader uniform type: uivec4 (4 unsigned int)
    RL_SHADER_UNIFORM_SAMPLER2D, // Shader uniform type: sampler2d
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum rlShaderAttributeDataType {
    RL_SHADER_ATTRIB_FLOAT = 0, // Shader attribute type: float
    RL_SHADER_ATTRIB_VEC2,      // Shader attribute type: vec2 (2 float)
    RL_SHADER_ATTRIB_VEC3,      // Shader attribute type: vec3 (3 float)
    RL_SHADER_ATTRIB_VEC4,      // Shader attribute type: vec4 (4 float)
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum rlFramebufferAttachType {
    RL_ATTACHMENT_COLOR_CHANNEL0 = 0, // Framebuffer attachment type: color 0
    RL_ATTACHMENT_COLOR_CHANNEL1 = 1, // Framebuffer attachment type: color 1
    RL_ATTACHMENT_COLOR_CHANNEL2 = 2, // Framebuffer attachment type: color 2
    RL_ATTACHMENT_COLOR_CHANNEL3 = 3, // Framebuffer attachment type: color 3
    RL_ATTACHMENT_COLOR_CHANNEL4 = 4, // Framebuffer attachment type: color 4
    RL_ATTACHMENT_COLOR_CHANNEL5 = 5, // Framebuffer attachment type: color 5
    RL_ATTACHMENT_COLOR_CHANNEL6 = 6, // Framebuffer attachment type: color 6
    RL_ATTACHMENT_COLOR_CHANNEL7 = 7, // Framebuffer attachment type: color 7
    RL_ATTACHMENT_DEPTH = 100,        // Framebuffer attachment type: depth
    RL_ATTACHMENT_STENCIL = 200,      // Framebuffer attachment type: stencil
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum rlFramebufferAttachTextureType {
    RL_ATTACHMENT_CUBEMAP_POSITIVE_X = 0, // Framebuffer texture attachment type: cubemap, +X side
    RL_ATTACHMENT_CUBEMAP_NEGATIVE_X = 1, // Framebuffer texture attachment type: cubemap, -X side
    RL_ATTACHMENT_CUBEMAP_POSITIVE_Y = 2, // Framebuffer texture attachment type: cubemap, +Y side
    RL_ATTACHMENT_CUBEMAP_NEGATIVE_Y = 3, // Framebuffer texture attachment type: cubemap, -Y side
    RL_ATTACHMENT_CUBEMAP_POSITIVE_Z = 4, // Framebuffer texture attachment type: cubemap, +Z side
    RL_ATTACHMENT_CUBEMAP_NEGATIVE_Z = 5, // Framebuffer texture attachment type: cubemap, -Z side
    RL_ATTACHMENT_TEXTURE2D = 100,        // Framebuffer texture attachment type: texture2d
    RL_ATTACHMENT_RENDERBUFFER = 200,     // Framebuffer texture attachment type: renderbuffer
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum rlCullMode {
    RL_CULL_FACE_FRONT = 0,
    RL_CULL_FACE_BACK,
}

// --- Types ---

#[derive(Debug, Default, Clone)]
pub struct rlVertexBuffer {
    pub elementCount: i32, // Number of elements in the buffer (QUADS)

    pub vertices: Vec<f32>, // Vertex position (XYZ - 3 components per vertex) (shader-location = 0)
    pub texcoords: Vec<f32>, // Vertex texture coordinates (UV - 2 components per vertex) (shader-location = 1)
    pub normals: Vec<f32>,   // Vertex normal (XYZ - 3 components per vertex) (shader-location = 2)
    pub colors: Vec<u8>,     // Vertex colors (RGBA - 4 components per vertex) (shader-location = 3)

    #[cfg(feature = "opengl_33")]
    pub indices: Vec<u32>, // Vertex indices (in case vertex data comes indexed) (6 indices per quad)
    #[cfg(feature = "gles2")]
    pub indices: Vec<u16>, // Vertex indices (in case vertex data comes indexed) (6 indices per quad)
    pub vaoId: u32,
    pub vboId: [u32; 5],
}

#[derive(Debug, Clone, Copy, Default)]
// Draw call type
// NOTE: Only texture changes register a new draw, other state-change-related elements are not
// used at this moment (vaoId, shaderId, matrices), raylib forces a batch draw call if any
// of those state-change happens (this is done in core module)
pub struct rlDrawCall {
    pub mode: i32,            // Drawing mode: LINES, TRIANGLES, QUADS
    pub vertexCount: i32,     // Number of vertex of the draw
    pub vertexAlignment: i32, // Number of vertex required for index alignment (LINES, TRIANGLES)
    //unsigned int vaoId;       // Vertex array id to be used on the draw -> Using RLGL.currentBatch->vertexBuffer.vaoId
    //unsigned int shaderId;    // Shader id to be used on the draw -> Using RLGL.currentShaderId
    pub textureId: i32, // Texture id to be used on the draw -> Use to create new draw call if changes

                        //Matrix projection;        // Projection matrix for this draw -> Using RLGL.projection by default
                        //Matrix modelview;         // Modelview matrix for this draw -> Using RLGL.modelview by default
}

#[derive(Debug, Default)]
pub struct rlRenderBatch {
    pub bufferCount: i32,
    pub currentBuffer: i32,
    //pub vertexBuffer: *mut rlVertexBuffer,
    pub vertexBuffer: Vec<rlVertexBuffer>,
    //pub draws: *mut rlDrawCall,
    pub draws: Vec<rlDrawCall>,
    pub drawCounter: i32,
    pub currentDepth: f32,
}

#[derive(Debug)]
pub struct ExtSupported {
    pub vao: bool,
    pub instancing: bool,
    pub texNPOT: bool,
    pub texDepth: bool,
    pub texDepthWebGL: bool,
    pub texFloat32: bool,
    pub texFloat16: bool,
    pub texCompDXT: bool,
    pub texCompETC1: bool,
    pub texCompETC2: bool,
    pub texCompPVRT: bool,
    pub texCompASTC: bool,
    pub texMirrorClamp: bool,
    pub texAnisoFilter: bool,
    pub computeShader: bool,
    pub ssbo: bool,
    pub maxAnisotropyLevel: f32,
    pub maxDepthBits: i32,
}

pub struct rlglState {
    pub vertexCounter: i32,
    pub texcoordx: f32,
    pub texcoordy: f32,
    pub normalx: f32,
    pub normaly: f32,
    pub normalz: f32,
    pub colorr: u8,
    pub colorg: u8,
    pub colorb: u8,
    pub colora: u8,

    pub currentMatrixMode: i32,
    pub currentMatrix: *mut Matrix,
    pub modelview: Matrix,
    pub projection: Matrix,
    pub transform: Matrix,
    pub transformRequired: bool,
    pub stack: [Matrix; RL_MAX_MATRIX_STACK_SIZE],
    pub stackCounter: i32,

    pub currentTextureId: u32,
    pub defaultTextureId: u32,
    pub activeTextureId: [u32; RL_DEFAULT_BATCH_MAX_TEXTURE_UNITS as usize],
    pub defaultVShaderId: u32,
    pub defaultFShaderId: u32,
    pub defaultShaderId: u32,
    pub defaultShaderLocs: *mut i32,
    pub currentShaderId: u32,
    pub currentShaderLocs: *mut i32,

    pub stereoRender: bool,            // Stereo rendering flag
    pub projectionStereo: [Matrix; 2], // VR stereo rendering eyes projection matrices
    pub viewOffsetStereo: [Matrix; 2], // VR stereo rendering eyes view offset matrices

    // Blending variables
    pub currentBlendMode: u32,           // Blending mode active
    pub glBlendSrcFactor: u32,           // Blending source factor
    pub glBlendDstFactor: u32,           // Blending destination factor
    pub glBlendEquation: u32,            // Blending equation
    pub glBlendSrcFactorRGB: u32,        // Blending source RGB factor
    pub glBlendDestFactorRGB: u32,       // Blending destination RGB factor
    pub glBlendSrcFactorAlpha: u32,      // Blending source alpha factor
    pub glBlendDestFactorAlpha: u32,     // Blending destination alpha factor
    pub glBlendEquationRGB: u32,         // Blending equation for RGB
    pub glBlendEquationAlpha: u32,       // Blending equation for alpha
    pub glCustomBlendModeModified: bool, // Custom blending factor and equation modification status

    pub framebufferWidth: i32,  // Current framebuffer width
    pub framebufferHeight: i32, // Current framebuffer height
}

pub struct rlglData {
    pub currentBatch: *mut rlRenderBatch,
    pub defaultBatch: rlRenderBatch,
    pub State: rlglState,
    pub ExtSupported: ExtSupported,
}

static mut IS_GPU_READY: bool = false;
pub static mut rlCullDistanceNear: f64 = RL_CULL_DISTANCE_NEAR;
pub static mut rlCullDistanceFar: f64 = RL_CULL_DISTANCE_FAR;

impl rlglState {
    pub const ZERO: Self = Self {
        vertexCounter: 0,
        texcoordx: 0.0,
        texcoordy: 0.0,
        normalx: 0.0,
        normaly: 0.0,
        normalz: 0.0,
        colorr: 0,
        colorg: 0,
        colorb: 0,
        colora: 0,

        currentMatrixMode: 0,
        currentMatrix: null_mut(),
        modelview: Matrix::ZERO,
        projection: Matrix::ZERO,
        transform: Matrix::ZERO,
        transformRequired: false,
        stack: [Matrix::ZERO; RL_MAX_MATRIX_STACK_SIZE],
        stackCounter: 0,

        currentTextureId: 0,
        defaultTextureId: 0,
        activeTextureId: [0; RL_DEFAULT_BATCH_MAX_TEXTURE_UNITS as usize],
        defaultVShaderId: 0,
        defaultFShaderId: 0,
        defaultShaderId: 0,
        defaultShaderLocs: null_mut(),
        currentShaderId: 0,
        currentShaderLocs: null_mut(),

        stereoRender: false,
        projectionStereo: [Matrix::ZERO, Matrix::ZERO],
        viewOffsetStereo: [Matrix::ZERO, Matrix::ZERO],

        currentBlendMode: 0,
        glBlendSrcFactor: 0,
        glBlendDstFactor: 0,
        glBlendEquation: 0,
        glBlendSrcFactorRGB: 0,
        glBlendDestFactorRGB: 0,
        glBlendSrcFactorAlpha: 0,
        glBlendDestFactorAlpha: 0,
        glBlendEquationRGB: 0,
        glBlendEquationAlpha: 0,
        glCustomBlendModeModified: false,

        framebufferWidth: 0,
        framebufferHeight: 0,
    };
}
impl ExtSupported {
    pub const ZERO: Self = Self {
        vao: false,
        instancing: false,
        texNPOT: false,
        texDepth: false,
        texDepthWebGL: false,
        texFloat32: false,
        texFloat16: false,
        texCompDXT: false,
        texCompETC1: false,
        texCompETC2: false,
        texCompPVRT: false,
        texCompASTC: false,
        texMirrorClamp: false,
        texAnisoFilter: false,
        computeShader: false,
        ssbo: false,
        maxAnisotropyLevel: 0.0,
        maxDepthBits: 0,
    };
}

impl Default for rlglData {
    fn default() -> Self {
        Self {
            currentBatch: null_mut(),
            defaultBatch: rlRenderBatch::default(),
            State: rlglState::ZERO,
            ExtSupported: ExtSupported::ZERO,
        }
    }
}

pub static mut RLGL: rlglData = rlglData {
    currentBatch: null_mut(),
    defaultBatch: rlRenderBatch {
        bufferCount: 0,
        currentBuffer: 0,
        vertexBuffer: Vec::new(),
        draws: Vec::new(),
        drawCounter: 0,
        currentDepth: 0.0,
    },
    State: rlglState::ZERO,
    ExtSupported: ExtSupported::ZERO,
};

// Load OpenGL extensions
// NOTE: External loader function must be provided
#[rustfmt::skip]
pub unsafe fn rlLoadExtensions(loader: *mut core::ffi::c_void) {

#[cfg(feature = "opengl_33")]     // Also defined for GRAPHICS_API_OPENGL_21
{
    // NOTE: glad is generated and contains only required OpenGL 3.3 Core extensions (and lower versions)

    use crate::external::gladLoadGL;
    if gladLoadGL(Some(core::mem::transmute(loader))) == 0 {
        warn!("GLAD: Cannot load OpenGL extensions");
    }
    else {
        info!("GLAD: OpenGL extensions loaded successfully");
    }

    // Get number of supported extensions
    let mut numExt: i32 = 0;
    gl::GetIntegerv(gl::NUM_EXTENSIONS, &mut numExt);

    info!("GL: Supported extensions count: {}", numExt);

    #[cfg(feature = "RLGL_SHOW_GL_DETAILS_INFO")]
    {
        // Get supported extensions list
        // WARNING: glGetStringi() not available on OpenGL 2.1
        info!("GL: OpenGL extensions:");

        for i in 0..numExt {
            info!(
                "    {:?}",
                std::ffi::CStr::from_ptr(gl::GetStringi(gl::EXTENSIONS, i as u32) as *const i8)
            );
        }
    }

    // Register supported extensions flags
    // OpenGL 3.3 extensions supported by default (core)
    RLGL.ExtSupported.vao = true;
    RLGL.ExtSupported.instancing = true;
    RLGL.ExtSupported.texNPOT = true;
    RLGL.ExtSupported.texFloat32 = true;
    RLGL.ExtSupported.texFloat16 = true;
    RLGL.ExtSupported.texDepth = true;
    RLGL.ExtSupported.maxDepthBits = 32;
    RLGL.ExtSupported.texAnisoFilter = true;
    RLGL.ExtSupported.texMirrorClamp = true;

    // Optional OpenGL 3.3 extensions
//    RLGL.ExtSupported.texCompASTC =
//        GLAD_GL_KHR_texture_compression_astc_hdr &&
//        GLAD_GL_KHR_texture_compression_astc_ldr;

//    RLGL.ExtSupported.texCompDXT =
//        GLAD_GL_EXT_texture_compression_s3tc;  // Texture compression: DXT

//    RLGL.ExtSupported.texCompETC2 =
//        GLAD_GL_ARB_ES3_compatibility;         // Texture compression: ETC2/EAC

    #[cfg(feature = "opengl_43")]
    {
        RLGL.ExtSupported.computeShader = GLAD_GL_ARB_compute_shader;
        RLGL.ExtSupported.ssbo = GLAD_GL_ARB_shader_storage_buffer_object;
    }
}

#[cfg(feature = "opengl_es3")]
{
    // Register supported extensions flags
    // OpenGL ES 3.0 extensions supported by default (or it should be)
    RLGL.ExtSupported.vao = true;
    RLGL.ExtSupported.instancing = true;
    RLGL.ExtSupported.texNPOT = true;
    RLGL.ExtSupported.texFloat32 = true;
    RLGL.ExtSupported.texFloat16 = true;
    RLGL.ExtSupported.texDepth = true;
    RLGL.ExtSupported.texDepthWebGL = true;
    RLGL.ExtSupported.maxDepthBits = 24;
    RLGL.ExtSupported.texAnisoFilter = true;
    RLGL.ExtSupported.texMirrorClamp = true;

    // TODO: Check for additional OpenGL ES 3.0 supported extensions:
    //RLGL.ExtSupported.texCompDXT = true;
    //RLGL.ExtSupported.texCompETC1 = true;
    //RLGL.ExtSupported.texCompETC2 = true;
    //RLGL.ExtSupported.texCompPVRT = true;
    //RLGL.ExtSupported.texCompASTC = true;
    //RLGL.ExtSupported.maxAnisotropyLevel = true;
    //RLGL.ExtSupported.computeShader = true;
    //RLGL.ExtSupported.ssbo = true;
}

#[cfg(feature = "gles2")]     // Also defined for GRAPHICS_API_OPENGL_ES_20
{
    #[cfg(any(
        feature = "PLATFORM_DESKTOP_GLFW",
        feature = "PLATFORM_DESKTOP_SDL"
    ))]
    {
        // TODO: Support GLAD loader for OpenGL ES 3.0
        if gladLoadGLES2(Some(core::mem::transmute(loader))) == 0 {
            warn!("GLAD: Cannot load OpenGL ES2.0 functions");
        }
        else {
            info!("GLAD: OpenGL ES 2.0 loaded successfully");
        }
    }

    // Get supported extensions list
    let mut numExt: i32 = 0;

    let extList =
        RL_CALLOC(512, core::mem::size_of::<*const i8>()) as *mut *const i8;

    let extensions =
        gl::GetString(gl::EXTENSIONS) as *const i8;

    // NOTE: String duplication required because glGetString() returns a const string
    let extensionsLength =
        libc::strlen(extensions) as i32;

    let extensionsDup =
        RL_CALLOC(
            (extensionsLength + 1) as usize,
            core::mem::size_of::<i8>()
        ) as *mut i8;

    libc::strncpy(extensionsDup, extensions, extensionsLength as usize);

    *extList.add(numExt as usize) = extensionsDup;

    for i in 0..extensionsLength {
        if *extensionsDup.add(i as usize) == b' ' as i8 {
            *extensionsDup.add(i as usize) = b'\0' as i8;

            numExt += 1;

            *extList.add(numExt as usize) =
                extensionsDup.add(i as usize + 1);
        }
    }

    info!("GL: Supported extensions count: {}", numExt);

    #[cfg(feature = "RLGL_SHOW_GL_DETAILS_INFO")]
    {
        info!("GL: OpenGL extensions:");

        for i in 0..numExt {
            info!(
                "    {:?}",
                std::ffi::CStr::from_ptr(*extList.add(i as usize))
            );
        }
    }

    // Check required extensions
    for i in 0..numExt {

        // Check VAO support
        // NOTE: Only check on OpenGL ES, OpenGL 3.3 has VAO support as core feature
        if libc::strcmp(
            *extList.add(i as usize),
            c"GL_OES_vertex_array_object".as_ptr()
        ) == 0
        {
            // The extension is supported by our hardware and driver, try to get related functions pointers
            // NOTE: emscripten does not support VAOs natively, it uses emulation and it reduces overall performance...

            glGenVertexArrays =
                core::mem::transmute(
                    (core::mem::transmute::<_, rlglLoadProc>(loader))
                    (c"glGenVertexArraysOES".as_ptr())
                );

            glBindVertexArray =
                core::mem::transmute(
                    (core::mem::transmute::<_, rlglLoadProc>(loader))
                    (c"glBindVertexArrayOES".as_ptr())
                );

            glDeleteVertexArrays =
                core::mem::transmute(
                    (core::mem::transmute::<_, rlglLoadProc>(loader))
                    (c"glDeleteVertexArraysOES".as_ptr())
                );

            //glIsVertexArray omitted

            if !glGenVertexArrays.is_none() &&
               !glBindVertexArray.is_none() &&
               !glDeleteVertexArrays.is_none()
            {
                RLGL.ExtSupported.vao = true;
            }
        }

        // Remaining extension checks omitted for brevity...
    }

    // Free extensions pointers
    RL_FREE(extList as *mut core::ffi::c_void);
    RL_FREE(extensionsDup as *mut core::ffi::c_void);
}

// Check OpenGL information and capabilities
//------------------------------------------------------------------------------

// Show current OpenGL and GLSL version
info!("GL: OpenGL device information:");

info!(
    "    > Vendor:   {:?}",
    std::ffi::CStr::from_ptr(gl::GetString(gl::VENDOR) as *const i8)
);

info!(
    "    > Renderer: {:?}",
    std::ffi::CStr::from_ptr(gl::GetString(gl::RENDERER) as *const i8)
);

info!(
    "    > Version:  {:?}",
    std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8)
);

info!(
    "    > GLSL:     {:?}",
    std::ffi::CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const i8)
);

//RLGL.loader = core::mem::transmute(loader);

// NOTE: Anisotropy levels capability is an extension
gl::GetFloatv(
GL_TEXTURE_MAX_ANISOTROPY_EXT,
    &mut RLGL.ExtSupported.maxAnisotropyLevel
);

#[cfg(feature = "RLGL_SHOW_GL_DETAILS_INFO")]
{
    // Show some OpenGL GPU capailities
    info!("GL: OpenGL capabilities:");

    let mut capability: i32 = 0;

    gl::GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut capability);
    info!("    GL_MAX_TEXTURE_SIZE: {}", capability);

    gl::GetIntegerv(gl::MAX_CUBE_MAP_TEXTURE_SIZE, &mut capability);
    info!("    GL_MAX_CUBE_MAP_TEXTURE_SIZE: {}", capability);

    gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut capability);
    info!("    GL_MAX_TEXTURE_IMAGE_UNITS: {}", capability);

    gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut capability);
    info!("    GL_MAX_VERTEX_ATTRIBS: {}", capability);
}

#[cfg(not(feature = "RLGL_SHOW_GL_DETAILS_INFO"))]
{
    // Show some basic info about GL supported features
    if RLGL.ExtSupported.vao {
        info!("GL: VAO extension detected, VAO functions loaded successfully");
    }
    else {
        warn!("GL: VAO extension not found, VAO not supported");
    }

    if RLGL.ExtSupported.texNPOT {
        info!("GL: NPOT textures extension detected, full NPOT textures supported");
    }
    else {
        warn!("GL: NPOT textures extension not found, limited NPOT support (no-mipmaps, no-repeat)");
    }

    if RLGL.ExtSupported.texCompDXT {
        info!("GL: DXT compressed textures supported");
    }

    if RLGL.ExtSupported.texCompETC1 {
        info!("GL: ETC1 compressed textures supported");
    }

    if RLGL.ExtSupported.texCompETC2 {
        info!("GL: ETC2/EAC compressed textures supported");
    }

    if RLGL.ExtSupported.texCompPVRT {
        info!("GL: PVRT compressed textures supported");
    }

    if RLGL.ExtSupported.texCompASTC {
        info!("GL: ASTC compressed textures supported");
    }

    if RLGL.ExtSupported.computeShader {
        info!("GL: Compute shaders supported");
    }

    if RLGL.ExtSupported.ssbo {
        info!("GL: Shader storage buffer objects supported");
    }
}

}

// Get current OpenGL version
pub fn rlGetVersion() -> i32 {
    let mut glVersion = 0;
    #[cfg(feature = "opengl_43")]
    {
        glVersion = rlGlVersion::RL_OPENGL_43 as i32;
    }
    #[cfg(feature = "opengl_33")]
    {
        glVersion = rlGlVersion::RL_OPENGL_33 as i32;
    }
    #[cfg(feature = "opengl_es3")]
    {
        glVersion = rlGlVersion::RL_OPENGL_ES_30 as i32;
    }
    #[cfg(feature = "gles2")]
    {
        glVersion = rlGlVersion::RL_OPENGL_ES_20 as i32;
    }

    return glVersion;
}

// Set current framebuffer width
pub unsafe fn rlSetFramebufferWidth(width: i32) {
    RLGL.State.framebufferWidth = width;
}

// Set current framebuffer height
pub unsafe fn rlSetFramebufferHeight(height: i32) {
    RLGL.State.framebufferHeight = height;
}

// Get default framebuffer width
pub unsafe fn rlGetFramebufferWidth() -> i32 {
    let width: i32 = RLGL.State.framebufferWidth;
    width
}

// Get default framebuffer height
pub unsafe fn rlGetFramebufferHeight() -> i32 {
    let height: i32 = RLGL.State.framebufferHeight;
    height
}

// Get default internal texture (white texture)
// NOTE: Default texture is a 1x1 pixel UNCOMPRESSED_R8G8B8A8
pub unsafe fn rlGetTextureIdDefault() -> u32 {
    let id: u32 = RLGL.State.defaultTextureId;
    id
}

// Get default shader id
pub unsafe fn rlGetShaderIdDefault() -> u32 {
    let id: u32 = RLGL.State.defaultShaderId;
    id
}

// Get default shader locs
pub unsafe fn rlGetShaderLocsDefault() -> *mut i32 {
    let locs: *mut i32 = RLGL.State.defaultShaderLocs;
    locs
}
#[rustfmt::skip]
pub unsafe fn rlLoadRenderBatch(numBuffers: i32, bufferElements: i32) -> rlRenderBatch
{
    let mut batch = rlRenderBatch::default();
    if (!IS_GPU_READY) { warn!("GL: GPU is not ready to load data, trying to load before InitWindow()?"); return batch; }

    // Initialize CPU (RAM) vertex buffers (position, texcoord, color data and indexes)
    //--------------------------------------------------------------------------------------------
    //batch.vertexBuffer = (rlVertexBuffer *)RL_CALLOC(numBuffers, sizeof(rlVertexBuffer));
    batch.vertexBuffer = vec![rlVertexBuffer::default(); numBuffers as usize];

    for i in 0..numBuffers
    {
        let i = i as usize;
        batch.vertexBuffer[i].elementCount = bufferElements;

        batch.vertexBuffer[i].vertices = vec![0.0f32; (bufferElements * 3 * 4) as usize]; // 3 float by vertex, 4 vertex by quad
        batch.vertexBuffer[i].texcoords = vec![0.0f32; (bufferElements * 2 * 4) as usize];    // 2 float by texcoord, 4 texcoord by quad
        batch.vertexBuffer[i].normals = vec![0.0f32; (bufferElements * 3 * 4) as usize];      // 3 float by vertex, 4 vertex by quad
        batch.vertexBuffer[i].colors = vec![0u8; (bufferElements * 4 * 4) as usize];   // 4 float by color, 4 colors by quad
        #[cfg(feature = "opengl_33")]
        {
            batch.vertexBuffer[i].indices = vec![0u32; (bufferElements * 6) as usize];      // 6 int by quad (indices)
        }
        #[cfg(feature = "gles2")]
        {
            batch.vertexBuffer[i].indices = vec![0u16; (bufferElements * 6) as usize];  // 6 int by quad (indices)
        }

        for j in 0..(3*4*bufferElements as usize) { batch.vertexBuffer[i].vertices[j] = 0.0; }
        for j in 0..(2*4*bufferElements as usize) { batch.vertexBuffer[i].texcoords[j] = 0.0; }
        for j in 0..(3*4*bufferElements as usize) { batch.vertexBuffer[i].normals[j] = 0.0; }
        for j in 0..(4*4*bufferElements as usize) { batch.vertexBuffer[i].colors[j] = 0u8; }

        let mut k = 0;

        // Indices can be initialized right now
        for j in (0..(6 * bufferElements as usize)).step_by(6) {
            batch.vertexBuffer[i].indices[j]     = 4 * k;
            batch.vertexBuffer[i].indices[j + 1] = 4 * k + 1;
            batch.vertexBuffer[i].indices[j + 2] = 4 * k + 2;
            batch.vertexBuffer[i].indices[j + 3] = 4 * k;
            batch.vertexBuffer[i].indices[j + 4] = 4 * k + 2;
            batch.vertexBuffer[i].indices[j + 5] = 4 * k + 3;
        
            k += 1;
        }

        RLGL.State.vertexCounter = 0;
    }

    info!("RLGL: Render batch vertex buffers loaded successfully in RAM (CPU)");
    //--------------------------------------------------------------------------------------------

    // Upload to GPU (VRAM) vertex data and initialize VAOs/VBOs
    //--------------------------------------------------------------------------------------------
    for i in 0..(numBuffers as usize)
    {
        if (RLGL.ExtSupported.vao)
        {
            // Initialize Quads VAO
            gl::GenVertexArrays(1, &mut batch.vertexBuffer[i].vaoId);
            gl::BindVertexArray(batch.vertexBuffer[i].vaoId);
        }

        // Quads - Vertex buffers binding and attributes enable
        // Vertex position buffer (shader-location = 0)
        gl::GenBuffers(1, &mut batch.vertexBuffer[i].vboId[0]);
        gl::BindBuffer(gl::ARRAY_BUFFER, batch.vertexBuffer[i].vboId[0]);
        gl::BufferData(gl::ARRAY_BUFFER, (bufferElements as usize *3*4*std::mem::size_of::<f32>()) as isize, batch.vertexBuffer[i].vertices.as_ptr() as *const std::ffi::c_void, gl::DYNAMIC_DRAW);
        gl::EnableVertexAttribArray(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_POSITION as usize)) as u32);
        gl::VertexAttribPointer(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_POSITION as usize)) as u32, 3, gl::FLOAT, 0, 0, std::ptr::null());

        // Vertex texcoord buffer (shader-location = 1)
        gl::GenBuffers(1, &mut batch.vertexBuffer[i].vboId[1]);
        gl::BindBuffer(gl::ARRAY_BUFFER, batch.vertexBuffer[i].vboId[1]);
        gl::BufferData(gl::ARRAY_BUFFER, (bufferElements as usize *2*4*std::mem::size_of::<f32>()) as isize, batch.vertexBuffer[i].texcoords.as_ptr() as *const std::ffi::c_void, gl::DYNAMIC_DRAW);
        gl::EnableVertexAttribArray(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_TEXCOORD01 as usize)) as u32);
        gl::VertexAttribPointer(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_TEXCOORD01 as usize)) as u32, 2, gl::FLOAT, 0, 0, std::ptr::null());

        // Vertex normal buffer (shader-location = 2)
        gl::GenBuffers(1, &mut batch.vertexBuffer[i].vboId[2]);
        gl::BindBuffer(gl::ARRAY_BUFFER, batch.vertexBuffer[i].vboId[2]);
        gl::BufferData(gl::ARRAY_BUFFER, (bufferElements as usize *3*4*std::mem::size_of::<f32>()) as isize, batch.vertexBuffer[i].normals.as_ptr() as *const std::ffi::c_void, gl::DYNAMIC_DRAW);
        gl::EnableVertexAttribArray(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_NORMAL as usize)) as u32);
        gl::VertexAttribPointer(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_NORMAL as usize)) as u32, 3, gl::FLOAT, 0, 0, std::ptr::null());

        // Vertex color buffer (shader-location = 3)
        gl::GenBuffers(1, &mut batch.vertexBuffer[i].vboId[3]);
        gl::BindBuffer(gl::ARRAY_BUFFER, batch.vertexBuffer[i].vboId[3]);
        gl::BufferData(gl::ARRAY_BUFFER, (bufferElements as usize *4*4*std::mem::size_of::<u8>()) as isize, batch.vertexBuffer[i].colors.as_ptr() as *const std::ffi::c_void, gl::DYNAMIC_DRAW);
        gl::EnableVertexAttribArray(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_COLOR as usize)) as u32);
        gl::VertexAttribPointer(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_COLOR as usize)) as u32, 4, gl::UNSIGNED_BYTE, gl::TRUE, 0, std::ptr::null());

        // Fill index buffer
        gl::GenBuffers(1, &mut batch.vertexBuffer[i].vboId[4]);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, batch.vertexBuffer[i].vboId[4]);
        #[cfg(feature = "opengl_33")]
        {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (bufferElements as usize *6*std::mem::size_of::<i32>()) as isize, batch.vertexBuffer[i].indices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
        }
        #[cfg(feature = "gles2")]
        {
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (bufferElements as usize *6*std::mem::size_of::<i16>()) as isize, batch.vertexBuffer[i].indices.as_ptr() as *const std::ffi::c_void, gl::STATIC_DRAW);
        }
    }

    info!("RLGL: Render batch vertex buffers loaded successfully in VRAM (GPU)");

    // Unbind the current VAO
    if (RLGL.ExtSupported.vao) {gl::BindVertexArray(0);}
    //--------------------------------------------------------------------------------------------

    // Init draw calls tracking system
    //--------------------------------------------------------------------------------------------
    batch.draws = vec![rlDrawCall::default(); RL_DEFAULT_BATCH_DRAWCALLS as usize];

    for i in 0..(RL_DEFAULT_BATCH_DRAWCALLS as usize)
    {
        batch.draws[i].mode = RL_QUADS;
        batch.draws[i].vertexCount = 0;
        batch.draws[i].vertexAlignment = 0;
        //batch.draws[i].vaoId = 0;
        //batch.draws[i].shaderId = 0;
        batch.draws[i].textureId = RLGL.State.defaultTextureId as i32;
        //batch.draws[i].RLGL.State.projection = rlMatrixIdentity();
        //batch.draws[i].RLGL.State.modelview = rlMatrixIdentity();
    }

    batch.bufferCount = numBuffers;    // Record buffer count
    batch.drawCounter = 1;             // Reset draws counter
    batch.currentDepth = -1.0;        // Reset depth value
    //--------------------------------------------------------------------------------------------

    return batch;
}

// Unload default internal buffers vertex data from CPU and GPU
pub unsafe fn rlUnloadRenderBatch(batch: &mut rlRenderBatch) {
    // Unbind everything
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

    // Unload all vertex buffers data
    for i in 0..(batch.bufferCount as usize) {
        // Unbind VAO attribs data
        if (RLGL.ExtSupported.vao) {
            gl::BindVertexArray(batch.vertexBuffer[i].vaoId);
            gl::DisableVertexAttribArray(RL_DEFAULT_SHADER_ATTRIB_LOCATION_POSITION);
            gl::DisableVertexAttribArray(RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD);
            gl::DisableVertexAttribArray(RL_DEFAULT_SHADER_ATTRIB_LOCATION_NORMAL);
            gl::DisableVertexAttribArray(RL_DEFAULT_SHADER_ATTRIB_LOCATION_COLOR);
            gl::BindVertexArray(0);
        }

        // Delete VBOs from GPU (VRAM)
        gl::DeleteBuffers(1, &batch.vertexBuffer[i].vboId[0]);
        gl::DeleteBuffers(1, &batch.vertexBuffer[i].vboId[1]);
        gl::DeleteBuffers(1, &batch.vertexBuffer[i].vboId[2]);
        gl::DeleteBuffers(1, &batch.vertexBuffer[i].vboId[3]);
        gl::DeleteBuffers(1, &batch.vertexBuffer[i].vboId[4]);

        // Delete VAOs from GPU (VRAM)
        if (RLGL.ExtSupported.vao) {
            gl::DeleteVertexArrays(1, &batch.vertexBuffer[i].vaoId);
        }

        // Free vertex arrays memory from CPU (RAM)
        batch.vertexBuffer[i].vertices.clear();
        batch.vertexBuffer[i].texcoords.clear();
        batch.vertexBuffer[i].normals.clear();
        batch.vertexBuffer[i].colors.clear();
        batch.vertexBuffer[i].indices.clear();
    }

    // Unload arrays
    batch.vertexBuffer.clear();
    batch.draws.clear();
}

// Draw render batch
// NOTE: Batch is reseted and current buffer is updated (for multi-buffer config)
#[rustfmt::skip]
pub unsafe fn rlDrawRenderBatch(batch: *mut rlRenderBatch)
{
    let batch = &mut *batch;
    // Update batch vertex buffers
    //------------------------------------------------------------------------------------------------------------
    // NOTE: If there is not vertex data, buffers doesn't need to be updated (vertexCount > 0)
    if (RLGL.State.vertexCounter > 0)
    {
        // Activate elements VAO
        if (RLGL.ExtSupported.vao) {gl::BindVertexArray(batch.vertexBuffer[batch.currentBuffer as usize].vaoId);}

        // TODO: If no data changed on the CPU arrays there is no need to re-upload data to GPU,
        // a flag can be used to detect changes but it would imply keeping a copy buffer and memcmp() both, does it worth it?

        // Vertex positions buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, batch.vertexBuffer[batch.currentBuffer as usize].vboId[0]);
        gl::BufferSubData(gl::ARRAY_BUFFER, 0, (RLGL.State.vertexCounter*3*std::mem::size_of::<f32>() as i32) as isize, batch.vertexBuffer[batch.currentBuffer as usize].vertices.as_ptr() as *const std::ffi::c_void);
        //gl::BufferData(gl::ARRAY_BUFFER, sizeof(float)*3*4*batch.vertexBuffer[batch.currentBuffer as usize].elementCount, batch.vertexBuffer[batch.currentBuffer as usize].vertices, gl::DYNAMIC_DRAW);  // Update all buffer

        // Texture coordinates buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, batch.vertexBuffer[batch.currentBuffer as usize].vboId[1]);
        gl::BufferSubData(gl::ARRAY_BUFFER, 0, (RLGL.State.vertexCounter*2*std::mem::size_of::<f32>() as i32) as isize, batch.vertexBuffer[batch.currentBuffer as usize].texcoords.as_ptr() as *const std::ffi::c_void);
        //gl::BufferData(gl::ARRAY_BUFFER, sizeof(float)*2*4*batch.vertexBuffer[batch.currentBuffer as usize].elementCount, batch.vertexBuffer[batch.currentBuffer as usize].texcoords, gl::DYNAMIC_DRAW); // Update all buffer

        // Normals buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, batch.vertexBuffer[batch.currentBuffer as usize].vboId[2]);
        gl::BufferSubData(gl::ARRAY_BUFFER, 0, (RLGL.State.vertexCounter*3*std::mem::size_of::<f32>() as i32) as isize, batch.vertexBuffer[batch.currentBuffer as usize].normals.as_ptr() as *const std::ffi::c_void);
        //gl::BufferData(gl::ARRAY_BUFFER, sizeof(float)*3*4*batch.vertexBuffer[batch.currentBuffer as usize].elementCount, batch.vertexBuffer[batch.currentBuffer as usize].normals, gl::DYNAMIC_DRAW); // Update all buffer

        // Colors buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, batch.vertexBuffer[batch.currentBuffer as usize].vboId[3]);
        gl::BufferSubData(gl::ARRAY_BUFFER, 0, (RLGL.State.vertexCounter*4*std::mem::size_of::<u8>() as i32) as isize, batch.vertexBuffer[batch.currentBuffer as usize].colors.as_ptr() as *const std::ffi::c_void);
        //gl::BufferData(gl::ARRAY_BUFFER, sizeof(float)*4*4*batch.vertexBuffer[batch.currentBuffer as usize].elementCount, batch.vertexBuffer[batch.currentBuffer as usize].colors, gl::DYNAMIC_DRAW);    // Update all buffer

        // NOTE: glMapBuffer() causes sync issue
        // If GPU is working with this buffer, glMapBuffer() will wait(stall) until GPU to finish its job
        // To avoid waiting (idle), glBufferData() can bee called first with NULL pointer before glMapBuffer()
        // Doing that, the previous data in PBO will be discarded and glMapBuffer() returns a new
        // allocated pointer immediately even if GPU is still working with the previous data

        // Another option: map the buffer object into client's memory
        //batch->vertexBuffer[batch->currentBuffer].vertices = (float *)glMapBuffer(GL_ARRAY_BUFFER, GL_READ_WRITE);
        //if (batch->vertexBuffer[batch->currentBuffer].vertices)
        //{
        //    Update vertex data
        //}
        //glUnmapBuffer(GL_ARRAY_BUFFER);

        // Unbind the current VAO
        if (RLGL.ExtSupported.vao) {gl::BindVertexArray(0);}
    }
    //------------------------------------------------------------------------------------------------------------

    // Draw batch vertex buffers (considering VR stereo if required)
    //------------------------------------------------------------------------------------------------------------
    let mut matProjection = RLGL.State.projection;
    let mut matModelView = RLGL.State.modelview;

    let mut eyeCount = 1;
    if (RLGL.State.stereoRender) {eyeCount = 2;}

    for eye in 0..(eyeCount as usize)
    {
        if (eyeCount == 2)
        {
            // Setup current eye viewport (half screen width)
            rlViewport((eye as i32 * RLGL.State.framebufferWidth) /2, 0, RLGL.State.framebufferWidth /2 , RLGL.State.framebufferHeight);

            // Set current eye view offset to modelview matrix
            rlSetMatrixModelview((matModelView * RLGL.State.viewOffsetStereo[eye]));
            // Set current eye projection matrix
            rlSetMatrixProjection(RLGL.State.projectionStereo[eye]);
        }

        // Draw buffers
        if (RLGL.State.vertexCounter > 0)
        {
            // Set current shader and upload current MVP matrix
            gl::UseProgram(RLGL.State.currentShaderId);

            // Create modelview-projection matrix and upload to shader
            let matMVP = (RLGL.State.modelview * RLGL.State.projection);
            gl::UniformMatrix4fv(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_MATRIX_MVP as usize)), 1, 0, matMVP.to_array().as_ptr());

            if (*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_MATRIX_PROJECTION as usize)) != -1)
            {
                gl::UniformMatrix4fv(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_MATRIX_PROJECTION as usize)), 1, 0, RLGL.State.projection.to_array().as_ptr());
            }

            // WARNING: For the following setup of the view, model, and normal matrices, it is expected that
            // transformations and rendering occur between rlPushMatrix() and rlPopMatrix()

            if (*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_MATRIX_VIEW as usize)) != -1)
            {
                gl::UniformMatrix4fv(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_MATRIX_VIEW as usize)), 1, 0, (RLGL.State.modelview).to_array().as_ptr());
            }

            if (*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_MATRIX_MODEL as usize)) != -1)
            {
                gl::UniformMatrix4fv(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_MATRIX_MODEL as usize)), 1, 0, (RLGL.State.transform).to_array().as_ptr());
            }

            if (*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_MATRIX_NORMAL as usize)) != -1)
            {
                gl::UniformMatrix4fv(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_MATRIX_NORMAL as usize)), 1, 0, (((RLGL.State.transform.invert().transpose()))).to_array().as_ptr());
            }

            if (RLGL.ExtSupported.vao) {gl::BindVertexArray(batch.vertexBuffer[batch.currentBuffer as usize].vaoId);}
            else
            {
                // Bind vertex attrib: position (shader-location = 0)
                gl::BindBuffer(gl::ARRAY_BUFFER, batch.vertexBuffer[batch.currentBuffer as usize].vboId[0]);
                gl::VertexAttribPointer(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_POSITION as usize)) as u32, 3, gl::FLOAT, 0, 0, std::ptr::null());
                gl::EnableVertexAttribArray(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_POSITION as usize)) as u32);

                // Bind vertex attrib: texcoord (shader-location = 1)
                gl::BindBuffer(gl::ARRAY_BUFFER, batch.vertexBuffer[batch.currentBuffer as usize].vboId[1]);
                gl::VertexAttribPointer(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_TEXCOORD01 as usize)) as u32, 2, gl::FLOAT, 0, 0, std::ptr::null());
                gl::EnableVertexAttribArray(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_TEXCOORD01 as usize)) as u32);

                // Bind vertex attrib: normal (shader-location = 2)
                gl::BindBuffer(gl::ARRAY_BUFFER, batch.vertexBuffer[batch.currentBuffer as usize].vboId[2]);
                gl::VertexAttribPointer(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_NORMAL as usize)) as u32, 3, gl::FLOAT, 0, 0, std::ptr::null());
                gl::EnableVertexAttribArray(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_NORMAL as usize)) as u32);

                // Bind vertex attrib: color (shader-location = 3)
                gl::BindBuffer(gl::ARRAY_BUFFER, batch.vertexBuffer[batch.currentBuffer as usize].vboId[3]);
                gl::VertexAttribPointer(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_COLOR as usize)) as u32, 4, gl::UNSIGNED_BYTE, gl::TRUE, 0, std::ptr::null());
                gl::EnableVertexAttribArray(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_COLOR as usize)) as u32);

                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, batch.vertexBuffer[batch.currentBuffer as usize].vboId[4]);
            }

            // Setup some default shader values
            gl::Uniform4f(*(RLGL.State.currentShaderLocs.add(rlShaderLocationIndex::RL_SHADER_LOC_COLOR_DIFFUSE as usize)), 1.0, 1.0, 1.0, 1.0);
            gl::Uniform1i(*(RLGL.State.currentShaderLocs.add(RL_SHADER_LOC_MAP_DIFFUSE as usize)), 0);  // Active default sampler2D: texture0

            // Activate additional sampler textures
            // Those additional textures will be common for all draw calls of the batch
            for i in 0..RL_DEFAULT_BATCH_MAX_TEXTURE_UNITS 
            {
                if (RLGL.State.activeTextureId[i as usize] > 0)
                {
                    gl::ActiveTexture(gl::TEXTURE0 + 1 + i as u32);
                    gl::BindTexture(gl::TEXTURE_2D, RLGL.State.activeTextureId[i as usize]);
                }
            }

            // Activate default sampler2D texture0 (one texture is always active for default batch shader)
            // NOTE: Batch system accumulates calls by texture0 changes, additional textures are enabled for all the draw calls
            gl::ActiveTexture(gl::TEXTURE0);

            let mut vertexOffset = 0;
            for i in 0..(batch.drawCounter as usize)
            {
                // Bind current draw call texture, activated as GL_TEXTURE0 and bound to sampler2D texture0 by default
                gl::BindTexture(gl::TEXTURE_2D, batch.draws[i].textureId as u32);

                if ((batch.draws[i].mode == RL_LINES) || (batch.draws[i].mode == RL_TRIANGLES)) {gl::DrawArrays(batch.draws[i].mode as u32, vertexOffset, batch.draws[i].vertexCount);}
                else
                {
                    #[cfg(feature = "opengl_33")]
                    {// The number of indices to be processed needs to be defined: elementCount*6
                    // NOTE: The final parameter tells the GPU the offset in bytes from the
                    // start of the index buffer to the location of the first index to process
                    gl::DrawElements(gl::TRIANGLES, batch.draws[i].vertexCount/4*6, gl::UNSIGNED_INT, (vertexOffset/4*6*std::mem::size_of::<u32>() as i32) as *const libc::c_void);}

                    #[cfg(feature = "gles2")]
                    {
                    gl::DrawElements(gl::TRIANGLES, batch.draws[i].vertexCount/4*6, gl::UNSIGNED_SHORT, (vertexOffset/4*6*std::mem::size_of::<u16>() as i32) as *const libc::c_void);
                    }
                }

                vertexOffset += (batch.draws[i].vertexCount + batch.draws[i].vertexAlignment);
            }

            if (!RLGL.ExtSupported.vao)
            {
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
            }

            gl::BindTexture(gl::TEXTURE_2D, 0);    // Unbind textures
        }

        if (RLGL.ExtSupported.vao) {gl::BindVertexArray(0); } // Unbind VAO

        gl::UseProgram(0);    // Unbind shader program
    }

    // Restore viewport to default measures
    if (eyeCount == 2) {rlViewport(0, 0, RLGL.State.framebufferWidth, RLGL.State.framebufferHeight);}
    //------------------------------------------------------------------------------------------------------------

    // Reset batch buffers
    //------------------------------------------------------------------------------------------------------------
    // Reset vertex counter for next frame
    RLGL.State.vertexCounter = 0;

    // Reset depth for next draw
    batch.currentDepth = -1.0;

    // Restore projection/modelview matrices
    RLGL.State.projection = matProjection;
    RLGL.State.modelview = matModelView;

    // Reset RLGL.currentBatch->draws array
    for i in 0..RL_DEFAULT_BATCH_DRAWCALLS
    {
        batch.draws[i as usize].mode = RL_QUADS;
        batch.draws[i as usize].vertexCount = 0;
        batch.draws[i as usize].textureId = RLGL.State.defaultTextureId as i32;
    }

    // Reset active texture units for next batch
    for i in 0..RL_DEFAULT_BATCH_MAX_TEXTURE_UNITS { RLGL.State.activeTextureId[i as usize] = 0; }

    // Reset draws counter to one draw for the batch
    batch.drawCounter = 1;
    //------------------------------------------------------------------------------------------------------------

    // Change to next buffer in the list (in case of multi-buffering)
    batch.currentBuffer+=1;
    if (batch.currentBuffer >= batch.bufferCount) {batch.currentBuffer = 0;}
}

// Set the active render batch for rlgl
pub unsafe fn rlSetRenderBatchActive(batch: *mut rlRenderBatch) {
    rlDrawRenderBatch(RLGL.currentBatch); // NOTE: Stereo rendering is checked inside

    if !(batch.is_null()) {
        RLGL.currentBatch = batch;
    } else {
        RLGL.currentBatch = &mut RLGL.defaultBatch;
    }
}

// Update and draw internal render batch
pub unsafe fn rlDrawRenderBatchActive() {
    rlDrawRenderBatch(RLGL.currentBatch); // NOTE: Stereo rendering is checked inside
}

// Check internal buffer overflow for a given number of vertex
// and force a rlRenderBatch draw call if required
#[rustfmt::skip]
pub unsafe fn rlCheckRenderBatchLimit(vCount: i32) -> bool
{
    let mut overflow = false;

    if ((RLGL.State.vertexCounter + vCount) >=
        ((*RLGL.currentBatch).vertexBuffer[(*RLGL.currentBatch).currentBuffer as usize].elementCount*4))
    {
        overflow = true;

        // Store current primitive drawing mode and texture id
        let currentMode = (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].mode;
        let currentTexture = (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].textureId;

        rlDrawRenderBatch(RLGL.currentBatch);    // NOTE: Stereo rendering is checked inside

        // Restore state of last batch so new vertices can be added
        (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].mode = currentMode;
        (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].textureId = currentTexture;
    }

    return overflow;
}

pub unsafe fn rlLoadTexture(
    data: *const std::ffi::c_void,
    width: i32,
    height: i32,
    format: i32,
    mipmapCount: i32,
) -> u32 {
    let mut id = 0;
    if (!IS_GPU_READY) {
        warn!("GL: GPU is not ready to load data, trying to load before InitWindow()?");
        return id;
    }

    gl::BindTexture(gl::TEXTURE_2D, 0); // Free any old binding

    // Check texture format support by OpenGL 1.1 (compressed textures not supported)
    if ((!RLGL.ExtSupported.texCompDXT)
        && ((format == PixelFormat::PIXELFORMAT_COMPRESSED_DXT1_RGB as i32)
            || (format == PixelFormat::PIXELFORMAT_COMPRESSED_DXT1_RGBA as i32)
            || (format == PixelFormat::PIXELFORMAT_COMPRESSED_DXT3_RGBA as i32)
            || (format == PixelFormat::PIXELFORMAT_COMPRESSED_DXT5_RGBA as i32)))
    {
        warn!("GL: DXT compressed texture format not supported");
        return id;
    }
    if ((!RLGL.ExtSupported.texCompETC1)
        && (format == PixelFormat::PIXELFORMAT_COMPRESSED_ETC1_RGB as i32))
    {
        warn!("GL: ETC1 compressed texture format not supported");
        return id;
    }

    if ((!RLGL.ExtSupported.texCompETC2)
        && ((format == PixelFormat::PIXELFORMAT_COMPRESSED_ETC2_RGB as i32)
            || (format == PixelFormat::PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA as i32)))
    {
        warn!("GL: ETC2 compressed texture format not supported");
        return id;
    }

    if ((!RLGL.ExtSupported.texCompPVRT)
        && ((format == PixelFormat::PIXELFORMAT_COMPRESSED_PVRT_RGB as i32)
            || (format == PixelFormat::PIXELFORMAT_COMPRESSED_PVRT_RGBA as i32)))
    {
        warn!("GL: PVRT compressed texture format not supported");
        return id;
    }

    if ((!RLGL.ExtSupported.texCompASTC)
        && ((format == PixelFormat::PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA as i32)
            || (format == PixelFormat::PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA as i32)))
    {
        warn!("GL: ASTC compressed texture format not supported");
        return id;
    }

    gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

    gl::GenTextures(1, &mut id); // Generate texture id

    gl::BindTexture(gl::TEXTURE_2D, id);

    let mut mipWidth = width;
    let mut mipHeight = height;
    let mut mipOffset = 0; // Mipmap data offset, only used for tracelog

    // NOTE: Added pointer math separately from function to avoid UBSAN complaining
    let mut dataPtr = std::ptr::null();
    if (!data.is_null()) {
        dataPtr = data;
    }

    // Load the different mipmap levels
    for i in 0..mipmapCount {
        let mipSize = rlGetPixelDataSize(mipWidth, mipHeight, format);

        let mut glInternalFormat = 0;
        let mut glFormat = 0;
        let mut glType = 0;
        rlGetGlTextureFormats(format, &mut glInternalFormat, &mut glFormat, &mut glType);

        debug!(
            "TEXTURE: Load mipmap level {} ({} x {}), size: {}, offset: {}",
            i, mipWidth, mipHeight, mipSize, mipOffset
        );

        if (glInternalFormat != 0) {
            if (format < PixelFormat::PIXELFORMAT_COMPRESSED_DXT1_RGB as i32) {
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    i,
                    glInternalFormat as i32,
                    mipWidth,
                    mipHeight,
                    0,
                    glFormat,
                    glType,
                    dataPtr,
                );
            } else {
                gl::CompressedTexImage2D(
                    gl::TEXTURE_2D,
                    i,
                    glInternalFormat,
                    mipWidth,
                    mipHeight,
                    0,
                    mipSize,
                    dataPtr,
                );
            }

            #[cfg(feature = "opengl_33")]
            {
                if (format == PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAYSCALE as i32) {
                    let swizzleMask = [gl::RED, gl::RED, gl::RED, gl::ONE];
                    gl::TexParameteriv(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_SWIZZLE_RGBA,
                        swizzleMask.as_ptr() as *const i32,
                    );
                } else if (format == PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA as i32) {
                    let swizzleMask = [gl::RED, gl::RED, gl::RED, gl::GREEN];
                    gl::TexParameteriv(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_SWIZZLE_RGBA,
                        swizzleMask.as_ptr() as *const i32,
                    );
                }
            }
        }

        mipWidth /= 2;
        mipHeight /= 2;
        mipOffset += mipSize; // Increment offset position to next mipmap
        if !(data.is_null()) {
            dataPtr.add(mipSize as usize);
        }

        // Security check for NPOT textures
        if (mipWidth < 1) {
            mipWidth = 1;
        }
        if (mipHeight < 1) {
            mipHeight = 1;
        }
    }

    // Texture parameters configuration
    // NOTE: gl::TexParameteri does NOT affect texture uploading
    #[cfg(feature = "gles2")]
    {
        // Check if texture is power-of-two (POT)
        let texIsPOT = false;

        if (((width > 0) && ((width & (width - 1)) == 0))
            && ((height > 0) && ((height & (height - 1)) == 0)))
        {
            texIsPOT = true;
        }

        // NOTE: OpenGL ES 2.0 with no GL_OES_texture_npot support (i.e. WebGL) has limited NPOT support, so CLAMP_TO_EDGE must be used
        if ((texIsPOT) || (RLGL.ExtSupported.texNPOT)) {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT); // Set texture to repeat on x-axis
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT); // Set texture to repeat on y-axis
        } else {
            // NOTE: If using negative texture coordinates (LoadOBJ()), it does not work!
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE); // Set texture to clamp on x-axis
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE);
            // Set texture to clamp on y-axis
        }
    }
    #[cfg(not(feature = "gles2"))]
    {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // Set texture to repeat on x-axis
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // Set texture to repeat on y-axis
    }
    // Magnification and minification filters
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32); // Alternative: GL_LINEAR
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32); // Alternative: GL_LINEAR

    #[cfg(feature = "opengl_33")]
    {
        if (mipmapCount > 1) {
            // Activate trilinear filtering if mipmaps are available
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );

            // Define the maximum number of mipmap levels to be used, 0 is base texture size
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, mipmapCount - 1);

            // Check if the loaded texture with mipmaps is complete,
            // uncomplete textures will draw in black if mipmap filtering is required
            //GLint complete = 0;
            //glGetTexParameteriv(gl::TEXTURE_2D, GL_TEXTURE_IMMUTABLE_FORMAT, &complete);
        }
    }

    // At this point texture is loaded in GPU and texture parameters configured

    // NOTE: If mipmaps were not in data, they are not generated automatically

    // Unbind current texture
    gl::BindTexture(gl::TEXTURE_2D, 0);

    if (id > 0) {
        info!(
            "TEXTURE: [ID {}] Texture loaded successfully ({}x{} | {} | {} mipmaps)",
            id,
            width,
            height,
            rlGetPixelFormatName(format),
            mipmapCount
        );
    } else {
        warn!("TEXTURE: Failed to load texture");
    }

    return id;
}

// Load depth texture/renderbuffer (to be attached to fbo)
// WARNING: OpenGL ES 2.0 requires GL_OES_depth_texture and WebGL requires WEBGL_depth_texture extensions
#[rustfmt::skip]
pub unsafe fn rlLoadTextureDepth(width: i32, height: i32, mut useRenderBuffer: bool) -> u32
{
    let mut id = 0;
    if (!IS_GPU_READY) { warn!("GL: GPU is not ready to load data, trying to load before InitWindow()?"); return id; }

    // In case depth textures were not supported, force renderbuffer usage
    if (!RLGL.ExtSupported.texDepth) { useRenderBuffer = true; }

    // NOTE: Letting the implementation to choose the best bit-depth
    // Possible formats: GL_DEPTH_COMPONENT16, GL_DEPTH_COMPONENT24, GL_DEPTH_COMPONENT32 and GL_DEPTH_COMPONENT32F
    let mut glInternalFormat = gl::DEPTH_COMPONENT;

#[cfg(feature = "gles2")]
{
    // WARNING: WebGL platform requires unsized internal format definition (GL_DEPTH_COMPONENT)
    // while other platforms using OpenGL ES 2.0 require/support sized internal formats depending on the GPU capabilities
    if (!RLGL.ExtSupported.texDepthWebGL || useRenderBuffer)
    {
        if (RLGL.ExtSupported.maxDepthBits == 32) { glInternalFormat = gl::DEPTH_COMPONENT32_OES; }
        else if (RLGL.ExtSupported.maxDepthBits == 24) { glInternalFormat = gl::DEPTH_COMPONENT24_OES; }
        else { glInternalFormat = gl::DEPTH_COMPONENT16; }
    }
}
#[cfg(feature = "opengl_33")]
{
    // NOTE: This sized internal format should also work for WebGL 2.0
    // WARNING: Specification only allows GL_DEPTH_COMPONENT32F for GL_FLOAT type
    // REF: https://registry.khronos.org/OpenGL-Refpages/es3.0/html/glTexImage2D.xhtml
    if (RLGL.ExtSupported.maxDepthBits == 24) { glInternalFormat = gl::DEPTH_COMPONENT24; }
    else { glInternalFormat = gl::DEPTH_COMPONENT16; }
}

    if (!useRenderBuffer && RLGL.ExtSupported.texDepth)
    {
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexImage2D(gl::TEXTURE_2D, 0, glInternalFormat as i32, width, height, 0, gl::DEPTH_COMPONENT, gl::UNSIGNED_INT, std::ptr::null());

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

        gl::BindTexture(gl::TEXTURE_2D, 0);

        info!("TEXTURE: Depth texture loaded successfully");
    }
    else
    {
        // Create the renderbuffer that will serve as the depth attachment for the framebuffer
        // NOTE: A renderbuffer is simpler than a texture and could offer better performance on embedded devices
        gl::GenRenderbuffers(1, &mut id);
        gl::BindRenderbuffer(gl::RENDERBUFFER, id);
        gl::RenderbufferStorage(gl::RENDERBUFFER, glInternalFormat, width, height);

        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

        info!("TEXTURE: [ID {}] Depth renderbuffer loaded successfully ({} bits)", id, if (RLGL.ExtSupported.maxDepthBits >= 24) { RLGL.ExtSupported.maxDepthBits } else { 16 });
    }

    return id;
}

// Update already loaded texture in GPU with new data
// WARNING: Not possible to know safely if internal texture format is the expected one...
pub unsafe fn rlUpdateTexture(
    id: u32,
    offsetX: i32,
    offsetY: i32,
    width: i32,
    height: i32,
    format: i32,
    data: *const std::ffi::c_void,
) {
    gl::BindTexture(gl::TEXTURE_2D, id);

    let mut glInternalFormat = 0;
    let mut glFormat = 0;
    let mut glType = 0;
    rlGetGlTextureFormats(format, &mut glInternalFormat, &mut glFormat, &mut glType);

    if ((glInternalFormat != 0) && (format < PixelFormat::PIXELFORMAT_COMPRESSED_DXT1_RGB as i32)) {
        gl::TexSubImage2D(
            gl::TEXTURE_2D,
            0,
            offsetX,
            offsetY,
            width,
            height,
            glFormat,
            glType,
            data,
        );
    } else {
        warn!(
            "TEXTURE: [ID {}] Failed to update for current texture format ({})",
            id, format
        );
    }
}

// Matrix operations
/// Choose the current matrix to be transformed
pub unsafe fn rlMatrixMode(mode: i32) {
    if (mode == RL_PROJECTION) {
        RLGL.State.currentMatrix = &raw mut RLGL.State.projection;
    } else if (mode == RL_MODELVIEW) {
        RLGL.State.currentMatrix = &raw mut RLGL.State.modelview;
    }
    //else if (mode == RL_TEXTURE) // Not supported

    RLGL.State.currentMatrixMode = mode;
}

/// Push the current matrix into RLGL.State.stack
pub unsafe fn rlPushMatrix() {
    if (RLGL.State.stackCounter >= RL_MAX_MATRIX_STACK_SIZE as i32) {
        error!("RLGL: Matrix stack overflow (RL_MAX_MATRIX_STACK_SIZE)");
    }

    if (RLGL.State.currentMatrixMode == RL_MODELVIEW) {
        RLGL.State.transformRequired = true;
        RLGL.State.currentMatrix = &raw mut RLGL.State.transform;
    }

    RLGL.State.stack[RLGL.State.stackCounter as usize] = *RLGL.State.currentMatrix;
    RLGL.State.stackCounter += 1;
}

/// Pop latest inserted matrix from RLGL.State.stack
pub unsafe fn rlPopMatrix() {
    if (RLGL.State.stackCounter > 0) {
        let mat = RLGL.State.stack[(RLGL.State.stackCounter - 1) as usize];
        *RLGL.State.currentMatrix = mat;
        RLGL.State.stackCounter -= 1;
    }

    if ((RLGL.State.stackCounter == 0) && (RLGL.State.currentMatrixMode == RL_MODELVIEW)) {
        RLGL.State.currentMatrix = &raw mut RLGL.State.modelview;
        RLGL.State.transformRequired = false;
    }
}

// Reset current matrix to identity matrix
pub unsafe fn rlLoadIdentity() {
    *RLGL.State.currentMatrix = Matrix::IDENTITY;
}

pub unsafe fn rlTranslatef(x: f32, y: f32, z: f32) {
    let mut matTranslation = Matrix::IDENTITY;

    // Set translation component of matrix
    matTranslation.m12 = x;
    matTranslation.m13 = y;
    matTranslation.m14 = z;

    // NOTE: Transposing matrix by multiplication order
    *RLGL.State.currentMatrix = matTranslation * *RLGL.State.currentMatrix;
}

/// Multiply the current matrix by a rotation matrix
/// NOTE: The provided angle must be in degrees
pub unsafe fn rlRotatef(angle: f32, mut x: f32, mut y: f32, mut z: f32) {
    let mut matRotation = Matrix::IDENTITY;

    // Axis vector (x, y, z) normalization
    let lengthSquared = x * x + y * y + z * z;
    if ((lengthSquared != 1.0) && (lengthSquared != 0.0)) {
        let inverseLength = 1.0 / (lengthSquared).sqrt();
        x *= inverseLength;
        y *= inverseLength;
        z *= inverseLength;
    }

    // Rotation matrix generation
    let sinres = angle.to_radians().sin();
    let cosres = angle.to_radians().cos();
    let t = 1.0 - cosres;

    matRotation.m0 = x * x * t + cosres;
    matRotation.m1 = y * x * t + z * sinres;
    matRotation.m2 = z * x * t - y * sinres;
    matRotation.m3 = 0.0;

    matRotation.m4 = x * y * t - z * sinres;
    matRotation.m5 = y * y * t + cosres;
    matRotation.m6 = z * y * t + x * sinres;
    matRotation.m7 = 0.0;

    matRotation.m8 = x * z * t + y * sinres;
    matRotation.m9 = y * z * t - x * sinres;
    matRotation.m10 = z * z * t + cosres;
    matRotation.m11 = 0.0;

    matRotation.m12 = 0.0;
    matRotation.m13 = 0.0;
    matRotation.m14 = 0.0;
    matRotation.m15 = 1.0;

    // NOTE: Transposing matrix by multiplication order
    *RLGL.State.currentMatrix = (matRotation * *RLGL.State.currentMatrix);
}

/// Multiply the current matrix by a scaling matrix
pub unsafe fn rlScalef(x: f32, y: f32, z: f32) {
    let mut matScale = Matrix::IDENTITY;

    // Set scale component of matrix
    matScale.m0 = x;
    matScale.m5 = y;
    matScale.m10 = z;

    // NOTE: Transposing matrix by multiplication order
    *RLGL.State.currentMatrix = (matScale * *RLGL.State.currentMatrix);
}

// Multiply the current matrix by another matrix
pub unsafe fn rlMultMatrixf(matf: &[f32; 16]) {
    // Matrix creation from array
    // Conversion from column-major to row-major memory order
    let mat = Matrix::new(
        matf[0], matf[4], matf[8], matf[12], matf[1], matf[5], matf[9], matf[13], matf[2], matf[6],
        matf[10], matf[14], matf[3], matf[7], matf[11], matf[15],
    );

    *RLGL.State.currentMatrix = (mat * *RLGL.State.currentMatrix);
}

/// Multiply the current matrix by a perspective matrix generated by parameters
pub unsafe fn rlFrustum(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
    let mut matFrustum = Matrix::IDENTITY;

    let rl = (right - left) as f32;
    let tb = (top - bottom) as f32;
    let fnn = (zfar - znear) as f32;

    matFrustum.m0 = (znear as f32 * 2.0) / rl;
    matFrustum.m1 = 0.0;
    matFrustum.m2 = 0.0;
    matFrustum.m3 = 0.0;

    matFrustum.m4 = 0.0;
    matFrustum.m5 = (znear as f32 * 2.0) / tb;
    matFrustum.m6 = 0.0;
    matFrustum.m7 = 0.0;

    matFrustum.m8 = (right as f32 + left as f32) / rl;
    matFrustum.m9 = (top as f32 + bottom as f32) / tb;
    matFrustum.m10 = -(zfar as f32 + znear as f32) / fnn;
    matFrustum.m11 = -1.0;

    matFrustum.m12 = 0.0;
    matFrustum.m13 = 0.0;
    matFrustum.m14 = -(zfar as f32 * znear as f32 * 2.0) / fnn;
    matFrustum.m15 = 0.0;

    *RLGL.State.currentMatrix = (*RLGL.State.currentMatrix * matFrustum);
}

/// Multiply the current matrix by an orthographic matrix generated by parameters
pub unsafe fn rlOrtho(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
    // NOTE: If left-right and top-botton values are equal it could create a division by zero,
    // response to it is platform/compiler dependent
    let mut matOrtho = Matrix::IDENTITY;

    let rl = (right - left) as f32;
    let tb = (top - bottom) as f32;
    let fnn = (zfar - znear) as f32;

    matOrtho.m0 = 2.0 / rl;
    matOrtho.m1 = 0.0;
    matOrtho.m2 = 0.0;
    matOrtho.m3 = 0.0;
    matOrtho.m4 = 0.0;
    matOrtho.m5 = 2.0 / tb;
    matOrtho.m6 = 0.0;
    matOrtho.m7 = 0.0;
    matOrtho.m8 = 0.0;
    matOrtho.m9 = 0.0;
    matOrtho.m10 = -2.0 / fnn;
    matOrtho.m11 = 0.0;
    matOrtho.m12 = -(left as f32 + right as f32) / rl;
    matOrtho.m13 = -(top as f32 + bottom as f32) / tb;
    matOrtho.m14 = -(zfar as f32 + znear as f32) / fnn;
    matOrtho.m15 = 1.0;

    *RLGL.State.currentMatrix = (*RLGL.State.currentMatrix * matOrtho);
}

/// Set the viewport area (transformation from normalized device coordinates to window coordinates)
pub unsafe fn rlViewport(x: i32, y: i32, width: i32, height: i32) {
    gl::Viewport(x, y, width, height);
}

pub unsafe fn rlSetClipPlanes(nearPlane: f64, farPlane: f64) {
    rlCullDistanceNear = nearPlane;
    rlCullDistanceFar = farPlane;
}

/// Get cull plane distance near
pub unsafe fn rlGetCullDistanceNear() -> f64 {
    return rlCullDistanceNear;
}

/// Get cull plane distance far
pub unsafe fn rlGetCullDistanceFar() -> f64 {
    return rlCullDistanceFar;
}

// Vertex level operations
// Initialize drawing mode (how to organize vertex)
pub unsafe fn rlBegin(mode: i32) {
    // Draw mode can be RL_LINES, RL_TRIANGLES and RL_QUADS
    // NOTE: In all three cases, vertex are accumulated over default internal vertex buffer
    if ((*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].mode != mode) {
        if ((*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].vertexCount
            > 0)
        {
            // Make sure current RLGL.currentBatch.draws[i].vertexCount is aligned a multiple of 4,
            // that way, following QUADS drawing will keep aligned with index processing
            // It implies adding some extra alignment vertex at the end of the draw,
            // those vertex are not processed but they are considered as an additional offset
            // for the next set of vertex to be drawn
            if ((*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].mode
                == RL_LINES)
            {
                (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1]
                    .vertexAlignment = if ((*RLGL.currentBatch).draws
                    [(*RLGL.currentBatch).drawCounter as usize - 1]
                    .vertexCount
                    < 4)
                {
                    (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1]
                        .vertexCount
                } else {
                    (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1]
                        .vertexCount
                        % 4
                };
            } else if ((*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1]
                .mode
                == RL_TRIANGLES)
            {
                (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1]
                    .vertexAlignment = if ((*RLGL.currentBatch).draws
                    [(*RLGL.currentBatch).drawCounter as usize - 1]
                    .vertexCount
                    < 4)
                {
                    1
                } else {
                    4 - ((*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1]
                        .vertexCount
                        % 4)
                };
            } else {
                (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1]
                    .vertexAlignment = 0;
            }

            if (!rlCheckRenderBatchLimit(
                (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1]
                    .vertexAlignment,
            )) {
                RLGL.State.vertexCounter += (*RLGL.currentBatch).draws
                    [(*RLGL.currentBatch).drawCounter as usize - 1]
                    .vertexAlignment;
                (*RLGL.currentBatch).drawCounter += 1;
            }
        }

        if ((*RLGL.currentBatch).drawCounter >= RL_DEFAULT_BATCH_DRAWCALLS) {
            rlDrawRenderBatch(RLGL.currentBatch);
        }

        (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].mode = mode;
        (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].textureId =
            RLGL.State.currentTextureId as i32;
        RLGL.State.currentTextureId = RLGL.State.defaultTextureId;
    }
}

pub unsafe fn rlEnd() {
    // NOTE: Depth increment is dependent on rlOrtho(): z-near and z-far values,
    // as well as depth buffer bit-depth (16bit or 24bit or 32bit)
    // Correct increment formula would be: depthInc = (zfar - znear)/pow(2, bits)
    (*RLGL.currentBatch).currentDepth += (1.0 / 20000.0);
}

// NOTE: Vertex position data is the basic information required for drawing
#[rustfmt::skip]
pub unsafe fn rlVertex3f(x: f32, y: f32, z: f32)
{
    let mut tx = x;
    let mut ty = y;
    let mut tz = z;

    // Transform provided vector if required
    if (RLGL.State.transformRequired)
    {
        tx = RLGL.State.transform.m0*x + RLGL.State.transform.m4*y + RLGL.State.transform.m8*z + RLGL.State.transform.m12;
        ty = RLGL.State.transform.m1*x + RLGL.State.transform.m5*y + RLGL.State.transform.m9*z + RLGL.State.transform.m13;
        tz = RLGL.State.transform.m2*x + RLGL.State.transform.m6*y + RLGL.State.transform.m10*z + RLGL.State.transform.m14;
    }

    // WARNING: Be careful with primitives breaking when launching a new batch!
    // RL_LINES comes in pairs, RL_TRIANGLES come in groups of 3 vertices and RL_QUADS come in groups of 4 vertices
    // Checking current draw.mode when a new vertex is required and finish the batch only if the draw.mode draw.vertexCount is %2, %3 or %4
    if (RLGL.State.vertexCounter > ((*RLGL.currentBatch).vertexBuffer[(*RLGL.currentBatch).currentBuffer as usize].elementCount*4 - 4))
    {
        if (((*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].mode == RL_LINES) &&
            ((*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].vertexCount%2 == 0))
        {
            // Reached the maximum number of vertices for RL_LINES drawing
            // Launch a draw call but keep current state for next vertices comming
            // NOTE: Adding +1 vertex to the check for some safety
            rlCheckRenderBatchLimit(2 + 1);
        }
        else if (((*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].mode == RL_TRIANGLES) &&
            ((*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].vertexCount%3 == 0))
        {
            rlCheckRenderBatchLimit(3 + 1);
        }
        else if (((*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].mode == RL_QUADS) &&
            ((*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].vertexCount%4 == 0))
        {
            rlCheckRenderBatchLimit(4 + 1);
        }
    }

    // Add vertices
    (*RLGL.currentBatch).vertexBuffer[(*RLGL.currentBatch).currentBuffer as usize].vertices[3 * RLGL.State.vertexCounter as usize] = tx;
    (*RLGL.currentBatch).vertexBuffer[(*RLGL.currentBatch).currentBuffer as usize].vertices[3 * RLGL.State.vertexCounter as usize + 1] = ty;
    (*RLGL.currentBatch).vertexBuffer[(*RLGL.currentBatch).currentBuffer as usize].vertices[3 * RLGL.State.vertexCounter as usize + 2] = tz;

    // Add current texcoord
    (*RLGL.currentBatch).vertexBuffer[(*RLGL.currentBatch).currentBuffer as usize].texcoords[2* RLGL.State.vertexCounter as usize] = RLGL.State.texcoordx;
    (*RLGL.currentBatch).vertexBuffer[(*RLGL.currentBatch).currentBuffer as usize].texcoords[2* RLGL.State.vertexCounter as usize + 1] = RLGL.State.texcoordy;

    // Add current normal
    (*RLGL.currentBatch).vertexBuffer[(*RLGL.currentBatch).currentBuffer as usize].normals[3*RLGL.State.vertexCounter as usize] = RLGL.State.normalx;
    (*RLGL.currentBatch).vertexBuffer[(*RLGL.currentBatch).currentBuffer as usize].normals[3*RLGL.State.vertexCounter as usize + 1] = RLGL.State.normaly;
    (*RLGL.currentBatch).vertexBuffer[(*RLGL.currentBatch).currentBuffer as usize].normals[3*RLGL.State.vertexCounter as usize + 2] = RLGL.State.normalz;

    // Add current color
    (*RLGL.currentBatch).vertexBuffer[(*RLGL.currentBatch).currentBuffer as usize].colors[4*RLGL.State.vertexCounter as usize] = RLGL.State.colorr;
    (*RLGL.currentBatch).vertexBuffer[(*RLGL.currentBatch).currentBuffer as usize].colors[4*RLGL.State.vertexCounter as usize + 1] = RLGL.State.colorg;
    (*RLGL.currentBatch).vertexBuffer[(*RLGL.currentBatch).currentBuffer as usize].colors[4*RLGL.State.vertexCounter as usize + 2] = RLGL.State.colorb;
    (*RLGL.currentBatch).vertexBuffer[(*RLGL.currentBatch).currentBuffer as usize].colors[4*RLGL.State.vertexCounter as usize + 3] = RLGL.State.colora;

    RLGL.State.vertexCounter+=1;
    (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].vertexCount+=1;
}

// Define one vertex (position)
pub unsafe fn rlVertex2f(x: f32, y: f32) {
    rlVertex3f(x, y, (*RLGL.currentBatch).currentDepth);
}
pub unsafe fn rlVertex2i(x: i32, y: i32) {
    rlVertex3f(x as f32, y as f32, (*RLGL.currentBatch).currentDepth);
}

// Define one vertex (texture coordinate)
// NOTE: Texture coordinates are limited to QUADS only
pub unsafe fn rlTexCoord2f(x: f32, y: f32) {
    RLGL.State.texcoordx = x;
    RLGL.State.texcoordy = y;
}

// Define one vertex (normal)
// NOTE: Normals limited to TRIANGLES only?
pub unsafe fn rlNormal3f(x: f32, y: f32, z: f32) {
    let mut normalx = x;
    let mut normaly = y;
    let mut normalz = z;
    if (RLGL.State.transformRequired) {
        normalx =
            RLGL.State.transform.m0 * x + RLGL.State.transform.m4 * y + RLGL.State.transform.m8 * z;
        normaly =
            RLGL.State.transform.m1 * x + RLGL.State.transform.m5 * y + RLGL.State.transform.m9 * z;
        normalz = RLGL.State.transform.m2 * x
            + RLGL.State.transform.m6 * y
            + RLGL.State.transform.m10 * z;
    }

    // NOTE: Default behavior assumes the normal vector is in the correct space for what the shader expects,
    // it could be not normalized to 0.0f..1.0f, magnitud can be useed for some effects
    /*
    // WARNING: Vector normalization if required
    float length = sqrtf(normalx*normalx + normaly*normaly + normalz*normalz);
    if (length != 0.0f)
    {
        float ilength = 1.0f/length;
        normalx *= ilength;
        normaly *= ilength;
        normalz *= ilength;
    }
    */
    RLGL.State.normalx = normalx;
    RLGL.State.normaly = normaly;
    RLGL.State.normalz = normalz;
}

pub unsafe fn rlColor4ub(x: u8, y: u8, z: u8, w: u8) {
    RLGL.State.colorr = x;
    RLGL.State.colorg = y;
    RLGL.State.colorb = z;
    RLGL.State.colora = w;
}

pub unsafe fn rlColor4f(r: f32, g: f32, b: f32, a: f32) {
    rlColor4ub(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8,
    );
}

// Define one vertex (color)
pub unsafe fn rlColor3f(x: f32, y: f32, z: f32) {
    rlColor4ub((x * 255.0) as u8, (y * 255.0) as u8, (z * 255.0) as u8, 255);
}

pub unsafe fn rlSetTexture(id: u32) {
    if id == 0 {
        // NOTE: If quads batch limit is reached, force a draw call and next batch starts
        if RLGL.State.vertexCounter
            >= (*RLGL.currentBatch).vertexBuffer[(*RLGL.currentBatch).currentBuffer as usize]
                .elementCount
                * 4
        {
            rlDrawRenderBatch(RLGL.currentBatch);
        }
        RLGL.State.currentTextureId = RLGL.State.defaultTextureId;
    } else {
        RLGL.State.currentTextureId = id;
        if (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].textureId
            != id as i32
        {
            if (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].vertexCount
                > 0
            {
                // Make sure current RLGL.currentBatch.draws[i].vertexCount is aligned a multiple of 4,
                // that way, following QUADS drawing will keep aligned with index processing
                // It implies adding some extra alignment vertex at the end of the draw,
                // those vertex are not processed but they are considered as an additional offset
                // for the next set of vertex to be drawn
                if (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].mode
                    == RL_LINES
                {
                    (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1]
                        .vertexAlignment = if (*RLGL.currentBatch).draws
                        [(*RLGL.currentBatch).drawCounter as usize - 1]
                        .vertexCount
                        < 4
                    {
                        (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1]
                            .vertexCount
                    } else {
                        (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1]
                            .vertexCount
                            % 4
                    };
                } else if (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1]
                    .mode
                    == RL_TRIANGLES
                {
                    (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1]
                        .vertexAlignment = if (*RLGL.currentBatch).draws
                        [(*RLGL.currentBatch).drawCounter as usize - 1]
                        .vertexCount
                        < 4
                    {
                        1
                    } else {
                        4 - ((*RLGL.currentBatch).draws
                            [(*RLGL.currentBatch).drawCounter as usize - 1]
                            .vertexCount
                            % 4)
                    };
                } else {
                    (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1]
                        .vertexAlignment = 0;
                }

                if !rlCheckRenderBatchLimit(
                    (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1]
                        .vertexAlignment,
                ) {
                    RLGL.State.vertexCounter += (*RLGL.currentBatch).draws
                        [(*RLGL.currentBatch).drawCounter as usize - 1]
                        .vertexAlignment;

                    (*RLGL.currentBatch).drawCounter += 1;

                    (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1]
                        .mode = (*RLGL.currentBatch).draws
                        [(*RLGL.currentBatch).drawCounter as usize - 2]
                        .mode;
                }
            }

            if (*RLGL.currentBatch).drawCounter >= RL_DEFAULT_BATCH_DRAWCALLS {
                rlDrawRenderBatch(RLGL.currentBatch);
            }

            (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].textureId =
                id as i32;
            (*RLGL.currentBatch).draws[(*RLGL.currentBatch).drawCounter as usize - 1].vertexCount =
                0;
        }
    }
}

pub unsafe fn rlActiveTextureSlot(slot: i32) {
    gl::ActiveTexture(gl::TEXTURE0 + slot as u32);
}

pub unsafe fn rlEnableTexture(id: u32) {
    gl::BindTexture(gl::TEXTURE_2D, id);
}

pub unsafe fn rlDisableTexture() {
    gl::BindTexture(gl::TEXTURE_2D, 0);
}

pub unsafe fn rlEnableTextureCubemap(id: u32) {
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, id);
}

pub unsafe fn rlDisableTextureCubemap() {
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
}

pub const GL_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FE;
/// Set texture parameters (wrap mode/filter mode)
pub unsafe fn rlTextureParameters(id: u32, param: i32, value: i32) {
    gl::BindTexture(gl::TEXTURE_2D, id);
    match param as u32 {
        RL_TEXTURE_WRAP_S | RL_TEXTURE_WRAP_T => {
            if value as u32 == RL_TEXTURE_WRAP_MIRROR_CLAMP {
                if RLGL.ExtSupported.texMirrorClamp {
                    gl::TexParameteri(gl::TEXTURE_2D, param as u32, value);
                } else {
                    warn!("GL: Clamp mirror wrap mode not supported (GL_MIRROR_CLAMP_EXT)",);
                }
            } else {
                gl::TexParameteri(gl::TEXTURE_2D, param as u32, value);
            }
        }

        RL_TEXTURE_MAG_FILTER | RL_TEXTURE_MIN_FILTER => {
            gl::TexParameteri(gl::TEXTURE_2D, param as u32, value);
        }

        RL_TEXTURE_FILTER_ANISOTROPIC => {
            // Reset anisotropy filter, in case it was set
            gl::TexParameterf(gl::TEXTURE_2D, GL_TEXTURE_MAX_ANISOTROPY_EXT, 1.0);

            if (value as f32) <= RLGL.ExtSupported.maxAnisotropyLevel {
                gl::TexParameterf(gl::TEXTURE_2D, GL_TEXTURE_MAX_ANISOTROPY_EXT, value as f32);
            } else if RLGL.ExtSupported.maxAnisotropyLevel > 0.0 {
                warn!(
                    "GL: Maximum anisotropic filter level supported is {}X level {}",
                    id, RLGL.ExtSupported.maxAnisotropyLevel as i32,
                );

                gl::TexParameterf(gl::TEXTURE_2D, GL_TEXTURE_MAX_ANISOTROPY_EXT, value as f32);
            } else {
                warn!("GL: Anisotropic filtering not supported");
            }
        }

        #[cfg(feature = "opengl_33")]
        RL_TEXTURE_MIPMAP_BIAS_RATIO => {
            gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_LOD_BIAS, value as f32 / 100.0);
        }

        _ => {}
    }

    gl::BindTexture(gl::TEXTURE_2D, 0);
}

pub unsafe fn rlCubemapParameters(id: u32, param: i32, value: i32) {
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, id);

    // Reset anisotropy filter, in case it was set
    gl::TexParameterf(gl::TEXTURE_CUBE_MAP, GL_TEXTURE_MAX_ANISOTROPY_EXT, 1.0);

    match param as u32 {
        RL_TEXTURE_WRAP_S | RL_TEXTURE_WRAP_T => {
            if value == RL_TEXTURE_WRAP_MIRROR_CLAMP as i32 {
                if RLGL.ExtSupported.texMirrorClamp {
                    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, param as u32, value);
                } else {
                    warn!("GL: Clamp mirror wrap mode not supported (GL_MIRROR_CLAMP_EXT)");
                }
            } else {
                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, param as u32, value);
            }
        }

        RL_TEXTURE_MAG_FILTER | RL_TEXTURE_MIN_FILTER => {
            gl::TexParameteri(gl::TEXTURE_CUBE_MAP, param as u32, value);
        }

        RL_TEXTURE_FILTER_ANISOTROPIC => {
            if (value as f32) <= RLGL.ExtSupported.maxAnisotropyLevel {
                gl::TexParameterf(
                    gl::TEXTURE_CUBE_MAP,
                    GL_TEXTURE_MAX_ANISOTROPY_EXT,
                    value as f32,
                );
            } else if RLGL.ExtSupported.maxAnisotropyLevel > 0.0 {
                warn!(
                    "GL: Maximum anisotropic filter level supported is {}X, level {}",
                    id, RLGL.ExtSupported.maxAnisotropyLevel as i32,
                );

                gl::TexParameterf(
                    gl::TEXTURE_CUBE_MAP,
                    GL_TEXTURE_MAX_ANISOTROPY_EXT,
                    value as f32,
                );
            } else {
                warn!("GL: Anisotropic filtering not supported");
            }
        }

        #[cfg(feature = "opengl_33")]
        RL_TEXTURE_MIPMAP_BIAS_RATIO => {
            gl::TexParameterf(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_LOD_BIAS,
                value as f32 / 100.0,
            );
        }

        _ => {}
    }

    gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
}

pub unsafe fn rlEnableShader(id: u32) {
    gl::UseProgram(id);
}

// Disable shader program
pub unsafe fn rlDisableShader() {
    gl::UseProgram(0);
}

// Enable rendering to texture (fbo)
pub unsafe fn rlEnableFramebuffer(id: u32) {
    gl::BindFramebuffer(gl::FRAMEBUFFER, id);
}

// return the active render texture (fbo)
pub unsafe fn rlGetActiveFramebuffer() -> u32 {
    let mut fboId: i32 = 0;

    #[cfg(any(feature = "opengl_33", feature = "opengl_es3",))]
    {
        gl::GetIntegerv(gl::DRAW_FRAMEBUFFER_BINDING, &mut fboId);
    }

    fboId as u32
}

// Disable rendering to texture
pub unsafe fn rlDisableFramebuffer() {
    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
}

// Blit active framebuffer to main framebuffer
pub unsafe fn rlBlitFramebuffer(
    srcX: i32,
    srcY: i32,
    srcWidth: i32,
    srcHeight: i32,
    dstX: i32,
    dstY: i32,
    dstWidth: i32,
    dstHeight: i32,
    bufferMask: i32,
) {
    #[cfg(any(feature = "opengl_33", feature = "opengl_es3"))]
    {
        gl::BlitFramebuffer(
            srcX,
            srcY,
            srcWidth,
            srcHeight,
            dstX,
            dstY,
            dstWidth,
            dstHeight,
            bufferMask as u32,
            gl::NEAREST,
        );
    }
}

// Bind framebuffer object (fbo)
pub unsafe fn rlBindFramebuffer(target: u32, framebuffer: u32) {
    gl::BindFramebuffer(target, framebuffer);
}

// Activate multiple draw color buffers
// NOTE: One color buffer is always active by default
pub unsafe fn rlActiveDrawBuffers(count: i32) {
    #[cfg(any(feature = "opengl_33", feature = "opengl_es3"))]
    {
        // NOTE: Maximum number of draw buffers supported is implementation dependent,
        // it can be queried with glGet*() but it must be at least 8
        //GLint maxDrawBuffers = 0;
        //glGetIntegerv(GL_MAX_DRAW_BUFFERS, &maxDrawBuffers);

        if (count > 0) {
            if (count > 8) {
                warn!("GL: Max color buffers limited to 8");
            } else {
                let buffers = [
                    gl::COLOR_ATTACHMENT0,
                    gl::COLOR_ATTACHMENT1,
                    gl::COLOR_ATTACHMENT2,
                    gl::COLOR_ATTACHMENT3,
                    gl::COLOR_ATTACHMENT4,
                    gl::COLOR_ATTACHMENT5,
                    gl::COLOR_ATTACHMENT6,
                    gl::COLOR_ATTACHMENT7,
                ];

                gl::DrawBuffers(count, buffers.as_ptr());
            }
        } else {
            warn!("GL: One color buffer active by default");
        }
    }
}

// Render state configuration
pub unsafe fn rlEnableColorBlend() {
    gl::Enable(gl::BLEND);
}
pub unsafe fn rlDisableColorBlend() {
    gl::Disable(gl::BLEND);
}
pub unsafe fn rlEnableDepthTest() {
    gl::Enable(gl::DEPTH_TEST);
}
pub unsafe fn rlDisableDepthTest() {
    gl::Disable(gl::DEPTH_TEST);
}
pub unsafe fn rlEnableDepthMask() {
    gl::DepthMask(gl::TRUE);
}
pub unsafe fn rlDisableDepthMask() {
    gl::DepthMask(gl::FALSE);
}
pub unsafe fn rlEnableBackfaceCulling() {
    gl::Enable(gl::CULL_FACE);
}
pub unsafe fn rlDisableBackfaceCulling() {
    gl::Disable(gl::CULL_FACE);
}

/// Set color mask active for screen read/draw
pub unsafe fn rlColorMask(r: bool, g: bool, b: bool, a: bool) {
    gl::ColorMask(r as u8, g as u8, b as u8, a as u8);
}

pub unsafe fn rlSetCullFace(mode: i32) {
    match mode {
        RL_CULL_FACE_BACK => gl::CullFace(gl::BACK),
        RL_CULL_FACE_FRONT => gl::CullFace(gl::FRONT),
        _ => {}
    }
}
pub unsafe fn rlEnableScissorTest() {
    gl::Enable(gl::SCISSOR_TEST);
}
pub unsafe fn rlDisableScissorTest() {
    gl::Disable(gl::SCISSOR_TEST);
}
pub unsafe fn rlScissor(x: i32, y: i32, width: i32, height: i32) {
    gl::Scissor(x, y, width, height);
}

pub unsafe fn rlEnableWireMode() {
    // NOTE: glPolygonMode() not available on OpenGL ES
    #[cfg(feature = "opengl_33")]
    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
}

pub unsafe fn rlDisableWireMode() {
    // NOTE: glPolygonMode() not available on OpenGL ES
    #[cfg(feature = "opengl_33")]
    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
}

pub unsafe fn rlEnablePointMode() {
    #[cfg(feature = "opengl_33")]
    {
        // NOTE: glPolygonMode() not available on OpenGL ES
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::POINT);
        gl::Enable(gl::PROGRAM_POINT_SIZE);
    }
}

pub unsafe fn rlDisablePointMode() {
    #[cfg(feature = "opengl_33")]
    // NOTE: glPolygonMode() not available on OpenGL ES
    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
}

pub unsafe fn rlSetLineWidth(width: f32) {
    gl::LineWidth(width);
}

pub unsafe fn rlGetLineWidth() -> f32 {
    let mut width: f32 = 0.0;
    gl::GetFloatv(gl::LINE_WIDTH, &mut width);
    width
}

pub unsafe fn rlSetPointSize(_size: f32) {
    // gl 11
    //gl::PointSize(size);
}

pub unsafe fn rlGetPointSize() -> f32 {
    let size: f32 = 1.0;
    // gl 11
    //gl::GetFloatv(gl::POINT_SIZE, &mut size);
    size
}

pub unsafe fn rlEnableSmoothLines() {
    #[cfg(feature = "opengl_33")]
    gl::Enable(gl::LINE_SMOOTH);
}

pub unsafe fn rlDisableSmoothLines() {
    #[cfg(feature = "opengl_33")]
    gl::Disable(gl::LINE_SMOOTH);
}

// Enable stereo rendering
pub unsafe fn rlEnableStereoRender() {
    RLGL.State.stereoRender = true;
}

// Disable stereo rendering
pub unsafe fn rlDisableStereoRender() {
    RLGL.State.stereoRender = false;
}

// Check if stereo render is enabled
pub unsafe fn rlIsStereoRenderEnabled() -> bool {
    return RLGL.State.stereoRender;
}

/// Clear color buffer with color
pub unsafe fn rlClearColor(r: u8, g: u8, b: u8, a: u8) {
    // Color values clamp to 0.0f(0) and 1.0f(255)
    gl::ClearColor(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    );
}

/// Clear used screen buffers (color and depth)
pub unsafe fn rlClearScreenBuffers() {
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // Clear used buffers: Color and Depth (Depth is used for 3D)

    //glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT | GL_STENCIL_BUFFER_BIT);     // Stencil buffer not used...
}

// Check and log OpenGL error codes
#[rustfmt::skip]
pub unsafe fn rlCheckErrors()
{
    let mut check = true;
    while check
    {
        let err = gl::GetError();
        match err
        {
            gl::NO_ERROR => { check = false; break;}
            0x0500 => { warn!("GL: Error detected: GL_INVALID_ENUM"); break;}
            0x0501 => { warn!("GL: Error detected: GL_INVALID_VALUE"); break;}
            0x0502 => { warn!("GL: Error detected: GL_INVALID_OPERATION"); break;}
            0x0503 => { warn!("GL: Error detected: GL_STACK_OVERFLOW"); break;}
            0x0504 => { warn!("GL: Error detected: GL_STACK_UNDERFLOW");  break;}
            0x0505 => { warn!("GL: Error detected: GL_OUT_OF_MEMORY"); break;}
            0x0506 => { warn!("GL: Error detected: GL_INVALID_FRAMEBUFFER_OPERATION"); break;}
            _ => { warn!("GL: Error detected: Unknown error code: %{}", err); break;}
        }
    }
}

/// Set blend mode
#[rustfmt::skip]
pub unsafe fn rlSetBlendMode(mode: i32)
{
    if ((RLGL.State.currentBlendMode != mode as u32) || ((mode == rlBlendMode::RL_BLEND_CUSTOM as i32 || mode == rlBlendMode::RL_BLEND_CUSTOM_SEPARATE as i32) && RLGL.State.glCustomBlendModeModified))
    {
        rlDrawRenderBatch(RLGL.currentBatch);

        match mode
        {
            val if val == rlBlendMode::RL_BLEND_ALPHA as i32 => { gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); gl::BlendEquation(gl::FUNC_ADD); }
            val if val == rlBlendMode::RL_BLEND_ADDITIVE as i32 => { gl::BlendFunc(gl::SRC_ALPHA, gl::ONE); gl::BlendEquation(gl::FUNC_ADD); }
            val if val == rlBlendMode::RL_BLEND_MULTIPLIED as i32 => { gl::BlendFunc(gl::DST_COLOR, gl::ONE_MINUS_SRC_ALPHA); gl::BlendEquation(gl::FUNC_ADD); }
            val if val == rlBlendMode::RL_BLEND_ADD_COLORS as i32 => { gl::BlendFunc(gl::ONE, gl::ONE); gl::BlendEquation(gl::FUNC_ADD); }
            val if val == rlBlendMode::RL_BLEND_SUBTRACT_COLORS as i32 => { gl::BlendFunc(gl::ONE, gl::ONE); gl::BlendEquation(gl::FUNC_SUBTRACT); }
            val if val == rlBlendMode::RL_BLEND_ALPHA_PREMULTIPLY as i32 => { gl::BlendFunc(gl::ONE, gl::ONE_MINUS_SRC_ALPHA); gl::BlendEquation(gl::FUNC_ADD); }
            val if val == rlBlendMode::RL_BLEND_CUSTOM as i32 => {
                // NOTE: Using GL blend src/dst factors and GL equation configured with rlSetBlendFactors()
                gl::BlendFunc(RLGL.State.glBlendSrcFactor, RLGL.State.glBlendDstFactor);
                gl::BlendEquation(RLGL.State.glBlendEquation);
            }
            val if val == rlBlendMode::RL_BLEND_CUSTOM_SEPARATE as i32 => {
                // NOTE: Using GL blend src/dst factors and GL equation configured with rlSetBlendFactorsSeparate()
                gl::BlendFuncSeparate(RLGL.State.glBlendSrcFactorRGB, RLGL.State.glBlendDestFactorRGB, RLGL.State.glBlendSrcFactorAlpha, RLGL.State.glBlendDestFactorAlpha);
                gl::BlendEquationSeparate(RLGL.State.glBlendEquationRGB, RLGL.State.glBlendEquationAlpha);
            } 
            _ => {}
        }

        RLGL.State.currentBlendMode = mode as u32;
        RLGL.State.glCustomBlendModeModified = false;
    }
}

// Set blending mode factor and equation
pub unsafe fn rlSetBlendFactors(glSrcFactor: i32, glDstFactor: i32, glEquation: i32) {
    if ((RLGL.State.glBlendSrcFactor != glSrcFactor as u32)
        || (RLGL.State.glBlendDstFactor != glDstFactor as u32)
        || (RLGL.State.glBlendEquation != glEquation as u32))
    {
        RLGL.State.glBlendSrcFactor = glSrcFactor as u32;
        RLGL.State.glBlendDstFactor = glDstFactor as u32;
        RLGL.State.glBlendEquation = glEquation as u32;

        RLGL.State.glCustomBlendModeModified = true;
    }
}

// Set blending mode factor and equation separately for RGB and alpha
pub unsafe fn rlSetBlendFactorsSeparate(
    glSrcRGB: i32,
    glDstRGB: i32,
    glSrcAlpha: i32,
    glDstAlpha: i32,
    glEqRGB: i32,
    glEqAlpha: i32,
) {
    if ((RLGL.State.glBlendSrcFactorRGB != glSrcRGB as u32)
        || (RLGL.State.glBlendDestFactorRGB != glDstRGB as u32)
        || (RLGL.State.glBlendSrcFactorAlpha != glSrcAlpha as u32)
        || (RLGL.State.glBlendDestFactorAlpha != glDstAlpha as u32)
        || (RLGL.State.glBlendEquationRGB != glEqRGB as u32)
        || (RLGL.State.glBlendEquationAlpha != glEqAlpha as u32))
    {
        RLGL.State.glBlendSrcFactorRGB = glSrcRGB as u32;
        RLGL.State.glBlendDestFactorRGB = glDstRGB as u32;
        RLGL.State.glBlendSrcFactorAlpha = glSrcAlpha as u32;
        RLGL.State.glBlendDestFactorAlpha = glDstAlpha as u32;
        RLGL.State.glBlendEquationRGB = glEqRGB as u32;
        RLGL.State.glBlendEquationAlpha = glEqAlpha as u32;

        RLGL.State.glCustomBlendModeModified = true;
    }
}

pub unsafe fn rlglInit(width: i32, height: i32) {
    IS_GPU_READY = true;

    // Init default white texture
    let pixels = [255, 255, 255, 255]; // 1 pixel RGBA (4 bytes)
    RLGL.State.defaultTextureId = rlLoadTexture(
        pixels.as_ptr() as *const std::ffi::c_void,
        1,
        1,
        PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32,
        1,
    );
    RLGL.State.currentTextureId = RLGL.State.defaultTextureId;

    if (RLGL.State.defaultTextureId != 0) {
        info!(
            "TEXTURE: [ID {}] Default texture loaded successfully",
            RLGL.State.defaultTextureId
        );
    } else {
        warn!("TEXTURE: Failed to load default texture");
    }

    // Init default Shader (customized for GL 3.3 and ES2)
    // Loaded: RLGL.State.defaultShaderId + RLGL.State.defaultShaderLocs
    rlLoadShaderDefault();
    RLGL.State.currentShaderId = RLGL.State.defaultShaderId;
    RLGL.State.currentShaderLocs = RLGL.State.defaultShaderLocs;

    // Init default vertex arrays buffers
    // Simulate that the default shader has the location RL_SHADER_LOC_VERTEX_NORMAL to bind the normal buffer for the default render batch
    *(RLGL
        .State
        .currentShaderLocs
        .add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_NORMAL as usize)) =
        RL_DEFAULT_SHADER_ATTRIB_LOCATION_NORMAL as i32;
    RLGL.defaultBatch =
        rlLoadRenderBatch(RL_DEFAULT_BATCH_BUFFERS, RL_DEFAULT_BATCH_BUFFER_ELEMENTS);
    *(RLGL
        .State
        .currentShaderLocs
        .add(rlShaderLocationIndex::RL_SHADER_LOC_VERTEX_NORMAL as usize)) = -1;
    RLGL.currentBatch = &mut RLGL.defaultBatch;

    // Init stack matrices (emulating OpenGL 1.1)
    for i in 0..RL_MAX_MATRIX_STACK_SIZE {
        RLGL.State.stack[i] = Matrix::IDENTITY;
    }

    // Init internal matrices
    RLGL.State.transform = Matrix::IDENTITY;
    RLGL.State.projection = Matrix::IDENTITY;
    RLGL.State.modelview = Matrix::IDENTITY;
    RLGL.State.currentMatrix = &mut RLGL.State.modelview;

    // Initialize OpenGL default states
    //----------------------------------------------------------
    // Init state: Depth test
    gl::DepthFunc(gl::LEQUAL); // Type of depth testing to apply
    gl::Disable(gl::DEPTH_TEST); // Disable depth testing for 2D (only used for 3D)

    // Init state: Blending mode
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); // Color blending function (how colors are mixed)
    gl::Enable(gl::BLEND); // Enable color blending (required to work with transparencies)

    // Init state: Culling
    // NOTE: All shapes/models triangles are drawn CCW
    gl::CullFace(gl::BACK); // Cull the back face (default)
    gl::FrontFace(gl::CCW); // Front face are defined counter clockwise (default)
    gl::Enable(gl::CULL_FACE); // Enable backface culling

    // Init state: Cubemap seamless
    #[cfg(feature = "opengl_33")]
    gl::Enable(gl::TEXTURE_CUBE_MAP_SEAMLESS); // Seamless cubemaps (not supported on OpenGL ES 2.0)

    // Store screen size into global variables
    RLGL.State.framebufferWidth = width;
    RLGL.State.framebufferHeight = height;

    // Init state: Color/Depth buffers clear
    gl::ClearColor(0.0, 0.0, 0.0, 1.0); // Set clear color (black)
    gl::ClearDepth(1.0); // Set clear depth value (default)
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); // Clear color and depth buffers (depth buffer required for 3D)

    info!("RLGL: Default OpenGL state initialized successfully");
    //----------------------------------------------------------
}

pub unsafe fn rlglClose() {
    rlUnloadRenderBatch(&mut RLGL.defaultBatch);
    rlUnloadShaderDefault();
    gl::DeleteTextures(1, &RLGL.State.defaultTextureId);

    info!(
        "TEXTURE: [ID {}] Default texture unloaded successfully",
        RLGL.State.defaultTextureId
    );
    IS_GPU_READY = false;
}

pub fn rlGetPixelFormatName(format: i32) -> &'static str {
    if format == PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAYSCALE as i32 {
        "GRAYSCALE"
    } else if format == PixelFormat::PIXELFORMAT_UNCOMPRESSED_GRAY_ALPHA as i32 {
        "GRAY_ALPHA"
    } else if format == PixelFormat::PIXELFORMAT_UNCOMPRESSED_R5G6B5 as i32 {
        "R5G6B5"
    } else if format == PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8 as i32 {
        "R8G8B8"
    } else if format == PixelFormat::PIXELFORMAT_UNCOMPRESSED_R5G5B5A1 as i32 {
        "R5G5B5A1"
    } else if format == PixelFormat::PIXELFORMAT_UNCOMPRESSED_R4G4B4A4 as i32 {
        "R4G4B4A4"
    } else if format == PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32 {
        "R8G8B8A8"
    } else if format == PixelFormat::PIXELFORMAT_UNCOMPRESSED_R32 as i32 {
        "R32"
    } else if format == PixelFormat::PIXELFORMAT_UNCOMPRESSED_R32G32B32 as i32 {
        "R32G32B32"
    } else if format == PixelFormat::PIXELFORMAT_UNCOMPRESSED_R32G32B32A32 as i32 {
        "R32G32B32A32"
    } else if format == PixelFormat::PIXELFORMAT_UNCOMPRESSED_R16 as i32 {
        "R16"
    } else if format == PixelFormat::PIXELFORMAT_UNCOMPRESSED_R16G16B16 as i32 {
        "R16G16B16"
    } else if format == PixelFormat::PIXELFORMAT_UNCOMPRESSED_R16G16B16A16 as i32 {
        "R16G16B16A16"
    } else if format == PixelFormat::PIXELFORMAT_COMPRESSED_DXT1_RGB as i32 {
        "DXT1_RGB"
    } else if format == PixelFormat::PIXELFORMAT_COMPRESSED_DXT1_RGBA as i32 {
        "DXT1_RGBA"
    } else if format == PixelFormat::PIXELFORMAT_COMPRESSED_DXT3_RGBA as i32 {
        "DXT3_RGBA"
    } else if format == PixelFormat::PIXELFORMAT_COMPRESSED_DXT5_RGBA as i32 {
        "DXT5_RGBA"
    } else if format == PixelFormat::PIXELFORMAT_COMPRESSED_ETC1_RGB as i32 {
        "ETC1_RGB"
    } else if format == PixelFormat::PIXELFORMAT_COMPRESSED_ETC2_RGB as i32 {
        "ETC2_RGB"
    } else if format == PixelFormat::PIXELFORMAT_COMPRESSED_ETC2_EAC_RGBA as i32 {
        "ETC2_RGBA"
    } else if format == PixelFormat::PIXELFORMAT_COMPRESSED_PVRT_RGB as i32 {
        "PVRT_RGB"
    } else if format == PixelFormat::PIXELFORMAT_COMPRESSED_PVRT_RGBA as i32 {
        "PVRT_RGBA"
    } else if format == PixelFormat::PIXELFORMAT_COMPRESSED_ASTC_4x4_RGBA as i32 {
        "ASTC_4x4_RGBA"
    } else if format == PixelFormat::PIXELFORMAT_COMPRESSED_ASTC_8x8_RGBA as i32 {
        "ASTC_8x8_RGBA"
    } else {
        "UNKNOWN"
    }
}

pub unsafe fn rlUnloadTexture(id: u32) {
    gl::DeleteTextures(1, &id);
}

unsafe fn rlLoadShaderDefault() {
    let vs = rlLoadShader(crate::core::DEFAULT_VSHADER, RL_VERTEX_SHADER as i32);
    let fs = rlLoadShader(crate::core::DEFAULT_FSHADER, RL_FRAGMENT_SHADER as i32);
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);

    gl::BindAttribLocation(
        program,
        RL_DEFAULT_SHADER_ATTRIB_LOCATION_POSITION,
        "vertexPosition\0".as_ptr() as *const i8,
    );
    gl::BindAttribLocation(
        program,
        RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD,
        "vertexTexCoord\0".as_ptr() as *const i8,
    );
    gl::BindAttribLocation(
        program,
        RL_DEFAULT_SHADER_ATTRIB_LOCATION_COLOR,
        "vertexColor\0".as_ptr() as *const i8,
    );

    gl::LinkProgram(program);
    RLGL.State.defaultShaderId = program;
    RLGL.State.currentShaderId = program;
}

unsafe fn rlUnloadShaderDefault() {
    gl::DeleteProgram(RLGL.State.defaultShaderId);
}

pub unsafe fn rlLoadShader(code: &str, shaderType: i32) -> u32 {
    let shader = gl::CreateShader(shaderType as u32);
    let c_str = std::ffi::CString::new(code).unwrap();
    gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
    gl::CompileShader(shader);
    shader
}

pub unsafe fn rlGetGlTextureFormats(
    format: i32,
    glInternalFormat: &mut u32,
    glFormat: &mut u32,
    glType: &mut u32,
) {
    *glInternalFormat = 0;
    *glFormat = 0;
    *glType = 0;

    match format {
        1 => {
            // GRAYSCALE
            *glInternalFormat = gl::R8;
            *glFormat = gl::RED;
            *glType = gl::UNSIGNED_BYTE;
        }
        2 => {
            // GRAY_ALPHA
            *glInternalFormat = gl::RG8;
            *glFormat = gl::RG;
            *glType = gl::UNSIGNED_BYTE;
        }
        7 => {
            // R8G8B8A8
            *glInternalFormat = gl::RGBA8;
            *glFormat = gl::RGBA;
            *glType = gl::UNSIGNED_BYTE;
        }
        _ => {
            // Simplified for now, add others as needed
        }
    }
}

pub unsafe fn rlGetPixelDataSize(width: i32, height: i32, format: i32) -> i32 {
    let mut bpp = 0;
    match format {
        1 => bpp = 8,
        2 => bpp = 16,
        3 => bpp = 16,
        4 => bpp = 24,
        5 => bpp = 16,
        6 => bpp = 16,
        7 => bpp = 32,
        _ => {}
    }
    (width * height * bpp) / 8
}

pub unsafe fn rlLoadFramebuffer() -> u32 {
    let mut id = 0;
    gl::GenFramebuffers(1, &mut id);
    id
}

pub unsafe fn rlFramebufferAttach(
    id: u32,
    texId: u32,
    attachType: i32,
    texType: i32,
    mipLevel: i32,
) {
    gl::BindFramebuffer(gl::FRAMEBUFFER, id);
    if attachType < 100 {
        // Color
        let attachment = gl::COLOR_ATTACHMENT0 + attachType as u32;
        if texType == 100 {
            // Texture2D
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, attachment, gl::TEXTURE_2D, texId, mipLevel);
        } else if texType == 200 {
            // Renderbuffer
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, attachment, gl::RENDERBUFFER, texId);
        }
    } else if attachType == 100 {
        // Depth
        if texType == 100 {
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::DEPTH_ATTACHMENT,
                gl::TEXTURE_2D,
                texId,
                mipLevel,
            );
        } else if texType == 200 {
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_ATTACHMENT,
                gl::RENDERBUFFER,
                texId,
            );
        }
    }
    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
}

pub unsafe fn rlFramebufferComplete(id: u32) -> bool {
    gl::BindFramebuffer(gl::FRAMEBUFFER, id);
    let status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    status == gl::FRAMEBUFFER_COMPLETE
}

pub unsafe fn rlUnloadFramebuffer(id: u32) {
    gl::DeleteFramebuffers(1, &id);
}

pub unsafe fn rlLoadVertexBuffer(buffer: *const std::ffi::c_void, size: i32, dynamic: bool) -> u32 {
    let mut id = 0;
    gl::GenBuffers(1, &mut id);
    gl::BindBuffer(gl::ARRAY_BUFFER, id);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        size as isize,
        buffer,
        if dynamic {
            gl::DYNAMIC_DRAW
        } else {
            gl::STATIC_DRAW
        },
    );
    id
}

pub unsafe fn rlLoadVertexBufferElement(
    buffer: *const std::ffi::c_void,
    size: i32,
    dynamic: bool,
) -> u32 {
    let mut id = 0;
    gl::GenBuffers(1, &mut id);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        size as isize,
        buffer,
        if dynamic {
            gl::DYNAMIC_DRAW
        } else {
            gl::STATIC_DRAW
        },
    );
    id
}

pub unsafe fn rlUpdateVertexBuffer(id: u32, data: *const std::ffi::c_void, size: i32, offset: i32) {
    gl::BindBuffer(gl::ARRAY_BUFFER, id);
    gl::BufferSubData(gl::ARRAY_BUFFER, offset as isize, size as isize, data);
}

pub unsafe fn rlUnloadVertexBuffer(id: u32) {
    gl::DeleteBuffers(1, &id);
}

pub unsafe fn rlLoadVertexArray() -> u32 {
    let mut id = 0;
    gl::GenVertexArrays(1, &mut id);
    id
}

pub unsafe fn rlUnloadVertexArray(id: u32) {
    gl::DeleteVertexArrays(1, &id);
}

pub unsafe fn rlSetVertexAttribute(
    index: u32,
    size: i32,
    type_: i32,
    normalized: bool,
    stride: i32,
    pointer: *const std::ffi::c_void,
) {
    gl::VertexAttribPointer(index, size, type_ as u32, normalized as u8, stride, pointer);
}

pub unsafe fn rlEnableVertexAttribute(index: u32) {
    gl::EnableVertexAttribArray(index);
}

pub unsafe fn rlDisableVertexAttribute(index: u32) {
    gl::DisableVertexAttribArray(index);
}

pub unsafe fn rlSetVertexAttributeDivisor(index: u32, divisor: u32) {
    gl::VertexAttribDivisor(index, divisor);
}

pub unsafe fn rlSetUniform(
    locIndex: i32,
    value: *const std::ffi::c_void,
    uniformType: i32,
    count: i32,
) {
    match uniformType {
        0 => gl::Uniform1fv(locIndex, count, value as *const f32),
        1 => gl::Uniform2fv(locIndex, count, value as *const f32),
        2 => gl::Uniform3fv(locIndex, count, value as *const f32),
        3 => gl::Uniform4fv(locIndex, count, value as *const f32),
        4 => gl::Uniform1iv(locIndex, count, value as *const i32),
        _ => {}
    }
}

pub unsafe fn rlSetUniformMatrix(locIndex: i32, mat: Matrix) {
    //gl::UniformMatrix4fv(locIndex, 1, gl::FALSE as u8, mat.to_cols_array().as_ptr());
}

pub unsafe fn rlSetUniformSampler(locIndex: i32, textureId: u32) {
    for i in 0..RL_DEFAULT_BATCH_MAX_TEXTURE_UNITS as usize {
        if RLGL.State.activeTextureId[i] == textureId {
            gl::Uniform1i(locIndex, (1 + i) as i32);
            return;
        }
    }
    for i in 0..RL_DEFAULT_BATCH_MAX_TEXTURE_UNITS as usize {
        if RLGL.State.activeTextureId[i] == 0 {
            gl::Uniform1i(locIndex, (1 + i) as i32);
            RLGL.State.activeTextureId[i] = textureId;
            break;
        }
    }
}

pub unsafe fn rlSetShader(id: u32, _locs: *mut i32) {
    if RLGL.State.currentShaderId != id {
        rlDrawRenderBatch(RLGL.currentBatch);
        RLGL.State.currentShaderId = id;
    }
}

pub unsafe fn rlGetMatrixModelview() -> Matrix {
    RLGL.State.modelview
}
pub unsafe fn rlGetMatrixProjection() -> Matrix {
    RLGL.State.projection
}
pub unsafe fn rlGetMatrixTransform() -> Matrix {
    RLGL.State.transform
}

pub unsafe fn rlSetMatrixModelview(view: Matrix) {
    RLGL.State.modelview = view;
}
pub unsafe fn rlSetMatrixProjection(proj: Matrix) {
    RLGL.State.projection = proj;
}
