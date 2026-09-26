//! Safe wrappers for the raw `rshapes` bindings.
//!
//! Every public function of `rshapes` is mirrored here with a `snake_case` name and a
//! signature that can be called without `unsafe`. Parameters that the raw binding spells
//! as a slice plus an explicit count collapse into a single slice, since the length is
//! available from `slice::len()`, and the out-parameter of
//! [`check_collision_lines`] becomes a `&mut Vector2`.
//!
//! The raw `rshapes` module is private, so this module is the only way to reach shape
//! drawing and collision functionality from outside the crate.

use crate::rshapes::*;
use crate::types::{Color, Rectangle, Texture2D, Vector2};

pub fn get_shapes_texture() -> Texture2D {
    unsafe { GetShapesTexture() }
}

pub fn get_shapes_texture_rectangle() -> Rectangle {
    unsafe { GetShapesTextureRectangle() }
}

pub fn draw_pixel(pos_x: i32, pos_y: i32, color: Color) {
    unsafe { DrawPixel(pos_x, pos_y, color) }
}

pub fn draw_pixel_v(position: Vector2, color: Color) {
    unsafe { DrawPixelV(position, color) }
}

pub fn draw_line(start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: Color) {
    unsafe { DrawLine(start_pos_x, start_pos_y, end_pos_x, end_pos_y, color) }
}

pub fn draw_line_ex(start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color) {
    unsafe { DrawLineEx(start_pos, end_pos, thick, color) }
}

pub fn draw_line_v(start_pos: Vector2, end_pos: Vector2, color: Color) {
    unsafe { DrawLineV(start_pos, end_pos, color) }
}

/// The point count is derived from `points.len()`.
pub fn draw_line_strip(points: &[Vector2], color: Color) {
    unsafe { DrawLineStrip(points, points.len() as i32, color) }
}

pub fn draw_line_bezier(start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color) {
    unsafe { DrawLineBezier(start_pos, end_pos, thick, color) }
}

pub fn draw_line_dashed(
    start_pos: Vector2,
    end_pos: Vector2,
    dash_size: i32,
    space_size: i32,
    color: Color,
) {
    unsafe { DrawLineDashed(start_pos, end_pos, dash_size, space_size, color) }
}

pub fn draw_triangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    unsafe { DrawTriangle(v1, v2, v3, color) }
}

pub fn draw_triangle_gradient(
    v1: Vector2,
    v2: Vector2,
    v3: Vector2,
    c1: Color,
    c2: Color,
    c3: Color,
) {
    unsafe { DrawTriangleGradient(v1, v2, v3, c1, c2, c3) }
}

pub fn draw_triangle_lines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    unsafe { DrawTriangleLines(v1, v2, v3, color) }
}

pub fn draw_triangle_lines_ex(v1: Vector2, v2: Vector2, v3: Vector2, thick: f32, color: Color) {
    unsafe { DrawTriangleLinesEx(v1, v2, v3, thick, color) }
}

/// The point count is derived from `points.len()`.
pub fn draw_triangle_fan(points: &[Vector2], color: Color) {
    unsafe { DrawTriangleFan(points, points.len() as i32, color) }
}

/// The point count is derived from `points.len()`.
pub fn draw_triangle_strip(points: &[Vector2], color: Color) {
    unsafe { DrawTriangleStrip(points, points.len() as i32, color) }
}

pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    unsafe { DrawRectangle(pos_x, pos_y, width, height, color) }
}

pub fn draw_rectangle_v(position: Vector2, size: Vector2, color: Color) {
    unsafe { DrawRectangleV(position, size, color) }
}

pub fn draw_rectangle_rec(rec: Rectangle, color: Color) {
    unsafe { DrawRectangleRec(rec, color) }
}

pub fn draw_rectangle_pro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color) {
    unsafe { DrawRectanglePro(rec, origin, rotation, color) }
}

pub fn draw_rectangle_gradient_v(
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    top: Color,
    bottom: Color,
) {
    unsafe { DrawRectangleGradientV(pos_x, pos_y, width, height, top, bottom) }
}

pub fn draw_rectangle_gradient_h(
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    left: Color,
    right: Color,
) {
    unsafe { DrawRectangleGradientH(pos_x, pos_y, width, height, left, right) }
}

pub fn draw_rectangle_gradient_ex(
    rec: Rectangle,
    col1: Color,
    col2: Color,
    col3: Color,
    col4: Color,
) {
    unsafe { DrawRectangleGradientEx(rec, col1, col2, col3, col4) }
}

pub fn draw_rectangle_lines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    unsafe { DrawRectangleLines(pos_x, pos_y, width, height, color) }
}

pub fn draw_rectangle_lines_ex(rec: Rectangle, thick: f32, color: Color) {
    unsafe { DrawRectangleLinesEx(rec, thick, color) }
}

pub fn draw_rectangle_rounded(rec: Rectangle, roundness: f32, segments: i32, color: Color) {
    unsafe { DrawRectangleRounded(rec, roundness, segments, color) }
}

pub fn draw_rectangle_rounded_lines(rec: Rectangle, roundness: f32, segments: i32, color: Color) {
    unsafe { DrawRectangleRoundedLines(rec, roundness, segments, color) }
}

pub fn draw_rectangle_rounded_lines_ex(
    rec: Rectangle,
    roundness: f32,
    segments: i32,
    thick: f32,
    color: Color,
) {
    unsafe { DrawRectangleRoundedLinesEx(rec, roundness, segments, thick, color) }
}

pub fn draw_poly(center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color) {
    unsafe { DrawPoly(center, sides, radius, rotation, color) }
}

pub fn draw_poly_lines(center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color) {
    unsafe { DrawPolyLines(center, sides, radius, rotation, color) }
}

pub fn draw_poly_lines_ex(
    center: Vector2,
    sides: i32,
    radius: f32,
    rotation: f32,
    thick: f32,
    color: Color,
) {
    unsafe { DrawPolyLinesEx(center, sides, radius, rotation, thick, color) }
}

pub fn draw_circle(center_x: i32, center_y: i32, radius: f32, color: Color) {
    unsafe { DrawCircle(center_x, center_y, radius, color) }
}

pub fn draw_circle_v(center: Vector2, radius: f32, color: Color) {
    unsafe { DrawCircleV(center, radius, color) }
}

pub fn draw_circle_gradient(center: Vector2, radius: f32, inner: Color, outer: Color) {
    unsafe { DrawCircleGradient(center, radius, inner, outer) }
}

pub fn draw_circle_sector(
    center: Vector2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: Color,
) {
    unsafe { DrawCircleSector(center, radius, start_angle, end_angle, segments, color) }
}

pub fn draw_circle_sector_lines(
    center: Vector2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: Color,
) {
    unsafe { DrawCircleSectorLines(center, radius, start_angle, end_angle, segments, color) }
}

pub fn draw_circle_sector_lines_ex(
    center: Vector2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    thick: f32,
    color: Color,
) {
    unsafe {
        DrawCircleSectorLinesEx(
            center,
            radius,
            start_angle,
            end_angle,
            segments,
            thick,
            color,
        )
    }
}

pub fn draw_circle_lines(center_x: i32, center_y: i32, radius: f32, color: Color) {
    unsafe { DrawCircleLines(center_x, center_y, radius, color) }
}

pub fn draw_circle_lines_v(center: Vector2, radius: f32, color: Color) {
    unsafe { DrawCircleLinesV(center, radius, color) }
}

pub fn draw_circle_lines_ex(center: Vector2, radius: f32, thick: f32, color: Color) {
    unsafe { DrawCircleLinesEx(center, radius, thick, color) }
}

pub fn draw_ellipse(center_x: i32, center_y: i32, radius_h: f32, radius_v: f32, color: Color) {
    unsafe { DrawEllipse(center_x, center_y, radius_h, radius_v, color) }
}

pub fn draw_ellipse_v(center: Vector2, radius_h: f32, radius_v: f32, color: Color) {
    unsafe { DrawEllipseV(center, radius_h, radius_v, color) }
}

pub fn draw_ellipse_lines(
    center_x: i32,
    center_y: i32,
    radius_h: f32,
    radius_v: f32,
    color: Color,
) {
    unsafe { DrawEllipseLines(center_x, center_y, radius_h, radius_v, color) }
}

pub fn draw_ellipse_lines_v(center: Vector2, radius_h: f32, radius_v: f32, color: Color) {
    unsafe { DrawEllipseLinesV(center, radius_h, radius_v, color) }
}

pub fn draw_ellipse_lines_ex(
    center: Vector2,
    radius_h: f32,
    radius_v: f32,
    thick: f32,
    color: Color,
) {
    unsafe { DrawEllipseLinesEx(center, radius_h, radius_v, thick, color) }
}

pub fn draw_ring(
    center: Vector2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: Color,
) {
    unsafe {
        DrawRing(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            segments,
            color,
        )
    }
}

pub fn draw_ring_lines(
    center: Vector2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: Color,
) {
    unsafe {
        DrawRingLines(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            segments,
            color,
        )
    }
}

#[allow(clippy::too_many_arguments)]
pub fn draw_ring_lines_ex(
    center: Vector2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    thick: f32,
    color: Color,
) {
    unsafe {
        DrawRingLinesEx(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            segments,
            thick,
            color,
        )
    }
}

/// The point count is derived from `points.len()`.
pub fn draw_spline_linear(points: &[Vector2], thick: f32, color: Color) {
    unsafe { DrawSplineLinear(points, points.len() as i32, thick, color) }
}

/// The point count is derived from `points.len()`.
pub fn draw_spline_basis(points: &[Vector2], thick: f32, color: Color) {
    unsafe { DrawSplineBasis(points, points.len() as i32, thick, color) }
}

/// The point count is derived from `points.len()`.
pub fn draw_spline_catmull_rom(points: &[Vector2], thick: f32, color: Color) {
    unsafe { DrawSplineCatmullRom(points, points.len() as i32, thick, color) }
}

/// The point count is derived from `points.len()`.
pub fn draw_spline_bezier_quadratic(points: &[Vector2], thick: f32, color: Color) {
    unsafe { DrawSplineBezierQuadratic(points, points.len() as i32, thick, color) }
}

/// The point count is derived from `points.len()`.
pub fn draw_spline_bezier_cubic(points: &[Vector2], thick: f32, color: Color) {
    unsafe { DrawSplineBezierCubic(points, points.len() as i32, thick, color) }
}

pub fn draw_spline_segment_linear(p1: Vector2, p2: Vector2, thick: f32, color: Color) {
    unsafe { DrawSplineSegmentLinear(p1, p2, thick, color) }
}

pub fn draw_spline_segment_basis(
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
    p4: Vector2,
    thick: f32,
    color: Color,
) {
    unsafe { DrawSplineSegmentBasis(p1, p2, p3, p4, thick, color) }
}

pub fn draw_spline_segment_catmull_rom(
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
    p4: Vector2,
    thick: f32,
    color: Color,
) {
    unsafe { DrawSplineSegmentCatmullRom(p1, p2, p3, p4, thick, color) }
}

pub fn draw_spline_segment_bezier_quadratic(
    p1: Vector2,
    c2: Vector2,
    p3: Vector2,
    thick: f32,
    color: Color,
) {
    unsafe { DrawSplineSegmentBezierQuadratic(p1, c2, p3, thick, color) }
}

pub fn draw_spline_segment_bezier_cubic(
    p1: Vector2,
    c2: Vector2,
    c3: Vector2,
    p4: Vector2,
    thick: f32,
    color: Color,
) {
    unsafe { DrawSplineSegmentBezierCubic(p1, c2, c3, p4, thick, color) }
}

pub fn get_spline_point_linear(start_pos: Vector2, end_pos: Vector2, t: f32) -> Vector2 {
    GetSplinePointLinear(start_pos, end_pos, t)
}

pub fn get_spline_point_basis(
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
    p4: Vector2,
    t: f32,
) -> Vector2 {
    GetSplinePointBasis(p1, p2, p3, p4, t)
}

pub fn get_spline_point_catmull_rom(
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
    p4: Vector2,
    t: f32,
) -> Vector2 {
    GetSplinePointCatmullRom(p1, p2, p3, p4, t)
}

pub fn get_spline_point_bezier_quadratic(
    start_pos: Vector2,
    control_pos: Vector2,
    end_pos: Vector2,
    t: f32,
) -> Vector2 {
    GetSplinePointBezierQuadratic(start_pos, control_pos, end_pos, t)
}

pub fn get_spline_point_bezier_cubic(
    start_pos: Vector2,
    start_control_pos: Vector2,
    end_control_pos: Vector2,
    end_pos: Vector2,
    t: f32,
) -> Vector2 {
    GetSplinePointBezierCubic(start_pos, start_control_pos, end_control_pos, end_pos, t)
}

pub fn check_collision_point_rec(point: Vector2, rec: Rectangle) -> bool {
    CheckCollisionPointRec(point, rec)
}

pub fn check_collision_point_circle(point: Vector2, center: Vector2, radius: f32) -> bool {
    CheckCollisionPointCircle(point, center, radius)
}

pub fn check_collision_point_triangle(
    point: Vector2,
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
) -> bool {
    CheckCollisionPointTriangle(point, p1, p2, p3)
}

/// The point count is derived from `points.len()`.
pub fn check_collision_point_poly(point: Vector2, points: &[Vector2]) -> bool {
    CheckCollisionPointPoly(point, points, points.len() as i32)
}

pub fn check_collision_recs(rec1: Rectangle, rec2: Rectangle) -> bool {
    CheckCollisionRecs(rec1, rec2)
}

pub fn check_collision_circles(
    center1: Vector2,
    radius1: f32,
    center2: Vector2,
    radius2: f32,
) -> bool {
    CheckCollisionCircles(center1, radius1, center2, radius2)
}

pub fn check_collision_circle_rec(center: Vector2, radius: f32, rec: Rectangle) -> bool {
    CheckCollisionCircleRec(center, radius, rec)
}

/// Writes the intersection point into `collision_point`.
pub fn check_collision_lines(
    start_pos1: Vector2,
    end_pos1: Vector2,
    start_pos2: Vector2,
    end_pos2: Vector2,
    collision_point: &mut Vector2,
) -> bool {
    unsafe { CheckCollisionLines(start_pos1, end_pos1, start_pos2, end_pos2, collision_point) }
}

pub fn check_collision_point_line(
    point: Vector2,
    p1: Vector2,
    p2: Vector2,
    threshold: i32,
) -> bool {
    CheckCollisionPointLine(point, p1, p2, threshold)
}

pub fn check_collision_circle_line(center: Vector2, radius: f32, p1: Vector2, p2: Vector2) -> bool {
    CheckCollisionCircleLine(center, radius, p1, p2)
}

pub fn get_collision_rec(rec1: Rectangle, rec2: Rectangle) -> Rectangle {
    GetCollisionRec(rec1, rec2)
}
