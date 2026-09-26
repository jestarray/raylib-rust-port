use std::marker::PhantomData;

use crate::{
    core::{
        begin_blend_mode, begin_drawing, begin_mode_2d, begin_scissor_mode, begin_shader_mode,
        begin_texture_mode, begin_vr_stereo_mode, clear_background, close_window,
        disable_event_waiting, enable_event_waiting, end_blend_mode, end_drawing, end_mode_2d,
        end_scissor_mode, end_shader_mode, end_texture_mode, end_vr_stereo_mode, init_window,
        is_window_ready, set_config_flags,
    },
    shapes::{
        draw_circle, draw_circle_gradient, draw_circle_lines, draw_circle_lines_v,
        draw_circle_sector, draw_circle_sector_lines, draw_circle_v, draw_ellipse,
        draw_ellipse_lines, draw_ellipse_lines_v, draw_ellipse_v, draw_line, draw_line_bezier,
        draw_line_dashed, draw_line_ex, draw_line_strip, draw_line_v, draw_pixel, draw_pixel_v,
        draw_poly, draw_poly_lines, draw_poly_lines_ex, draw_rectangle, draw_rectangle_gradient_ex,
        draw_rectangle_gradient_h, draw_rectangle_gradient_v, draw_rectangle_lines,
        draw_rectangle_lines_ex, draw_rectangle_pro, draw_rectangle_rec, draw_rectangle_rounded,
        draw_rectangle_rounded_lines, draw_rectangle_rounded_lines_ex, draw_rectangle_v, draw_ring,
        draw_ring_lines, draw_spline_basis, draw_spline_bezier_cubic, draw_spline_bezier_quadratic,
        draw_spline_catmull_rom, draw_spline_linear, draw_spline_segment_basis,
        draw_spline_segment_bezier_cubic, draw_spline_segment_bezier_quadratic,
        draw_spline_segment_catmull_rom, draw_spline_segment_linear, draw_triangle,
        draw_triangle_fan, draw_triangle_lines, draw_triangle_strip, get_shapes_texture,
        get_shapes_texture_rectangle, get_spline_point_basis, get_spline_point_bezier_cubic,
        get_spline_point_bezier_quadratic, get_spline_point_catmull_rom, get_spline_point_linear,
    },
    text::{draw_fps, draw_text, draw_text_codepoint, draw_text_ex, draw_text_pro},
    textures::{
        draw_texture, draw_texture_ex, draw_texture_n_patch, draw_texture_pro, draw_texture_rec,
        draw_texture_v,
    },
    types::{
        BlendMode, Camera2D, Color, ConfigFlags::*, Font, NPatchInfo, Rectangle, RenderTexture2D,
        Shader, Texture, Texture2D, TraceLogLevel, Vector2, VrStereoConfig,
    },
};

#[derive(Debug)]
pub struct RaylibHandle(PhantomData<std::rc::Rc<()>>);

impl Drop for RaylibHandle {
    fn drop(&mut self) {
        if is_window_ready() {
            close_window();
        }
    }
}

/// Fluent builder for configuring the raylib window before it is created.
///
/// Obtain a `RaylibBuilder` via [`init`], chain the desired options, then call [`build`] to open
/// the window and receive a [`RaylibHandle`].
///
/// [`build`]: RaylibBuilder::build
///
/// # Examples
///
/// ```no_run
/// use raylib::handle::{self, RaylibDraw};
/// use raylib::types::Color;
///
/// let mut builder = handle::init();
/// let mut rl = builder.size(1280, 720).title("My Game").vsync().msaa_4x().build();
///
/// // rl is now ready; enter the game loop.
/// while !raylib::sdl::window_should_close() {
///     let mut d = rl.begin_drawing();
///     d.clear_background(Color::RAYWHITE);
/// }
/// ```
#[derive(Debug, Default)]
pub struct RaylibBuilder<'a> {
    fullscreen_mode: bool,
    window_resizable: bool,
    window_undecorated: bool,
    window_transparent: bool,
    msaa_4x_hint: bool,
    vsync_hint: bool,
    window_hidden: bool,
    window_always_run: bool,
    window_minimized: bool,
    window_maximized: bool,
    window_unfocused: bool,
    window_topmost: bool,
    window_highdpi: bool,
    window_mouse_passthrough: bool,
    borderless_windowed_mode: bool,
    interlaced_hint: bool,
    log_level: TraceLogLevel,
    width: i32,
    height: i32,
    title: &'a str,
}
#[inline]
#[must_use]
/// Creates a `RaylibBuilder` for choosing window options before initialization.
pub fn init<'a>() -> RaylibBuilder<'a> {
    RaylibBuilder {
        width: 640,
        height: 480,
        title: "raylib-rs",
        ..Default::default()
    }
}

impl<'a> RaylibBuilder<'a> {
    /// Sets the window to be fullscreen.
    pub const fn fullscreen(&mut self) -> &mut Self {
        self.fullscreen_mode = true;
        self
    }

    /// Sets the window to be resizable.
    pub const fn resizable(&mut self) -> &mut Self {
        self.window_resizable = true;
        self
    }

    /// Sets the window to be undecorated (without a border).
    pub const fn undecorated(&mut self) -> &mut Self {
        self.window_undecorated = true;
        self
    }

    /// Sets the window to be transparent.
    pub const fn transparent(&mut self) -> &mut Self {
        self.window_transparent = true;
        self
    }

    /// Hints that 4x MSAA (anti-aliasing) should be enabled. The system's graphics drivers may override this setting.
    pub const fn msaa_4x(&mut self) -> &mut Self {
        self.msaa_4x_hint = true;
        self
    }

    /// Hints that vertical sync (VSync) should be enabled. The system's graphics drivers may override this setting.
    pub const fn vsync(&mut self) -> &mut Self {
        self.vsync_hint = true;
        self
    }

    /// Set to hide window
    pub const fn hidden(&mut self) -> &mut Self {
        self.window_hidden = true;
        self
    }

    /// Set to allow windows running while minimized
    pub const fn always_run(&mut self) -> &mut Self {
        self.window_always_run = true;
        self
    }

    /// Set to minimize window (iconify)
    pub const fn minimized(&mut self) -> &mut Self {
        self.window_minimized = true;
        self
    }

    /// Set to maximize window (expanded to monitor)
    pub const fn maximized(&mut self) -> &mut Self {
        self.window_maximized = true;
        self
    }

    /// Set to window non focused
    pub const fn unfocused(&mut self) -> &mut Self {
        self.window_unfocused = true;
        self
    }

    /// Set to window always on top
    pub const fn topmost(&mut self) -> &mut Self {
        self.window_topmost = true;
        self
    }

    /// Set to support HighDPI
    pub const fn highdpi(&mut self) -> &mut Self {
        self.window_highdpi = true;
        self
    }

    /// Set to support mouse passthrough, only supported when [`Self::undecorated`]
    pub const fn mouse_passthrough(&mut self) -> &mut Self {
        self.window_mouse_passthrough = true;
        self
    }

    /// Set to run program in borderless windowed mode
    pub const fn borderless_windowed_mode(&mut self) -> &mut Self {
        self.borderless_windowed_mode = true;
        self
    }

    /// Set to try enabling interlaced video format (for V3D)
    pub const fn interlaced_hint(&mut self) -> &mut Self {
        self.interlaced_hint = true;
        self
    }

    /// Sets the window's width.
    pub const fn width(&mut self, w: i32) -> &mut Self {
        self.width = w;
        self
    }

    /// Sets the window's height.
    pub const fn height(&mut self, h: i32) -> &mut Self {
        self.height = h;
        self
    }

    /// Sets the window's width and height.
    pub const fn size(&mut self, w: i32, h: i32) -> &mut Self {
        self.width = w;
        self.height = h;
        self
    }

    /// Sets the window title.
    pub const fn title(&mut self, text: &'a str) -> &mut Self {
        self.title = text;
        self
    }

    /// Builds and initializes a Raylib window.
    ///
    /// # Panics
    ///
    /// Attempting to initialize Raylib more than once will result in a panic.
    pub fn build(&self) -> RaylibHandle {
        let mut flags = 0u32;
        if self.fullscreen_mode {
            flags |= FLAG_FULLSCREEN_MODE as u32;
        }
        if self.window_resizable {
            flags |= FLAG_WINDOW_RESIZABLE as u32;
        }
        if self.window_undecorated {
            flags |= FLAG_WINDOW_UNDECORATED as u32;
        }
        if self.window_hidden {
            flags |= FLAG_WINDOW_HIDDEN as u32;
        }
        if self.window_minimized {
            flags |= FLAG_WINDOW_MINIMIZED as u32;
        }
        if self.window_maximized {
            flags |= FLAG_WINDOW_MAXIMIZED as u32;
        }
        if self.window_unfocused {
            flags |= FLAG_WINDOW_UNFOCUSED as u32;
        }
        if self.window_topmost {
            flags |= FLAG_WINDOW_TOPMOST as u32;
        }
        if self.window_always_run {
            flags |= FLAG_WINDOW_ALWAYS_RUN as u32;
        }
        if self.window_transparent {
            flags |= FLAG_WINDOW_TRANSPARENT as u32;
        }
        if self.window_highdpi {
            flags |= FLAG_WINDOW_HIGHDPI as u32;
        }
        if self.window_mouse_passthrough {
            flags |= FLAG_WINDOW_MOUSE_PASSTHROUGH as u32;
        }
        if self.borderless_windowed_mode {
            flags |= FLAG_BORDERLESS_WINDOWED_MODE as u32;
        }
        if self.msaa_4x_hint {
            flags |= FLAG_MSAA_4X_HINT as u32;
        }
        if self.vsync_hint {
            flags |= FLAG_VSYNC_HINT as u32;
        }
        if self.interlaced_hint {
            flags |= FLAG_INTERLACED_HINT as u32;
        }

        set_config_flags(flags);

        self.log_level.set_trace_log_level();

        let rl = init_raylib(self.width, self.height, self.title);

        return rl;
    }
}

/// Initializes window and OpenGL context.
///
/// # Panics
///
/// Attempting to initialize Raylib more than once will result in a panic.
fn init_raylib(width: i32, height: i32, title: &str) -> RaylibHandle {
    if is_window_ready() {
        panic!("Attempted to initialize raylib-rs more than once!");
    } else {
        init_window(width, height, title);
        if !is_window_ready() {
            panic!("Attempting to create window failed!");
        }
        RaylibHandle(PhantomData)
    }
}

impl RaylibHandle {
    #[inline]
    #[must_use]
    /// Setup canvas (framebuffer) to start drawing.
    pub fn begin_drawing<'a>(&'a mut self) -> RaylibDrawHandle<'a> {
        begin_drawing();

        RaylibDrawHandle(self)
    }
}

/// - [`RaylibDraw`] — drawing methods available on the guard.
/// - [`RaylibTextureMode`], [`RaylibMode2D`], [`RaylibMode3D`] — nested mode guards.
pub struct RaylibDrawHandle<'a>(&'a mut RaylibHandle);

impl Drop for RaylibDrawHandle<'_> {
    fn drop(&mut self) {
        end_drawing();
    }
}

impl std::ops::Deref for RaylibDrawHandle<'_> {
    type Target = RaylibHandle;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl std::ops::DerefMut for RaylibDrawHandle<'_> {
    fn deref_mut(&mut self) -> &mut RaylibHandle {
        self.0
    }
}
impl RaylibDraw for RaylibDrawHandle<'_> {}

#[rustfmt::skip]
pub trait RaylibDraw {
    #[inline]
    fn clear_background(&mut self, color: Color) { clear_background(color); }

    #[inline]
    #[must_use]
    fn get_shapes_texture(&self) -> Texture2D { get_shapes_texture() }

    #[inline]
    #[must_use]
    fn get_shapes_texture_rectangle(&self) -> Rectangle { get_shapes_texture_rectangle() }

    #[inline]
    fn set_shapes_texture(&mut self, texture: Texture2D, rec: Rectangle) { texture.set_shapes_texture(rec); }

    // SHAPES
    #[inline]
    fn draw_pixel(&mut self, pos_x: i32, pos_y: i32, color: Color) { draw_pixel(pos_x, pos_y, color); }

    #[inline]
    fn draw_pixel_v(&mut self, position: Vector2, color: Color) { draw_pixel_v(position, color); }

    #[inline]
    fn draw_line(&mut self, start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: Color) { draw_line(start_pos_x, start_pos_y, end_pos_x, end_pos_y, color); }

    #[inline]
    fn draw_line_v(&mut self, start_pos: Vector2, end_pos: Vector2, color: Color) { draw_line_v(start_pos, end_pos, color); }

    #[inline]
    fn draw_line_ex(&mut self, start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color) { draw_line_ex(start_pos, end_pos, thick, color); }

    #[inline]
    fn draw_line_bezier(&mut self, start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color) { draw_line_bezier(start_pos, end_pos, thick, color); }

    /// `dash_size` is the length of each painted segment; `space_size` is the
    /// gap between segments. Both are in pixels.
    #[inline]
    fn draw_line_dashed(&mut self, start_pos: Vector2, end_pos: Vector2, dash_size: i32, space_size: i32, color: Color) { draw_line_dashed(start_pos, end_pos, dash_size, space_size, color); }

    #[inline]
    fn draw_line_strip(&mut self, points: &[Vector2], color: Color) { draw_line_strip(points, color); }

    #[inline]
    fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: f32, color: Color) { draw_circle(center_x, center_y, radius, color); }
    #[inline]
    fn draw_circle_sector(&mut self, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: Color) { draw_circle_sector(center, radius, start_angle, end_angle, segments, color); }

    #[inline]
    fn draw_circle_sector_lines(&mut self, center: Vector2, radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: Color) { draw_circle_sector_lines(center, radius, start_angle, end_angle, segments, color); }

    #[inline]
    fn draw_circle_gradient(&mut self, center: Vector2, radius: f32, inner: Color, outer: Color) { draw_circle_gradient(center, radius, inner, outer); }

    #[inline]
    fn draw_circle_v(&mut self, center: Vector2, radius: f32, color: Color) { draw_circle_v(center, radius, color); }

    #[inline]
    fn draw_circle_lines(&mut self, center_x: i32, center_y: i32, radius: f32, color: Color) { draw_circle_lines(center_x, center_y, radius, color); }

    #[inline]
    fn draw_circle_lines_v(&mut self, center: Vector2, radius: f32, color: Color) { draw_circle_lines_v(center, radius, color); }

    #[inline]
    fn draw_ellipse(&mut self, center_x: i32, center_y: i32, radius_h: f32, radius_v: f32, color: Color) { draw_ellipse(center_x, center_y, radius_h, radius_v, color); }

    #[inline]
    fn draw_ellipse_v(&mut self, center: Vector2, radius_h: f32, radius_v: f32, color: Color) { draw_ellipse_v(center, radius_h, radius_v, color); }

    #[inline]
    fn draw_ellipse_lines(&mut self, center_x: i32, center_y: i32, radius_h: f32, radius_v: f32, color: Color) { draw_ellipse_lines(center_x, center_y, radius_h, radius_v, color); }

    #[inline]
    fn draw_ellipse_lines_v(&mut self, center: Vector2, radius_h: f32, radius_v: f32, color: Color) { draw_ellipse_lines_v(center, radius_h, radius_v, color); }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn draw_ring(&mut self, center: Vector2, inner_radius: f32, outer_radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: Color) { draw_ring(center, inner_radius, outer_radius, start_angle, end_angle, segments, color); }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn draw_ring_lines(&mut self, center: Vector2, inner_radius: f32, outer_radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: Color) { draw_ring_lines(center, inner_radius, outer_radius, start_angle, end_angle, segments, color); }

    #[inline]
    fn draw_rectangle(&mut self, pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) { draw_rectangle(pos_x, pos_y, width, height, color); }

    #[inline]
    fn draw_rectangle_v(&mut self, position: Vector2, size: Vector2, color: Color) { draw_rectangle_v(position, size, color); }

    #[inline]
    fn draw_rectangle_rec(&mut self, rec: Rectangle, color: Color) { draw_rectangle_rec(rec, color); }

    #[inline]
    fn draw_rectangle_pro(&mut self, rec: Rectangle, origin: Vector2, rotation: f32, color: Color) { draw_rectangle_pro(rec, origin, rotation, color); }

    /// **NOTE**: Gradient goes from `bottom` to `top`.
    #[inline]
    fn draw_rectangle_gradient_v(&mut self, pos_x: i32, pos_y: i32, width: i32, height: i32, top: Color, bottom: Color) { draw_rectangle_gradient_v(pos_x, pos_y, width, height, top, bottom); }

    /// **NOTE**: Gradient goes from `left` to `right`.
    #[inline]
    fn draw_rectangle_gradient_h(&mut self, pos_x: i32, pos_y: i32, width: i32, height: i32, left: Color, right: Color) { draw_rectangle_gradient_h(pos_x, pos_y, width, height, left, right); }

    /// **NOTE**: Colors refer to corners, starting at top-left corner and going counter-clockwise.
    #[inline]
    fn draw_rectangle_gradient_ex(&mut self, rec: Rectangle, col1: Color, col2: Color, col3: Color, col4: Color) { draw_rectangle_gradient_ex(rec, col1, col2, col3, col4); }

    #[inline]
    fn draw_rectangle_lines(&mut self, pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) { draw_rectangle_lines(pos_x, pos_y, width, height, color); }

    #[inline]
    fn draw_rectangle_lines_ex(&mut self, rec: Rectangle, thick: f32, color: Color) { draw_rectangle_lines_ex(rec, thick, color); }
    #[inline]
    fn draw_rectangle_rounded(&mut self, rec: Rectangle, roundness: f32, segments: i32, color: Color) { draw_rectangle_rounded(rec, roundness, segments, color); }

    #[inline]
    fn draw_rectangle_rounded_lines(&mut self, rec: Rectangle, roundness: f32, segments: i32, color: Color) { draw_rectangle_rounded_lines(rec, roundness, segments, color); }

    #[inline]
    fn draw_rectangle_rounded_lines_ex(&mut self, rec: Rectangle, roundness: f32, segments: i32, thick: f32, color: Color) { draw_rectangle_rounded_lines_ex(rec, roundness, segments, thick, color); }
    #[inline]
    fn draw_triangle(&mut self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) { draw_triangle(v1, v2, v3, color); }

    #[inline]
    fn draw_triangle_lines(&mut self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) { draw_triangle_lines(v1, v2, v3, color); }

    #[inline]
    fn draw_triangle_fan(&mut self, points: &[Vector2], color: Color) { draw_triangle_fan(points, color); }

    #[inline]
    fn draw_triangle_strip(&mut self, points: &[Vector2], color: Color) { draw_triangle_strip(points, color); }

    #[inline]
    fn draw_poly(&mut self, center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color) { draw_poly(center, sides, radius, rotation, color); }

    #[inline]
    fn draw_poly_lines(&mut self, center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color) { draw_poly_lines(center, sides, radius, rotation, color); }

    #[inline]
    fn draw_texture(&mut self, texture: &Texture, pos_x: i32, pos_y: i32, tint: Color) { draw_texture(texture, pos_x, pos_y, tint); }

    #[inline]
    fn draw_texture_v(&mut self, texture: &Texture, pos: Vector2, tint: Color) { draw_texture_v(texture, pos, tint); }

    #[inline]
    fn draw_texture_ex(&mut self, texture: &Texture, position: Vector2, rotation: f32, scale: f32, tint: Color) { draw_texture_ex(texture, position, rotation, scale, tint); }

    #[inline]
    fn draw_texture_rec(&mut self, texture: &Texture, source: impl Into<Rectangle>, position: Vector2, tint: Color) { draw_texture_rec(texture, source.into(), position, tint); }

    #[inline]
    fn draw_texture_pro(&mut self, texture: &Texture, source: impl Into<Rectangle>, dest: impl Into<Rectangle>, origin: Vector2, rotation: f32, tint: Color) { draw_texture_pro(texture, source.into(), dest.into(), origin, rotation, tint); }

    #[inline]
    fn draw_texture_n_patch(&mut self, texture: &Texture, n_patch_info: NPatchInfo, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color) { draw_texture_n_patch(texture, n_patch_info, dest, origin, rotation, tint); }

    #[inline]
    fn draw_fps(&mut self, pos_x: i32, pos_y: i32) { draw_fps(pos_x, pos_y); }

    #[inline]
    fn draw_text(&mut self, text: &str, x: i32, y: i32, font_size: i32, color: Color) { draw_text(text, x, y, font_size, color); }
    #[inline]
    fn draw_text_ex(&mut self, font: &Font, text: &str, position: Vector2, font_size: f32, spacing: f32, tint: Color) { draw_text_ex(font, text, position, font_size, spacing, tint); }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn draw_text_pro(&mut self, font: &Font, text: &str, position: Vector2, origin: Vector2, rotation: f32, font_size: f32, spacing: f32, tint: Color) { draw_text_pro(font, text, position, origin, rotation, font_size, spacing, tint); }

    #[inline]
    fn draw_text_codepoint(&mut self, font: &Font, codepoint: i32, position: Vector2, font_size: f32, tint: Color) { draw_text_codepoint(font, codepoint, position, font_size, tint); }

    /// Enable waiting for events when the handle is dropped, no automatic event polling
    #[inline]
    fn enable_event_waiting(&self) { enable_event_waiting(); }

    /// Disable waiting for events when the handle is dropped, no automatic event polling
    #[inline]
    fn disable_event_waiting(&self) { disable_event_waiting(); }

    #[inline]
    fn draw_poly_lines_ex(&mut self, center: Vector2, sides: i32, radius: f32, rotation: f32, thick: f32, color: Color) { draw_poly_lines_ex(center, sides, radius, rotation, thick, color); }
    /// Draw spline: Linear, minimum 2 points
    #[inline]
    fn draw_spline_linear(&mut self, points: &[Vector2], thick: f32, color: Color) { draw_spline_linear(points, thick, color); }
    /// Draw spline: B-Spline, minimum 4 points
    #[inline]
    fn draw_spline_basis(&mut self, points: &[Vector2], thick: f32, color: Color) { draw_spline_basis(points, thick, color); }
    /// Draw spline: Catmull-Rom, minimum 4 points
    #[inline]
    fn draw_spline_catmull_rom(&mut self, points: &[Vector2], thick: f32, color: Color) { draw_spline_catmull_rom(points, thick, color); }

    /// Draw spline: Quadratic Bezier, minimum 3 points (1 control point): [p1, c2, p3, c4...]
    #[inline]
    fn draw_spline_bezier_quadratic(&mut self, points: &[Vector2], thick: f32, color: Color) { draw_spline_bezier_quadratic(points, thick, color); }

    /// Draw spline: Cubic Bezier, minimum 4 points (2 control points): [p1, c2, c3, p4, c5, c6...]
    #[inline]
    fn draw_spline_bezier_cubic(&mut self, points: &[Vector2], thick: f32, color: Color) { draw_spline_bezier_cubic(points, thick, color); }

    /// Draw spline segment: Linear, 2 points
    #[inline]
    fn draw_spline_segment_linear(&mut self, p1: Vector2, p2: Vector2, thick: f32, color: Color) { draw_spline_segment_linear(p1, p2, thick, color); }

    /// Draw spline segment: B-Spline, 4 points
    #[inline]
    fn draw_spline_segment_basis(&mut self, p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thick: f32, color: Color) { draw_spline_segment_basis(p1, p2, p3, p4, thick, color); }

    /// Draw spline segment: Catmull-Rom, 4 points
    #[inline]
    fn draw_spline_segment_catmull_rom(&mut self, p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thick: f32, color: Color) { draw_spline_segment_catmull_rom(p1, p2, p3, p4, thick, color); }

    /// Draw spline segment: Quadratic Bezier, 2 points, 1 control point
    #[inline]
    fn draw_spline_segment_bezier_quadratic(&mut self, p1: Vector2, c2: Vector2, p3: Vector2, thick: f32, color: Color) { draw_spline_segment_bezier_quadratic(p1, c2, p3, thick, color); }

    /// Draw spline segment: Cubic Bezier, 2 points, 2 control points
    #[inline]
    fn draw_spline_segment_bezier_cubic(&mut self, p1: Vector2, c2: Vector2, c3: Vector2, p4: Vector2, thick: f32, color: Color) { draw_spline_segment_bezier_cubic(p1, c2, c3, p4, thick, color); }

    #[inline]
    #[must_use]
    fn get_spline_point_linear(&mut self, start_pos: Vector2, end_pos: Vector2, t: f32) -> Vector2 { get_spline_point_linear(start_pos, end_pos, t) }

    #[inline]
    #[must_use]
    fn get_spline_point_basis(&mut self, p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, t: f32) -> Vector2 { get_spline_point_basis(p1, p2, p3, p4, t) }

    #[inline]
    #[must_use]
    fn get_spline_point_catmull_rom(&mut self, p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, t: f32) -> Vector2 { get_spline_point_catmull_rom(p1, p2, p3, p4, t) }

    #[inline]
    #[must_use]
    fn get_spline_point_bezier_quad(&mut self, start_pos: Vector2, control_pos: Vector2, end_pos: Vector2, t: f32) -> Vector2 { get_spline_point_bezier_quadratic(start_pos, control_pos, end_pos, t) }

    #[inline]
    #[must_use]
    fn get_spline_point_bezier_cubic(&mut self, start_pos: Vector2, start_control_pos: Vector2, end_control_pos: Vector2, end_pos: Vector2, t: f32) -> Vector2 { get_spline_point_bezier_cubic(start_pos, start_control_pos, end_control_pos, end_pos, t) }
}

pub struct RaylibTextureMode<'a, 'b, T: 'a>(&'a mut T, PhantomData<&'b mut RenderTexture2D>);

impl<'a, T: 'a> Drop for RaylibTextureMode<'a, '_, T> {
    fn drop(&mut self) {
        end_texture_mode();
    }
}
impl<'a, T: 'a> std::ops::Deref for RaylibTextureMode<'a, '_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a, T: 'a> std::ops::DerefMut for RaylibTextureMode<'a, '_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0
    }
}

pub trait RaylibTextureModeExt
where
    Self: Sized,
{
    /// Begin drawing to render texture.
    #[inline]
    #[must_use]
    fn begin_texture_mode<'a, 'b>(
        &'a mut self,
        framebuffer: &'b mut RenderTexture2D,
    ) -> RaylibTextureMode<'a, 'b, Self> {
        begin_texture_mode(framebuffer.clone());
        RaylibTextureMode(self, PhantomData)
    }
}

impl RaylibTextureModeExt for RaylibDrawHandle<'_> {}
impl RaylibTextureModeExt for RaylibHandle {}
impl<'a, T: 'a> RaylibDraw for RaylibTextureMode<'a, '_, T> {}

pub struct RaylibMode2D<'a, T: 'a>(&'a mut T);
impl<'a, T: 'a> Drop for RaylibMode2D<'a, T> {
    fn drop(&mut self) {
        end_mode_2d()
    }
}
impl<'a, T: 'a> std::ops::Deref for RaylibMode2D<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a, T: 'a> std::ops::DerefMut for RaylibMode2D<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0
    }
}

pub trait RaylibMode2DExt
where
    Self: Sized,
{
    #[allow(non_snake_case)]
    #[inline]
    #[must_use]
    fn begin_mode2D(&mut self, camera: Camera2D) -> RaylibMode2D<'_, Self> {
        begin_mode_2d(camera);
        RaylibMode2D(self)
    }
}
impl<D: RaylibDraw> RaylibMode2DExt for D {}
impl<'a, T: 'a> RaylibDraw for RaylibMode2D<'a, T> {}

pub struct RaylibShaderMode<'a, 'b, T: 'a>(&'a mut T, PhantomData<&'b mut Shader>);

impl<'a, T: 'a> Drop for RaylibShaderMode<'a, '_, T> {
    fn drop(&mut self) {
        end_shader_mode();
    }
}
impl<'a, T: 'a> std::ops::Deref for RaylibShaderMode<'a, '_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a, T: 'a> std::ops::DerefMut for RaylibShaderMode<'a, '_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0
    }
}
pub trait RaylibShaderModeExt
where
    Self: Sized,
{
    /// Begin custom shader drawing.
    #[must_use]
    #[inline]
    fn begin_shader_mode<'a, 'b>(
        &'a mut self,
        shader: &'b mut Shader,
    ) -> RaylibShaderMode<'a, 'b, Self> {
        begin_shader_mode(shader);
        RaylibShaderMode(self, PhantomData)
    }
}

impl<D: RaylibDraw> RaylibShaderModeExt for D {}
impl<'a, T: 'a> RaylibDraw for RaylibShaderMode<'a, '_, T> {}

pub struct RaylibBlendMode<'a, T: 'a>(&'a mut T);
impl<'a, T: 'a> Drop for RaylibBlendMode<'a, T> {
    fn drop(&mut self) {
        end_blend_mode()
    }
}
impl<'a, T: 'a> std::ops::Deref for RaylibBlendMode<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a, T: 'a> std::ops::DerefMut for RaylibBlendMode<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0
    }
}
pub trait RaylibBlendModeExt
where
    Self: Sized,
{
    #[inline]
    #[must_use]
    fn begin_blend_mode(&mut self, blend_mode: BlendMode) -> RaylibBlendMode<'_, Self> {
        begin_blend_mode(blend_mode);
        RaylibBlendMode(self)
    }
}

impl<D: RaylibDraw> RaylibBlendModeExt for D {}
impl<'a, T: 'a> RaylibDraw for RaylibBlendMode<'a, T> {}

pub struct RaylibScissorMode<'a, T: 'a>(&'a mut T);
impl<'a, T: 'a> Drop for RaylibScissorMode<'a, T> {
    fn drop(&mut self) {
        end_scissor_mode();
    }
}
impl<'a, T: 'a> std::ops::Deref for RaylibScissorMode<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a, T: 'a> std::ops::DerefMut for RaylibScissorMode<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0
    }
}

pub trait RaylibScissorModeExt
where
    Self: Sized,
{
    #[must_use]
    #[inline]
    fn begin_scissor_mode(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> RaylibScissorMode<'_, Self> {
        begin_scissor_mode(x, y, width, height);
        RaylibScissorMode(self)
    }
}

impl<D: RaylibDraw> RaylibScissorModeExt for D {}
impl<'a, T: 'a> RaylibDraw for RaylibScissorMode<'a, T> {}

pub struct RaylibVRMode<'a, 'b, T: 'a>(
    &'a T,
    PhantomData<&'a mut T>,
    PhantomData<&'b mut VrStereoConfig>,
);
impl<'a, T: 'a> Drop for RaylibVRMode<'a, '_, T> {
    fn drop(&mut self) {
        end_vr_stereo_mode()
    }
}
impl<'a, T: 'a> std::ops::Deref for RaylibVRMode<'a, '_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

pub trait RaylibVRModeExt
where
    Self: Sized,
{
    /// Begin stereo rendering (requires VR simulator).
    #[inline]
    #[must_use]
    fn begin_vr_stereo_mode<'a, 'b>(
        &'a mut self,
        vr_config: &'b mut VrStereoConfig,
    ) -> RaylibVRMode<'a, 'b, Self> {
        begin_vr_stereo_mode((*vr_config).clone());
        RaylibVRMode(self, PhantomData, PhantomData)
    }
}

impl<D: RaylibDraw> RaylibVRModeExt for D {}
impl<'a, T: 'a> RaylibDraw for RaylibVRMode<'a, '_, T> {}
