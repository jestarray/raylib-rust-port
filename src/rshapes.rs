use crate::types::{Color, Rectangle, Vector2};
use crate::rlgl::{self, RL_LINES, RL_QUADS, RL_TRIANGLES};

pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    draw_rectangle_v(Vector2::new(pos_x as f32, pos_y as f32), Vector2::new(width as f32, height as f32), color);
}

pub fn draw_rectangle_v(position: Vector2, size: Vector2, color: Color) {
    draw_rectangle_pro(
        Rectangle::new(position.x, position.y, size.x, size.y),
        Vector2::new(0.0, 0.0),
        0.0,
        color,
    );
}

pub fn draw_rectangle_pro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color) {
    let mut top_left = Vector2::new(0.0, 0.0);
    let mut top_right = Vector2::new(0.0, 0.0);
    let mut bottom_left = Vector2::new(0.0, 0.0);
    let mut bottom_right = Vector2::new(0.0, 0.0);

    if rotation == 0.0 {
        let x = rec.x - origin.x;
        let y = rec.y - origin.y;
        top_left = Vector2::new(x, y);
        top_right = Vector2::new(x + rec.width, y);
        bottom_left = Vector2::new(x, y + rec.height);
        bottom_right = Vector2::new(x + rec.width, y + rec.height);
    } else {
        let sin = (rotation * crate::math::DEG2RAD).sin();
        let cos = (rotation * crate::math::DEG2RAD).cos();

        let dx = -origin.x;
        let dy = -origin.y;

        top_left.x = rec.x + dx * cos - dy * sin;
        top_left.y = rec.y + dx * sin + dy * cos;

        top_right.x = rec.x + (dx + rec.width) * cos - dy * sin;
        top_right.y = rec.y + (dx + rec.width) * sin + dy * cos;

        bottom_left.x = rec.x + dx * cos - (dy + rec.height) * sin;
        bottom_left.y = rec.y + dx * sin + (dy + rec.height) * cos;

        bottom_right.x = rec.x + (dx + rec.width) * cos - (dy + rec.height) * sin;
        bottom_right.y = rec.y + (dx + rec.width) * sin + (dy + rec.height) * cos;
    }

    rlgl::rl_set_texture(rlgl::rl_get_texture_id_default());
    rlgl::rl_begin(RL_QUADS);
    rlgl::rl_color4ub(color.r, color.g, color.b, color.a);
    rlgl::rl_vertex2f(top_left.x, top_left.y);
    rlgl::rl_vertex2f(bottom_left.x, bottom_left.y);
    rlgl::rl_vertex2f(bottom_right.x, bottom_right.y);
    rlgl::rl_vertex2f(top_right.x, top_right.y);
    rlgl::rl_end();
}

pub fn draw_rectangle_gradient_v(x: i32, y: i32, width: i32, height: i32, color1: Color, color2: Color) {
    rlgl::rl_set_texture(rlgl::rl_get_texture_id_default());
    rlgl::rl_begin(RL_QUADS);
    rlgl::rl_color4ub(color1.r, color1.g, color1.b, color1.a);
    rlgl::rl_vertex2f(x as f32, y as f32);
    rlgl::rl_color4ub(color2.r, color2.g, color2.b, color2.a);
    rlgl::rl_vertex2f(x as f32, (y + height) as f32);
    rlgl::rl_vertex2f((x + width) as f32, (y + height) as f32);
    rlgl::rl_color4ub(color1.r, color1.g, color1.b, color1.a);
    rlgl::rl_vertex2f((x + width) as f32, y as f32);
    rlgl::rl_end();
}

pub fn draw_rectangle_gradient_h(x: i32, y: i32, width: i32, height: i32, color1: Color, color2: Color) {
    rlgl::rl_set_texture(rlgl::rl_get_texture_id_default());
    rlgl::rl_begin(RL_QUADS);
    rlgl::rl_color4ub(color1.r, color1.g, color1.b, color1.a);
    rlgl::rl_vertex2f(x as f32, y as f32);
    rlgl::rl_vertex2f(x as f32, (y + height) as f32);
    rlgl::rl_color4ub(color2.r, color2.g, color2.b, color2.a);
    rlgl::rl_vertex2f((x + width) as f32, (y + height) as f32);
    rlgl::rl_vertex2f((x + width) as f32, y as f32);
    rlgl::rl_end();
}

pub fn draw_line(start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: Color) {
    rlgl::rl_set_texture(rlgl::rl_get_texture_id_default());
    rlgl::rl_begin(RL_LINES);
    rlgl::rl_color4ub(color.r, color.g, color.b, color.a);
    rlgl::rl_vertex2f(start_pos_x as f32, start_pos_y as f32);
    rlgl::rl_vertex2f(end_pos_x as f32, end_pos_y as f32);
    rlgl::rl_end();
}

pub fn draw_circle(center_x: i32, center_y: i32, radius: f32, color: Color) {
    draw_circle_v(Vector2::new(center_x as f32, center_y as f32), radius, color);
}

pub fn draw_circle_v(center: Vector2, radius: f32, color: Color) {
    let segments = 36; // A reasonable default
    draw_circle_sector(center, radius, 0.0, 360.0, segments, color);
}

pub fn draw_circle_sector(center: Vector2, radius: f32, start_angle: f32, end_angle: f32, segments: i32, color: Color) {
    if radius <= 0.0 {
        return;
    }

    let step_length = (end_angle - start_angle) / (segments as f32);
    let mut angle = start_angle;

    rlgl::rl_begin(RL_TRIANGLES);
    rlgl::rl_color4ub(color.r, color.g, color.b, color.a);

    for _ in 0..segments {
        let p1 = Vector2::new(
            center.x + (angle * crate::math::DEG2RAD).cos() * radius,
            center.y + (angle * crate::math::DEG2RAD).sin() * radius,
        );
        let p2 = Vector2::new(
            center.x + ((angle + step_length) * crate::math::DEG2RAD).cos() * radius,
            center.y + ((angle + step_length) * crate::math::DEG2RAD).sin() * radius,
        );

        rlgl::rl_vertex2f(center.x, center.y);
        rlgl::rl_vertex2f(p1.x, p1.y);
        rlgl::rl_vertex2f(p2.x, p2.y);

        angle += step_length;
    }
    rlgl::rl_end();
}

pub fn draw_rectangle_rec(rec: Rectangle, color: Color) {
    draw_rectangle_pro(rec, Vector2::new(0.0, 0.0), 0.0, color);
}

pub fn draw_rectangle_lines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    rlgl::rl_set_texture(rlgl::rl_get_texture_id_default());
    rlgl::rl_begin(RL_LINES);
    rlgl::rl_color4ub(color.r, color.g, color.b, color.a);
    
    rlgl::rl_vertex2f(pos_x as f32, pos_y as f32);
    rlgl::rl_vertex2f((pos_x + width) as f32, pos_y as f32);
    
    rlgl::rl_vertex2f((pos_x + width) as f32, pos_y as f32);
    rlgl::rl_vertex2f((pos_x + width) as f32, (pos_y + height) as f32);
    
    rlgl::rl_vertex2f((pos_x + width) as f32, (pos_y + height) as f32);
    rlgl::rl_vertex2f(pos_x as f32, (pos_y + height) as f32);
    
    rlgl::rl_vertex2f(pos_x as f32, (pos_y + height) as f32);
    rlgl::rl_vertex2f(pos_x as f32, pos_y as f32);
    
    rlgl::rl_end();
}
