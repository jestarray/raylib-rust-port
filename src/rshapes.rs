#![allow(unused_variables)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(clippy::missing_safety_doc, unused_parens, static_mut_refs)]
#![allow(
    clippy::too_many_arguments,
    clippy::needless_return,
    clippy::manual_range_contains,
    clippy::field_reassign_with_default,
    clippy::manual_map,
    clippy::match_like_matches_macro,
    clippy::upper_case_acronyms,
    clippy::let_and_return,
    clippy::double_parens,
)]
use crate::rlgl::{
    rlBegin, rlColor4ub, rlEnd, rlGetMatrixTransform, rlNormal3f, rlSetTexture, rlTexCoord2f,
    rlVertex2f, RL_LINES, RL_QUADS, RL_TRIANGLES,
};
use crate::types::{Color, Rectangle, Texture2D, Vector2};
use std::f32::consts::PI;
const DEG2RAD: f32 = PI / 180.0;
const SMOOTH_CIRCLE_ERROR_RATE: f32 = 0.5;
const SPLINE_SEGMENT_DIVISIONS: i32 = 24;
pub static mut texShapes: Texture2D = Texture2D {
    id: 1,
    width: 1,
    height: 1,
    mipmaps: 1,
    format: 7,
};
pub static mut texShapesRec: Rectangle = Rectangle {
    x: 0.0,
    y: 0.0,
    width: 1.0,
    height: 1.0,
};

const RAD2DEG: f32 = 180.0 / PI;

//----------------------------------------------------------------------------------
// Module Functions Definition
//----------------------------------------------------------------------------------
// Set texture and rectangle to be used on shapes drawing
// NOTE: It can be useful when using basic shapes and one single font,
// defining a font char white rectangle would allow drawing everything in a single draw call
pub unsafe fn SetShapesTexture(texture: Texture2D, rec: Rectangle)
{
    // Reset texture to default pixel if required
    // WARNING: Shapes texture should be probably better validated,
    // it can break the rendering of all shapes if misused
    if (texture.id == 0) || (rec.width == 0.0) || (rec.height == 0.0)
    {
        texShapes = Texture2D { id: 1, width: 1, height: 1, mipmaps: 1, format: 7 };
        texShapesRec = Rectangle { x: 0.0, y: 0.0, width: 1.0, height: 1.0 };
    }
    else
    {
        texShapes = texture;
        texShapesRec = rec;
    }
}

// Get texture that is used for shapes drawing
pub unsafe fn GetShapesTexture() -> Texture2D
{
    return texShapes;
}

// Get texture source rectangle that is used for shapes drawing
pub unsafe fn GetShapesTextureRectangle() -> Rectangle
{
    return texShapesRec;
}

// Draw a pixel
pub unsafe fn DrawPixel(posX: i32, posY: i32, color: Color)
{
    DrawPixelV(Vector2 { x: (posX as f32), y: (posY as f32) }, color);
}

// Draw a pixel (Vector version)
pub unsafe fn DrawPixelV(position: Vector2, color: Color)
{
    rlSetTexture(GetShapesTexture().id);
    let shapeRect: Rectangle = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);

        rlNormal3f(0.0, 0.0, 1.0);
        rlColor4ub(color.r, color.g, color.b, color.a);

        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(position.x, position.y);

        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(position.x, position.y + 1.0);

        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(position.x + 1.0, position.y + 1.0);

        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(position.x + 1.0, position.y);

    rlEnd();

    rlSetTexture(0);
}

// Draw a line (using gl lines)
pub unsafe fn DrawLine(startPosX: i32, startPosY: i32, endPosX: i32, endPosY: i32, color: Color)
{
    rlBegin(RL_LINES);
        rlColor4ub(color.r, color.g, color.b, color.a);
        rlVertex2f((startPosX as f32), (startPosY as f32));
        rlVertex2f((endPosX as f32), (endPosY as f32));
    rlEnd();
}

// Draw a line defining thickness
pub unsafe fn DrawLineEx(startPos: Vector2, endPos: Vector2, thick: f32, color: Color)
{
    let delta: Vector2 = Vector2 { x: endPos.x - startPos.x, y: endPos.y - startPos.y };
    let length: f32 = (delta.x*delta.x + delta.y*delta.y).sqrt();

    if (length > 0.0) && (thick > 0.0)
    {
        let scale: f32 = thick/(2.0*length);

        let radius: Vector2 = Vector2 { x: -scale*delta.y, y: scale*delta.x };
        let strip: [ Vector2; 4 ] = [
            Vector2 { x: startPos.x - radius.x, y: startPos.y - radius.y },
            Vector2 { x: startPos.x + radius.x, y: startPos.y + radius.y },
            Vector2 { x: endPos.x - radius.x, y: endPos.y - radius.y },
            Vector2 { x: endPos.x + radius.x, y: endPos.y + radius.y }
        ];

        DrawTriangleStrip(&strip, 4, color);
    }
}

// Draw a line (using gl lines)
pub unsafe fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color)
{
    rlBegin(RL_LINES);
        rlColor4ub(color.r, color.g, color.b, color.a);
        rlVertex2f(startPos.x, startPos.y);
        rlVertex2f(endPos.x, endPos.y);
    rlEnd();
}

// Draw lines sequuence (using gl lines)
pub unsafe fn DrawLineStrip(points: &[Vector2], pointCount: i32, color: Color)
{
    if pointCount < 2 { return; } // Security check

    rlBegin(RL_LINES);
        rlColor4ub(color.r, color.g, color.b, color.a);

        for i in 0..pointCount - 1
        {
            rlVertex2f(points[(i as usize)].x, points[(i as usize)].y);
            rlVertex2f(points[((i + 1) as usize)].x, points[((i + 1) as usize)].y);
        }
    rlEnd();
}

// Draw line using cubic-bezier spline, in-out interpolation, no control points
pub unsafe fn DrawLineBezier(startPos: Vector2, endPos: Vector2, thick: f32, color: Color)
{
    let mut previous: Vector2 = startPos;
    let mut current: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    let mut points: [ Vector2; ((2*SPLINE_SEGMENT_DIVISIONS + 2) as usize) ] = [ Vector2 { x: 0.0, y: 0.0 }; ((2*SPLINE_SEGMENT_DIVISIONS + 2) as usize) ];

    for i in 1..=SPLINE_SEGMENT_DIVISIONS
    {
        // Cubic easing in-out
        // NOTE: Easing is calculated only for y position value
        current.y = EaseCubicInOut((i as f32), startPos.y, endPos.y - startPos.y, (SPLINE_SEGMENT_DIVISIONS as f32));
        current.x = previous.x + (endPos.x - startPos.x)/(SPLINE_SEGMENT_DIVISIONS as f32);

        let dy: f32 = current.y - previous.y;
        let dx: f32 = current.x - previous.x;
        let size: f32 = 0.5*thick/(dx*dx+dy*dy).sqrt();

        if i == 1
        {
            points[0].x = previous.x + dy*size;
            points[0].y = previous.y - dx*size;
            points[1].x = previous.x - dy*size;
            points[1].y = previous.y + dx*size;
        }

        points[((2*i + 1) as usize)].x = current.x - dy*size;
        points[((2*i + 1) as usize)].y = current.y + dx*size;
        points[((2*i) as usize)].x = current.x + dy*size;
        points[((2*i) as usize)].y = current.y - dx*size;

        previous = current;
    }

    DrawTriangleStrip(&points, 2*SPLINE_SEGMENT_DIVISIONS + 2, color);
}

// Draw a dashed line
pub unsafe fn DrawLineDashed(startPos: Vector2, endPos: Vector2, dashSize: i32, spaceSize: i32, color: Color)
{
    // Calculate the vector and length of the line
    let dx: f32 = endPos.x - startPos.x;
    let dy: f32 = endPos.y - startPos.y;
    let lineLength: f32 = (dx*dx + dy*dy).sqrt();

    // If the line is too short for dashing or dash size is invalid, draw a solid thick line
    if (lineLength < ((dashSize + spaceSize) as f32)) || (dashSize <= 0)
    {
        DrawLineV(startPos, endPos, color);
        return;
    }

    // Calculate the normalized direction vector of the line
    let invLineLength: f32 = 1.0/lineLength;
    let dirX: f32 = dx*invLineLength;
    let dirY: f32 = dy*invLineLength;

    let mut currentPos: Vector2 = startPos;
    let mut distanceTraveled: f32 = 0.0;

    rlBegin(RL_LINES);
        rlColor4ub(color.r, color.g, color.b, color.a);

        while distanceTraveled < lineLength
        {
            // Calculate the end of the current dash
            let mut dashEndDist: f32 = distanceTraveled + (dashSize as f32);
            if dashEndDist > lineLength { dashEndDist = lineLength; }

            let dashEndPos: Vector2 = Vector2 { x: startPos.x + dashEndDist*dirX, y: startPos.y + dashEndDist*dirY };

            // Draw the dash segment
            rlVertex2f(currentPos.x, currentPos.y);
            rlVertex2f(dashEndPos.x, dashEndPos.y);

            // Update the distance traveled and move the current position for the next dash
            distanceTraveled = dashEndDist + (spaceSize as f32);
            currentPos.x = startPos.x + distanceTraveled*dirX;
            currentPos.y = startPos.y + distanceTraveled*dirY;
        }
    rlEnd();
}

// Draw a triangle
// NOTE: Vertex must be provided in counter-clockwise order
pub unsafe fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color)
{
    DrawTriangleGradient(v1, v2, v3, color, color, color);
}

// Draw triangle with interpolated colors (vertex in counter-clockwise order!)
pub unsafe fn DrawTriangleGradient(v1: Vector2, v2: Vector2, v3: Vector2, c1: Color, c2: Color, c3: Color)
{
    rlSetTexture(GetShapesTexture().id);
    let shapeRect: Rectangle = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);
        rlNormal3f(0.0, 0.0, 1.0);

        rlColor4ub(c1.r, c1.g, c1.b, c1.a);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(v1.x, v1.y);

        rlColor4ub(c2.r, c2.g, c2.b, c2.a);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(v2.x, v2.y);

        rlColor4ub(c3.r, c3.g, c3.b, c3.a);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(v3.x, v3.y);

        rlColor4ub(c3.r, c3.g, c3.b, c3.a);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(v3.x, v3.y);
    rlEnd();

    rlSetTexture(0);
}

// Draw a triangle using lines
// NOTE: Vertex must be provided in counter-clockwise order
pub unsafe fn DrawTriangleLines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color)
{
    rlBegin(RL_LINES);
        rlColor4ub(color.r, color.g, color.b, color.a);
        rlVertex2f(v1.x, v1.y);
        rlVertex2f(v2.x, v2.y);

        rlVertex2f(v2.x, v2.y);
        rlVertex2f(v3.x, v3.y);

        rlVertex2f(v3.x, v3.y);
        rlVertex2f(v1.x, v1.y);
    rlEnd();
}

// Draw a triangle using lines with thickness
// NOTE: Vertex must be provided in counter-clockwise order
pub unsafe fn DrawTriangleLinesEx(mut v1: Vector2, mut v2: Vector2, mut v3: Vector2, thick: f32, color: Color)
{
    /*
    A sketch to make things simpler

    The exterior points are v1-3, the interior points are v4-6, and the exterior edges are e1-3

             v1
             /\
            /v4\
           //  \\
          //    \\
      e3 //      \\ e2
        //        \\
       //          \\
      //v5        v6\\
     v2==============v3
             e1
    */

    let e1: Vector2 = Vector2 {x: v2.x - v3.x, y: v2.y - v3.y};
    let e2: Vector2 = Vector2 {x: v3.x - v1.x, y: v3.y - v1.y};
    let e3: Vector2 = Vector2 {x: v1.x - v2.x, y: v1.y - v2.y};

    let e1Length: f32 = (e1.x*e1.x + e1.y*e1.y).sqrt();
    let e2Length: f32 = (e2.x*e2.x + e2.y*e2.y).sqrt();
    let e3Length: f32 = (e3.x*e3.x + e3.y*e3.y).sqrt();

    let perimeter: f32 = e1Length + e2Length + e3Length;
    let semiperimeter: f32 = perimeter/2.0;

    // The incenter of a triangle is equidistant from each edge, which is useful for drawing a nice looking outline
    let incenter: Vector2 = Vector2 {
        x: (e1Length*v1.x + e2Length*v2.x + e3Length*v3.x)/perimeter,
        y: (e1Length*v1.y + e2Length*v2.y + e3Length*v3.y)/perimeter
    };

    // The inradius of a triangle is the radius of the biggest circle that can fit inside of said triangle
    // That circle is also centered on the incenter
    let inradius: f32 = (((semiperimeter - e1Length)*(semiperimeter - e2Length)*(semiperimeter - e3Length))/semiperimeter).sqrt();

    // The triangle (v1, v2, v3) will be scaled by this to get (v4, v5, v6)
    let scale: f32 = 1.0 - thick/inradius;

    // Just a filled-in triangle
    if scale <= 0.0
    {
        DrawTriangle(v1, v2, v3, color);
        return;
    }

    // In order for the scaling to be correct, the incenter has to be at the origin (0, 0) when scaling
    let mut v4: Vector2 = Vector2 {x: incenter.x + (v1.x - incenter.x)*scale, y: incenter.y + (v1.y - incenter.y)*scale};
    let mut v5: Vector2 = Vector2 {x: incenter.x + (v2.x - incenter.x)*scale, y: incenter.y + (v2.y - incenter.y)*scale};
    let mut v6: Vector2 = Vector2 {x: incenter.x + (v3.x - incenter.x)*scale, y: incenter.y + (v3.y - incenter.y)*scale};

    // Swap the vertices so the winding order is correct
    if thick < 0.0
    {
        std::mem::swap(&mut v1, &mut v4);

        std::mem::swap(&mut v2, &mut v5);

        std::mem::swap(&mut v3, &mut v6);
    }

    rlSetTexture(GetShapesTexture().id);
    let shapeRect: Rectangle = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);

        rlColor4ub(color.r, color.g, color.b, color.a);

        // Edge 3
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(v1.x, v1.y);

        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(v2.x, v2.y);

        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(v5.x, v5.y);

        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(v4.x, v4.y);

        // Edge 1
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(v2.x, v2.y);

        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(v3.x, v3.y);

        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(v6.x, v6.y);

        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(v5.x, v5.y);

        // Edge 2
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(v3.x, v3.y);

        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(v1.x, v1.y);

        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(v4.x, v4.y);

        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(v6.x, v6.y);

    rlEnd();

    rlSetTexture(0);
}

// Draw a triangle fan defined by points
// NOTE: First vertex provided is the center, shared by all triangles
// By default, following vertex should be provided in counter-clockwise order
pub unsafe fn DrawTriangleFan(points: &[Vector2], pointCount: i32, color: Color)
{
    if pointCount >= 3
    {
        rlSetTexture(GetShapesTexture().id);
        let shapeRect: Rectangle = GetShapesTextureRectangle();

        rlBegin(RL_QUADS);
            rlColor4ub(color.r, color.g, color.b, color.a);

            for i in 1..pointCount - 1
            {
                rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                rlVertex2f(points[0].x, points[0].y);

                rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                rlVertex2f(points[(i as usize)].x, points[(i as usize)].y);

                rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                rlVertex2f(points[((i + 1) as usize)].x, points[((i + 1) as usize)].y);

                rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                rlVertex2f(points[((i + 1) as usize)].x, points[((i + 1) as usize)].y);
            }
        rlEnd();
        rlSetTexture(0);
    }
}

// Draw a triangle strip defined by points
// NOTE: Every new vertex connects with previous two
pub unsafe fn DrawTriangleStrip(points: &[Vector2], pointCount: i32, color: Color)
{
    if pointCount >= 3
    {
        rlBegin(RL_TRIANGLES);
            rlColor4ub(color.r, color.g, color.b, color.a);

            for i in 2..pointCount
            {
                if (i%2) == 0
                {
                    rlVertex2f(points[(i as usize)].x, points[(i as usize)].y);
                    rlVertex2f(points[((i - 2) as usize)].x, points[((i - 2) as usize)].y);
                    rlVertex2f(points[((i - 1) as usize)].x, points[((i - 1) as usize)].y);
                }
                else
                {
                    rlVertex2f(points[(i as usize)].x, points[(i as usize)].y);
                    rlVertex2f(points[((i - 1) as usize)].x, points[((i - 1) as usize)].y);
                    rlVertex2f(points[((i - 2) as usize)].x, points[((i - 2) as usize)].y);
                }
            }
        rlEnd();
    }
}

// Draw a color-filled rectangle
pub unsafe fn DrawRectangle(posX: i32, posY: i32, width: i32, height: i32, color: Color)
{
    DrawRectangleV(Vector2 { x: (posX as f32), y: (posY as f32) }, Vector2 { x: (width as f32), y: (height as f32) }, color);
}

// Draw a color-filled rectangle (Vector version)
// NOTE: On OpenGL 3.3 and ES2 using QUADS to avoid drawing order issues
pub unsafe fn DrawRectangleV(position: Vector2, size: Vector2, color: Color)
{
    DrawRectanglePro(Rectangle { x: position.x, y: position.y, width: size.x, height: size.y }, Vector2 { x: 0.0, y: 0.0 }, 0.0, color);
}

// Draw a color-filled rectangle
pub unsafe fn DrawRectangleRec(rec: Rectangle, color: Color)
{
    DrawRectanglePro(rec, Vector2 { x: 0.0, y: 0.0 }, 0.0, color);
}

// Draw a color-filled rectangle with pro parameters
pub unsafe fn DrawRectanglePro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color)
{
    let mut topLeft: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut topRight: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut bottomLeft: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut bottomRight: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    // Only calculate rotation if needed
    if rotation == 0.0
    {
        let x: f32 = rec.x - origin.x;
        let y: f32 = rec.y - origin.y;
        topLeft = Vector2 { x, y };
        topRight = Vector2 { x: x + rec.width, y };
        bottomLeft = Vector2 { x, y: y + rec.height };
        bottomRight = Vector2 { x: x + rec.width, y: y + rec.height };
    }
    else
    {
        let sinRotation: f32 = (rotation*DEG2RAD).sin();
        let cosRotation: f32 = (rotation*DEG2RAD).cos();
        let x: f32 = rec.x;
        let y: f32 = rec.y;
        let dx: f32 = -origin.x;
        let dy: f32 = -origin.y;

        topLeft.x = x + dx*cosRotation - dy*sinRotation;
        topLeft.y = y + dx*sinRotation + dy*cosRotation;

        topRight.x = x + (dx + rec.width)*cosRotation - dy*sinRotation;
        topRight.y = y + (dx + rec.width)*sinRotation + dy*cosRotation;

        bottomLeft.x = x + dx*cosRotation - (dy + rec.height)*sinRotation;
        bottomLeft.y = y + dx*sinRotation + (dy + rec.height)*cosRotation;

        bottomRight.x = x + (dx + rec.width)*cosRotation - (dy + rec.height)*sinRotation;
        bottomRight.y = y + (dx + rec.width)*sinRotation + (dy + rec.height)*cosRotation;
    }

    rlSetTexture(GetShapesTexture().id);
    let shapeRect: Rectangle = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);

        rlNormal3f(0.0, 0.0, 1.0);
        rlColor4ub(color.r, color.g, color.b, color.a);

        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(topLeft.x, topLeft.y);

        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(bottomLeft.x, bottomLeft.y);

        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(bottomRight.x, bottomRight.y);

        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(topRight.x, topRight.y);

    rlEnd();

    rlSetTexture(0);
}

// Draw a vertical-gradient-filled rectangle
pub unsafe fn DrawRectangleGradientV(posX: i32, posY: i32, width: i32, height: i32, top: Color, bottom: Color)
{
    DrawRectangleGradientEx(Rectangle { x: (posX as f32), y: (posY as f32), width: (width as f32), height: (height as f32) }, top, bottom, bottom, top);
}

// Draw a horizontal-gradient-filled rectangle
pub unsafe fn DrawRectangleGradientH(posX: i32, posY: i32, width: i32, height: i32, left: Color, right: Color)
{
    DrawRectangleGradientEx(Rectangle { x: (posX as f32), y: (posY as f32), width: (width as f32), height: (height as f32) }, left, left, right, right);
}

// Draw a gradient-filled rectangle with custom vertex colors, counter-clockwise color order
pub unsafe fn DrawRectangleGradientEx(rec: Rectangle, col1: Color, col2: Color, col3: Color, col4: Color)
{
    rlSetTexture(GetShapesTexture().id);
    let shapeRect: Rectangle = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);
        rlNormal3f(0.0, 0.0, 1.0);

        // NOTE: Default raylib font character 95 is a white square
        rlColor4ub(col1.r, col1.g, col1.b, col1.a);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(rec.x, rec.y);

        rlColor4ub(col2.r, col2.g, col2.b, col2.a);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(rec.x, rec.y + rec.height);

        rlColor4ub(col3.r, col3.g, col3.b, col3.a);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(rec.x + rec.width, rec.y + rec.height);

        rlColor4ub(col4.r, col4.g, col4.b, col4.a);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(rec.x + rec.width, rec.y);
    rlEnd();

    rlSetTexture(0);
}

// Draw rectangle outline
// WARNING: All Draw*Lines() functions use RL_LINES for drawing,
// it implies flushing the current batch and changing draw mode to RL_LINES
// but it solves another issue: https://github.com/raysan5/raylib/issues/3884
pub unsafe fn DrawRectangleLines(posX: i32, posY: i32, width: i32, height: i32, color: Color)
{
    let mat: crate::types::Matrix = rlGetMatrixTransform();
    let xOffset: f32 = 0.5/mat.x_axis.x;
    let yOffset: f32 = 0.5/mat.y_axis.y;

    rlBegin(RL_LINES);
        rlColor4ub(color.r, color.g, color.b, color.a);
        rlVertex2f((posX as f32) + xOffset, (posY as f32) + yOffset);
        rlVertex2f((posX as f32) + (width as f32) - xOffset, (posY as f32) + yOffset);

        rlVertex2f((posX as f32) + (width as f32) - xOffset, (posY as f32) + yOffset);
        rlVertex2f((posX as f32) + (width as f32) - xOffset, (posY as f32) + (height as f32) - yOffset);

        rlVertex2f((posX as f32) + (width as f32) - xOffset, (posY as f32) + (height as f32) - yOffset);
        rlVertex2f((posX as f32) + xOffset, (posY as f32) + (height as f32) - yOffset);

        rlVertex2f((posX as f32) + xOffset, (posY as f32) + (height as f32) - yOffset);
        rlVertex2f((posX as f32) + xOffset, (posY as f32) + yOffset);
    rlEnd();

/*
// Previous implementation, it has issues... but it does not require view matrix...
#if SUPPORT_QUADS_DRAW_MODE
    DrawRectangle(posX, posY, width, 1, color);
    DrawRectangle(posX + width - 1, posY + 1, 1, height - 2, color);
    DrawRectangle(posX, posY + height - 1, width, 1, color);
    DrawRectangle(posX, posY + 1, 1, height - 2, color);
#else
    rlBegin(RL_LINES);
        rlColor4ub(color.r, color.g, color.b, color.a);
        rlVertex2f((float)posX, (float)posY);
        rlVertex2f((float)posX + (float)width, (float)posY + 1);

        rlVertex2f((float)posX + (float)width, (float)posY + 1);
        rlVertex2f((float)posX + (float)width, (float)posY + (float)height);

        rlVertex2f((float)posX + (float)width, (float)posY + (float)height);
        rlVertex2f((float)posX + 1, (float)posY + (float)height);

        rlVertex2f((float)posX + 1, (float)posY + (float)height);
        rlVertex2f((float)posX + 1, (float)posY + 1);
    rlEnd();
#endif
*/
}

// Draw rectangle outline with line thickness
pub unsafe fn DrawRectangleLinesEx(rec: Rectangle, mut thick: f32, color: Color)
{
    if (thick > rec.width/2.0) || (thick > rec.height/2.0)
    {
        if rec.width >= rec.height { thick = rec.height/2.0; }
        else if rec.width <= rec.height { thick = rec.width/2.0; }
    }

    if thick > 0.0
    {
        // When rec = { x, y, 8.0f, 6.0f } and thick = 2, the following
        // four rectangles are drawn ([T]op, [B]ottom, [L]eft, [R]ight):
        //
        //   TTTTTTTT
        //   TTTTTTTT
        //   LL    RR
        //   LL    RR
        //   BBBBBBBB
        //   BBBBBBBB
        //

        let top: Rectangle = Rectangle { x: rec.x, y: rec.y, width: rec.width, height: thick };
        let bottom: Rectangle = Rectangle { x: rec.x, y: rec.y - thick + rec.height, width: rec.width, height: thick };
        let left: Rectangle = Rectangle { x: rec.x, y: rec.y + thick, width: thick, height: rec.height - thick*2.0 };
        let right: Rectangle = Rectangle { x: rec.x - thick + rec.width, y: rec.y + thick, width: thick, height: rec.height - thick*2.0 };

        DrawRectangleRec(top, color);
        DrawRectangleRec(bottom, color);
        DrawRectangleRec(left, color);
        DrawRectangleRec(right, color);
    }
    else
    {
        // When rec = { x, y, 8.0f, 6.0f } and thick = -2, the following
        // four rectangles are drawn ([T]op, [B]ottom, [L]eft, [R]ight):
        //
        //   TTTTTTTTTTTT
        //   TTTTTTTTTTTT
        //   LL        RR
        //   LL        RR
        //   LL        RR
        //   LL        RR
        //   LL        RR
        //   LL        RR
        //   BBBBBBBBBBBB
        //   BBBBBBBBBBBB
        //

        thick *= -1.0;

        let top: Rectangle = Rectangle { x: rec.x - thick, y: rec.y - thick, width: rec.width + thick*2.0, height: thick };
        let bottom: Rectangle = Rectangle { x: rec.x - thick, y: rec.y + rec.height, width: rec.width + thick*2.0, height: thick};
        let left: Rectangle = Rectangle { x: rec.x - thick, y: rec.y, width: thick, height: rec.height };
        let right: Rectangle = Rectangle { x: rec.x + rec.width, y: rec.y, width: thick, height: rec.height };

        DrawRectangleRec(top, color);
        DrawRectangleRec(bottom, color);
        DrawRectangleRec(left, color);
        DrawRectangleRec(right, color);
    }
}

// Draw rectangle with rounded edges
pub unsafe fn DrawRectangleRounded(rec: Rectangle, mut roundness: f32, mut segments: i32, color: Color)
{
    // Not a rounded rectangle
    if roundness <= 0.0
    {
        DrawRectangleRec(rec, color);
        return;
    }

    if roundness >= 1.0 { roundness = 1.0; }

    // Calculate corner radius
    let radius: f32 = if (rec.width > rec.height) { (rec.height*roundness)/2.0 } else { (rec.width*roundness)/2.0 };
    if radius <= 0.0 { return; }

    // Calculate number of segments to use for the corners
    if segments < 1
    {
        // Calculate the maximum angle between segments based on the error rate (usually 0.5f)
        let th: f32 = (2.0*(1.0 - SMOOTH_CIRCLE_ERROR_RATE/radius).powf(2.0) - 1.0).acos();
        segments = (((2.0*PI/th)/4.0).ceil() as i32);
        if segments <= 0 { segments = 4; }
    }

    let stepLength: f32 = 90.0/(segments as f32);

    /*
    Quick sketch to make sense of all of this,
    there are 9 parts to draw, also mark the 12 points used

          P0____________________P1
          /|                    |\
         /1|          2         |3\
     P7 /__|____________________|__\ P2
       |   |P8                P9|   |
       | 8 |          9         | 4 |
       | __|____________________|__ |
     P6 \  |P11              P10|  / P3
         \7|          6         |5/
          \|____________________|/
          P5                    P4
    */

    // The x-coordinates used for the rounded rect
    let x0 = rec.x + radius;
    let x1 = (rec.x + rec.width) - radius;
    let x2 = rec.x + rec.width;
    let x3 = rec.x;

    // The y-coordinates used for the rounded rect
    let y0 = rec.y;
    let y1 = rec.y + radius;
    let y2 = (rec.y + rec.height) - radius;
    let y3 = rec.y + rec.height;

    let point = [
        Vector2::new( x0, y0 ), // P0
        Vector2::new( x1, y0 ), // P1
        Vector2::new( x2, y1 ), // P2
        Vector2::new( x2, y2 ), // P3
        Vector2::new( x1, y3 ), // P4
        Vector2::new( x0, y3 ), // P5
        Vector2::new( x3, y2 ), // P6
        Vector2::new( x3, y1 ), // P7
        Vector2::new( x0, y1 ), // P8
        Vector2::new( x1, y1 ), // P9
        Vector2::new( x1, y2 ), // P10
        Vector2::new( x0, y2 )  // P11
    ];

    let centers: [ Vector2; 4 ] = [ point[8], point[9], point[10], point[11] ];
    let angles: [ f32; 4 ] = [ 180.0, 270.0, 0.0, 90.0 ];

    rlSetTexture(GetShapesTexture().id);
    let shapeRect: Rectangle = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);
        // Draw all the 4 corners: [1] Upper Left Corner, [3] Upper Right Corner, [5] Lower Right Corner, [7] Lower Left Corner
        for k in 0..4 // Hope the compiler is smart enough to unroll this loop
        {
            let mut angle: f32 = angles[(k as usize)];
            let center: Vector2 = centers[(k as usize)];

            // NOTE: Every QUAD actually represents two segments
            for i in 0..segments/2
            {
                rlColor4ub(color.r, color.g, color.b, color.a);
                rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                rlVertex2f(center.x, center.y);

                rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                rlVertex2f(center.x + (DEG2RAD*(angle + stepLength*2.0)).cos()*radius, center.y + (DEG2RAD*(angle + stepLength*2.0)).sin()*radius);

                rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*radius, center.y + (DEG2RAD*(angle + stepLength)).sin()*radius);

                rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                rlVertex2f(center.x + (DEG2RAD*angle).cos()*radius, center.y + (DEG2RAD*angle).sin()*radius);

                angle += (stepLength*2.0);
            }

            // NOTE: In case number of segments is odd, adding one last piece to the cake
            if (segments%2 != 0)
            {
                rlColor4ub(color.r, color.g, color.b, color.a);
                rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                rlVertex2f(center.x, center.y);

                rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*radius, center.y + (DEG2RAD*(angle + stepLength)).sin()*radius);

                rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                rlVertex2f(center.x + (DEG2RAD*angle).cos()*radius, center.y + (DEG2RAD*angle).sin()*radius);

                rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                rlVertex2f(center.x, center.y);
            }
        }

        // [2] Upper Rectangle
        rlColor4ub(color.r, color.g, color.b, color.a);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[0].x, point[0].y);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[8].x, point[8].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[9].x, point[9].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[1].x, point[1].y);

        // [4] Right Rectangle
        rlColor4ub(color.r, color.g, color.b, color.a);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[2].x, point[2].y);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[9].x, point[9].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[10].x, point[10].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[3].x, point[3].y);

        // [6] Bottom Rectangle
        rlColor4ub(color.r, color.g, color.b, color.a);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[11].x, point[11].y);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[5].x, point[5].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[4].x, point[4].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[10].x, point[10].y);

        // [8] Left Rectangle
        rlColor4ub(color.r, color.g, color.b, color.a);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[7].x, point[7].y);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[6].x, point[6].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[11].x, point[11].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[8].x, point[8].y);

        // [9] Middle Rectangle
        rlColor4ub(color.r, color.g, color.b, color.a);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[8].x, point[8].y);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[11].x, point[11].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[10].x, point[10].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[9].x, point[9].y);

    rlEnd();
    rlSetTexture(0);
}

// Draw rectangle with rounded edges
pub unsafe fn DrawRectangleRoundedLines(rec: Rectangle, mut roundness: f32, mut segments: i32, color: Color)
{
    // Not a rounded rectangle
    if roundness <= 0.0
    {
        DrawRectangleLines((rec.x as i32), (rec.y as i32), (rec.width as i32), (rec.height as i32), color);
        return;
    }

    if roundness >= 1.0 { roundness = 1.0; }

    // Calculate corner radius
    let radius: f32 = if (rec.width > rec.height) { (rec.height*roundness)/2.0 } else { (rec.width*roundness)/2.0 };
    if radius <= 0.0 { return; }

    // Calculate number of segments to use for the corners
    if segments < 1
    {
        // Calculate the maximum angle between segments based on the error rate (usually 0.5f)
        let th: f32 = (2.0*(1.0 - SMOOTH_CIRCLE_ERROR_RATE/radius).powf(2.0) - 1.0).acos();
        segments = (((2.0*PI/th)/4.0).ceil() as i32);
        if segments <= 0 { segments = 4; }
    }

    let stepLength: f32 = 90.0/(segments as f32);

    /*
    Quick sketch to make sense of all of this,
    marks the 8 + 4 (corner centers P8-11) points used

           P0 ------------------ P1
          /                        \
         /                          \
     P7 /                            \ P2
       |    *P8               P9*     |
       |                              |
       |                              |
     P6 \   *P11             P10*    / P3
         \                          /
          \                        /
           P5 ------------------ P4
    */

    // The x-coordinates used for the outline
    let x0: f32 = rec.x + radius + 0.5;
    let x1: f32 = (rec.x + rec.width) - radius - 0.5;
    let x2: f32 = rec.x + rec.width - 0.5;
    let x3: f32 = rec.x + 0.5;

    // The y-coordinates used for the outline
    let y0: f32 = rec.y + 0.5;
    let y1: f32 = rec.y + radius + 0.5;
    let y2: f32 = (rec.y + rec.height) - radius - 0.5;
    let y3: f32 = rec.y + rec.height - 0.5;

    let point: [ Vector2; 8 ] = [
        Vector2 {x: x0, y: y0}, // P0
        Vector2 {x: x1, y: y0}, // P1
        Vector2 {x: x2, y: y1}, // P2
        Vector2 {x: x2, y: y2}, // P3
        Vector2 {x: x1, y: y3}, // P4
        Vector2 {x: x0, y: y3}, // P5
        Vector2 {x: x3, y: y2}, // P6
        Vector2 {x: x3, y: y1}, // P7
    ];

    let centers: [ Vector2; 4 ] = [
        Vector2 {x: x0, y: y1}, // P16
        Vector2 {x: x1, y: y1}, // P17
        Vector2 {x: x1, y: y2}, // P18
        Vector2 {x: x0, y: y2}  // P19
    ];

    let angles: [ f32; 4 ] = [ 180.0, 270.0, 0.0, 90.0 ];

    rlBegin(RL_LINES);
        // Draw all the 4 corners first: Upper Left Corner, Upper Right Corner, Lower Right Corner, Lower Left Corner
        for k in 0..4 // Hope the compiler is smart enough to unroll this loop
        {
            let mut angle: f32 = angles[(k as usize)];
            let center: Vector2 = centers[(k as usize)];

            for i in 0..segments
            {
                rlColor4ub(color.r, color.g, color.b, color.a);
                rlVertex2f(center.x + (DEG2RAD*angle).cos()*radius, center.y + (DEG2RAD*angle).sin()*radius);
                rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*radius, center.y + (DEG2RAD*(angle + stepLength)).sin()*radius);
                angle += stepLength;
            }
        }

        // And now the remaining 4 lines
        for i in (0..8).step_by(2)
        {
            rlColor4ub(color.r, color.g, color.b, color.a);
            rlVertex2f(point[(i as usize)].x, point[(i as usize)].y);
            rlVertex2f(point[((i + 1) as usize)].x, point[((i + 1) as usize)].y);
        }
    rlEnd();
}

// Draw rectangle with rounded edges outline with line thickness
pub unsafe fn DrawRectangleRoundedLinesEx(mut rec: Rectangle, mut roundness: f32, mut segments: i32, thick: f32, color: Color)
{
    // Not a rounded rectangle
    if roundness <= 0.0
    {
        DrawRectangleLinesEx(rec, thick, color);
        return;
    }

    if roundness >= 1.0 { roundness = 1.0; }

    let mut roundedOutlineThick: f32 = 0.0;
    let mut outerRadius: f32 = 0.0;
    let mut innerRadius: f32 = 0.0;
    if thick >= 0.0
    {
        // Calculate corner radius
        let radius = if (rec.width > rec.height) { (rec.height*roundness)/2.0 } else { (rec.width*roundness)/2.0 };
        if radius <= 0.0 { return; }

        outerRadius = radius;
        innerRadius = outerRadius - thick;

        // The maximum thickness the outline can have and still be rounded on the interior edge is equal to the corner radius
        // Put another way, when `innerRadius <= 0`, the interior of the outline is just a normal rectangle with no rounding
        if innerRadius <= 0.0
        {
            innerRadius = 0.0;
            roundedOutlineThick = outerRadius;

            // Draw the not-rounded portion of the outline
            DrawRectangleLinesEx(Rectangle { x: rec.x + outerRadius, y: rec.y + outerRadius, width: rec.width - outerRadius*2.0, height: rec.height - outerRadius*2.0 }, thick - outerRadius, color);
        }
        else
        {
            roundedOutlineThick = thick;
        }

        // Calculate number of segments to use for the corners
        if segments < 1
        {
            // Calculate the maximum angle between segments based on the error rate (usually 0.5f)
            let th: f32 = (2.0*(1.0 - SMOOTH_CIRCLE_ERROR_RATE/outerRadius).powf(2.0) - 1.0).acos();
            segments = (((2.0*PI/th)/4.0).ceil() as i32);
            if segments <= 0 { segments = 4; }
        }
    }
    else
    {
        // Calculate corner radius
        let radius = if (rec.width > rec.height) { (rec.height*roundness)/2.0 } else { (rec.width*roundness)/2.0 };
        if radius <= 0.0 { return; } // Only possible if the rectangle has 0 width or height

        // Expand the rectangle
        rec.x += thick;
        rec.y += thick;
        rec.width -= thick*2.0;
        rec.height -= thick*2.0;

        innerRadius = radius;
        outerRadius = innerRadius - thick;
        roundedOutlineThick = -thick;

        // Calculate number of segments to use for the corners
        if segments < 1
        {
            // Calculate the maximum angle between segments based on the error rate (usually 0.5f)
            let th: f32 = (2.0*(1.0 - SMOOTH_CIRCLE_ERROR_RATE/innerRadius).powf(2.0) - 1.0).acos();
            segments = (((2.0*PI/th)/4.0).ceil() as i32);
            if segments <= 0 { segments = 4; }
        }
    }

    let stepLength: f32 = 90.0/(segments as f32);

    /*
    Quick sketch to make sense of all of this,
    marks the 16 + 4 (corner centers P16-19) points used

           P0 ================== P1
          // P8                P9 \\
         //                        \\
     P7 // P15                  P10 \\ P2
       ||   *P16             P17*    ||
       ||                            ||
       || P14                   P11  ||
     P6 \\  *P19             P18*   // P3
         \\                        //
          \\ P13              P12 //
           P5 ================== P4
    */

    // The x-coordinates used for the outline
    let x0: f32 = rec.x + outerRadius;
    let x1: f32 = (rec.x + rec.width) - outerRadius;
    let x2: f32 = rec.x + rec.width;
    let x3: f32 = rec.x;
    let x4: f32 = rec.x + rec.width - roundedOutlineThick;
    let x5: f32 = rec.x + roundedOutlineThick;

    // The y-coordinates used for the outline
    let y0: f32 = rec.y;
    let y1: f32 = rec.y + outerRadius;
    let y2: f32 = (rec.y + rec.height) - outerRadius;
    let y3: f32 = rec.y + rec.height;
    let y4: f32 = rec.y + roundedOutlineThick;
    let y5: f32 = rec.y + rec.height - roundedOutlineThick;

    let point: [ Vector2; 16 ] = [
        Vector2 {x: x0, y: y0}, // P0
        Vector2 {x: x1, y: y0}, // P1
        Vector2 {x: x2, y: y1}, // P2
        Vector2 {x: x2, y: y2}, // P3
        Vector2 {x: x1, y: y3}, // P4
        Vector2 {x: x0, y: y3}, // P5
        Vector2 {x: x3, y: y2}, // P6
        Vector2 {x: x3, y: y1}, // P7
        Vector2 {x: x0, y: y4}, // P8
        Vector2 {x: x1, y: y4}, // P9
        Vector2 {x: x4, y: y1}, // P10
        Vector2 {x: x4, y: y2}, // P11
        Vector2 {x: x1, y: y5}, // P12
        Vector2 {x: x0, y: y5}, // P13
        Vector2 {x: x5, y: y2}, // P14
        Vector2 {x: x5, y: y1}  // P15
    ];

    let centers: [ Vector2; 4 ] = [
        Vector2 {x: x0, y: y1}, // P16
        Vector2 {x: x1, y: y1}, // P17
        Vector2 {x: x1, y: y2}, // P18
        Vector2 {x: x0, y: y2}  // P19
    ];

    let angles: [ f32; 4 ] = [ 180.0, 270.0, 0.0, 90.0 ];

    rlSetTexture(GetShapesTexture().id);
    let shapeRect: Rectangle = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);

        // Draw all the 4 corners first: Upper Left Corner, Upper Right Corner, Lower Right Corner, Lower Left Corner
        for k in 0..4 // Hope the compiler is smart enough to unroll this loop
        {
            let mut angle: f32 = angles[(k as usize)];
            let center: Vector2 = centers[(k as usize)];
            for i in 0..segments
            {
                rlColor4ub(color.r, color.g, color.b, color.a);

                rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                rlVertex2f(center.x + (DEG2RAD*angle).cos()*innerRadius, center.y + (DEG2RAD*angle).sin()*innerRadius);

                rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*innerRadius, center.y + (DEG2RAD*(angle + stepLength)).sin()*innerRadius);

                rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*outerRadius, center.y + (DEG2RAD*(angle + stepLength)).sin()*outerRadius);

                rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                rlVertex2f(center.x + (DEG2RAD*angle).cos()*outerRadius, center.y + (DEG2RAD*angle).sin()*outerRadius);

                angle += stepLength;
            }
        }

        // Upper rectangle
        rlColor4ub(color.r, color.g, color.b, color.a);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[0].x, point[0].y);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[8].x, point[8].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[9].x, point[9].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[1].x, point[1].y);

        // Right rectangle
        rlColor4ub(color.r, color.g, color.b, color.a);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[2].x, point[2].y);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[10].x, point[10].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[11].x, point[11].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[3].x, point[3].y);

        // Lower rectangle
        rlColor4ub(color.r, color.g, color.b, color.a);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[13].x, point[13].y);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[5].x, point[5].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[4].x, point[4].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[12].x, point[12].y);

        // Left rectangle
        rlColor4ub(color.r, color.g, color.b, color.a);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[15].x, point[15].y);
        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[7].x, point[7].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
        rlVertex2f(point[6].x, point[6].y);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
        rlVertex2f(point[14].x, point[14].y);

    rlEnd();
    rlSetTexture(0);
}

// Draw a polygon of n sides
pub unsafe fn DrawPoly(center: Vector2, mut sides: i32, radius: f32, rotation: f32, color: Color)
{
    if sides < 3 { sides = 3; }
    let mut centralAngle: f32 = rotation*DEG2RAD;
    let angleStep: f32 = 360.0/(sides as f32)*DEG2RAD;

    rlSetTexture(GetShapesTexture().id);
    let shapeRect: Rectangle = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);
        for i in 0..sides
        {
            rlColor4ub(color.r, color.g, color.b, color.a);
            let nextAngle: f32 = centralAngle + angleStep;

            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x, center.y);

            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (centralAngle).cos()*radius, center.y + (centralAngle).sin()*radius);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x + (nextAngle).cos()*radius, center.y + (nextAngle).sin()*radius);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (centralAngle).cos()*radius, center.y + (centralAngle).sin()*radius);

            centralAngle = nextAngle;
        }
    rlEnd();
    rlSetTexture(0);
}

// Draw a polygon outline of n sides
pub unsafe fn DrawPolyLines(center: Vector2, mut sides: i32, radius: f32, rotation: f32, color: Color)
{
    if sides < 3 { sides = 3; }
    let mut centralAngle: f32 = rotation*DEG2RAD;
    let angleStep: f32 = 360.0/(sides as f32)*DEG2RAD;

    rlBegin(RL_LINES);
        for i in 0..sides
        {
            rlColor4ub(color.r, color.g, color.b, color.a);

            rlVertex2f(center.x + (centralAngle).cos()*radius, center.y + (centralAngle).sin()*radius);
            rlVertex2f(center.x + (centralAngle + angleStep).cos()*radius, center.y + (centralAngle + angleStep).sin()*radius);

            centralAngle += angleStep;
        }
    rlEnd();
}

pub unsafe fn DrawPolyLinesEx(center: Vector2, mut sides: i32, radius: f32, rotation: f32, mut thick: f32, color: Color)
{
    if sides < 3 { sides = 3; }
    let mut centralAngle: f32 = rotation*DEG2RAD;
    let exteriorAngle: f32 = 360.0/(sides as f32)*DEG2RAD;
    let apothem: f32 = radius*(DEG2RAD*180.0/(sides as f32)).cos();

    let mut outerRadius: f32 = 0.0;
    let mut innerRadius: f32 = 0.0;
    if thick >= 0.0
    {
        outerRadius = radius;
        innerRadius = (0.0f32).max(radius - thick*(radius/apothem));
    }
    else
    {
        thick *= -1.0;
        outerRadius = radius + thick*(radius/apothem);
        innerRadius = radius;
    }

    rlSetTexture(GetShapesTexture().id);
    let shapeRect: Rectangle = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);
        for i in 0..sides
        {
            rlColor4ub(color.r, color.g, color.b, color.a);
            let nextAngle: f32 = centralAngle + exteriorAngle;

            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (centralAngle).cos()*outerRadius, center.y + (centralAngle).sin()*outerRadius);

            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x + (centralAngle).cos()*innerRadius, center.y + (centralAngle).sin()*innerRadius);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (nextAngle).cos()*innerRadius, center.y + (nextAngle).sin()*innerRadius);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x + (nextAngle).cos()*outerRadius, center.y + (nextAngle).sin()*outerRadius);

            centralAngle = nextAngle;
        }
    rlEnd();
    rlSetTexture(0);
}

// Draw a color-filled circle
pub unsafe fn DrawCircle(centerX: i32, centerY: i32, radius: f32, color: Color)
{
    DrawCircleV(Vector2 { x: (centerX as f32), y: (centerY as f32) }, radius, color);
}

// Draw a color-filled circle (Vector version)
// NOTE: On OpenGL 3.3 and ES2 using QUADS to avoid drawing order issues
pub unsafe fn DrawCircleV(center: Vector2, radius: f32, color: Color)
{
    DrawCircleSector(center, radius, 0.0, 360.0, 36, color);
}

// Draw a gradient-filled circle
pub unsafe fn DrawCircleGradient(center: Vector2, radius: f32, inner: Color, outer: Color)
{
    rlBegin(RL_TRIANGLES);
        for i in (0..360).step_by(10)
        {
            rlColor4ub(inner.r, inner.g, inner.b, inner.a);
            rlVertex2f(center.x, center.y);
            rlColor4ub(outer.r, outer.g, outer.b, outer.a);
            rlVertex2f(center.x + (DEG2RAD*((i + 10) as f32)).cos()*radius, center.y + (DEG2RAD*((i + 10) as f32)).sin()*radius);
            rlColor4ub(outer.r, outer.g, outer.b, outer.a);
            rlVertex2f(center.x + (DEG2RAD*(i as f32)).cos()*radius, center.y + (DEG2RAD*(i as f32)).sin()*radius);
        }
    rlEnd();
}

// Draw a piece of a circle
pub unsafe fn DrawCircleSector(center: Vector2, mut radius: f32, mut startAngle: f32, mut endAngle: f32, mut segments: i32, color: Color)
{
    if startAngle == endAngle { return; }
    if radius <= 0.0 { return; }  // Avoid div by zero

    // Function expects (endAngle > startAngle)
    if endAngle < startAngle
    {
        // Swap values
        std::mem::swap(&mut startAngle, &mut endAngle);
    }

    // Drawing a whole circle, things get weird without limiting the circle to 360 degrees
    if endAngle - startAngle >= 360.0 { endAngle = startAngle + 360.0; }

    let minSegments: i32 = (((endAngle - startAngle)/90.0).ceil() as i32);

    if segments < minSegments
    {
        // Calculate the maximum angle between segments based on the error rate (usually 0.5f)
        let th: f32 = (2.0*(1.0 - SMOOTH_CIRCLE_ERROR_RATE/radius).powf(2.0) - 1.0).acos();
        segments = (((endAngle - startAngle)*(2.0*PI/th)/360.0).ceil() as i32);

        if segments <= 0 { segments = minSegments; }
    }

    let stepLength: f32 = (endAngle - startAngle)/(segments as f32);
    let mut angle: f32 = startAngle;

    rlSetTexture(GetShapesTexture().id);
    let shapeRect: Rectangle = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);

        // NOTE: Every QUAD actually represents two segments
        for i in 0..segments/2
        {
            rlColor4ub(color.r, color.g, color.b, color.a);

            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x, center.y);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength*2.0)).cos()*radius, center.y + (DEG2RAD*(angle + stepLength*2.0)).sin()*radius);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*radius, center.y + (DEG2RAD*(angle + stepLength)).sin()*radius);

            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*radius, center.y + (DEG2RAD*angle).sin()*radius);

            angle += (stepLength*2.0);
        }

        // NOTE: In case number of segments is odd, adding one last piece to the cake
        if (((segments as u32))%2) == 1
        {
            rlColor4ub(color.r, color.g, color.b, color.a);

            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x, center.y);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*radius, center.y + (DEG2RAD*(angle + stepLength)).sin()*radius);

            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*radius, center.y + (DEG2RAD*angle).sin()*radius);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x, center.y);
        }

    rlEnd();

    rlSetTexture(0);
}

// Draw a piece of a circle outlines
pub unsafe fn DrawCircleSectorLines(center: Vector2, mut radius: f32, mut startAngle: f32, mut endAngle: f32, mut segments: i32, color: Color)
{
    if startAngle == endAngle { return; }
    if radius <= 0.0 { return; }  // Avoid div by zero issue

    // Function expects (endAngle > startAngle)
    if endAngle < startAngle
    {
        // Swap values
        std::mem::swap(&mut startAngle, &mut endAngle);
    }

    let mut showCapLines: bool = true;
    // Drawing a whole circle, things get weird without limiting the circle to 360 degrees
    if endAngle - startAngle >= 360.0
    {
        showCapLines = false;
        endAngle = startAngle + 360.0;
    }

    let minSegments: i32 = (((endAngle - startAngle)/90.0).ceil() as i32);

    if segments < minSegments
    {
        // Calculate the maximum angle between segments based on the error rate (usually 0.5f)
        let th: f32 = (2.0*(1.0 - SMOOTH_CIRCLE_ERROR_RATE/radius).powf(2.0) - 1.0).acos();
        segments = (((endAngle - startAngle)*(2.0*PI/th)/360.0).ceil() as i32);

        if segments <= 0 { segments = minSegments; }
    }

    let stepLength: f32 = (endAngle - startAngle)/(segments as f32);
    let mut angle: f32 = startAngle;

    rlBegin(RL_LINES);
        if showCapLines
        {
            rlColor4ub(color.r, color.g, color.b, color.a);
            rlVertex2f(center.x, center.y);
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*radius, center.y + (DEG2RAD*angle).sin()*radius);
        }

        for i in 0..segments
        {
            rlColor4ub(color.r, color.g, color.b, color.a);

            rlVertex2f(center.x + (DEG2RAD*angle).cos()*radius, center.y + (DEG2RAD*angle).sin()*radius);
            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*radius, center.y + (DEG2RAD*(angle + stepLength)).sin()*radius);

            angle += stepLength;
        }

        if showCapLines
        {
            rlColor4ub(color.r, color.g, color.b, color.a);
            rlVertex2f(center.x, center.y);
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*radius, center.y + (DEG2RAD*angle).sin()*radius);
        }
    rlEnd();
}

// Draw a piece of a circle outlines with thickness
pub unsafe fn DrawCircleSectorLinesEx(center: Vector2, mut radius: f32, mut startAngle: f32, mut endAngle: f32, mut segments: i32, thick: f32, color: Color)
{
    if startAngle == endAngle { return; }
    if radius <= 0.0 { return; }  // Avoid div by zero issue

    // Function expects (endAngle > startAngle)
    if endAngle < startAngle
    {
        // Swap values
        std::mem::swap(&mut startAngle, &mut endAngle);
    }

    let mut showCapLines: bool = true;
    // Drawing a whole circle, things get weird without limiting the circle to 360 degrees
    if endAngle - startAngle >= 360.0
    {
        showCapLines = thick >= 0.0;
        endAngle = startAngle + 360.0;
    }

    let minSegments: i32 = (((endAngle - startAngle)/90.0).ceil() as i32);

    if segments < minSegments
    {
        // Calculate the maximum angle between segments based on the error rate (usually 0.5f)
        let th: f32 = (2.0*(1.0 - SMOOTH_CIRCLE_ERROR_RATE/radius).powf(2.0) - 1.0).acos();
        segments = (((endAngle - startAngle)*(2.0*PI/th)/360.0).ceil() as i32);

        if segments <= 0 { segments = minSegments; }
    }

    let stepLength: f32 = (endAngle - startAngle)/(segments as f32);
    let mut angle: f32 = startAngle;

    /*
    A sketch to help make things clearer

    NOTE: Some considerations are different when `thick` is negative
          The vertices used here are still relevant, but would instead be outside of the circle
          S0 is always the center

    The circle sector outline is drawn in 3 main pieces, the circle outline, cap 1, and cap 2
    The circle outline is self explanatory
    Cap 1 covers the `startAngle` edge and cap 2 covers the `endAngle` edge
    S0 is the first shared point between the caps, and also the circle's center
    S1 is the second shared point (sometimes not shared) between the caps
      S1 is also C0 and C3 in this sketch. In certain cases, S1 goes outside of
      the circle and C0 and C3 become different points
    C1 is one of cap 1's vertices that is on the inside edge of the circle outline
    C2 is like C1, but is also on the `startAngle` edge
    C4 is cap 2's vertex that corresponds with C1
    C5 is cap 2's vertex that corresponds with C2, except on the `endAngle` edge

                   [][][][][]
               [][]        []
             []      []C4[]C5
           []    [][]  {}  {}
         []    []      {}  {}
       []    []        {}C {} <- endAngle
       []  []          {}a {}
     []    []          {}p {}
     []  []            {}2 {}
     []  []            {}  {}
     []  []            {}  {}     startAngle
     []  []            {}  S0{}{}{}{}{}{}{}{}C2[][]
     []  []            {}{}       Cap1       []  []
     []  []            S1{}{}{}{}{}{}{}{}{}{}C1  []
     []  []                                  []  []
     []    []                              []    []
       []  []       Not filled in          []  []
       []    []                          []    []
         []    []                      []    []
           []    [][]              [][]    []
             []      [][][][][][][]      []
               [][]  Circle outline  [][]
                   [][][][][][][][][]

    [] = Circle outline edge pixel
    {} = Cap outline edge pixel
    */

    // We are not drawing a circle, we are drawing an n-sided polygon
    // So, we need to adjust the outline thickness of the "circle" for it to look correct with fewer segments
    let apothem: f32 = radius*(DEG2RAD*((endAngle - startAngle)/2.0)/(segments as f32)).cos();
    let radiusThick: f32 = thick*(radius/apothem);

    let mut outerRadius: f32 = radius;
    let mut innerRadius: f32 = radius - radiusThick;

    if thick >= 0.0
    {
        if thick >= innerRadius
        {
            DrawCircleSector(center, radius, startAngle, endAngle, segments, color);
            return;
        }
    }
    else
    {
        std::mem::swap(&mut outerRadius, &mut innerRadius);
    }

    // Cap 1 vertices
    let mut c0: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut c1: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut c2: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    // Cap 2 vertices
    let mut c3: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut c4: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut c5: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    // The number of angle steps that come before C1 (from `startAngle`, counter clockwise)
    let mut stepsBeforeC1: i32 = 0;
    let mut s1OutsideOfCircle: bool = false;
    // The number of angle steps that come before C0 (from `startAngle`, counter clockwise)
    // Only used if S1 is outside of the circle
    let mut stepsBeforeC0: i32 = 0;

    if showCapLines
    {
        if thick >= 0.0
        {
            c2 = Vector2 { x: center.x + (DEG2RAD*startAngle).cos()*innerRadius, y: center.y + (DEG2RAD*startAngle).sin()*innerRadius };
            c5 = Vector2 { x: center.x + (DEG2RAD*endAngle).cos()*innerRadius, y: center.y + (DEG2RAD*endAngle).sin()*innerRadius };

            // For C1 and C4, we need to find the point that lies on the circle (n-sided polygon, actually)
            // We want C1 and C4 to be `thick` pixels perpendicularly from the `startAngle` and `endAngle` edges
            // and to be on the `innerRadius` edge

            let c1Angle: f32 = RAD2DEG*(thick/innerRadius).asin();

            // There are more segments before C1 than there are segments being drawn,
            // so the whole circle sector must be covered
            if c1Angle/stepLength >= (segments as f32)
            {
                DrawCircleSector(center, radius, startAngle, endAngle, segments, color);
                return;
            }

            // Do this after the previous check just in case `stepLength` is really small and
            // dividing by it produces a very large number
            stepsBeforeC1 = ((c1Angle/stepLength) as i32);

            // The angles of the vertices on the circle outline before and after C1
            let vertexAngleBeforeC1: f32 = stepLength*(stepsBeforeC1 as f32);
            let vertexAngleAfterC1: f32 = stepLength*(((stepsBeforeC1 + 1) as f32));

            /*
            Here is another sketch

            We know which outline line segment C1 is on (`vertexAngleBeforeC1` and `vertexAngleAfterC1`)
            Now we just need to know where on that line segment C1 is

            We can change our frame of reference so that `startAngle` is 0 degrees and `center` is at the origin (0, 0)
            This makes the math much simpler because now we can just go straight down by `thick` pixels and
            use the horizontal line that passes through that point to determine where C1 is on our line segment

            The line segment is defined by p1 and p2, we need C1, which is on that edge
            The 'y' axis of C1 is equal to `thick` (within this modified frame of reference)

                         p1
                         /|
                        / |
                       /  |
                      /   |
                     /    |
                    /     |
                   /      |
                  /       |
                C1---------  <-- y axis = `thick`
                /
               /
             p2
            */

            let mut p1: Vector2 = Vector2 { x: (DEG2RAD*vertexAngleBeforeC1).cos()*innerRadius, y: (DEG2RAD*vertexAngleBeforeC1).sin()*innerRadius };
            let mut p2: Vector2 = Vector2 { x: (DEG2RAD*vertexAngleAfterC1).cos()*innerRadius, y: (DEG2RAD*vertexAngleAfterC1).sin()*innerRadius };

            // Find the `t` of C1 between p1 and p2 ('t' as in `Lerp(start, end, t)`)
            // This is used to lerp between the actual vertices (outside of our modified frame of reference)
            // before and after C1
            let mut t: f32 = (p1.y - thick)/(p1.y - p2.y);

            let mut vertexBeforeCap1Vertex: Vector2 = Vector2 { x: center.x + (DEG2RAD*(startAngle + vertexAngleBeforeC1)).cos()*innerRadius, y: center.y + (DEG2RAD*(startAngle + vertexAngleBeforeC1)).sin()*innerRadius };
            let mut vertexAfterCap1Vertex: Vector2 = Vector2 { x: center.x + (DEG2RAD*(startAngle + vertexAngleAfterC1)).cos()*innerRadius, y: center.y + (DEG2RAD*(startAngle + vertexAngleAfterC1)).sin()*innerRadius };

            c1.x = vertexBeforeCap1Vertex.x + (vertexAfterCap1Vertex.x - vertexBeforeCap1Vertex.x)*t;
            c1.y = vertexBeforeCap1Vertex.y + (vertexAfterCap1Vertex.y - vertexBeforeCap1Vertex.y)*t;

            let mut vertexBeforeCap2Vertex: Vector2 = Vector2 { x: center.x + (DEG2RAD*(endAngle - vertexAngleBeforeC1)).cos()*innerRadius, y: center.y + (DEG2RAD*(endAngle - vertexAngleBeforeC1)).sin()*innerRadius };
            let mut vertexAfterCap2Vertex: Vector2 = Vector2 { x: center.x + (DEG2RAD*(endAngle - vertexAngleAfterC1)).cos()*innerRadius, y: center.y + (DEG2RAD*(endAngle - vertexAngleAfterC1)).sin()*innerRadius };

            c4.x = vertexBeforeCap2Vertex.x + (vertexAfterCap2Vertex.x - vertexBeforeCap2Vertex.x)*t;
            c4.y = vertexBeforeCap2Vertex.y + (vertexAfterCap2Vertex.y - vertexBeforeCap2Vertex.y)*t;

            /*
            Another sketch couldn't hurt

            This is a "zoomed in" view of the center of the circle sector
            You can see where S0 is, and where we want S1 to be
            `innerAngleBetweenCapEnds` is the angle of the diagonal line ('//') between the cap ends
            `S1Length` is the length of that line

            Since the caps are always parallel to `startAngle` and `endAngle`,
            we always have a right triangle we can use to determine where S1 is

             []      []
             []      []
             []  C   [] <- endAngle
             []  a   []
             []  p   []
             []  2   []  startAngle
             [][][][]S0[][][][][][]
                   //[]
                 //  [] Cap 1
               //    []
             S1      [][][][][][][]
            */

            let innerAngleBetweenCapEnds: f32 = ((endAngle - 90.0) - (startAngle + 90.0))/2.0;
            let s1Length: f32 = thick/(DEG2RAD*innerAngleBetweenCapEnds).cos();

            // As `startAngle` and `endAngle` draw more of a circle, S1 goes further out from the center
            // It can go so far that it is outside of the circle, by a lot
            // This case needs to be detected and handled
            // If S1 is within the circle, nothing special needs to happen
            // But, if S1 is outside of the circle, we need to find the two points (C0 and C3) where
            // the line segments C0->C1 and C0->C3 intersect the circle outline,
            // using the same method we used to find C1 and C4
            if (innerAngleBetweenCapEnds < 90.0) && (s1Length <= innerRadius)
            {
                // S1 is inside of the circle

                let betweenStartAndEndAngle: f32 = (endAngle + startAngle)/2.0;
                c0 = Vector2 { x: center.x + (DEG2RAD*betweenStartAndEndAngle).cos()*s1Length, y: center.y + (DEG2RAD*betweenStartAndEndAngle).sin()*s1Length };
                c3 = c0;

                p1 = Vector2 { x: c1.x - center.x, y: c1.y - center.y };
                p2 = Vector2 { x: c0.x - center.x, y: c0.y - center.y };

                // Copied from "raymath.h" Vector2Angle()
                let dot: f32 = p1.x*p2.x + p1.y*p2.y;
                let det: f32 = p1.x*p2.y - p1.y*p2.x;
                let c1ToS1Angle: f32 = (det).atan2(dot);

                // If C1 and C4 are on the wrong side of S1, the whole circle sector is covered
                if c1ToS1Angle < 0.0
                {
                    DrawCircleSector(center, radius, startAngle, endAngle, segments, color);
                    return;
                }
            }
            else
            {
                // S1 is outside of the circle

                if endAngle - startAngle <= 180.0
                {
                    DrawCircleSector(center, radius, startAngle, endAngle, segments, color);
                    return;
                }

                s1OutsideOfCircle = true;

                stepsBeforeC0 = (((180.0 + RAD2DEG*(thick/-innerRadius).asin())/stepLength) as i32);

                // Reuse the code for finding C1 and C4 to find C0 and C3

                let vertexAngleBeforeC0: f32 = stepLength*(stepsBeforeC0 as f32);
                let vertexAngleAfterC0: f32 = stepLength*(((stepsBeforeC0 + 1) as f32));

                p1 = Vector2 { x: (DEG2RAD*vertexAngleBeforeC0).cos()*innerRadius, y: (DEG2RAD*vertexAngleBeforeC0).sin()*innerRadius };
                p2 = Vector2 { x: (DEG2RAD*vertexAngleAfterC0).cos()*innerRadius, y: (DEG2RAD*vertexAngleAfterC0).sin()*innerRadius };

                t = (p1.y - thick)/(p1.y - p2.y);

                vertexBeforeCap1Vertex = Vector2 { x: center.x + (DEG2RAD*(startAngle + vertexAngleBeforeC0)).cos()*innerRadius, y: center.y + (DEG2RAD*(startAngle + vertexAngleBeforeC0)).sin()*innerRadius };
                vertexAfterCap1Vertex = Vector2 { x: center.x + (DEG2RAD*(startAngle + vertexAngleAfterC0)).cos()*innerRadius, y: center.y + (DEG2RAD*(startAngle + vertexAngleAfterC0)).sin()*innerRadius };

                c0.x = vertexBeforeCap1Vertex.x + (vertexAfterCap1Vertex.x - vertexBeforeCap1Vertex.x)*t;
                c0.y = vertexBeforeCap1Vertex.y + (vertexAfterCap1Vertex.y - vertexBeforeCap1Vertex.y)*t;

                vertexBeforeCap2Vertex = Vector2 { x: center.x + (DEG2RAD*(endAngle - vertexAngleBeforeC0)).cos()*innerRadius, y: center.y + (DEG2RAD*(endAngle - vertexAngleBeforeC0)).sin()*innerRadius };
                vertexAfterCap2Vertex = Vector2 { x: center.x + (DEG2RAD*(endAngle - vertexAngleAfterC0)).cos()*innerRadius, y: center.y + (DEG2RAD*(endAngle - vertexAngleAfterC0)).sin()*innerRadius };

                c3.x = vertexBeforeCap2Vertex.x + (vertexAfterCap2Vertex.x - vertexBeforeCap2Vertex.x)*t;
                c3.y = vertexBeforeCap2Vertex.y + (vertexAfterCap2Vertex.y - vertexBeforeCap2Vertex.y)*t;
            }
        }
        else
        {
            let outerAngleBetweenCapEnds: f32 = ((endAngle + 90.0) - (startAngle - 90.0))/2.0;
            let s1Length: f32 = thick/(DEG2RAD*outerAngleBetweenCapEnds).cos();
            let betweenStartAndEndAngle: f32 = 180.0 + (endAngle + startAngle)/2.0;
            c0 = Vector2 { x: center.x + (DEG2RAD*betweenStartAndEndAngle).cos()*s1Length, y: center.y + (DEG2RAD*betweenStartAndEndAngle).sin()*s1Length };
            c3 = c0;

            c2 = Vector2 { x: center.x + (DEG2RAD*startAngle).cos()*outerRadius, y: center.y + (DEG2RAD*startAngle).sin()*outerRadius };
            c5 = Vector2 { x: center.x + (DEG2RAD*endAngle).cos()*outerRadius, y: center.y + (DEG2RAD*endAngle).sin()*outerRadius };

            // Change the frame of reference so that `center` is the origin and `startAngle` is 0 degrees

            let mut c0Translated: Vector2 = Vector2 { x: c0.x - center.x, y: c0.y - center.y };
            let mut circleVertex1: Vector2 = Vector2 { x: c2.x - center.x, y: c2.y - center.y };
            let mut circleVertex2: Vector2 = Vector2 { x: (DEG2RAD*(startAngle + stepLength)).cos()*outerRadius, y: (DEG2RAD*(startAngle + stepLength)).sin()*outerRadius };

            // Copied from "raymath.h" Vector2Rotate()
            let mut tempX: f32 = c0Translated.x;
            c0Translated.x = (-DEG2RAD*startAngle).cos()*tempX - (-DEG2RAD*startAngle).sin()*c0Translated.y;
            c0Translated.y = (-DEG2RAD*startAngle).sin()*tempX + (-DEG2RAD*startAngle).cos()*c0Translated.y;

            tempX = circleVertex1.x;
            circleVertex1.x = (-DEG2RAD*startAngle).cos()*tempX - (-DEG2RAD*startAngle).sin()*circleVertex1.y;
            circleVertex1.y = (-DEG2RAD*startAngle).sin()*tempX + (-DEG2RAD*startAngle).cos()*circleVertex1.y;

            tempX = circleVertex2.x;
            circleVertex2.x = (-DEG2RAD*startAngle).cos()*tempX - (-DEG2RAD*startAngle).sin()*circleVertex2.y;
            circleVertex2.y = (-DEG2RAD*startAngle).sin()*tempX + (-DEG2RAD*startAngle).cos()*circleVertex2.y;

            // Figure out the line that `circleVertex1` and `circleVertex2` are on
            let mut rise: f32 = circleVertex1.y - circleVertex2.y;
            let mut run: f32 = circleVertex1.x - circleVertex2.x;
            // Get where that line intersects the horizontal line that `c0Translated` is on
            let c1Rise: f32 = c0Translated.y - circleVertex1.y;
            let c1Run: f32 = (c1Rise/rise)*run;
            let c1DistanceFromC0: f32 = (circleVertex1.x + c1Run) - c0Translated.x;

            c1 = Vector2 { x: c0.x + (DEG2RAD*startAngle).cos()*c1DistanceFromC0, y: c0.y + (DEG2RAD*startAngle).sin()*c1DistanceFromC0 };
            c4 = Vector2 { x: c0.x + (DEG2RAD*endAngle).cos()*c1DistanceFromC0, y: c0.y + (DEG2RAD*endAngle).sin()*c1DistanceFromC0 };

            if c1DistanceFromC0 < 0.0
            {
                // The caps are intersecting each other

                let mut circleVertex3: Vector2 = Vector2 { x: c5.x - center.x, y: c5.y - center.y };
                let mut circleVertex4: Vector2 = Vector2 { x: (DEG2RAD*(endAngle - stepLength)).cos()*outerRadius, y: (DEG2RAD*(endAngle - stepLength)).sin()*outerRadius };

                tempX = circleVertex3.x;
                circleVertex3.x = (-DEG2RAD*startAngle).cos()*tempX - (-DEG2RAD*startAngle).sin()*circleVertex3.y;
                circleVertex3.y = (-DEG2RAD*startAngle).sin()*tempX + (-DEG2RAD*startAngle).cos()*circleVertex3.y;

                tempX = circleVertex4.x;
                circleVertex4.x = (-DEG2RAD*startAngle).cos()*tempX - (-DEG2RAD*startAngle).sin()*circleVertex4.y;
                circleVertex4.y = (-DEG2RAD*startAngle).sin()*tempX + (-DEG2RAD*startAngle).cos()*circleVertex4.y;

                // `startAngle` is 0 degrees within this frame of reference,
                // so C1 just goes horizontally out from C0
                let mut c1Translated: Vector2 = Vector2 { x: c0Translated.x + c1DistanceFromC0, y: c0Translated.y };

                // Make `circleVertex2` the origin
                circleVertex1.x -= circleVertex2.x;
                circleVertex1.y -= circleVertex2.y;
                circleVertex3.x -= circleVertex2.x;
                circleVertex3.y -= circleVertex2.y;
                circleVertex4.x -= circleVertex2.x;
                circleVertex4.y -= circleVertex2.y;
                c1Translated.x -= circleVertex2.x;
                c1Translated.y -= circleVertex2.y;

                // Make the line between `circleVertex1` and `circleVertex2` a horizontal line
                let theta: f32 = (circleVertex1.y).atan2(circleVertex1.x);

                // Copied from "raymath.h" Vector2Rotate()
                tempX = circleVertex1.x;
                circleVertex1.x = (-theta).cos()*tempX - (-theta).sin()*circleVertex1.y;
                circleVertex1.y = (-theta).sin()*tempX + (-theta).cos()*circleVertex1.y;

                tempX = circleVertex3.x;
                circleVertex3.x = (-theta).cos()*tempX - (-theta).sin()*circleVertex3.y;
                circleVertex3.y = (-theta).sin()*tempX + (-theta).cos()*circleVertex3.y;

                tempX = circleVertex4.x;
                circleVertex4.x = (-theta).cos()*tempX - (-theta).sin()*circleVertex4.y;
                circleVertex4.y = (-theta).sin()*tempX + (-theta).cos()*circleVertex4.y;

                tempX = c1Translated.x;
                c1Translated.x = (-theta).cos()*tempX - (-theta).sin()*c1Translated.y;
                c1Translated.y = (-theta).sin()*tempX + (-theta).cos()*c1Translated.y;

                // Find where the line that `circleVertex3` and `circleVertex4` are on would intersect the
                // line segment defined by `circleVertex1` and `c1Translated`
                rise = circleVertex3.y - circleVertex4.y;
                run = circleVertex3.x - circleVertex4.x;
                let targetRise: f32 = -circleVertex3.y;
                let targetX: f32 = circleVertex3.x + (targetRise/rise)*run;

                let t: f32 = (c1Translated.x - targetX)/(c1Translated.x - circleVertex1.x);

                c1 = Vector2 { x: c1.x + (c2.x - c1.x)*t, y: c1.y + (c2.y - c1.y)*t };
                c4 = c1;
                c0 = c1;
                c3 = c1;
            }

            // Swap vertices to correct the winding order
            std::mem::swap(&mut c0, &mut c2);

            std::mem::swap(&mut c3, &mut c5);
        }
    }

    rlSetTexture(GetShapesTexture().id);
    let shapeRect: Rectangle = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);

        rlColor4ub(color.r, color.g, color.b, color.a);

        // Draw the circle outline
        for i in 0..segments
        {
            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*outerRadius, center.y + (DEG2RAD*angle).sin()*outerRadius);

            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*innerRadius, center.y + (DEG2RAD*angle).sin()*innerRadius);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*innerRadius, center.y + (DEG2RAD*(angle + stepLength)).sin()*innerRadius);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*outerRadius, center.y + (DEG2RAD*(angle + stepLength)).sin()*outerRadius);

            angle += stepLength;
        }

        // Draw the caps
        if showCapLines
        {
            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x, center.y);

            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(c0.x, c0.y);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(c1.x, c1.y);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(c2.x, c2.y);


            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x, center.y);

            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(c5.x, c5.y);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(c4.x, c4.y);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(c3.x, c3.y);

            // Some extra work may be needed when `thick` is positive
            if thick >= 0.0
            {
                // Fill in the gaps between cap 1 and the circle outline and cap 2 and the circle outline
                if stepsBeforeC1 > 0
                {
                    // Draw quads using pairs of vertices on the circle outline
                    angle = 0.0;
                    for i in 0..stepsBeforeC1/2
                    {
                        // Cap1
                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(c1.x, c1.y);

                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(center.x + (DEG2RAD*(startAngle + (angle + stepLength*2.0))).cos()*innerRadius, center.y + (DEG2RAD*(startAngle + (angle + stepLength*2.0))).sin()*innerRadius);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(center.x + (DEG2RAD*(startAngle + (angle + stepLength))).cos()*innerRadius, center.y + (DEG2RAD*(startAngle + (angle + stepLength))).sin()*innerRadius);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(center.x + (DEG2RAD*(startAngle + angle)).cos()*innerRadius, center.y + (DEG2RAD*(startAngle + angle)).sin()*innerRadius);

                        // Cap2
                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(c4.x, c4.y);

                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(center.x + (DEG2RAD*(endAngle - angle)).cos()*innerRadius, center.y + (DEG2RAD*(endAngle - angle)).sin()*innerRadius);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(center.x + (DEG2RAD*(endAngle - (angle + stepLength))).cos()*innerRadius, center.y + (DEG2RAD*(endAngle - (angle + stepLength))).sin()*innerRadius);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(center.x + (DEG2RAD*(endAngle - (angle + stepLength*2.0))).cos()*innerRadius, center.y + (DEG2RAD*(endAngle - (angle + stepLength*2.0))).sin()*innerRadius);

                        angle += stepLength*2.0;
                    }

                    if stepsBeforeC1%2 == 1
                    {
                        // Cap1
                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(c1.x, c1.y);

                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(c1.x, c1.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(center.x + (DEG2RAD*(startAngle + (angle + stepLength))).cos()*innerRadius, center.y + (DEG2RAD*(startAngle + (angle + stepLength))).sin()*innerRadius);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(center.x + (DEG2RAD*(startAngle + angle)).cos()*innerRadius, center.y + (DEG2RAD*(startAngle + angle)).sin()*innerRadius);

                        // Cap2
                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(c4.x, c4.y);

                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(c4.x, c4.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(center.x + (DEG2RAD*(endAngle - angle)).cos()*innerRadius, center.y + (DEG2RAD*(endAngle - angle)).sin()*innerRadius);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(center.x + (DEG2RAD*(endAngle - (angle + stepLength))).cos()*innerRadius, center.y + (DEG2RAD*(endAngle - (angle + stepLength))).sin()*innerRadius);
                    }
                }

                // Fill in the gap between C0, C3 and the circle outline
                if s1OutsideOfCircle
                {
                    let mut verticesBetweenC0andC3: i32 = (segments - stepsBeforeC0*2) - 1;

                    // No gap to fill
                    if verticesBetweenC0andC3 == 0
                    {
                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(center.x, center.y);

                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(center.x, center.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(c3.x, c3.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(c0.x, c0.y);
                    }
                    // There's a gap to fill
                    else
                    {
                        // Triangle touching C0
                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(center.x, center.y);

                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(center.x, center.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(center.x + (DEG2RAD*(startAngle + stepLength*((stepsBeforeC0 + 1) as f32))).cos()*innerRadius, center.y + (DEG2RAD*(startAngle + stepLength*((stepsBeforeC0 + 1) as f32))).sin()*innerRadius);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(c0.x, c0.y);

                        // Triangle touching C3
                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(center.x, center.y);

                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(center.x, center.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(c3.x, c3.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(center.x + (DEG2RAD*(endAngle - stepLength*((stepsBeforeC0 + 1) as f32))).cos()*innerRadius, center.y + (DEG2RAD*(endAngle - stepLength*((stepsBeforeC0 + 1) as f32))).sin()*innerRadius);

                        // Triangles between the previous two
                        verticesBetweenC0andC3 -= 1;
                        angle = startAngle + stepLength*((stepsBeforeC0 + 1) as f32);
                        for i in 0..verticesBetweenC0andC3/2
                        {
                            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                            rlVertex2f(center.x, center.y);

                            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength*2.0)).cos()*innerRadius, center.y + (DEG2RAD*(angle + stepLength*2.0)).sin()*innerRadius);

                            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*innerRadius, center.y + (DEG2RAD*(angle + stepLength)).sin()*innerRadius);

                            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                            rlVertex2f(center.x + (DEG2RAD*angle).cos()*innerRadius, center.y + (DEG2RAD*angle).sin()*innerRadius);

                            angle += stepLength*2.0;
                        }

                        if verticesBetweenC0andC3%2 == 1
                        {
                            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                            rlVertex2f(center.x, center.y);

                            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                            rlVertex2f(center.x, center.y);

                            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*innerRadius, center.y + (DEG2RAD*(angle + stepLength)).sin()*innerRadius);

                            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                            rlVertex2f(center.x + (DEG2RAD*angle).cos()*innerRadius, center.y + (DEG2RAD*angle).sin()*innerRadius);
                        }
                    }
                }
            }
        }
    rlEnd();

    rlSetTexture(0);
}

// Draw circle outline
pub unsafe fn DrawCircleLines(centerX: i32, centerY: i32, radius: f32, color: Color)
{
    DrawCircleLinesV(Vector2 { x: (centerX as f32), y: (centerY as f32) }, radius, color);
}

// Draw circle outline (Vector version)
pub unsafe fn DrawCircleLinesV(center: Vector2, radius: f32, color: Color)
{
    rlBegin(RL_LINES);
        rlColor4ub(color.r, color.g, color.b, color.a);

        // NOTE: Circle outline is drawn pixel by pixel every degree (0 to 360)
        for i in (0..360).step_by(10)
        {
            rlVertex2f(center.x + (DEG2RAD*(i as f32)).cos()*radius, center.y + (DEG2RAD*(i as f32)).sin()*radius);
            rlVertex2f(center.x + (DEG2RAD*((i + 10) as f32)).cos()*radius, center.y + (DEG2RAD*((i + 10) as f32)).sin()*radius);
        }
    rlEnd();
}

pub unsafe fn DrawCircleLinesEx(center: Vector2, radius: f32, thick: f32, color: Color)
{
    DrawRing(center, radius - thick, radius, 0.0, 360.0, 36, color);
}

// Draw ellipse
pub unsafe fn DrawEllipse(centerX: i32, centerY: i32, radiusH: f32, radiusV: f32, color: Color)
{
    DrawEllipseV(Vector2 { x: (centerX as f32), y: (centerY as f32) }, radiusH, radiusV, color);
}

// Draw ellipse (Vector version)
pub unsafe fn DrawEllipseV(center: Vector2, radiusH: f32, radiusV: f32, color: Color)
{
    rlBegin(RL_TRIANGLES);
        for i in (0..360).step_by(10)
        {
            rlColor4ub(color.r, color.g, color.b, color.a);
            rlVertex2f(center.x,  center.y);
            rlVertex2f(center.x + (DEG2RAD*((i + 10) as f32)).cos()*radiusH, center.y + (DEG2RAD*((i + 10) as f32)).sin()*radiusV);
            rlVertex2f(center.x + (DEG2RAD*(i as f32)).cos()*radiusH, center.y + (DEG2RAD*(i as f32)).sin()*radiusV);
        }
    rlEnd();
}

// Draw ellipse outline
pub unsafe fn DrawEllipseLines(centerX: i32, centerY: i32, radiusH: f32, radiusV: f32, color: Color)
{
    DrawEllipseLinesV(Vector2 { x: (centerX as f32), y: (centerY as f32) }, radiusH, radiusV, color);
}

// Draw ellipse outline
pub unsafe fn DrawEllipseLinesV(center: Vector2, radiusH: f32, radiusV: f32, color: Color)
{
    rlBegin(RL_LINES);
        for i in (0..360).step_by(10)
        {
            rlColor4ub(color.r, color.g, color.b, color.a);
            rlVertex2f(center.x + (DEG2RAD*((i + 10) as f32)).cos()*radiusH, center.y + (DEG2RAD*((i + 10) as f32)).sin()*radiusV);
            rlVertex2f(center.x + (DEG2RAD*(i as f32)).cos()*radiusH, center.y + (DEG2RAD*(i as f32)).sin()*radiusV);
        }
    rlEnd();
}

// Draw ellipse outline with thickness
pub unsafe fn DrawEllipseLinesEx(center: Vector2, radiusH: f32, radiusV: f32, thick: f32, color: Color)
{
    let mut outerRadiusH: f32 = radiusH; let mut innerRadiusH: f32 = radiusH - thick;
    let mut outerRadiusV: f32 = radiusV; let mut innerRadiusV: f32 = radiusV - thick;

    if thick >= 0.0 {
        // Just a filled-in ellipse
        if innerRadiusH <= 0.0 || innerRadiusV <= 0.0
        {
            DrawEllipseV(center, radiusH, radiusV, color);
            return;
        }
    }
    else
    {
        // The outline is growing outside of the ellipse, so swap the inner and outer radius
        std::mem::swap(&mut outerRadiusH, &mut innerRadiusH);

        std::mem::swap(&mut outerRadiusV, &mut innerRadiusV);
    }

    rlSetTexture(GetShapesTexture().id);
    let shapeRect: Rectangle = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);

        rlColor4ub(color.r, color.g, color.b, color.a);

        for i in (0..360).step_by(10)
        {
            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*(i as f32)).cos()*innerRadiusH, center.y + (DEG2RAD*(i as f32)).sin()*innerRadiusV);

            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*((i + 10) as f32)).cos()*innerRadiusH, center.y + (DEG2RAD*((i + 10) as f32)).sin()*innerRadiusV);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*((i + 10) as f32)).cos()*outerRadiusH, center.y + (DEG2RAD*((i + 10) as f32)).sin()*outerRadiusV);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*(i as f32)).cos()*outerRadiusH, center.y + (DEG2RAD*(i as f32)).sin()*outerRadiusV);
        }
    rlEnd();

    rlSetTexture(0);
}

// Draw ring
pub unsafe fn DrawRing(center: Vector2, mut innerRadius: f32, mut outerRadius: f32, mut startAngle: f32, mut endAngle: f32, mut segments: i32, color: Color)
{
    if startAngle == endAngle { return; }

    // Function expects (outerRadius > innerRadius)
    if outerRadius < innerRadius
    {
        std::mem::swap(&mut outerRadius, &mut innerRadius);

        if outerRadius <= 0.0 { return; }
    }

    // Function expects (endAngle > startAngle)
    if endAngle < startAngle
    {
        // Swap values
        std::mem::swap(&mut startAngle, &mut endAngle);
    }

    // Drawing a whole circle, things get weird without limiting the circle to 360 degrees
    if endAngle - startAngle >= 360.0 { endAngle = startAngle + 360.0; }

    let minSegments: i32 = (((endAngle - startAngle)/90.0).ceil() as i32);

    if segments < minSegments
    {
        // Calculate the maximum angle between segments based on the error rate (usually 0.5f)
        let th: f32 = (2.0*(1.0 - SMOOTH_CIRCLE_ERROR_RATE/outerRadius).powf(2.0) - 1.0).acos();
        segments = (((endAngle - startAngle)*(2.0*PI/th)/360.0).ceil() as i32);

        if segments <= 0 { segments = minSegments; }
    }

    // Not a ring
    if innerRadius <= 0.0
    {
        DrawCircleSector(center, outerRadius, startAngle, endAngle, segments, color);
        return;
    }

    let stepLength: f32 = (endAngle - startAngle)/(segments as f32);
    let mut angle: f32 = startAngle;

    rlSetTexture(GetShapesTexture().id);
    let shapeRect: Rectangle = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);
        for i in 0..segments
        {
            rlColor4ub(color.r, color.g, color.b, color.a);

            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*outerRadius, center.y + (DEG2RAD*angle).sin()*outerRadius);

            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*innerRadius, center.y + (DEG2RAD*angle).sin()*innerRadius);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*innerRadius, center.y + (DEG2RAD*(angle + stepLength)).sin()*innerRadius);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*outerRadius, center.y + (DEG2RAD*(angle + stepLength)).sin()*outerRadius);

            angle += stepLength;
        }
    rlEnd();

    rlSetTexture(0);
}

// Draw ring outline
pub unsafe fn DrawRingLines(center: Vector2, mut innerRadius: f32, mut outerRadius: f32, mut startAngle: f32, mut endAngle: f32, mut segments: i32, color: Color)
{
    if startAngle == endAngle { return; }

    // Function expects (outerRadius > innerRadius)
    if outerRadius < innerRadius
    {
        std::mem::swap(&mut outerRadius, &mut innerRadius);

        if outerRadius <= 0.0 { return; }
    }

    // Function expects (endAngle > startAngle)
    if endAngle < startAngle
    {
        // Swap values
        std::mem::swap(&mut startAngle, &mut endAngle);
    }

    let mut showCapLines: bool = true;
    // Drawing a whole circle, things get weird without limiting the circle to 360 degrees
    if endAngle - startAngle >= 360.0
    {
        showCapLines = false;
        endAngle = startAngle + 360.0;
    }

    let minSegments: i32 = (((endAngle - startAngle)/90.0).ceil() as i32);

    if segments < minSegments
    {
        // Calculate the maximum angle between segments based on the error rate (usually 0.5f)
        let th: f32 = (2.0*(1.0 - SMOOTH_CIRCLE_ERROR_RATE/outerRadius).powf(2.0) - 1.0).acos();
        segments = (((endAngle - startAngle)*(2.0*PI/th)/360.0).ceil() as i32);

        if segments <= 0 { segments = minSegments; }
    }

    if innerRadius <= 0.0
    {
        DrawCircleSectorLines(center, outerRadius, startAngle, endAngle, segments, color);
        return;
    }

    let stepLength: f32 = (endAngle - startAngle)/(segments as f32);
    let mut angle: f32 = startAngle;

    rlBegin(RL_LINES);
        if showCapLines
        {
            rlColor4ub(color.r, color.g, color.b, color.a);
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*outerRadius, center.y + (DEG2RAD*angle).sin()*outerRadius);
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*innerRadius, center.y + (DEG2RAD*angle).sin()*innerRadius);
        }

        for i in 0..segments
        {
            rlColor4ub(color.r, color.g, color.b, color.a);

            rlVertex2f(center.x + (DEG2RAD*angle).cos()*outerRadius, center.y + (DEG2RAD*angle).sin()*outerRadius);
            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*outerRadius, center.y + (DEG2RAD*(angle + stepLength)).sin()*outerRadius);

            rlVertex2f(center.x + (DEG2RAD*angle).cos()*innerRadius, center.y + (DEG2RAD*angle).sin()*innerRadius);
            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*innerRadius, center.y + (DEG2RAD*(angle + stepLength)).sin()*innerRadius);

            angle += stepLength;
        }

        if showCapLines
        {
            rlColor4ub(color.r, color.g, color.b, color.a);
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*outerRadius, center.y + (DEG2RAD*angle).sin()*outerRadius);
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*innerRadius, center.y + (DEG2RAD*angle).sin()*innerRadius);
        }
    rlEnd();
}

// Draw ring outline with line thickness
pub unsafe fn DrawRingLinesEx(center: Vector2, mut innerRadius: f32, mut outerRadius: f32, mut startAngle: f32, mut endAngle: f32, mut segments: i32, thick: f32, color: Color)
{
    if startAngle == endAngle { return; }

    // Function expects (outerRadius > innerRadius)
    if outerRadius < innerRadius
    {
        std::mem::swap(&mut outerRadius, &mut innerRadius);

        if outerRadius <= 0.0 { return; }
    }

    // Function expects (endAngle > startAngle)
    if endAngle < startAngle
    {
        // Swap values
        std::mem::swap(&mut startAngle, &mut endAngle);
    }

    let mut showCapLines: bool = true;
    // Drawing a whole circle, things get weird without limiting the circle to 360 degrees
    if endAngle - startAngle >= 360.0
    {
        showCapLines = thick >= 0.0;
        endAngle = startAngle + 360.0;
    }

    let minSegments: i32 = (((endAngle - startAngle)/90.0).ceil() as i32);

    if segments < minSegments
    {
        // Calculate the maximum angle between segments based on the error rate (usually 0.5f)
        let th: f32 = (2.0*(1.0 - SMOOTH_CIRCLE_ERROR_RATE/outerRadius).powf(2.0) - 1.0).acos();
        segments = (((endAngle - startAngle)*(2.0*PI/th)/360.0).ceil() as i32);

        if segments <= 0 { segments = minSegments; }
    }

    let stepLength: f32 = (endAngle - startAngle)/(segments as f32);

    // We are not drawing a circle, we are drawing an n-sided polygon
    // So, we need to adjust the outline thickness of the "circle" for it to look correct with fewer segments
    let apothem: f32 = outerRadius*(DEG2RAD*((endAngle - startAngle)/2.0)/(segments as f32)).cos();
    let radiusThick: f32 = thick*(outerRadius/apothem);

    // These names can be confusing, but they are useful
    // Since 2 rings are being drawn, there are 4 radiuses (or radii)
    // "Inner" means closer to the center, "outer" means farther from the center
    // Sorted from farthest to closest you get:
    //   1. outerOuterRadius (farthest)
    //   2. innerOuterRadius
    //   3. outerInnerRadius
    //   4. innerInnerRadius (closest)
    let mut innerOuterRadius: f32 = 0.0;
    let mut outerOuterRadius: f32 = 0.0;
    let mut innerInnerRadius: f32 = 0.0;
    let mut outerInnerRadius: f32 = 0.0;

    if thick >= 0.0
    {
        innerRadius = (0.0f32).max(innerRadius);

        // Just a filled-in ring
        if radiusThick > (outerRadius - innerRadius)/2.0
        {
            DrawRing(center, innerRadius, outerRadius, startAngle, endAngle, segments, color);
            return;
        }

        innerInnerRadius = innerRadius;
        outerInnerRadius = innerInnerRadius + radiusThick;

        outerOuterRadius = outerRadius;
        innerOuterRadius = outerOuterRadius - radiusThick;
    }
    else
    {
        // Just a circle sector outline
        if innerRadius <= 0.0
        {
            DrawCircleSectorLinesEx(center, outerRadius, startAngle, endAngle, segments, thick, color);
            return;
        }

        outerInnerRadius = innerRadius;
        innerInnerRadius = (0.0f32).max(outerInnerRadius + radiusThick);

        innerOuterRadius = outerRadius;
        outerOuterRadius = innerOuterRadius - radiusThick;
    }

    // For positive `thick` values
    let mut stepsBeforeInner: i32 = 0;
    let mut stepsBeforeOuter: i32 = 0;
    let mut tInner: f32 = 0.0;
    let mut tOuter: f32 = 0.0;
    let mut innerAnglesCrossEachOther: bool = false;

    // For negative `thick` values
    let mut cap1SecondInnerVertex: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut cap1SecondOuterVertex: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut cap2SecondInnerVertex: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut cap2SecondOuterVertex: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut capsIntersect: bool = false;
    let mut capIntersectionVertex: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    if showCapLines
    {
        if thick >= 0.0
        {
            // Get the angle of the arc that has `thick` length along the inner and outer radii
            let cap1InnerAngleEnd: f32 = RAD2DEG*(thick/outerInnerRadius);
            let cap1OuterAngleEnd: f32 = RAD2DEG*(thick/innerOuterRadius);

            // Just a filled-in ring
            if endAngle - startAngle < cap1OuterAngleEnd*2.0
            {
                DrawRing(center, innerRadius, outerRadius, startAngle, endAngle, segments, color);
                return;
            }

            if endAngle - startAngle < cap1InnerAngleEnd*2.0 { innerAnglesCrossEachOther = true; }

            stepsBeforeInner = ((cap1InnerAngleEnd/stepLength) as i32);
            stepsBeforeOuter = ((cap1OuterAngleEnd/stepLength) as i32);

            // We need to find where `cap1InnerAngleEnd` intersects the edge defined
            // by `beforeInnerVertex` and `afterInnerVertex`
            //
            // We can make this easy by making `center` the origin (0, 0) and
            // making `cap1InnerAngleEnd` 0 degrees (a horizontal line)
            //
            // With that, we know these lines intersect when 'y' equals 0,
            // so we just need to solve for 't' (as in `Lerp(start, end, t)`)
            let beforeInnerVertex: Vector2 = Vector2 { x: (DEG2RAD*((stepsBeforeInner as f32)*stepLength - cap1InnerAngleEnd)).cos()*outerInnerRadius, y: (DEG2RAD*((stepsBeforeInner as f32)*stepLength - cap1InnerAngleEnd)).sin()*outerInnerRadius };
            let afterInnerVertex: Vector2 = Vector2 { x: (DEG2RAD*(((stepsBeforeInner + 1) as f32)*stepLength - cap1InnerAngleEnd)).cos()*outerInnerRadius, y: (DEG2RAD*(((stepsBeforeInner + 1) as f32)*stepLength - cap1InnerAngleEnd)).sin()*outerInnerRadius };
            tInner = beforeInnerVertex.y/(beforeInnerVertex.y - afterInnerVertex.y);

            // The same as above, but for the outer edge
            let beforeOuterVertex: Vector2 = Vector2 { x: (DEG2RAD*((stepsBeforeOuter as f32)*stepLength - cap1OuterAngleEnd)).cos()*innerOuterRadius, y: (DEG2RAD*((stepsBeforeOuter as f32)*stepLength - cap1OuterAngleEnd)).sin()*innerOuterRadius };
            let afterOuterVertex: Vector2 = Vector2 { x: (DEG2RAD*(((stepsBeforeOuter + 1) as f32)*stepLength - cap1OuterAngleEnd)).cos()*innerOuterRadius, y: (DEG2RAD*(((stepsBeforeOuter + 1) as f32)*stepLength - cap1OuterAngleEnd)).sin()*innerOuterRadius };
            tOuter = beforeOuterVertex.y/(beforeOuterVertex.y - afterOuterVertex.y);
        }
        else
        {
            // "Cap 1" is the outline on `startAngle` and "Cap 2" is the outline on `endAngle`

            /*
            A sketch to help make all this a little more understandable
            (This is an overly simplified representation of cap 1)

             I2[][][][][]O2  <- y = thick
             []          []
             []          []
             I0----------O0  <- angle = 0 degrees, y = 0
             []          []
             I1          O1  <- angle = stepLength

            Cap 2 is a mirror copy of cap 1, the inside and outside vertices switch sides

            We're using a frame of reference where `center` is (0, 0) and `startAngle` is 0 degrees

            I0 is `innerInnerRadius` distance from `center` at `starAngle`
            I1 is `innerInnerRadius` distance from `center` at `starAngle + stepLength`
            I2 goes out from I0 perpendicular to `startAngle`
            O0 is the same as I0, except using `outerOuterRadius` instead of `innerInnerRadius`
            O1 is the same as I1, except using `outerOuterRadius` instead of `innerInnerRadius`
            O2 is the same as I2, except goes out from O0

            The intersection cases between the caps edges are:
              1. No intersections, easy
              2. The I0->I2 and I2->O2 edges intersect between the caps
              3. The I0->I2 and O0->O2 edges intersect between the caps

            Notice that cap 1 and 2's I2->O2 and O0->O2 edges can't intersect at the same time,
            and, if there's any intersection, I0->I2 is one of the edges
            */

            let mut cap1O0: Vector2 = Vector2 { x: outerOuterRadius, y: 0.0 };
            let mut cap1O1: Vector2 = Vector2 { x: (DEG2RAD*stepLength).cos()*outerOuterRadius, y: (DEG2RAD*stepLength).sin()*outerOuterRadius };

            // Assuming a linear interpolation such as `value = Lerp(start, end, t)`
            // We can find O2 by getting its 't' between O1.y and O0.y (which is always greater than 1)
            // We can solve for `t` using `t = (start - value)/(start - end)`
            // Since we know `end = 0` we can simplify it to `t = (start - value)/start`
            let tOuter: f32 = (cap1O1.y - thick)/cap1O1.y;
            let mut cap1O2: Vector2 = Vector2 { x: cap1O1.x + (cap1O0.x - cap1O1.x)*tOuter, y: thick };

            //Vector2 cap1I0 = { innerInnerRadius, 0.0f }; // Not used

            let capLongEdgeLength: f32 = outerOuterRadius - innerInnerRadius;
            let mut cap1I2: Vector2 = Vector2 { x: cap1O2.x - capLongEdgeLength, y: thick };

            let mut cap2O0: Vector2 = Vector2 { x: (DEG2RAD*(endAngle - startAngle)).cos()*outerOuterRadius, y: (DEG2RAD*(endAngle - startAngle)).sin()*outerOuterRadius };
            let mut cap2O1: Vector2 = Vector2 { x: (DEG2RAD*(endAngle - startAngle - stepLength)).cos()*outerOuterRadius, y: (DEG2RAD*(endAngle - startAngle - stepLength)).sin()*outerOuterRadius };
            let mut cap2O2: Vector2 = Vector2 { x: cap2O1.x + (cap2O0.x - cap2O1.x)*tOuter, y: cap2O1.y + (cap2O0.y - cap2O1.y)*tOuter };

            let mut cap2I0: Vector2 = Vector2 { x: (DEG2RAD*(endAngle - startAngle)).cos()*innerInnerRadius, y: (DEG2RAD*(endAngle - startAngle)).sin()*innerInnerRadius };
            let mut cap2I2: Vector2 = Vector2 { x: cap2O2.x - (DEG2RAD*(endAngle - startAngle)).cos()*capLongEdgeLength, y: cap2O2.y - (DEG2RAD*(endAngle - startAngle)).sin()*capLongEdgeLength};

            // The 't' of the intersection between I2 and O2 (`Lerp(I2, O2, t)`)
            let mut tCapLongEdgeCross: f32 = -1.0;
            // Avoid division by zero
            if cap2I2.y - cap2O2.y != 0.0
            {
                // Find where the long edge of cap 2 intersects the long edge of cap 1
                tCapLongEdgeCross = (cap2I2.y - thick)/(cap2I2.y - cap2O2.y);
                if (tCapLongEdgeCross >= 0.0) && (tCapLongEdgeCross <= 1.0) { capsIntersect = true; }
            }

            // Rotate the frame of reference so that cap 1's I0->I2 edge is a vertical line
            let rotateBy: f32 = -DEG2RAD*stepLength/2.0;

            // Copied from "raymath.h" Vector2Rotate()
            // Though we only use the x axis, so we ignore the y axis
            let cosres: f32 = (rotateBy).cos();
            let sinres: f32 = (rotateBy).sin();

            cap1I2.x = cap1I2.x*cosres - cap1I2.y*sinres;
            cap1O2.x = cap1O2.x*cosres - cap1O2.y*sinres;
            cap2I0.x = cap2I0.x*cosres - cap2I0.y*sinres;
            cap2I2.x = cap2I2.x*cosres - cap2I2.y*sinres;
            cap2O0.x = cap2O0.x*cosres - cap2O0.y*sinres;
            cap2O2.x = cap2O2.x*cosres - cap2O2.y*sinres;

            // The 't' of the intersection between I0 and I2 (`Lerp(I0, I2, t)`)
            let mut tCrossInner: f32 = -1.0;
            // Avoid division by zero
            if cap2I0.x - cap2I2.x != 0.0 { tCrossInner = (cap2I0.x - cap1I2.x)/(cap2I0.x - cap2I2.x); }
            // Make sure `tCrossInner` is 0 when it should be (mitigate floating-point rounding woes)
            if innerInnerRadius <= 0.0 { tCrossInner = 0.0; }

            // The 't' of the intersection between O0 and O2 (`Lerp(O0, O2, t)`)
            let mut tCrossOuter: f32 = -1.0;
            // Avoid division by zero
            if cap2O0.x - cap2O2.x != 0.0 { tCrossOuter = (cap2O0.x - cap1O2.x)/(cap2O0.x - cap2O2.x); }

            // With our additional information, calculate the vertices we need
            // outside of our modified frame of reference

            cap1O0 = Vector2 { x: center.x + (DEG2RAD*startAngle).cos()*outerOuterRadius, y: center.y + (DEG2RAD*startAngle).sin()*outerOuterRadius };
            cap1O1 = Vector2 { x: center.x + (DEG2RAD*(startAngle + stepLength)).cos()*outerOuterRadius, y: center.y + (DEG2RAD*(startAngle + stepLength)).sin()*outerOuterRadius };
            cap1O2 = Vector2 { x: cap1O1.x + (cap1O0.x - cap1O1.x)*tOuter, y: cap1O1.y + (cap1O0.y - cap1O1.y)*tOuter };

            cap2O0 = Vector2 { x: center.x + (DEG2RAD*endAngle).cos()*outerOuterRadius, y: center.y + (DEG2RAD*endAngle).sin()*outerOuterRadius };
            cap2O1 = Vector2 { x: center.x + (DEG2RAD*(endAngle - stepLength)).cos()*outerOuterRadius, y: center.y + (DEG2RAD*(endAngle - stepLength)).sin()*outerOuterRadius };
            cap2O2 = Vector2 { x: cap2O1.x + (cap2O0.x - cap2O1.x)*tOuter, y: cap2O1.y + (cap2O0.y - cap2O1.y)*tOuter };

            //cap1I0 = (Vector2){ center.x + cosf(DEG2RAD*startAngle)*innerInnerRadius, center.y + sinf(DEG2RAD*startAngle)*innerInnerRadius };
            cap1I2 = Vector2 { x: cap1O2.x - (DEG2RAD*startAngle).cos()*capLongEdgeLength, y: cap1O2.y - (DEG2RAD*startAngle).sin()*capLongEdgeLength };

            cap2I0 = Vector2 { x: center.x + (DEG2RAD*endAngle).cos()*innerInnerRadius, y: center.y + (DEG2RAD*endAngle).sin()*innerInnerRadius };
            cap2I2 = Vector2 { x: cap2O2.x - (DEG2RAD*endAngle).cos()*capLongEdgeLength, y: cap2O2.y - (DEG2RAD*endAngle).sin()*capLongEdgeLength };

            if capsIntersect
            {
                capIntersectionVertex = Vector2 { x: cap2I2.x + (cap2O2.x - cap2I2.x)*tCapLongEdgeCross, y: cap2I2.y + (cap2O2.y - cap2I2.y)*tCapLongEdgeCross };

                cap2I2 = Vector2 { x: cap2I0.x + (cap2I2.x - cap2I0.x)*tCrossInner, y: cap2I0.y + (cap2I2.y - cap2I0.y)*tCrossInner };
                cap1I2 = cap2I2;
            }
            else if (tCrossOuter >= 0.0) && (tCrossOuter <= 1.0)
            {
                cap2O2 = Vector2 { x: cap2O0.x + (cap2O2.x - cap2O0.x)*tCrossOuter, y: cap2O0.y + (cap2O2.y - cap2O0.y)*tCrossOuter };
                cap1O2 = cap2O2;

                cap2I2 = Vector2 { x: cap2I0.x + (cap2I2.x - cap2I0.x)*tCrossInner, y: cap2I0.y + (cap2I2.y - cap2I0.y)*tCrossInner };
                cap1I2 = cap2I2;
            }

            cap1SecondInnerVertex = cap1I2;
            cap1SecondOuterVertex = cap1O2;
            cap2SecondInnerVertex = cap2I2;
            cap2SecondOuterVertex = cap2O2;
        }
    }

    let mut angle: f32 = startAngle;

    rlSetTexture(GetShapesTexture().id);
    let shapeRect: Rectangle = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);

        rlColor4ub(color.r, color.g, color.b, color.a);

        for i in 0..segments
        {
            // `innerRadius` outline
            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*outerInnerRadius, center.y + (DEG2RAD*angle).sin()*outerInnerRadius);

            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*innerInnerRadius, center.y + (DEG2RAD*angle).sin()*innerInnerRadius);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*innerInnerRadius, center.y + (DEG2RAD*(angle + stepLength)).sin()*innerInnerRadius);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*outerInnerRadius, center.y + (DEG2RAD*(angle + stepLength)).sin()*outerInnerRadius);

            // `outerRadius` outline
            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*outerOuterRadius, center.y + (DEG2RAD*angle).sin()*outerOuterRadius);

            rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*innerOuterRadius, center.y + (DEG2RAD*angle).sin()*innerOuterRadius);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*innerOuterRadius, center.y + (DEG2RAD*(angle + stepLength)).sin()*innerOuterRadius);

            rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*outerOuterRadius, center.y + (DEG2RAD*(angle + stepLength)).sin()*outerOuterRadius);

            angle += stepLength;
        }

        if showCapLines
        {
            if thick >= 0.0
            {
                angle = 0.0;

                for i in 0..stepsBeforeOuter
                {
                    // Cap 1
                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(startAngle + angle)).cos()*outerInnerRadius, center.y + (DEG2RAD*(startAngle + angle)).sin()*outerInnerRadius);

                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(startAngle + angle + stepLength)).cos()*outerInnerRadius, center.y + (DEG2RAD*(startAngle + angle + stepLength)).sin()*outerInnerRadius);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(startAngle + angle + stepLength)).cos()*innerOuterRadius, center.y + (DEG2RAD*(startAngle + angle + stepLength)).sin()*innerOuterRadius);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(startAngle + angle)).cos()*innerOuterRadius, center.y + (DEG2RAD*(startAngle + angle)).sin()*innerOuterRadius);

                    // Cap 2
                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(endAngle - angle)).cos()*outerInnerRadius, center.y + (DEG2RAD*(endAngle - angle)).sin()*outerInnerRadius);

                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(endAngle - angle)).cos()*innerOuterRadius, center.y + (DEG2RAD*(endAngle - angle)).sin()*innerOuterRadius);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(endAngle - angle - stepLength)).cos()*innerOuterRadius, center.y + (DEG2RAD*(endAngle - angle - stepLength)).sin()*innerOuterRadius);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(endAngle - angle - stepLength)).cos()*outerInnerRadius, center.y + (DEG2RAD*(endAngle - angle - stepLength)).sin()*outerInnerRadius);

                    angle += stepLength;
                }

                // We've already moved `stepsBeforeOuter` steps from each end
                let totalStepsLeft: i32 = segments - stepsBeforeOuter*2;
                let innerStepsLeft: i32 = stepsBeforeInner - stepsBeforeOuter;

                // Cap 1
                let cap1OuterVertexBeforeEnd: Vector2 = Vector2 { x: center.x + (DEG2RAD*(startAngle + angle)).cos()*innerOuterRadius, y: center.y + (DEG2RAD*(startAngle + angle)).sin()*innerOuterRadius };
                let cap1OuterVertexAfterEnd: Vector2 = Vector2 { x: center.x + (DEG2RAD*(startAngle + angle + stepLength)).cos()*innerOuterRadius, y: center.y + (DEG2RAD*(startAngle + angle + stepLength)).sin()*innerOuterRadius };
                let cap1InnerVertexBeforeEnd: Vector2 = Vector2 { x: center.x + (DEG2RAD*(startAngle + angle + (innerStepsLeft as f32)*stepLength)).cos()*outerInnerRadius, y: center.y + (DEG2RAD*(startAngle + angle + (innerStepsLeft as f32)*stepLength)).sin()*outerInnerRadius };
                let cap1InnerVertexAfterEnd: Vector2 = Vector2 { x: center.x + (DEG2RAD*(startAngle + angle + ((innerStepsLeft + 1) as f32)*stepLength)).cos()*outerInnerRadius, y: center.y + (DEG2RAD*(startAngle + angle + ((innerStepsLeft + 1) as f32)*stepLength)).sin()*outerInnerRadius };
                let cap1InnerVertexEnd: Vector2 = Vector2 { x: cap1InnerVertexBeforeEnd.x + (cap1InnerVertexAfterEnd.x - cap1InnerVertexBeforeEnd.x)*tInner, y: cap1InnerVertexBeforeEnd.y + (cap1InnerVertexAfterEnd.y - cap1InnerVertexBeforeEnd.y)*tInner };
                let cap1OuterVertexEnd: Vector2 = Vector2 { x: cap1OuterVertexBeforeEnd.x + (cap1OuterVertexAfterEnd.x - cap1OuterVertexBeforeEnd.x)*tOuter, y: cap1OuterVertexBeforeEnd.y + (cap1OuterVertexAfterEnd.y - cap1OuterVertexBeforeEnd.y)*tOuter };

                // Cap 2
                let cap2OuterVertexBeforeEnd: Vector2 = Vector2 { x: center.x + (DEG2RAD*(endAngle - angle)).cos()*innerOuterRadius, y: center.y + (DEG2RAD*(endAngle - angle)).sin()*innerOuterRadius };
                let cap2OuterVertexAfterEnd: Vector2 = Vector2 { x: center.x + (DEG2RAD*(endAngle - angle - stepLength)).cos()*innerOuterRadius, y: center.y + (DEG2RAD*(endAngle - angle - stepLength)).sin()*innerOuterRadius };
                let cap2InnerVertexBeforeEnd: Vector2 = Vector2 { x: center.x + (DEG2RAD*(endAngle - angle - (innerStepsLeft as f32)*stepLength)).cos()*outerInnerRadius, y: center.y + (DEG2RAD*(endAngle - angle - (innerStepsLeft as f32)*stepLength)).sin()*outerInnerRadius };
                let cap2InnerVertexAfterEnd: Vector2 = Vector2 { x: center.x + (DEG2RAD*(endAngle - angle - ((innerStepsLeft + 1) as f32)*stepLength)).cos()*outerInnerRadius, y: center.y + (DEG2RAD*(endAngle - angle - ((innerStepsLeft + 1) as f32)*stepLength)).sin()*outerInnerRadius };
                let cap2InnerVertexEnd: Vector2 = Vector2 { x: cap2InnerVertexBeforeEnd.x + (cap2InnerVertexAfterEnd.x - cap2InnerVertexBeforeEnd.x)*tInner, y: cap2InnerVertexBeforeEnd.y + (cap2InnerVertexAfterEnd.y - cap2InnerVertexBeforeEnd.y)*tInner };
                let cap2OuterVertexEnd: Vector2 = Vector2 { x: cap2OuterVertexBeforeEnd.x + (cap2OuterVertexAfterEnd.x - cap2OuterVertexBeforeEnd.x)*tOuter, y: cap2OuterVertexBeforeEnd.y + (cap2OuterVertexAfterEnd.y - cap2OuterVertexBeforeEnd.y)*tOuter };

                let stepsCount: i32 = if (innerAnglesCrossEachOther) { totalStepsLeft/2 } else { innerStepsLeft };

                // Iterate over pairs of steps
                for i in 0..stepsCount/2
                {
                    // Cap 1
                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(cap1OuterVertexBeforeEnd.x, cap1OuterVertexBeforeEnd.y);

                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(startAngle + angle)).cos()*outerInnerRadius, center.y + (DEG2RAD*(startAngle + angle)).sin()*outerInnerRadius);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(startAngle + angle + stepLength)).cos()*outerInnerRadius, center.y + (DEG2RAD*(startAngle + angle + stepLength)).sin()*outerInnerRadius);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(startAngle + angle + stepLength*2.0)).cos()*outerInnerRadius, center.y + (DEG2RAD*(startAngle + angle + stepLength*2.0)).sin()*outerInnerRadius);

                    // Cap 2
                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(cap2OuterVertexBeforeEnd.x, cap2OuterVertexBeforeEnd.y);

                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(endAngle - angle - stepLength*2.0)).cos()*outerInnerRadius, center.y + (DEG2RAD*(endAngle - angle - stepLength*2.0)).sin()*outerInnerRadius);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(endAngle - angle - stepLength)).cos()*outerInnerRadius, center.y + (DEG2RAD*(endAngle - angle - stepLength)).sin()*outerInnerRadius);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(endAngle - angle)).cos()*outerInnerRadius, center.y + (DEG2RAD*(endAngle - angle)).sin()*outerInnerRadius);

                    angle += stepLength*2.0;
                }

                // Handle the last step if there's an odd amount
                if stepsCount%2 == 1
                {
                    // Cap 1
                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(cap1OuterVertexBeforeEnd.x, cap1OuterVertexBeforeEnd.y);

                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(cap1OuterVertexBeforeEnd.x, cap1OuterVertexBeforeEnd.y);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(startAngle + angle)).cos()*outerInnerRadius, center.y + (DEG2RAD*(startAngle + angle)).sin()*outerInnerRadius);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(startAngle + angle + stepLength)).cos()*outerInnerRadius, center.y + (DEG2RAD*(startAngle + angle + stepLength)).sin()*outerInnerRadius);

                    // Cap 2
                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(cap2OuterVertexBeforeEnd.x, cap2OuterVertexBeforeEnd.y);

                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(cap2OuterVertexBeforeEnd.x, cap2OuterVertexBeforeEnd.y);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(endAngle - angle - stepLength)).cos()*outerInnerRadius, center.y + (DEG2RAD*(endAngle - angle - stepLength)).sin()*outerInnerRadius);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(center.x + (DEG2RAD*(endAngle - angle)).cos()*outerInnerRadius, center.y + (DEG2RAD*(endAngle - angle)).sin()*outerInnerRadius);

                    angle += stepLength;
                }

                // When the inner angles coming from `startAngle` and `endAngle` cross each other,
                // the `*innerVertexEnd` vertices go past each other and cause the geometry to intersect itself
                if innerAnglesCrossEachOther
                {
                    // We need to find where the line defined by `cap1InnerVertexEnd` and `cap1OuterVertexEnd` intersects
                    // the line defined by `cap2InnerVertexEnd` and `cap2OuterVertexEnd`
                    // That point is then used instead to prevent the outline from intersecting itself

                    // Make `cap1InnerVertexEnd` the origin and the angle to `cap1OuterVertexEnd` 0 degrees
                    let tempCap1OuterVertexEnd: Vector2 = Vector2 { x: cap1OuterVertexEnd.x - cap1InnerVertexEnd.x, y: cap1OuterVertexEnd.y - cap1InnerVertexEnd.y };
                    let tempCap2InnerVertexEnd: Vector2 = Vector2 { x: cap2InnerVertexEnd.x - cap1InnerVertexEnd.x, y: cap2InnerVertexEnd.y - cap1InnerVertexEnd.y };
                    let tempCap2OuterVertexEnd: Vector2 = Vector2 { x: cap2OuterVertexEnd.x - cap1InnerVertexEnd.x, y: cap2OuterVertexEnd.y - cap1InnerVertexEnd.y };

                    let rotateBy: f32 = -(tempCap1OuterVertexEnd.y).atan2(tempCap1OuterVertexEnd.x);
                    // We only need the y coordinates, so only rotate the y coordinates
                    let start: f32 = (rotateBy).sin()*tempCap2InnerVertexEnd.x + (rotateBy).cos()*tempCap2InnerVertexEnd.y;
                    let end: f32 = (rotateBy).sin()*tempCap2OuterVertexEnd.x + (rotateBy).cos()*tempCap2OuterVertexEnd.y;
                    let tCross: f32 = start/(start - end);

                    let intersection: Vector2 = Vector2 { x: cap2InnerVertexEnd.x + (cap2OuterVertexEnd.x - cap2InnerVertexEnd.x)*tCross, y: cap2InnerVertexEnd.y + (cap2OuterVertexEnd.y - cap2InnerVertexEnd.y)*tCross };

                    if segments%2 == 0
                    {
                        // There are an even number of segments, so there's 1 vertex exactly in the middle

                        let middleInnerVertex: Vector2 = Vector2 { x: center.x + (DEG2RAD*(startAngle + angle)).cos()*outerInnerRadius, y: center.y + (DEG2RAD*(startAngle + angle)).sin()*outerInnerRadius };

                        // Cap 1
                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(intersection.x, intersection.y);

                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(cap1OuterVertexEnd.x, cap1OuterVertexEnd.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(cap1OuterVertexBeforeEnd.x, cap1OuterVertexBeforeEnd.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(middleInnerVertex.x, middleInnerVertex.y);

                        // Cap 2
                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(intersection.x, intersection.y);

                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(middleInnerVertex.x, middleInnerVertex.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(cap2OuterVertexBeforeEnd.x, cap2OuterVertexBeforeEnd.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(cap2OuterVertexEnd.x, cap2OuterVertexEnd.y);
                    }
                    else
                    {
                        // There are an odd number of segments, so there are 2 vertices in the middle

                        let middleInnerVertex1: Vector2 = Vector2 { x: center.x + (DEG2RAD*(startAngle + angle)).cos()*outerInnerRadius, y: center.y + (DEG2RAD*(startAngle + angle)).sin()*outerInnerRadius };
                        let middleInnerVertex2: Vector2 = Vector2 { x: center.x + (DEG2RAD*(endAngle - angle)).cos()*outerInnerRadius, y: center.y + (DEG2RAD*(endAngle - angle)).sin()*outerInnerRadius };

                        // Cap 1
                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(intersection.x, intersection.y);

                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(cap1OuterVertexEnd.x, cap1OuterVertexEnd.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(cap1OuterVertexBeforeEnd.x, cap1OuterVertexBeforeEnd.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(middleInnerVertex1.x, middleInnerVertex1.y);

                        // Cap 2
                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(intersection.x, intersection.y);

                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(middleInnerVertex2.x, middleInnerVertex2.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(cap2OuterVertexBeforeEnd.x, cap2OuterVertexBeforeEnd.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(cap2OuterVertexEnd.x, cap2OuterVertexEnd.y);

                        // Triangle between the caps
                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(intersection.x, intersection.y);

                        rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(intersection.x, intersection.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                        rlVertex2f(middleInnerVertex1.x, middleInnerVertex1.y);

                        rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                        rlVertex2f(middleInnerVertex2.x, middleInnerVertex2.y);
                    }
                }
                else
                {
                    // Cap 1
                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(cap1OuterVertexBeforeEnd.x, cap1OuterVertexBeforeEnd.y);

                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(cap1InnerVertexBeforeEnd.x, cap1InnerVertexBeforeEnd.y);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(cap1InnerVertexEnd.x, cap1InnerVertexEnd.y);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(cap1OuterVertexEnd.x, cap1OuterVertexEnd.y);

                    // Cap 2
                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(cap2OuterVertexBeforeEnd.x, cap2OuterVertexBeforeEnd.y);

                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(cap2OuterVertexEnd.x, cap2OuterVertexEnd.y);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(cap2InnerVertexEnd.x, cap2InnerVertexEnd.y);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(cap2InnerVertexBeforeEnd.x, cap2InnerVertexBeforeEnd.y);
                }
            }
            else
            {
                // Cap 1
                let cap1FirstInnerVertex: Vector2 = Vector2 { x: center.x + (DEG2RAD*startAngle).cos()*innerInnerRadius, y: center.y + (DEG2RAD*startAngle).sin()*innerInnerRadius };
                let cap1FirstOuterVertex: Vector2 = Vector2 { x: center.x + (DEG2RAD*startAngle).cos()*outerOuterRadius, y: center.y + (DEG2RAD*startAngle).sin()*outerOuterRadius };

                rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                rlVertex2f(cap1FirstInnerVertex.x, cap1FirstInnerVertex.y);

                rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                rlVertex2f(cap1FirstOuterVertex.x, cap1FirstOuterVertex.y);

                rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                rlVertex2f(cap1SecondOuterVertex.x, cap1SecondOuterVertex.y);

                rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                rlVertex2f(cap1SecondInnerVertex.x, cap1SecondInnerVertex.y);

                // Cap 2
                let cap2FirstInnerVertex: Vector2 = Vector2 { x: center.x + (DEG2RAD*endAngle).cos()*innerInnerRadius, y: center.y + (DEG2RAD*endAngle).sin()*innerInnerRadius };
                let cap2FirstOuterVertex: Vector2 = Vector2 { x: center.x + (DEG2RAD*endAngle).cos()*outerOuterRadius, y: center.y + (DEG2RAD*endAngle).sin()*outerOuterRadius };

                rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                rlVertex2f(cap2FirstInnerVertex.x, cap2FirstInnerVertex.y);

                rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                rlVertex2f(cap2SecondInnerVertex.x, cap2SecondInnerVertex.y);

                rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                rlVertex2f(cap2SecondOuterVertex.x, cap2SecondOuterVertex.y);

                rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                rlVertex2f(cap2FirstOuterVertex.x, cap2FirstOuterVertex.y);

                if capsIntersect
                {
                    // Cap 1
                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(cap1SecondInnerVertex.x, cap1SecondInnerVertex.y);

                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(cap1SecondInnerVertex.x, cap1SecondInnerVertex.y);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(cap1SecondOuterVertex.x, cap1SecondOuterVertex.y);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(capIntersectionVertex.x, capIntersectionVertex.y);

                    // Cap 2
                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(cap2SecondInnerVertex.x, cap2SecondInnerVertex.y);

                    rlTexCoord2f(shapeRect.x/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(capIntersectionVertex.x, capIntersectionVertex.y);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), shapeRect.y/(texShapes.height as f32));
                    rlVertex2f(cap2SecondOuterVertex.x, cap2SecondOuterVertex.y);

                    rlTexCoord2f((shapeRect.x + shapeRect.width)/(texShapes.width as f32), (shapeRect.y + shapeRect.height)/(texShapes.height as f32));
                    rlVertex2f(cap2SecondInnerVertex.x, cap2SecondInnerVertex.y);
                }
            }
        }
    rlEnd();
}

//----------------------------------------------------------------------------------
// Module Functions Definition - Splines functions
//----------------------------------------------------------------------------------

// Draw spline: linear, minimum 2 points
pub unsafe fn DrawSplineLinear(points: &[Vector2], pointCount: i32, thick: f32, color: Color)
{
    if pointCount < 2 { return; }

#[cfg(feature="SUPPORT_SPLINE_MITERS")]
    {
    let mut prevNormal: Vector2 = Vector2 {x: -(points[1].y - points[0].y), y: (points[1].x - points[0].x)};
    let prevLength: f32 = (prevNormal.x*prevNormal.x + prevNormal.y*prevNormal.y).sqrt();

    if prevLength > 0.0
    {
        prevNormal.x /= prevLength;
        prevNormal.y /= prevLength;
    }
    else
    {
        prevNormal.x = 0.0;
        prevNormal.y = 0.0;
    }

    let mut prevRadius: Vector2 = Vector2 { x: 0.5*thick*prevNormal.x, y: 0.5*thick*prevNormal.y };

    for i in 0..pointCount - 1
    {
        let mut normal: Vector2 = Vector2 { x: 0.0, y: 0.0 };

        if i < pointCount - 2
        {
            normal = Vector2 {x: -(points[((i + 2) as usize)].y - points[((i + 1) as usize)].y), y: (points[((i + 2) as usize)].x - points[((i + 1) as usize)].x)};
            let normalLength: f32 = (normal.x*normal.x + normal.y*normal.y).sqrt();

            if normalLength > 0.0
            {
                normal.x /= normalLength;
                normal.y /= normalLength;
            }
            else
            {
                normal.x = 0.0;
                normal.y = 0.0;
            }
        }
        else
        {
            normal = prevNormal;
        }

        let mut radius: Vector2 = Vector2 { x: prevNormal.x + normal.x, y: prevNormal.y + normal.y };
        let radiusLength: f32 = (radius.x*radius.x + radius.y*radius.y).sqrt();

        if radiusLength > 0.0
        {
            radius.x /= radiusLength;
            radius.y /= radiusLength;
        }
        else
        {
            radius.x = 0.0;
            radius.y = 0.0;
        }

        let cosTheta: f32 = radius.x*normal.x + radius.y*normal.y;

        if cosTheta != 0.0
        {
            radius.x *= (thick*0.5/cosTheta);
            radius.y *= (thick*0.5/cosTheta);
        }
        else
        {
            radius.x = 0.0;
            radius.y = 0.0;
        }

        let strip: [ Vector2; 4 ] = [
            Vector2 { x: points[(i as usize)].x - prevRadius.x, y: points[(i as usize)].y - prevRadius.y },
            Vector2 { x: points[(i as usize)].x + prevRadius.x, y: points[(i as usize)].y + prevRadius.y },
            Vector2 { x: points[((i + 1) as usize)].x - radius.x, y: points[((i + 1) as usize)].y - radius.y },
            Vector2 { x: points[((i + 1) as usize)].x + radius.x, y: points[((i + 1) as usize)].y + radius.y }
        ];

        DrawTriangleStrip(&strip, 4, color);

        prevRadius = radius;
        prevNormal = normal;
    }

    }
    #[cfg(not(feature="SUPPORT_SPLINE_MITERS"))]   // !SUPPORT_SPLINE_MITERS
    {

    let mut delta: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut length: f32 = 0.0;
    let mut scale: f32 = 0.0;

    for i in 0..pointCount - 1
    {
        delta = Vector2 { x: points[((i + 1) as usize)].x - points[(i as usize)].x, y: points[((i + 1) as usize)].y - points[(i as usize)].y };
        length = (delta.x*delta.x + delta.y*delta.y).sqrt();

        if length > 0.0 { scale = thick/(2.0*length); }

        let radius: Vector2 = Vector2 { x: -scale*delta.y, y: scale*delta.x };
        let strip: [ Vector2; 4 ] = [
            Vector2 { x: points[(i as usize)].x - radius.x, y: points[(i as usize)].y - radius.y },
            Vector2 { x: points[(i as usize)].x + radius.x, y: points[(i as usize)].y + radius.y },
            Vector2 { x: points[((i + 1) as usize)].x - radius.x, y: points[((i + 1) as usize)].y - radius.y },
            Vector2 { x: points[((i + 1) as usize)].x + radius.x, y: points[((i + 1) as usize)].y + radius.y }
        ];

        DrawTriangleStrip(&strip, 4, color);
    }
    }

#[cfg(feature="SUPPORT_SPLINE_SEGMENT_CAPS")]
    {
    // TODO: Add spline segment rounded caps at the begin/end of the spline?
    }
}

// Draw spline: B-Spline, minimum 4 points
pub unsafe fn DrawSplineBasis(points: &[Vector2], pointCount: i32, thick: f32, color: Color)
{
    if pointCount < 4 { return; }

    let mut a: [ f32; 4 ] = [ 0.0; 4 ];
    let mut b: [ f32; 4 ] = [ 0.0; 4 ];
    let mut dy: f32 = 0.0;
    let mut dx: f32 = 0.0;
    let mut size: f32 = 0.0;

    let mut currentPoint: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut nextPoint: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut vertices: [ Vector2; ((2*SPLINE_SEGMENT_DIVISIONS + 2) as usize) ] = [ Vector2 { x: 0.0, y: 0.0 }; ((2*SPLINE_SEGMENT_DIVISIONS + 2) as usize) ];

    for i in 0..(pointCount - 3)
    {
        let mut t: f32 = 0.0;
        let p1: Vector2 = points[(i as usize)]; let p2: Vector2 = points[((i + 1) as usize)]; let p3: Vector2 = points[((i + 2) as usize)]; let p4: Vector2 = points[((i + 3) as usize)];

        a[0] = (-p1.x + 3.0*p2.x - 3.0*p3.x + p4.x)/6.0;
        a[1] = (3.0*p1.x - 6.0*p2.x + 3.0*p3.x)/6.0;
        a[2] = (-3.0*p1.x + 3.0*p3.x)/6.0;
        a[3] = (p1.x + 4.0*p2.x + p3.x)/6.0;

        b[0] = (-p1.y + 3.0*p2.y - 3.0*p3.y + p4.y)/6.0;
        b[1] = (3.0*p1.y - 6.0*p2.y + 3.0*p3.y)/6.0;
        b[2] = (-3.0*p1.y + 3.0*p3.y)/6.0;
        b[3] = (p1.y + 4.0*p2.y + p3.y)/6.0;

        currentPoint.x = a[3];
        currentPoint.y = b[3];

        if i == 0 { DrawCircleV(currentPoint, thick/2.0, color); }   // Draw init line circle-cap

        if i > 0
        {
            vertices[0].x = currentPoint.x + dy*size;
            vertices[0].y = currentPoint.y - dx*size;
            vertices[1].x = currentPoint.x - dy*size;
            vertices[1].y = currentPoint.y + dx*size;
        }

        for j in 1..=SPLINE_SEGMENT_DIVISIONS
        {
            t = ((j as f32))/((SPLINE_SEGMENT_DIVISIONS as f32));

            nextPoint.x = a[3] + t*(a[2] + t*(a[1] + t*a[0]));
            nextPoint.y = b[3] + t*(b[2] + t*(b[1] + t*b[0]));

            dy = nextPoint.y - currentPoint.y;
            dx = nextPoint.x - currentPoint.x;
            size = 0.5*thick/(dx*dx+dy*dy).sqrt();

            if (i == 0) && (j == 1)
            {
                vertices[0].x = currentPoint.x + dy*size;
                vertices[0].y = currentPoint.y - dx*size;
                vertices[1].x = currentPoint.x - dy*size;
                vertices[1].y = currentPoint.y + dx*size;
            }

            vertices[((2*j + 1) as usize)].x = nextPoint.x - dy*size;
            vertices[((2*j + 1) as usize)].y = nextPoint.y + dx*size;
            vertices[((2*j) as usize)].x = nextPoint.x + dy*size;
            vertices[((2*j) as usize)].y = nextPoint.y - dx*size;

            currentPoint = nextPoint;
        }

        DrawTriangleStrip(&vertices, 2*SPLINE_SEGMENT_DIVISIONS + 2, color);
    }

    // Cap circle drawing at the end of every segment
    DrawCircleV(currentPoint, thick/2.0, color);
}

// Draw spline: Catmull-Rom, minimum 4 points
pub unsafe fn DrawSplineCatmullRom(points: &[Vector2], pointCount: i32, thick: f32, color: Color)
{
    if pointCount < 4 { return; }

    let mut dy: f32 = 0.0;
    let mut dx: f32 = 0.0;
    let mut size: f32 = 0.0;

    let mut currentPoint: Vector2 = points[1];
    let mut nextPoint: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut vertices: [ Vector2; ((2*SPLINE_SEGMENT_DIVISIONS + 2) as usize) ] = [ Vector2 { x: 0.0, y: 0.0 }; ((2*SPLINE_SEGMENT_DIVISIONS + 2) as usize) ];

    DrawCircleV(currentPoint, thick/2.0, color);   // Draw init line circle-cap

    for i in 0..(pointCount - 3)
    {
        let mut t: f32 = 0.0;
        let p1: Vector2 = points[(i as usize)]; let p2: Vector2 = points[((i + 1) as usize)]; let p3: Vector2 = points[((i + 2) as usize)]; let p4: Vector2 = points[((i + 3) as usize)];

        if i > 0
        {
            vertices[0].x = currentPoint.x + dy*size;
            vertices[0].y = currentPoint.y - dx*size;
            vertices[1].x = currentPoint.x - dy*size;
            vertices[1].y = currentPoint.y + dx*size;
        }

        for j in 1..=SPLINE_SEGMENT_DIVISIONS
        {
            t = ((j as f32))/((SPLINE_SEGMENT_DIVISIONS as f32));

            let q0: f32 = (-t*t*t) + (2.0*t*t) + -t;
            let q1: f32 = (3.0*t*t*t) + (-5.0*t*t) + 2.0;
            let q2: f32 = (-3.0*t*t*t) + (4.0*t*t) + t;
            let q3: f32 = t*t*t - t*t;

            nextPoint.x = 0.5*((p1.x*q0) + (p2.x*q1) + (p3.x*q2) + (p4.x*q3));
            nextPoint.y = 0.5*((p1.y*q0) + (p2.y*q1) + (p3.y*q2) + (p4.y*q3));

            dy = nextPoint.y - currentPoint.y;
            dx = nextPoint.x - currentPoint.x;
            size = (0.5*thick)/(dx*dx + dy*dy).sqrt();

            if (i == 0) && (j == 1)
            {
                vertices[0].x = currentPoint.x + dy*size;
                vertices[0].y = currentPoint.y - dx*size;
                vertices[1].x = currentPoint.x - dy*size;
                vertices[1].y = currentPoint.y + dx*size;
            }

            vertices[((2*j + 1) as usize)].x = nextPoint.x - dy*size;
            vertices[((2*j + 1) as usize)].y = nextPoint.y + dx*size;
            vertices[((2*j) as usize)].x = nextPoint.x + dy*size;
            vertices[((2*j) as usize)].y = nextPoint.y - dx*size;

            currentPoint = nextPoint;
        }

        DrawTriangleStrip(&vertices, 2*SPLINE_SEGMENT_DIVISIONS + 2, color);
    }

    // Cap circle drawing at the end of every segment
    DrawCircleV(currentPoint, thick/2.0, color);
}

// Draw spline: Quadratic Bezier, minimum 3 points (1 control point): [p1, c2, p3, c4...]
pub unsafe fn DrawSplineBezierQuadratic(points: &[Vector2], pointCount: i32, thick: f32, color: Color)
{
    if pointCount >= 3
    {
        for i in (0..pointCount - 2).step_by(2) { DrawSplineSegmentBezierQuadratic(points[(i as usize)], points[((i + 1) as usize)], points[((i + 2) as usize)], thick, color); }

        // Cap circle drawing at the end of every segment
        //for (int i = 2; i < pointCount - 2; i += 2) DrawCircleV(points[i], thick/2.0f, color);
    }
}

// Draw spline: Cubic Bezier, minimum 4 points (2 control points): [p1, c2, c3, p4, c5, c6...]
pub unsafe fn DrawSplineBezierCubic(points: &[Vector2], pointCount: i32, thick: f32, color: Color)
{
    if pointCount >= 4
    {
        for i in (0..pointCount - 3).step_by(3) { DrawSplineSegmentBezierCubic(points[(i as usize)], points[((i + 1) as usize)], points[((i + 2) as usize)], points[((i + 3) as usize)], thick, color); }

        // Cap circle drawing at the end of every segment
        //for (int i = 3; i < pointCount - 3; i += 3) DrawCircleV(points[i], thick/2.0f, color);
    }
}

// Draw spline segment: Linear, 2 points
pub unsafe fn DrawSplineSegmentLinear(p1: Vector2, p2: Vector2, thick: f32, color: Color)
{
    // NOTE: For the linear spline no subdivisions are used, only a single quad

    let delta: Vector2 = Vector2 { x: p2.x - p1.x, y: p2.y - p1.y };
    let length: f32 = (delta.x*delta.x + delta.y*delta.y).sqrt();

    if (length > 0.0) && (thick > 0.0)
    {
        let scale: f32 = thick/(2.0*length);

        let radius: Vector2 = Vector2 { x: -scale*delta.y, y: scale*delta.x };
        let strip: [ Vector2; 4 ] = [
            Vector2 { x: p1.x - radius.x, y: p1.y - radius.y },
            Vector2 { x: p1.x + radius.x, y: p1.y + radius.y },
            Vector2 { x: p2.x - radius.x, y: p2.y - radius.y },
            Vector2 { x: p2.x + radius.x, y: p2.y + radius.y }
        ];

        DrawTriangleStrip(&strip, 4, color);
    }
}

// Draw spline segment: B-Spline, 4 points
pub unsafe fn DrawSplineSegmentBasis(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thick: f32, color: Color)
{
    let step: f32 = 1.0/(SPLINE_SEGMENT_DIVISIONS as f32);

    let mut currentPoint: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut nextPoint: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut t: f32 = 0.0;

    let mut points: [ Vector2; ((2*SPLINE_SEGMENT_DIVISIONS + 2) as usize) ] = [ Vector2 { x: 0.0, y: 0.0 }; ((2*SPLINE_SEGMENT_DIVISIONS + 2) as usize) ];

    let mut a: [ f32; 4 ] = [ 0.0; 4 ];
    let mut b: [ f32; 4 ] = [ 0.0; 4 ];

    a[0] = (-p1.x + 3.0*p2.x - 3.0*p3.x + p4.x)/6.0;
    a[1] = (3.0*p1.x - 6.0*p2.x + 3.0*p3.x)/6.0;
    a[2] = (((-3) as f32)*p1.x + 3.0*p3.x)/6.0;
    a[3] = (p1.x + 4.0*p2.x + p3.x)/6.0;

    b[0] = (-p1.y + 3.0*p2.y - 3.0*p3.y + p4.y)/6.0;
    b[1] = (3.0*p1.y - 6.0*p2.y + 3.0*p3.y)/6.0;
    b[2] = (((-3) as f32)*p1.y + 3.0*p3.y)/6.0;
    b[3] = (p1.y + 4.0*p2.y + p3.y)/6.0;

    currentPoint.x = a[3];
    currentPoint.y = b[3];

    for i in 0..=SPLINE_SEGMENT_DIVISIONS
    {
        t = step*(i as f32);

        nextPoint.x = a[3] + t*(a[2] + t*(a[1] + t*a[0]));
        nextPoint.y = b[3] + t*(b[2] + t*(b[1] + t*b[0]));

        let dy: f32 = nextPoint.y - currentPoint.y;
        let dx: f32 = nextPoint.x - currentPoint.x;
        let size: f32 = (0.5*thick)/(dx*dx + dy*dy).sqrt();

        if i == 1
        {
            points[0].x = currentPoint.x + dy*size;
            points[0].y = currentPoint.y - dx*size;
            points[1].x = currentPoint.x - dy*size;
            points[1].y = currentPoint.y + dx*size;
        }

        points[((2*i + 1) as usize)].x = nextPoint.x - dy*size;
        points[((2*i + 1) as usize)].y = nextPoint.y + dx*size;
        points[((2*i) as usize)].x = nextPoint.x + dy*size;
        points[((2*i) as usize)].y = nextPoint.y - dx*size;

        currentPoint = nextPoint;
    }

    DrawTriangleStrip(&points, 2*SPLINE_SEGMENT_DIVISIONS+2, color);
}

// Draw spline segment: Catmull-Rom, 4 points
pub unsafe fn DrawSplineSegmentCatmullRom(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thick: f32, color: Color)
{
    let step: f32 = 1.0/(SPLINE_SEGMENT_DIVISIONS as f32);

    let mut currentPoint: Vector2 = p1;
    let mut nextPoint: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut t: f32 = 0.0;

    let mut points: [ Vector2; ((2*SPLINE_SEGMENT_DIVISIONS + 2) as usize) ] = [ Vector2 { x: 0.0, y: 0.0 }; ((2*SPLINE_SEGMENT_DIVISIONS + 2) as usize) ];

    for i in 0..=SPLINE_SEGMENT_DIVISIONS
    {
        t = step*(i as f32);

        let q0: f32 = (((-1) as f32)*t*t*t) + (2.0*t*t) + (((-1) as f32)*t);
        let q1: f32 = (3.0*t*t*t) + (((-5) as f32)*t*t) + 2.0;
        let q2: f32 = (((-3) as f32)*t*t*t) + (4.0*t*t) + t;
        let q3: f32 = t*t*t - t*t;

        nextPoint.x = 0.5*((p1.x*q0) + (p2.x*q1) + (p3.x*q2) + (p4.x*q3));
        nextPoint.y = 0.5*((p1.y*q0) + (p2.y*q1) + (p3.y*q2) + (p4.y*q3));

        let dy: f32 = nextPoint.y - currentPoint.y;
        let dx: f32 = nextPoint.x - currentPoint.x;
        let size: f32 = (0.5*thick)/(dx*dx + dy*dy).sqrt();

        if i == 1
        {
            points[0].x = currentPoint.x + dy*size;
            points[0].y = currentPoint.y - dx*size;
            points[1].x = currentPoint.x - dy*size;
            points[1].y = currentPoint.y + dx*size;
        }

        points[((2*i + 1) as usize)].x = nextPoint.x - dy*size;
        points[((2*i + 1) as usize)].y = nextPoint.y + dx*size;
        points[((2*i) as usize)].x = nextPoint.x + dy*size;
        points[((2*i) as usize)].y = nextPoint.y - dx*size;

        currentPoint = nextPoint;
    }

    DrawTriangleStrip(&points, 2*SPLINE_SEGMENT_DIVISIONS + 2, color);
}

// Draw spline segment: Quadratic Bezier, 2 points, 1 control point
pub unsafe fn DrawSplineSegmentBezierQuadratic(p1: Vector2, c2: Vector2, p3: Vector2, thick: f32, color: Color)
{
    let step: f32 = 1.0/(SPLINE_SEGMENT_DIVISIONS as f32);

    let mut previous: Vector2 = p1;
    let mut current: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut t: f32 = 0.0;

    let mut points: [ Vector2; ((2*SPLINE_SEGMENT_DIVISIONS + 2) as usize) ] = [ Vector2 { x: 0.0, y: 0.0 }; ((2*SPLINE_SEGMENT_DIVISIONS + 2) as usize) ];

    for i in 1..=SPLINE_SEGMENT_DIVISIONS
    {
        t = step*(i as f32);

        let a: f32 = (1.0 - t).powf(2.0);
        let b: f32 = 2.0*(1.0 - t)*t;
        let c: f32 = (t).powf(2.0);

        // NOTE: The easing functions aren't suitable here because they don't take a control point
        current.y = a*p1.y + b*c2.y + c*p3.y;
        current.x = a*p1.x + b*c2.x + c*p3.x;

        let dy: f32 = current.y - previous.y;
        let dx: f32 = current.x - previous.x;
        let size: f32 = 0.5*thick/(dx*dx+dy*dy).sqrt();

        if i == 1
        {
            points[0].x = previous.x + dy*size;
            points[0].y = previous.y - dx*size;
            points[1].x = previous.x - dy*size;
            points[1].y = previous.y + dx*size;
        }

        points[((2*i + 1) as usize)].x = current.x - dy*size;
        points[((2*i + 1) as usize)].y = current.y + dx*size;
        points[((2*i) as usize)].x = current.x + dy*size;
        points[((2*i) as usize)].y = current.y - dx*size;

        previous = current;
    }

    DrawTriangleStrip(&points, 2*SPLINE_SEGMENT_DIVISIONS + 2, color);
}

// Draw spline segment: Cubic Bezier, 2 points, 2 control points
pub unsafe fn DrawSplineSegmentBezierCubic(p1: Vector2, c2: Vector2, c3: Vector2, p4: Vector2, thick: f32, color: Color)
{
    let step: f32 = 1.0/(SPLINE_SEGMENT_DIVISIONS as f32);

    let mut previous: Vector2 = p1;
    let mut current: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    let mut t: f32 = 0.0;

    let mut points: [ Vector2; ((2*SPLINE_SEGMENT_DIVISIONS + 2) as usize) ] = [ Vector2 { x: 0.0, y: 0.0 }; ((2*SPLINE_SEGMENT_DIVISIONS + 2) as usize) ];

    for i in 1..=SPLINE_SEGMENT_DIVISIONS
    {
        t = step*(i as f32);

        let a: f32 = (1.0 - t).powf(3.0);
        let b: f32 = 3.0*(1.0 - t).powf(2.0)*t;
        let c: f32 = 3.0*(1.0 - t)*(t).powf(2.0);
        let d: f32 = (t).powf(3.0);

        current.y = a*p1.y + b*c2.y + c*c3.y + d*p4.y;
        current.x = a*p1.x + b*c2.x + c*c3.x + d*p4.x;

        let dy: f32 = current.y - previous.y;
        let dx: f32 = current.x - previous.x;
        let size: f32 = 0.5*thick/(dx*dx+dy*dy).sqrt();

        if i == 1
        {
            points[0].x = previous.x + dy*size;
            points[0].y = previous.y - dx*size;
            points[1].x = previous.x - dy*size;
            points[1].y = previous.y + dx*size;
        }

        points[((2*i + 1) as usize)].x = current.x - dy*size;
        points[((2*i + 1) as usize)].y = current.y + dx*size;
        points[((2*i) as usize)].x = current.x + dy*size;
        points[((2*i) as usize)].y = current.y - dx*size;

        previous = current;
    }

    DrawTriangleStrip(&points, 2*SPLINE_SEGMENT_DIVISIONS + 2, color);
}

// Get spline point for a given t [0.0f .. 1.0f], Linear
pub fn GetSplinePointLinear(startPos: Vector2, endPos: Vector2, t: f32) -> Vector2
{
    let mut point: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    point.x = startPos.x*(1.0 - t) + endPos.x*t;
    point.y = startPos.y*(1.0 - t) + endPos.y*t;

    return point;
}

// Get spline point for a given t [0.0f .. 1.0f], B-Spline
pub fn GetSplinePointBasis(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, t: f32) -> Vector2
{
    let mut point: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    let mut a: [ f32; 4 ] = [ 0.0; 4 ];
    let mut b: [ f32; 4 ] = [ 0.0; 4 ];

    a[0] = (-p1.x + 3.0*p2.x - 3.0*p3.x + p4.x)/6.0;
    a[1] = (3.0*p1.x - 6.0*p2.x + 3.0*p3.x)/6.0;
    a[2] = (((-3) as f32)*p1.x + 3.0*p3.x)/6.0;
    a[3] = (p1.x + 4.0*p2.x + p3.x)/6.0;

    b[0] = (-p1.y + 3.0*p2.y - 3.0*p3.y + p4.y)/6.0;
    b[1] = (3.0*p1.y - 6.0*p2.y + 3.0*p3.y)/6.0;
    b[2] = (((-3) as f32)*p1.y + 3.0*p3.y)/6.0;
    b[3] = (p1.y + 4.0*p2.y + p3.y)/6.0;

    point.x = a[3] + t*(a[2] + t*(a[1] + t*a[0]));
    point.y = b[3] + t*(b[2] + t*(b[1] + t*b[0]));

    return point;
}

// Get spline point for a given t [0.0f .. 1.0f], Catmull-Rom
pub fn GetSplinePointCatmullRom(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, t: f32) -> Vector2
{
    let mut point: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    let q0: f32 = (((-1) as f32)*t*t*t) + (2.0*t*t) + (((-1) as f32)*t);
    let q1: f32 = (3.0*t*t*t) + (((-5) as f32)*t*t) + 2.0;
    let q2: f32 = (((-3) as f32)*t*t*t) + (4.0*t*t) + t;
    let q3: f32 = t*t*t - t*t;

    point.x = 0.5*((p1.x*q0) + (p2.x*q1) + (p3.x*q2) + (p4.x*q3));
    point.y = 0.5*((p1.y*q0) + (p2.y*q1) + (p3.y*q2) + (p4.y*q3));

    return point;
}

// Get spline point for a given t [0.0f .. 1.0f], Quadratic Bezier
pub fn GetSplinePointBezierQuadratic(startPos: Vector2, controlPos: Vector2, endPos: Vector2, t: f32) -> Vector2
{
    let mut point: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    let a: f32 = (1.0 - t).powf(2.0);
    let b: f32 = 2.0*(1.0 - t)*t;
    let c: f32 = (t).powf(2.0);

    point.y = a*startPos.y + b*controlPos.y + c*endPos.y;
    point.x = a*startPos.x + b*controlPos.x + c*endPos.x;

    return point;
}

// Get spline point for a given t [0.0f .. 1.0f], Cubic Bezier
pub fn GetSplinePointBezierCubic(startPos: Vector2, startControlPos: Vector2, endControlPos: Vector2, endPos: Vector2, t: f32) -> Vector2
{
    let mut point: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    let a: f32 = (1.0 - t).powf(3.0);
    let b: f32 = 3.0*(1.0 - t).powf(2.0)*t;
    let c: f32 = 3.0*(1.0 - t)*(t).powf(2.0);
    let d: f32 = (t).powf(3.0);

    point.y = a*startPos.y + b*startControlPos.y + c*endControlPos.y + d*endPos.y;
    point.x = a*startPos.x + b*startControlPos.x + c*endControlPos.x + d*endPos.x;

    return point;
}

//----------------------------------------------------------------------------------
// Module Functions Definition - Collision Detection functions
//----------------------------------------------------------------------------------

// Check if point is inside rectangle
pub fn CheckCollisionPointRec(point: Vector2, rec: Rectangle) -> bool
{
    let mut collision: bool = false;

    if (point.x >= rec.x) && (point.x < (rec.x + rec.width)) && (point.y >= rec.y) && (point.y < (rec.y + rec.height)) { collision = true; }

    return collision;
}

// Check if point is inside circle
pub fn CheckCollisionPointCircle(point: Vector2, center: Vector2, radius: f32) -> bool
{
    let mut collision: bool = false;

    let distanceSquared: f32 = (point.x - center.x)*(point.x - center.x) + (point.y - center.y)*(point.y - center.y);

    if distanceSquared <= radius*radius { collision = true; }

    return collision;
}

// Check if point is inside a triangle defined by three points (p1, p2, p3)
pub fn CheckCollisionPointTriangle(point: Vector2, p1: Vector2, p2: Vector2, p3: Vector2) -> bool
{
    let mut collision: bool = false;

    let alpha: f32 = ((p2.y - p3.y)*(point.x - p3.x) + (p3.x - p2.x)*(point.y - p3.y)) /
                  ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y));

    let beta: f32 = ((p3.y - p1.y)*(point.x - p3.x) + (p1.x - p3.x)*(point.y - p3.y)) /
                 ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y));

    let gamma: f32 = 1.0 - alpha - beta;

    if (alpha > 0.0) && (beta > 0.0) && (gamma > 0.0) { collision = true; }

    return collision;
}

// Check if point is within a polygon described by array of vertices
// NOTE: Based on http://jeffreythompson.org/collision-detection/poly-point.php
pub fn CheckCollisionPointPoly(point: Vector2, points: &[Vector2], pointCount: i32) -> bool
{
    let mut collision: bool = false;

    if pointCount > 2
    {
        let mut i: i32 = 0;
        let mut j: i32 = pointCount - 1;
        while i < pointCount
        {
            if (points[(i as usize)].y > point.y) != (points[(j as usize)].y > point.y) &&
                (point.x < (points[(j as usize)].x - points[(i as usize)].x)*(point.y - points[(i as usize)].y)/(points[(j as usize)].y - points[(i as usize)].y) + points[(i as usize)].x)
            {
                collision = !collision;
            }
            j = i;
            i += 1;
        }
    }

    return collision;
}

// Check collision between two rectangles
pub fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle) -> bool
{
    let mut collision: bool = false;

    if (rec1.x < (rec2.x + rec2.width) && (rec1.x + rec1.width) > rec2.x) &&
        (rec1.y < (rec2.y + rec2.height) && (rec1.y + rec1.height) > rec2.y) { collision = true; }

    return collision;
}

// Check collision between two circles
pub fn CheckCollisionCircles(center1: Vector2, radius1: f32, center2: Vector2, radius2: f32) -> bool
{
    let mut collision: bool = false;

    let dx: f32 = center2.x - center1.x;      // X distance between centers
    let dy: f32 = center2.y - center1.y;      // Y distance between centers

    let distanceSquared: f32 = dx*dx + dy*dy; // Distance between centers squared
    let radiusSum: f32 = radius1 + radius2;

    collision = (distanceSquared <= (radiusSum*radiusSum));

    return collision;
}

// Check collision between circle and rectangle
// NOTE: Reviewed version to take into account corner limit case
pub fn CheckCollisionCircleRec(center: Vector2, radius: f32, rec: Rectangle) -> bool
{
    let mut collision: bool = false;

    let recCenterX: f32 = rec.x + rec.width/2.0;
    let recCenterY: f32 = rec.y + rec.height/2.0;

    let dx: f32 = (center.x - recCenterX).abs();
    let dy: f32 = (center.y - recCenterY).abs();

    if (dx <= (rec.width/2.0 + radius)) && (dy <= (rec.height/2.0 + radius))
    {
        if dx <= (rec.width/2.0) { collision = true; }
        else if dy <= (rec.height/2.0) { collision = true; }
        else
        {
            let cornerDistanceSq: f32 = (dx - rec.width/2.0)*(dx - rec.width/2.0) +
                (dy - rec.height/2.0)*(dy - rec.height/2.0);

            collision = (cornerDistanceSq <= (radius*radius));
        }
    }

    return collision;
}

// Check the collision between two lines defined by two points each, returns collision point by reference
// REF: https://en.wikipedia.org/wiki/Line–line_intersection#Given_two_points_on_each_line_segment
pub unsafe fn CheckCollisionLines(startPos1: Vector2, endPos1: Vector2, startPos2: Vector2, endPos2: Vector2, collisionPoint: *mut Vector2) -> bool
{
    let mut collision: bool = false;

    let rx: f32 = endPos1.x - startPos1.x;
    let ry: f32 = endPos1.y - startPos1.y;
    let sx: f32 = endPos2.x - startPos2.x;
    let sy: f32 = endPos2.y - startPos2.y;

    let div: f32 = rx*sy - ry*sx;

    if (div).abs() >= f32::EPSILON
    {
        let s12x: f32 = startPos2.x - startPos1.x;
        let s12y: f32 = startPos2.y - startPos1.y;

        let t: f32 = (s12x*sy - s12y*sx)/div;
        let u: f32 = (s12x*ry - s12y*rx)/div;

        if (0.0 <= t) && (t <= 1.0) && (0.0 <= u) && (u <= 1.0)
        {
            if !collisionPoint.is_null()
            {
                (*collisionPoint).x = startPos1.x + t*rx;
                (*collisionPoint).y = startPos1.y + t*ry;
            }

            collision = true;
        }
    }

    return collision;
}

// Check if point belongs to line created between two points [p1] and [p2] with defined margin in pixels [threshold]
pub fn CheckCollisionPointLine(point: Vector2, p1: Vector2, p2: Vector2, threshold: i32) -> bool
{
    let mut collision: bool = false;

    let dxc: f32 = point.x - p1.x;
    let dyc: f32 = point.y - p1.y;
    let dxl: f32 = p2.x - p1.x;
    let dyl: f32 = p2.y - p1.y;
    let cross: f32 = dxc*dyl - dyc*dxl;

    if (cross).abs() < ((threshold as f32)*((dxl).abs()).max((dyl).abs()))
    {
        if (dxl).abs() >= (dyl).abs() { collision = if (dxl > 0.0) { ((p1.x <= point.x) && (point.x <= p2.x)) } else { ((p2.x <= point.x) && (point.x <= p1.x)) }; }
        else { collision = if (dyl > 0.0) { ((p1.y <= point.y) && (point.y <= p2.y)) } else { ((p2.y <= point.y) && (point.y <= p1.y)) }; }
    }

    return collision;
}

// Check if circle collides with a line created between two points [p1] and [p2]
pub fn CheckCollisionCircleLine(center: Vector2, radius: f32, p1: Vector2, p2: Vector2) -> bool
{
    let mut collision: bool = false;

    let dx: f32 = p1.x - p2.x;
    let dy: f32 = p1.y - p2.y;

    if ((dx).abs() + (dy).abs()) <= f32::EPSILON
    {
        collision = CheckCollisionCircles(p1, 0.0, center, radius);
    }
    else
    {
        let lengthSQ: f32 = ((dx*dx) + (dy*dy));
        let mut dotProduct: f32 = (((center.x - p1.x)*(p2.x - p1.x)) + ((center.y - p1.y)*(p2.y - p1.y)))/(lengthSQ);

        if dotProduct > 1.0 { dotProduct = 1.0; }
        else if dotProduct < 0.0 { dotProduct = 0.0; }

        let dx2: f32 = (p1.x - (dotProduct*(dx))) - center.x;
        let dy2: f32 = (p1.y - (dotProduct*(dy))) - center.y;
        let distanceSQ: f32 = ((dx2*dx2) + (dy2*dy2));

        if distanceSQ <= radius*radius { collision = true; }
    }

    return collision;
}

// Get collision rectangle for two rectangles collision
pub fn GetCollisionRec(rec1: Rectangle, rec2: Rectangle) -> Rectangle
{
    let mut overlap: Rectangle = Rectangle { x: 0.0, y: 0.0, width: 0.0, height: 0.0 };

    let left: f32 = if (rec1.x > rec2.x) { rec1.x } else { rec2.x };
    let right1: f32 = rec1.x + rec1.width;
    let right2: f32 = rec2.x + rec2.width;
    let right: f32 = if (right1 < right2) { right1 } else { right2 };
    let top: f32 = if (rec1.y > rec2.y) { rec1.y } else { rec2.y };
    let bottom1: f32 = rec1.y + rec1.height;
    let bottom2: f32 = rec2.y + rec2.height;
    let bottom: f32 = if (bottom1 < bottom2) { bottom1 } else { bottom2 };

    if (left < right) && (top < bottom)
    {
        overlap.x = left;
        overlap.y = top;
        overlap.width = right - left;
        overlap.height = bottom - top;
    }

    return overlap;
}

//----------------------------------------------------------------------------------
// Module Internal Functions Definition
//----------------------------------------------------------------------------------

// Cubic easing in-out
// NOTE: Used by DrawLineBezier() only
fn EaseCubicInOut(mut t: f32, b: f32, c: f32, d: f32) -> f32
{
    let mut result: f32 = 0.0;

    if ({ t /= 0.5*d; t }) < 1.0 { result = 0.5*c*t*t*t + b; }
    else
    {
        t -= 2.0;
        result = 0.5*c*(t*t*t + 2.0) + b;
    }

    return result;
}
