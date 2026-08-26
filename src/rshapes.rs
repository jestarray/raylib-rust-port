use crate::rlgl::{
    self, rlColor4ub, rlNormal3f, rlSetTexture, rlTexCoord2f, rlVertex2f, RL_LINES, RL_QUADS,
    RL_TRIANGLES,
};
use crate::types::{Color, Rectangle, Texture2D, Vector2};

pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    draw_rectangle_v(
        Vector2::new(pos_x as f32, pos_y as f32),
        Vector2::new(width as f32, height as f32),
        color,
    );
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
    let mut topLeft = Vector2::new(0.0, 0.0);
    let mut topRight = Vector2::new(0.0, 0.0);
    let mut bottomLeft = Vector2::new(0.0, 0.0);
    let mut bottomRight = Vector2::new(0.0, 0.0);

    if rotation == 0.0 {
        let x = rec.x - origin.x;
        let y = rec.y - origin.y;
        topLeft = Vector2::new(x, y);
        topRight = Vector2::new(x + rec.width, y);
        bottomLeft = Vector2::new(x, y + rec.height);
        bottomRight = Vector2::new(x + rec.width, y + rec.height);
    } else {
        let sin = (rotation * crate::math::DEG2RAD).sin();
        let cos = (rotation * crate::math::DEG2RAD).cos();

        let dx = -origin.x;
        let dy = -origin.y;

        topLeft.x = rec.x + dx * cos - dy * sin;
        topLeft.y = rec.y + dx * sin + dy * cos;

        topRight.x = rec.x + (dx + rec.width) * cos - dy * sin;
        topRight.y = rec.y + (dx + rec.width) * sin + dy * cos;

        bottomLeft.x = rec.x + dx * cos - (dy + rec.height) * sin;
        bottomLeft.y = rec.y + dx * sin + (dy + rec.height) * cos;

        bottomRight.x = rec.x + (dx + rec.width) * cos - (dy + rec.height) * sin;
        bottomRight.y = rec.y + (dx + rec.width) * sin + (dy + rec.height) * cos;
    }

    unsafe {
        rlgl::rlSetTexture(GetShapesTexture().id);
        let shapeRect = GetShapesTextureRectangle();
        rlgl::rlBegin(RL_QUADS);
        rlNormal3f(0.0, 0.0, 1.0);
        rlColor4ub(color.r, color.g, color.b, color.a);

        rlTexCoord2f(
            shapeRect.x / texShapes.width as f32,
            shapeRect.y / texShapes.height as f32,
        );
        rlVertex2f(topLeft.x, topLeft.y);

        rlTexCoord2f(
            shapeRect.x / texShapes.width as f32,
            (shapeRect.y + shapeRect.height) / texShapes.height as f32,
        );
        rlVertex2f(bottomLeft.x, bottomLeft.y);

        rlTexCoord2f(
            (shapeRect.x + shapeRect.width) / texShapes.width as f32,
            (shapeRect.y + shapeRect.height) / texShapes.height as f32,
        );
        rlVertex2f(bottomRight.x, bottomRight.y);

        rlTexCoord2f(
            (shapeRect.x + shapeRect.width) / texShapes.width as f32,
            shapeRect.y / texShapes.height as f32,
        );
        rlVertex2f(topRight.x, topRight.y);
        rlgl::rlEnd();
        rlSetTexture(0);
    }
}

// Get texture that is used for shapes drawing
pub static mut texShapes: Texture2D = Texture2D {
    id: 1,
    width: 1,
    height: 1,
    mipmaps: 1,
    format: 7,
};
pub fn GetShapesTexture() -> Texture2D {
    unsafe {
        return texShapes;
    }
}
pub static mut texShapesRec: Rectangle = Rectangle {
    x: 0.0,
    y: 0.0,
    width: 1.0,
    height: 1.0,
}; // Texture source rectangle used on shapes drawing
   // Get texture source rectangle that is used for shapes drawing
pub fn GetShapesTextureRectangle() -> Rectangle {
    unsafe {
        return texShapesRec;
    }
}

// Set texture and rectangle to be used on shapes drawing
// NOTE: It can be useful when using basic shapes and one single font,
// defining a font char white rectangle would allow drawing everything in a single draw call
pub fn SetShapesTexture(texture: Texture2D, source: Rectangle) {
    // Reset texture to default pixel if required
    // WARNING: Shapes texture should be probably better validated,
    // it can break the rendering of all shapes if misused
    unsafe {
        if ((texture.id == 0) || (source.width == 0.0) || (source.height == 0.0)) {
            texShapes = Texture2D {
                id: 1,
                width: 1,
                height: 1,
                mipmaps: 1,
                format: 7,
            };
            texShapesRec = Rectangle {
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
            };
        } else {
            texShapes = texture;
            texShapesRec = source;
        }
    }
}

pub fn draw_rectangle_gradient_v(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color1: Color,
    color2: Color,
) {
    unsafe {
        rlgl::rlSetTexture(rlgl::RLGL.State.defaultTextureId);
        rlgl::rlBegin(RL_QUADS);
        rlgl::rlColor4ub(color1.r, color1.g, color1.b, color1.a);
        rlgl::rlVertex2f(x as f32, y as f32);
        rlgl::rlColor4ub(color2.r, color2.g, color2.b, color2.a);
        rlgl::rlVertex2f(x as f32, (y + height) as f32);
        rlgl::rlVertex2f((x + width) as f32, (y + height) as f32);
        rlgl::rlColor4ub(color1.r, color1.g, color1.b, color1.a);
        rlgl::rlVertex2f((x + width) as f32, y as f32);
        rlgl::rlEnd();
    }
}

pub fn draw_rectangle_gradient_h(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color1: Color,
    color2: Color,
) {
    unsafe {
        rlgl::rlSetTexture(rlgl::RLGL.State.defaultTextureId);
        rlgl::rlBegin(RL_QUADS);
        rlgl::rlColor4ub(color1.r, color1.g, color1.b, color1.a);
        rlgl::rlVertex2f(x as f32, y as f32);
        rlgl::rlVertex2f(x as f32, (y + height) as f32);
        rlgl::rlColor4ub(color2.r, color2.g, color2.b, color2.a);
        rlgl::rlVertex2f((x + width) as f32, (y + height) as f32);
        rlgl::rlVertex2f((x + width) as f32, y as f32);
        rlgl::rlEnd();
    }
}

pub fn draw_line(start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: Color) {
    unsafe {
        rlgl::rlSetTexture(rlgl::RLGL.State.defaultTextureId);
        rlgl::rlBegin(RL_LINES);
        rlgl::rlColor4ub(color.r, color.g, color.b, color.a);
        rlgl::rlVertex2f(start_pos_x as f32, start_pos_y as f32);
        rlgl::rlVertex2f(end_pos_x as f32, end_pos_y as f32);
        rlgl::rlEnd();
    }
}

pub fn draw_circle(center_x: i32, center_y: i32, radius: f32, color: Color) {
    draw_circle_v(
        Vector2::new(center_x as f32, center_y as f32),
        radius,
        color,
    );
}

pub fn draw_circle_v(center: Vector2, radius: f32, color: Color) {
    let segments = 36; // A reasonable default
    draw_circle_sector(center, radius, 0.0, 360.0, segments, color);
}

pub fn draw_circle_sector(
    center: Vector2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: Color,
) {
    if radius <= 0.0 {
        return;
    }

    let step_length = (end_angle - start_angle) / (segments as f32);
    let mut angle = start_angle;

    unsafe {
        rlgl::rlBegin(RL_TRIANGLES);
        rlgl::rlColor4ub(color.r, color.g, color.b, color.a);

        for _ in 0..segments {
            let p1 = Vector2::new(
                center.x + (angle * crate::math::DEG2RAD).cos() * radius,
                center.y + (angle * crate::math::DEG2RAD).sin() * radius,
            );
            let p2 = Vector2::new(
                center.x + ((angle + step_length) * crate::math::DEG2RAD).cos() * radius,
                center.y + ((angle + step_length) * crate::math::DEG2RAD).sin() * radius,
            );

            rlgl::rlVertex2f(center.x, center.y);
            rlgl::rlVertex2f(p1.x, p1.y);
            rlgl::rlVertex2f(p2.x, p2.y);

            angle += step_length;
        }
        rlgl::rlEnd();
    }
}

pub fn draw_rectangle_rec(rec: Rectangle, color: Color) {
    draw_rectangle_pro(rec, Vector2::new(0.0, 0.0), 0.0, color);
}

pub fn draw_rectangle_lines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    unsafe {
        rlgl::rlSetTexture(rlgl::RLGL.State.defaultTextureId);
        rlgl::rlBegin(RL_LINES);
        rlgl::rlColor4ub(color.r, color.g, color.b, color.a);

        rlgl::rlVertex2f(pos_x as f32, pos_y as f32);
        rlgl::rlVertex2f((pos_x + width) as f32, pos_y as f32);

        rlgl::rlVertex2f((pos_x + width) as f32, pos_y as f32);
        rlgl::rlVertex2f((pos_x + width) as f32, (pos_y + height) as f32);

        rlgl::rlVertex2f((pos_x + width) as f32, (pos_y + height) as f32);
        rlgl::rlVertex2f(pos_x as f32, (pos_y + height) as f32);

        rlgl::rlVertex2f(pos_x as f32, (pos_y + height) as f32);
        rlgl::rlVertex2f(pos_x as f32, pos_y as f32);

        rlgl::rlEnd();
    }
}
