#![allow(missing_safety_doc)]
use crate::types::{Color, Matrix, Rectangle, Vector2};
use gl;
use glam::{Mat4, Vec3};
use log::{debug, info, warn};

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

// --- Enums ---

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    UncompressedGrayscale = 1,
    UncompressedGrayAlpha,
    UncompressedR5G6B5,
    UncompressedR8G8B8,
    UncompressedR5G5B5A1,
    UncompressedR4G4B4A4,
    UncompressedR8G8B8A8,
    UncompressedR32,
    UncompressedR32G32B32,
    UncompressedR32G32B32A32,
    UncompressedR16,
    UncompressedR16G16B16,
    UncompressedR16G16B16A16,
    CompressedDxt1Rgb,
    CompressedDxt1Rgba,
    CompressedDxt3Rgba,
    CompressedDxt5Rgba,
    CompressedEtc1Rgb,
    CompressedEtc2Rgb,
    CompressedEtc2EacRgba,
    CompressedPvrtRgb,
    CompressedPvrtRgba,
    CompressedAstc4x4Rgba,
    CompressedAstc8x8Rgba,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramebufferAttachment {
    ColorChannel0 = 0,
    ColorChannel1,
    ColorChannel2,
    ColorChannel3,
    ColorChannel4,
    ColorChannel5,
    ColorChannel6,
    ColorChannel7,
    Depth = 100,
    Stencil = 200,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramebufferAttachTexture {
    CubemapPositiveX = 0,
    CubemapNegativeX,
    CubemapPositiveY,
    CubemapNegativeY,
    CubemapPositiveZ,
    CubemapNegativeZ,
    Texture2d = 100,
    Renderbuffer = 200,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFilter {
    Point = 0,
    Bilinear,
    Trilinear,
    Anisotropic4x,
    Anisotropic8x,
    Anisotropic16x,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Alpha = 0,
    Additive,
    Multiplied,
    AddColors,
    SubtractColors,
    AlphaPremultiply,
    Custom,
    CustomSeparate,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderLocationIndex {
    VertexPosition = 0,
    VertexTexcoord01,
    VertexTexcoord02,
    VertexNormal,
    VertexTangent,
    VertexColor,
    MatrixMvp,
    MatrixView,
    MatrixProjection,
    MatrixModel,
    MatrixNormal,
    VectorView,
    ColorDiffuse,
    ColorSpecular,
    ColorAmbient,
    MapAlbedo,
    MapMetalness,
    MapNormal,
    MapRoughness,
    MapOcclusion,
    MapEmission,
    MapHeight,
    MapCubemap,
    MapIrradiance,
    MapPrefilter,
    MapBrdf,
}

pub const RL_SHADER_LOC_MAP_DIFFUSE: i32 = ShaderLocationIndex::MapAlbedo as i32;
pub const RL_SHADER_LOC_MAP_SPECULAR: i32 = ShaderLocationIndex::MapMetalness as i32;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderUniformDataType {
    Float = 0,
    Vec2,
    Vec3,
    Vec4,
    Int,
    Ivec2,
    Ivec3,
    Ivec4,
    Uint,
    Uivec2,
    Uivec3,
    Uivec4,
    Sampler2d,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderAttributeDataType {
    Float = 0,
    Vec2,
    Vec3,
    Vec4,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramebufferAttachType {
    ColorChannel0 = 0,
    ColorChannel1 = 1,
    ColorChannel2 = 2,
    ColorChannel3 = 3,
    ColorChannel4 = 4,
    ColorChannel5 = 5,
    ColorChannel6 = 6,
    ColorChannel7 = 7,
    Depth = 100,
    Stencil = 200,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramebufferAttachTextureType {
    CubemapPositiveX = 0,
    CubemapNegativeX = 1,
    CubemapPositiveY = 2,
    CubemapNegativeY = 3,
    CubemapPositiveZ = 4,
    CubemapNegativeZ = 5,
    Texture2d = 100,
    Renderbuffer = 200,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullMode {
    Front = 0,
    Back,
}

// --- Types ---

#[derive(Debug)]
pub struct rlVertexBuffer {
    pub elementCount: i32,
    pub vertices: *mut f32,
    pub texcoords: *mut f32,
    pub normals: *mut f32,
    pub colors: *mut u8,
    pub indices: *mut u32,
    pub vaoId: u32,
    pub vboId: [u32; 5],
}

#[derive(Debug, Clone, Copy)]
pub struct rlDrawCall {
    pub mode: i32,
    pub vertexCount: i32,
    pub vertexAlignment: i32,
    pub textureId: u32,
}

#[derive(Debug)]
pub struct rlRenderBatch {
    pub bufferCount: i32,
    pub currentBuffer: i32,
    pub vertexBuffer: *mut rlVertexBuffer,
    pub draws: *mut rlDrawCall,
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

    pub stereoRender: u32,             // Stereo rendering flag
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

pub static mut RLGL: rlglData = unsafe { std::mem::zeroed() };

// Statics
static mut IS_GPU_READY: bool = false;

// --- Functions ---

pub unsafe fn rlglInit(width: i32, height: i32) {
    if IS_GPU_READY {
        return;
    }

    // Initialize state
    RLGL.State.currentMatrixMode = RL_PROJECTION;
    RLGL.State.projection =
        Mat4::orthographic_rh_gl(0.0, width as f32, height as f32, 0.0, -1.0, 1.0);

    RLGL.State.currentMatrixMode = RL_MODELVIEW;
    RLGL.State.modelview = Matrix::IDENTITY;
    RLGL.State.transform = Matrix::IDENTITY;
    RLGL.State.colorr = 255;
    RLGL.State.colorg = 255;
    RLGL.State.colorb = 255;
    RLGL.State.colora = 255;

    // Detect extensions
    rlLoadExtensions();

    // Load default shader
    rlLoadShaderDefault();

    // Load default texture
    let pixels: [u8; 4] = [255, 255, 255, 255];
    RLGL.State.defaultTextureId = rlLoadTexture(
        pixels.as_ptr() as *const _,
        1,
        1,
        PixelFormat::UncompressedR8G8B8A8 as i32,
        1,
    );
    RLGL.State.activeTextureId[0] = RLGL.State.defaultTextureId;

    // Load default batch
    RLGL.defaultBatch =
        rlLoadRenderBatch(RL_DEFAULT_BATCH_BUFFERS, RL_DEFAULT_BATCH_BUFFER_ELEMENTS);
    RLGL.currentBatch = &mut RLGL.defaultBatch;

    RLGL.State.framebufferWidth = width;
    RLGL.State.framebufferHeight = height;

    IS_GPU_READY = true;
}

pub unsafe fn rlglClose() {
    rlUnloadRenderBatch(&mut RLGL.defaultBatch);
    rlUnloadShaderDefault();
    rlUnloadTexture(RLGL.State.defaultTextureId);
    IS_GPU_READY = false;
}

pub unsafe fn rlLoadExtensions() {
    // In Rust, we rely on the `gl` crate which usually loads everything via `load_with`.
    // We assume the caller (core.rs) has already loaded the functions.

    // For Desktop OpenGL 3.3 Core, most extensions are core features.
    RLGL.ExtSupported.vao = true;
    RLGL.ExtSupported.instancing = true;
    RLGL.ExtSupported.texNPOT = true;
    RLGL.ExtSupported.texDepth = true;
    RLGL.ExtSupported.texFloat32 = true;
    RLGL.ExtSupported.texFloat16 = true;
    RLGL.ExtSupported.maxDepthBits = 32;
    RLGL.ExtSupported.texAnisoFilter = true;
    RLGL.ExtSupported.texMirrorClamp = true;

    #[cfg(feature = "gles2")]
    {
        // On GLES2, we would check extensions here using glGetString(GL_EXTENSIONS)
        // This is a simplified port.
    }
}

// Matrix operations
pub unsafe fn rlMatrixMode(mode: i32) {
    if RLGL.State.currentMatrixMode != mode {
        rlDrawRenderBatchActive();
        RLGL.State.currentMatrixMode = mode;
    }
}

pub unsafe fn rlPushMatrix() {
    if RLGL.State.stackCounter >= RL_MAX_MATRIX_STACK_SIZE as i32 {
        return;
    }

    let mat = match RLGL.State.currentMatrixMode {
        RL_MODELVIEW => {
            RLGL.State.transformRequired = true;
            RLGL.State.modelview
        }
        RL_PROJECTION => RLGL.State.projection,
        _ => Matrix::IDENTITY,
    };

    RLGL.State.stack[RLGL.State.stackCounter as usize] = mat;
    RLGL.State.stackCounter += 1;

    if RLGL.State.currentMatrixMode == RL_MODELVIEW {
        RLGL.State.transform = mat;
    }
}

pub unsafe fn rlPopMatrix() {
    if RLGL.State.stackCounter > 0 {
        RLGL.State.stackCounter -= 1;
        let mat = RLGL.State.stack[RLGL.State.stackCounter as usize];

        match RLGL.State.currentMatrixMode {
            RL_MODELVIEW => {
                RLGL.State.transform = mat;
            }
            RL_PROJECTION => RLGL.State.projection = mat,
            _ => {}
        }

        if RLGL.State.stackCounter == 0 {
            RLGL.State.transformRequired = false;
        }
    }
}

pub unsafe fn rlLoadIdentity() {
    if !RLGL.State.transformRequired || RLGL.State.currentMatrixMode == RL_PROJECTION {
        rlDrawRenderBatchActive();
    }
    match RLGL.State.currentMatrixMode {
        RL_MODELVIEW => {
            if RLGL.State.transformRequired {
                RLGL.State.transform = Matrix::IDENTITY;
            } else {
                RLGL.State.modelview = Matrix::IDENTITY;
            }
        }
        RL_PROJECTION => RLGL.State.projection = Matrix::IDENTITY,
        _ => {}
    }
}

pub unsafe fn rlTranslatef(x: f32, y: f32, z: f32) {
    if !RLGL.State.transformRequired || RLGL.State.currentMatrixMode == RL_PROJECTION {
        rlDrawRenderBatchActive();
    }
    let mat = Mat4::from_translation(Vec3::new(x, y, z));
    match RLGL.State.currentMatrixMode {
        RL_MODELVIEW => {
            if RLGL.State.transformRequired {
                RLGL.State.transform = RLGL.State.transform * mat;
            } else {
                RLGL.State.modelview = RLGL.State.modelview * mat;
            }
        }
        RL_PROJECTION => RLGL.State.projection = RLGL.State.projection * mat,
        _ => {}
    }
}

pub unsafe fn rlRotatef(angle: f32, x: f32, y: f32, z: f32) {
    if !RLGL.State.transformRequired || RLGL.State.currentMatrixMode == RL_PROJECTION {
        rlDrawRenderBatchActive();
    }
    let axis = Vec3::new(x, y, z).normalize_or_zero();
    let mat = Mat4::from_axis_angle(axis, angle * crate::math::DEG2RAD);
    match RLGL.State.currentMatrixMode {
        RL_MODELVIEW => {
            if RLGL.State.transformRequired {
                RLGL.State.transform = RLGL.State.transform * mat;
            } else {
                RLGL.State.modelview = RLGL.State.modelview * mat;
            }
        }
        RL_PROJECTION => RLGL.State.projection = RLGL.State.projection * mat,
        _ => {}
    }
}

pub unsafe fn rlScalef(x: f32, y: f32, z: f32) {
    if !RLGL.State.transformRequired || RLGL.State.currentMatrixMode == RL_PROJECTION {
        rlDrawRenderBatchActive();
    }
    let mat = Mat4::from_scale(Vec3::new(x, y, z));
    match RLGL.State.currentMatrixMode {
        RL_MODELVIEW => {
            if RLGL.State.transformRequired {
                RLGL.State.transform = RLGL.State.transform * mat;
            } else {
                RLGL.State.modelview = RLGL.State.modelview * mat;
            }
        }
        RL_PROJECTION => RLGL.State.projection = RLGL.State.projection * mat,
        _ => {}
    }
}

pub unsafe fn rlMultMatrixf(matf: *const f32) {
    if !RLGL.State.transformRequired || RLGL.State.currentMatrixMode == RL_PROJECTION {
        rlDrawRenderBatchActive();
    }
    let slice = std::slice::from_raw_parts(matf, 16);
    let mat = Matrix::from_cols_array(slice.try_into().unwrap());
    match RLGL.State.currentMatrixMode {
        RL_MODELVIEW => {
            if RLGL.State.transformRequired {
                RLGL.State.transform = RLGL.State.transform * mat;
            } else {
                RLGL.State.modelview = RLGL.State.modelview * mat;
            }
        }
        RL_PROJECTION => RLGL.State.projection = RLGL.State.projection * mat,
        _ => {}
    }
}

pub unsafe fn rlOrtho(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
    if !RLGL.State.transformRequired || RLGL.State.currentMatrixMode == RL_PROJECTION {
        rlDrawRenderBatchActive();
    }
    let mat = Mat4::orthographic_rh_gl(
        left as f32,
        right as f32,
        bottom as f32,
        top as f32,
        znear as f32,
        zfar as f32,
    );
    match RLGL.State.currentMatrixMode {
        RL_MODELVIEW => {
            if RLGL.State.transformRequired {
                RLGL.State.transform = RLGL.State.transform * mat;
            } else {
                RLGL.State.modelview = RLGL.State.modelview * mat;
            }
        }
        RL_PROJECTION => RLGL.State.projection = RLGL.State.projection * mat,
        _ => {}
    }
}

pub unsafe fn rlViewport(x: i32, y: i32, width: i32, height: i32) {
    gl::Viewport(x, y, width, height);
}

// Vertex level operations
pub unsafe fn rlBegin(mode: i32) {
    let batch = &mut *RLGL.currentBatch;
    let last_draw_idx = (batch.drawCounter - 1) as usize;

    if batch.draws.add(last_draw_idx).read().mode != mode {
        if batch.draws.add(last_draw_idx).read().vertexCount > 0 {
            let mut last_draw = batch.draws.add(last_draw_idx).read();
            if last_draw.mode == RL_LINES {
                last_draw.vertexAlignment = last_draw.vertexCount % 2;
            } else if last_draw.mode == RL_TRIANGLES {
                last_draw.vertexAlignment = (3 - (last_draw.vertexCount % 3)) % 3;
            } else {
                last_draw.vertexAlignment = 0;
            }

            if !rlCheckRenderBatchLimit(last_draw.vertexAlignment) {
                RLGL.State.vertexCounter += last_draw.vertexAlignment;
                batch.draws.add(last_draw_idx).write(last_draw);

                if batch.drawCounter >= RL_DEFAULT_BATCH_DRAWCALLS {
                    rlDrawRenderBatch(RLGL.currentBatch);
                }

                batch
                    .draws
                    .add(batch.drawCounter as usize)
                    .write(rlDrawCall {
                        mode,
                        vertexCount: 0,
                        vertexAlignment: 0,
                        textureId: RLGL.State.activeTextureId[0],
                    });
                batch.drawCounter += 1;
            }
        } else {
            (*batch.draws.add(last_draw_idx)).mode = mode;
            (*batch.draws.add(last_draw_idx)).textureId = RLGL.State.activeTextureId[0];
        }
    }
}

pub unsafe fn rlEnd() {
    let batch = &mut *RLGL.currentBatch;
    batch.currentDepth += 1.0 / 20000.0;
}

pub unsafe fn rlVertex3f(x: f32, y: f32, z: f32) {
    let mut tx = x;
    let mut ty = y;
    let mut tz = z;

    if RLGL.State.transformRequired {
        let v = RLGL.State.transform.transform_point3(Vec3::new(x, y, z));
        tx = v.x;
        ty = v.y;
        tz = v.z;
    }

    let batch = &mut *RLGL.currentBatch;
    let buffer = &mut *batch.vertexBuffer.add(batch.currentBuffer as usize);

    if RLGL.State.vertexCounter >= buffer.elementCount * 4 {
        rlCheckRenderBatchLimit(4);
    }

    let vc = RLGL.State.vertexCounter as usize;
    buffer.vertices.add(vc * 3).write(tx);
    buffer.vertices.add(vc * 3 + 1).write(ty);
    buffer.vertices.add(vc * 3 + 2).write(tz);

    buffer.texcoords.add(vc * 2).write(RLGL.State.texcoordx);
    buffer.texcoords.add(vc * 2 + 1).write(RLGL.State.texcoordy);

    buffer.normals.add(vc * 3).write(RLGL.State.normalx);
    buffer.normals.add(vc * 3 + 1).write(RLGL.State.normaly);
    buffer.normals.add(vc * 3 + 2).write(RLGL.State.normalz);

    buffer.colors.add(vc * 4).write(RLGL.State.colorr);
    buffer.colors.add(vc * 4 + 1).write(RLGL.State.colorg);
    buffer.colors.add(vc * 4 + 2).write(RLGL.State.colorb);
    buffer.colors.add(vc * 4 + 3).write(RLGL.State.colora);

    RLGL.State.vertexCounter += 1;
    (*batch.draws.add((batch.drawCounter - 1) as usize)).vertexCount += 1;
}

pub unsafe fn rlVertex2f(x: f32, y: f32) {
    rlVertex3f(x, y, 0.0);
}
pub unsafe fn rlTexCoord2f(x: f32, y: f32) {
    RLGL.State.texcoordx = x;
    RLGL.State.texcoordy = y;
}
pub unsafe fn rlNormal3f(x: f32, y: f32, z: f32) {
    RLGL.State.normalx = x;
    RLGL.State.normaly = y;
    RLGL.State.normalz = z;
}
pub unsafe fn rlColor4ub(r: u8, g: u8, b: u8, a: u8) {
    RLGL.State.colorr = r;
    RLGL.State.colorg = g;
    RLGL.State.colorb = b;
    RLGL.State.colora = a;
}

// Render Batch Management
pub unsafe fn rlLoadRenderBatch(numBuffers: i32, bufferElements: i32) -> rlRenderBatch {
    let mut batch: rlRenderBatch = std::mem::zeroed();
    batch.bufferCount = numBuffers;
    batch.vertexBuffer = libc::malloc(numBuffers as usize * std::mem::size_of::<rlVertexBuffer>())
        as *mut rlVertexBuffer;

    for i in 0..numBuffers {
        let buffer = &mut *batch.vertexBuffer.add(i as usize);
        buffer.elementCount = bufferElements;
        buffer.vertices = libc::malloc(bufferElements as usize * 4 * 3 * 4) as *mut f32;
        buffer.texcoords = libc::malloc(bufferElements as usize * 4 * 2 * 4) as *mut f32;
        buffer.normals = libc::malloc(bufferElements as usize * 4 * 3 * 4) as *mut f32;
        buffer.colors = libc::malloc(bufferElements as usize * 4 * 4) as *mut u8;
        buffer.indices = libc::malloc(bufferElements as usize * 6 * 4) as *mut u32;

        // Initialize indices for quads (2 triangles)
        for j in 0..bufferElements {
            let base = (j * 6) as isize;
            let v = (j * 4) as u32;
            buffer.indices.offset(base).write(v);
            buffer.indices.offset(base + 1).write(v + 1);
            buffer.indices.offset(base + 2).write(v + 2);
            buffer.indices.offset(base + 3).write(v);
            buffer.indices.offset(base + 4).write(v + 2);
            buffer.indices.offset(base + 5).write(v + 3);
        }

        if RLGL.ExtSupported.vao {
            gl::GenVertexArrays(1, &mut buffer.vaoId);
            gl::BindVertexArray(buffer.vaoId);
        }

        gl::GenBuffers(5, buffer.vboId.as_mut_ptr());

        // Position
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer.vboId[0]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (bufferElements * 4 * 3 * 4) as isize,
            std::ptr::null(),
            gl::DYNAMIC_DRAW,
        );
        gl::EnableVertexAttribArray(RL_DEFAULT_SHADER_ATTRIB_LOCATION_POSITION);
        gl::VertexAttribPointer(
            RL_DEFAULT_SHADER_ATTRIB_LOCATION_POSITION,
            3,
            gl::FLOAT,
            gl::FALSE as u8,
            0,
            std::ptr::null(),
        );

        // TexCoord
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer.vboId[1]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (bufferElements * 4 * 2 * 4) as isize,
            std::ptr::null(),
            gl::DYNAMIC_DRAW,
        );
        gl::EnableVertexAttribArray(RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD);
        gl::VertexAttribPointer(
            RL_DEFAULT_SHADER_ATTRIB_LOCATION_TEXCOORD,
            2,
            gl::FLOAT,
            gl::FALSE as u8,
            0,
            std::ptr::null(),
        );

        // Normal
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer.vboId[2]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (bufferElements * 4 * 3 * 4) as isize,
            std::ptr::null(),
            gl::DYNAMIC_DRAW,
        );
        gl::EnableVertexAttribArray(RL_DEFAULT_SHADER_ATTRIB_LOCATION_NORMAL);
        gl::VertexAttribPointer(
            RL_DEFAULT_SHADER_ATTRIB_LOCATION_NORMAL,
            3,
            gl::FLOAT,
            gl::FALSE as u8,
            0,
            std::ptr::null(),
        );

        // Color
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer.vboId[3]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (bufferElements * 4 * 4) as isize,
            std::ptr::null(),
            gl::DYNAMIC_DRAW,
        );
        gl::EnableVertexAttribArray(RL_DEFAULT_SHADER_ATTRIB_LOCATION_COLOR);
        gl::VertexAttribPointer(
            RL_DEFAULT_SHADER_ATTRIB_LOCATION_COLOR,
            4,
            gl::UNSIGNED_BYTE,
            gl::TRUE as u8,
            0,
            std::ptr::null(),
        );

        // Indices
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer.vboId[4]);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (bufferElements * 6 * 4) as isize,
            buffer.indices as *const _,
            gl::STATIC_DRAW,
        );

        if RLGL.ExtSupported.vao {
            gl::BindVertexArray(0);
        }
    }

    batch.draws =
        libc::malloc(RL_DEFAULT_BATCH_DRAWCALLS as usize * std::mem::size_of::<rlDrawCall>())
            as *mut rlDrawCall;
    batch.drawCounter = 1;
    (*batch.draws).mode = RL_QUADS;
    (*batch.draws).textureId = RLGL.State.defaultTextureId;
    batch.currentDepth = -1.0;

    batch
}

pub unsafe fn rlUnloadRenderBatch(batch: &mut rlRenderBatch) {
    for i in 0..batch.bufferCount {
        let buffer = &mut *batch.vertexBuffer.add(i as usize);
        gl::DeleteBuffers(5, buffer.vboId.as_ptr());
        if RLGL.ExtSupported.vao {
            gl::DeleteVertexArrays(1, &buffer.vaoId);
        }
        libc::free(buffer.vertices as *mut _);
        libc::free(buffer.texcoords as *mut _);
        libc::free(buffer.normals as *mut _);
        libc::free(buffer.colors as *mut _);
        libc::free(buffer.indices as *mut _);
    }
    libc::free(batch.vertexBuffer as *mut _);
    libc::free(batch.draws as *mut _);
}

pub unsafe fn rlDrawRenderBatchActive() {
    rlDrawRenderBatch(RLGL.currentBatch);
}

pub unsafe fn rlDrawRenderBatch(batch: *mut rlRenderBatch) {
    if RLGL.State.vertexCounter == 0 {
        return;
    }

    let b = &mut *batch;
    let buffer = &mut *b.vertexBuffer.add(b.currentBuffer as usize);

    if RLGL.ExtSupported.vao {
        gl::BindVertexArray(buffer.vaoId);
    } else {
        // Fallback for non-VAO (GLES2)
        gl::BindBuffer(gl::ARRAY_BUFFER, buffer.vboId[0]);
        // ... set pointers ...
    }

    gl::BindBuffer(gl::ARRAY_BUFFER, buffer.vboId[0]);
    gl::BufferSubData(
        gl::ARRAY_BUFFER,
        0,
        (RLGL.State.vertexCounter * 3 * 4) as isize,
        buffer.vertices as *const _,
    );
    gl::BindBuffer(gl::ARRAY_BUFFER, buffer.vboId[1]);
    gl::BufferSubData(
        gl::ARRAY_BUFFER,
        0,
        (RLGL.State.vertexCounter * 2 * 4) as isize,
        buffer.texcoords as *const _,
    );
    gl::BindBuffer(gl::ARRAY_BUFFER, buffer.vboId[2]);
    gl::BufferSubData(
        gl::ARRAY_BUFFER,
        0,
        (RLGL.State.vertexCounter * 3 * 4) as isize,
        buffer.normals as *const _,
    );
    gl::BindBuffer(gl::ARRAY_BUFFER, buffer.vboId[3]);
    gl::BufferSubData(
        gl::ARRAY_BUFFER,
        0,
        (RLGL.State.vertexCounter * 4) as isize,
        buffer.colors as *const _,
    );

    rlEnableShader(RLGL.State.currentShaderId);

    // Set MVP
    let mvp = RLGL.State.projection * RLGL.State.modelview;
    let mvp_loc = gl::GetUniformLocation(RLGL.State.currentShaderId, "mvp\0".as_ptr() as *const i8);
    gl::UniformMatrix4fv(mvp_loc, 1, gl::FALSE as u8, mvp.to_cols_array().as_ptr());

    let mut vertex_offset = 0;
    for i in 0..b.drawCounter {
        let draw = *b.draws.add(i as usize);
        if draw.vertexCount > 0 {
            gl::BindTexture(gl::TEXTURE_2D, draw.textureId);

            if draw.mode == RL_QUADS {
                gl::DrawElements(
                    gl::TRIANGLES,
                    (draw.vertexCount / 4) * 6,
                    gl::UNSIGNED_INT,
                    (vertex_offset / 4 * 6 * 4) as *const _,
                );
            } else {
                gl::DrawArrays(draw.mode as u32, vertex_offset, draw.vertexCount);
            }
            vertex_offset += draw.vertexCount + draw.vertexAlignment;
        }
    }

    RLGL.State.vertexCounter = 0;
    b.drawCounter = 1;
    (*b.draws).vertexCount = 0;
    (*b.draws).vertexAlignment = 0;
    (*b.draws).mode = RL_QUADS;
    (*b.draws).textureId = RLGL.State.defaultTextureId;
    b.currentDepth = -1.0;
}

pub unsafe fn rlCheckRenderBatchLimit(vCount: i32) -> bool {
    let batch = &mut *RLGL.currentBatch;
    let buffer = &*batch.vertexBuffer.add(batch.currentBuffer as usize);
    if (RLGL.State.vertexCounter + vCount) >= buffer.elementCount * 4 {
        rlDrawRenderBatch(RLGL.currentBatch);
        return true;
    }
    false
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
        && ((format == PixelFormat::CompressedDxt1Rgb as i32)
            || (format == PixelFormat::CompressedDxt1Rgba as i32)
            || (format == PixelFormat::CompressedDxt3Rgba as i32)
            || (format == PixelFormat::CompressedDxt5Rgba as i32)))
    {
        warn!("GL: DXT compressed texture format not supported");
        return id;
    }
    if ((!RLGL.ExtSupported.texCompETC1) && (format == PixelFormat::CompressedEtc1Rgb as i32)) {
        warn!("GL: ETC1 compressed texture format not supported");
        return id;
    }

    if ((!RLGL.ExtSupported.texCompETC2)
        && ((format == PixelFormat::CompressedEtc2Rgb as i32)
            || (format == PixelFormat::CompressedEtc2EacRgba as i32)))
    {
        warn!("GL: ETC2 compressed texture format not supported");
        return id;
    }

    if ((!RLGL.ExtSupported.texCompPVRT)
        && ((format == PixelFormat::CompressedPvrtRgb as i32)
            || (format == PixelFormat::CompressedPvrtRgba as i32)))
    {
        warn!("GL: PVRT compressed texture format not supported");
        return id;
    }

    if ((!RLGL.ExtSupported.texCompASTC)
        && ((format == PixelFormat::CompressedAstc4x4Rgba as i32)
            || (format == PixelFormat::CompressedAstc8x8Rgba as i32)))
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
            if (format < PixelFormat::CompressedDxt1Rgb as i32) {
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
                if (format == PixelFormat::UncompressedGrayscale as i32) {
                    let swizzleMask = [gl::RED, gl::RED, gl::RED, gl::ONE];
                    gl::TexParameteriv(
                        gl::TEXTURE_2D,
                        gl::TEXTURE_SWIZZLE_RGBA,
                        swizzleMask.as_ptr() as *const i32,
                    );
                } else if (format == PixelFormat::UncompressedGrayAlpha as i32) {
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

pub fn rlGetPixelFormatName(format: i32) -> &'static str {
    if format == PixelFormat::UncompressedGrayscale as i32 {
        "GRAYSCALE"
    } else if format == PixelFormat::UncompressedGrayAlpha as i32 {
        "GRAY_ALPHA"
    } else if format == PixelFormat::UncompressedR5G6B5 as i32 {
        "R5G6B5"
    } else if format == PixelFormat::UncompressedR8G8B8 as i32 {
        "R8G8B8"
    } else if format == PixelFormat::UncompressedR5G5B5A1 as i32 {
        "R5G5B5A1"
    } else if format == PixelFormat::UncompressedR4G4B4A4 as i32 {
        "R4G4B4A4"
    } else if format == PixelFormat::UncompressedR8G8B8A8 as i32 {
        "R8G8B8A8"
    } else if format == PixelFormat::UncompressedR32 as i32 {
        "R32"
    } else if format == PixelFormat::UncompressedR32G32B32 as i32 {
        "R32G32B32"
    } else if format == PixelFormat::UncompressedR32G32B32A32 as i32 {
        "R32G32B32A32"
    } else if format == PixelFormat::UncompressedR16 as i32 {
        "R16"
    } else if format == PixelFormat::UncompressedR16G16B16 as i32 {
        "R16G16B16"
    } else if format == PixelFormat::UncompressedR16G16B16A16 as i32 {
        "R16G16B16A16"
    } else if format == PixelFormat::CompressedDxt1Rgb as i32 {
        "DXT1_RGB"
    } else if format == PixelFormat::CompressedDxt1Rgba as i32 {
        "DXT1_RGBA"
    } else if format == PixelFormat::CompressedDxt3Rgba as i32 {
        "DXT3_RGBA"
    } else if format == PixelFormat::CompressedDxt5Rgba as i32 {
        "DXT5_RGBA"
    } else if format == PixelFormat::CompressedEtc1Rgb as i32 {
        "ETC1_RGB"
    } else if format == PixelFormat::CompressedEtc2Rgb as i32 {
        "ETC2_RGB"
    } else if format == PixelFormat::CompressedEtc2EacRgba as i32 {
        "ETC2_RGBA"
    } else if format == PixelFormat::CompressedPvrtRgb as i32 {
        "PVRT_RGB"
    } else if format == PixelFormat::CompressedPvrtRgba as i32 {
        "PVRT_RGBA"
    } else if format == PixelFormat::CompressedAstc4x4Rgba as i32 {
        "ASTC_4x4_RGBA"
    } else if format == PixelFormat::CompressedAstc8x8Rgba as i32 {
        "ASTC_8x8_RGBA"
    } else {
        "UNKNOWN"
    }
}

pub unsafe fn rlUnloadTexture(id: u32) {
    gl::DeleteTextures(1, &id);
}

pub unsafe fn rlEnableShader(id: u32) {
    gl::UseProgram(id);
    RLGL.State.currentShaderId = id;
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

pub unsafe fn rlSetTexture(id: u32) {
    if RLGL.State.activeTextureId[0] != id {
        rlCheckRenderBatchLimit(0);
        let batch = &mut *RLGL.currentBatch;
        let last_draw = &mut *batch.draws.add((batch.drawCounter - 1) as usize);
        if last_draw.vertexCount > 0 {
            batch
                .draws
                .add(batch.drawCounter as usize)
                .write(rlDrawCall {
                    mode: last_draw.mode,
                    vertexCount: 0,
                    vertexAlignment: 0,
                    textureId: id,
                });
            batch.drawCounter += 1;
        } else {
            last_draw.textureId = id;
        }
        RLGL.State.activeTextureId[0] = id;
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

pub unsafe fn rlTextureParameters(id: u32, param: i32, value: i32) {
    gl::BindTexture(gl::TEXTURE_2D, id);
    gl::TexParameteri(gl::TEXTURE_2D, param as u32, value);
    gl::BindTexture(gl::TEXTURE_2D, 0);
}

pub unsafe fn rlCubemapParameters(id: u32, param: i32, value: i32) {
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, id);
    gl::TexParameteri(gl::TEXTURE_CUBE_MAP, param as u32, value);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
}

pub unsafe fn rlDisableShader() {
    gl::UseProgram(0);
}

pub unsafe fn rlEnableFramebuffer(id: u32) {
    gl::BindFramebuffer(gl::FRAMEBUFFER, id);
}

pub unsafe fn rlDisableFramebuffer() {
    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
}

pub unsafe fn rlGetActiveFramebuffer() -> u32 {
    let mut fbo_id: i32 = 0;
    gl::GetIntegerv(gl::DRAW_FRAMEBUFFER_BINDING, &mut fbo_id);
    fbo_id as u32
}

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

pub unsafe fn rlBindFramebuffer(target: u32, framebuffer: u32) {
    gl::BindFramebuffer(target, framebuffer);
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
    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
}

pub unsafe fn rlDisableWireMode() {
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

pub unsafe fn rlEnablePointMode() {
    gl::PolygonMode(gl::FRONT_AND_BACK, gl::POINT);
}

pub unsafe fn rlDisablePointMode() {
    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
}

pub unsafe fn rlSetPointSize(size: f32) {
    gl::PointSize(size);
}

pub unsafe fn rlGetPointSize() -> f32 {
    let mut size: f32 = 0.0;
    gl::GetFloatv(gl::POINT_SIZE, &mut size);
    size
}

pub unsafe fn rlEnableSmoothLines() {
    gl::Enable(gl::LINE_SMOOTH);
}

pub unsafe fn rlDisableSmoothLines() {
    gl::Disable(gl::LINE_SMOOTH);
}

pub unsafe fn rlClearColor(r: u8, g: u8, b: u8, a: u8) {
    gl::ClearColor(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32 / 255.0,
    );
}

pub unsafe fn rlClearScreenBuffers() {
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
}

pub unsafe fn rlCheckErrors() {
    let err = gl::GetError();
    if err != gl::NO_ERROR {
        // Log error
    }
}

pub unsafe fn rlSetBlendMode(mode: i32) {
    rlDrawRenderBatch(RLGL.currentBatch);
    match mode {
        0 => {
            // Alpha
            gl::BlendEquation(gl::FUNC_ADD);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        1 => {
            // Additive
            gl::BlendEquation(gl::FUNC_ADD);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
        }
        2 => {
            // Multiplied
            gl::BlendEquation(gl::FUNC_ADD);
            gl::BlendFunc(gl::DST_COLOR, gl::ZERO);
        }
        3 => {
            // Add Colors
            gl::BlendEquation(gl::FUNC_ADD);
            gl::BlendFunc(gl::ONE, gl::ONE);
        }
        4 => {
            // Subtract Colors
            gl::BlendEquation(gl::FUNC_REVERSE_SUBTRACT);
            gl::BlendFunc(gl::ONE, gl::ONE);
        }
        5 => {
            // Alpha Premultiply
            gl::BlendEquation(gl::FUNC_ADD);
            gl::BlendFunc(gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
        }
        _ => {}
    }
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

pub unsafe fn rlLoadTextureDepth(width: i32, height: i32, useRenderBuffer: bool) -> u32 {
    let mut id = 0;
    if useRenderBuffer {
        gl::GenRenderbuffers(1, &mut id);
        gl::BindRenderbuffer(gl::RENDERBUFFER, id);
        gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT24, width, height);
        gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
    } else {
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::DEPTH_COMPONENT as i32,
            width,
            height,
            0,
            gl::DEPTH_COMPONENT,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
    id
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
    gl::UniformMatrix4fv(locIndex, 1, gl::FALSE as u8, mat.to_cols_array().as_ptr());
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
