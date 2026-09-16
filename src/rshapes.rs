#![allow(non_snake_case, non_upper_case_globals)]
#![cfg_attr(rustfmt, rustfmt_skip)]
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
    DrawPixelV(Vector2 { x: posX as f32, y: posY as f32 }, color);
}

// Draw a pixel (Vector version)
pub unsafe fn DrawPixelV(position: Vector2, color: Color)
{
    rlSetTexture(GetShapesTexture().id);
    let shapeRect = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);

        rlNormal3f(0.0, 0.0, 1.0);
        rlColor4ub(color.r, color.g, color.b, color.a);

        rlTexCoord2f(shapeRect.x/texShapes.width as f32, shapeRect.y/texShapes.height as f32);
        rlVertex2f(position.x, position.y);

        rlTexCoord2f(shapeRect.x/texShapes.width as f32, (shapeRect.y + shapeRect.height)/texShapes.height as f32);
        rlVertex2f(position.x, position.y + 1.0);

        rlTexCoord2f((shapeRect.x + shapeRect.width)/texShapes.width as f32, (shapeRect.y + shapeRect.height)/texShapes.height as f32);
        rlVertex2f(position.x + 1.0, position.y + 1.0);

        rlTexCoord2f((shapeRect.x + shapeRect.width)/texShapes.width as f32, shapeRect.y/texShapes.height as f32);
        rlVertex2f(position.x + 1.0, position.y);

    rlEnd();

    rlSetTexture(0);
}

// Draw a line (using gl lines)
pub unsafe fn DrawLine(startPosX: i32, startPosY: i32, endPosX: i32, endPosY: i32, color: Color)
{
    rlBegin(RL_LINES);
        rlColor4ub(color.r, color.g, color.b, color.a);
        rlVertex2f(startPosX as f32, startPosY as f32);
        rlVertex2f(endPosX as f32, endPosY as f32);
    rlEnd();
}

// Draw a line defining thickness
pub unsafe fn DrawLineEx(startPos: Vector2, endPos: Vector2, thick: f32, color: Color)
{
    let delta = Vector2 { x: endPos.x - startPos.x, y: endPos.y - startPos.y };
    let length = (delta.x*delta.x + delta.y*delta.y).sqrt();

    if (length > 0.0) && (thick > 0.0)
    {
        let scale = thick/(2.0*length);

        let radius = Vector2 { x: -scale*delta.y, y: scale*delta.x };
        let strip = [
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
            rlVertex2f(points[i as usize].x, points[i as usize].y);
            rlVertex2f(points[(i + 1) as usize].x, points[(i + 1) as usize].y);
        }
    rlEnd();
}

// Draw line using cubic-bezier spline, in-out interpolation, no control points
pub unsafe fn DrawLineBezier(startPos: Vector2, endPos: Vector2, thick: f32, color: Color)
{
    let mut previous = startPos;
    let mut current = Vector2::default();

    let mut points = [Vector2 { x: 0.0, y: 0.0 }; (2*SPLINE_SEGMENT_DIVISIONS + 2) as usize];

    for i in 1..=SPLINE_SEGMENT_DIVISIONS
    {
        // Cubic easing in-out
        // NOTE: Easing is calculated only for y position value
        current.y = EaseCubicInOut(i as f32, startPos.y, endPos.y - startPos.y, SPLINE_SEGMENT_DIVISIONS as f32);
        current.x = previous.x + (endPos.x - startPos.x)/SPLINE_SEGMENT_DIVISIONS as f32;

        let dy = current.y - previous.y;
        let dx = current.x - previous.x;
        let size = 0.5*thick/(dx*dx + dy*dy).sqrt();

        if i == 1
        {
            points[0].x = previous.x + dy*size;
            points[0].y = previous.y - dx*size;
            points[1].x = previous.x - dy*size;
            points[1].y = previous.y + dx*size;
        }

        points[(2*i + 1) as usize].x = current.x - dy*size;
        points[(2*i + 1) as usize].y = current.y + dx*size;
        points[(2*i) as usize].x = current.x + dy*size;
        points[(2*i) as usize].y = current.y - dx*size;

        previous = current;
    }

    DrawTriangleStrip(&points, 2*SPLINE_SEGMENT_DIVISIONS + 2, color);
}

// Draw a dashed line
pub unsafe fn DrawLineDashed(startPos: Vector2, endPos: Vector2, dashSize: i32, spaceSize: i32, color: Color)
{
    // Calculate the vector and length of the line
    let dx = endPos.x - startPos.x;
    let dy = endPos.y - startPos.y;
    let lineLength = (dx*dx + dy*dy).sqrt();

    // If the line is too short for dashing or dash size is invalid, draw a solid thick line
    if (lineLength < (dashSize + spaceSize) as f32) || (dashSize <= 0)
    {
        DrawLineV(startPos, endPos, color);
        return;
    }

    // Calculate the normalized direction vector of the line
    let invLineLength = 1.0/lineLength;
    let dirX = dx*invLineLength;
    let dirY = dy*invLineLength;

    let mut currentPos = startPos;
    let mut distanceTraveled = 0.0;

    rlBegin(RL_LINES);
        rlColor4ub(color.r, color.g, color.b, color.a);

        while distanceTraveled < lineLength
        {
            // Calculate the end of the current dash
            let mut dashEndDist = distanceTraveled + dashSize as f32;
            if dashEndDist > lineLength { dashEndDist = lineLength; }

            let dashEndPos = Vector2 { x: startPos.x + dashEndDist*dirX, y: startPos.y + dashEndDist*dirY };

            // Draw the dash segment
            rlVertex2f(currentPos.x, currentPos.y);
            rlVertex2f(dashEndPos.x, dashEndPos.y);

            // Update the distance traveled and move the current position for the next dash
            distanceTraveled = dashEndDist + spaceSize as f32;
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
    let shapeRect = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);
        rlNormal3f(0.0, 0.0, 1.0);

        rlColor4ub(c1.r, c1.g, c1.b, c1.a);
        rlTexCoord2f(shapeRect.x/texShapes.width as f32, shapeRect.y/texShapes.height as f32);
        rlVertex2f(v1.x, v1.y);

        rlColor4ub(c2.r, c2.g, c2.b, c2.a);
        rlTexCoord2f(shapeRect.x/texShapes.width as f32, (shapeRect.y + shapeRect.height)/texShapes.height as f32);
        rlVertex2f(v2.x, v2.y);

        rlColor4ub(c3.r, c3.g, c3.b, c3.a);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/texShapes.width as f32, (shapeRect.y + shapeRect.height)/texShapes.height as f32);
        rlVertex2f(v3.x, v3.y);

        rlColor4ub(c3.r, c3.g, c3.b, c3.a);
        rlTexCoord2f((shapeRect.x + shapeRect.width)/texShapes.width as f32, shapeRect.y/texShapes.height as f32);
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

    let e1 = Vector2 { x: v2.x - v3.x, y: v2.y - v3.y };
    let e2 = Vector2 { x: v3.x - v1.x, y: v3.y - v1.y };
    let e3 = Vector2 { x: v1.x - v2.x, y: v1.y - v2.y };

    let e1Length = (e1.x*e1.x + e1.y*e1.y).sqrt();
    let e2Length = (e2.x*e2.x + e2.y*e2.y).sqrt();
    let e3Length = (e3.x*e3.x + e3.y*e3.y).sqrt();

    let perimeter = e1Length + e2Length + e3Length;
    let semiperimeter = perimeter/2.0;

    // The incenter of a triangle is equidistant from each edge, which is useful for drawing a nice looking outline
    let incenter = Vector2 {
        x: (e1Length*v1.x + e2Length*v2.x + e3Length*v3.x)/perimeter,
        y: (e1Length*v1.y + e2Length*v2.y + e3Length*v3.y)/perimeter
    };

    // The inradius of a triangle is the radius of the biggest circle that can fit inside of said triangle
    // That circle is also centered on the incenter
    let inradius = (((semiperimeter - e1Length)*(semiperimeter - e2Length)*(semiperimeter - e3Length))/semiperimeter).sqrt();

    // The triangle (v1, v2, v3) will be scaled by this to get (v4, v5, v6)
    let scale = 1.0 - thick/inradius;

    // Just a filled-in triangle
    if scale <= 0.0
    {
        DrawTriangle(v1, v2, v3, color);
        return;
    }

    // In order for the scaling to be correct, the incenter has to be at the origin (0, 0) when scaling
    let mut v4 = Vector2 { x: incenter.x + (v1.x - incenter.x)*scale, y: incenter.y + (v1.y - incenter.y)*scale };
    let mut v5 = Vector2 { x: incenter.x + (v2.x - incenter.x)*scale, y: incenter.y + (v2.y - incenter.y)*scale };
    let mut v6 = Vector2 { x: incenter.x + (v3.x - incenter.x)*scale, y: incenter.y + (v3.y - incenter.y)*scale };

    // Swap the vertices so the winding order is correct
    if thick < 0.0
    {
        std::mem::swap(&mut v1, &mut v4);
        std::mem::swap(&mut v2, &mut v5);
        std::mem::swap(&mut v3, &mut v6);
    }

    let strip = [v1, v4, v2, v5, v3, v6, v1, v4];
    DrawTriangleStrip(&strip, strip.len() as i32, color);
}

// Draw a triangle fan defined by points
// NOTE: First vertex provided is the center, shared by all triangles
// By default, following vertex should be provided in counter-clockwise order
pub unsafe fn DrawTriangleFan(points: &[Vector2], pointCount: i32, color: Color)
{
    if pointCount >= 3
    {
        rlSetTexture(GetShapesTexture().id);
        let shapeRect = GetShapesTextureRectangle();

        rlBegin(RL_QUADS);
            rlColor4ub(color.r, color.g, color.b, color.a);

            for i in 1..pointCount - 1
            {
                rlTexCoord2f(shapeRect.x/texShapes.width as f32, shapeRect.y/texShapes.height as f32);
                rlVertex2f(points[0].x, points[0].y);

                rlTexCoord2f(shapeRect.x/texShapes.width as f32, (shapeRect.y + shapeRect.height)/texShapes.height as f32);
                rlVertex2f(points[i as usize].x, points[i as usize].y);

                rlTexCoord2f((shapeRect.x + shapeRect.width)/texShapes.width as f32, (shapeRect.y + shapeRect.height)/texShapes.height as f32);
                rlVertex2f(points[(i + 1) as usize].x, points[(i + 1) as usize].y);

                rlTexCoord2f((shapeRect.x + shapeRect.width)/texShapes.width as f32, shapeRect.y/texShapes.height as f32);
                rlVertex2f(points[(i + 1) as usize].x, points[(i + 1) as usize].y);
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
                    rlVertex2f(points[i as usize].x, points[i as usize].y);
                    rlVertex2f(points[(i - 2) as usize].x, points[(i - 2) as usize].y);
                    rlVertex2f(points[(i - 1) as usize].x, points[(i - 1) as usize].y);
                }
                else
                {
                    rlVertex2f(points[i as usize].x, points[i as usize].y);
                    rlVertex2f(points[(i - 1) as usize].x, points[(i - 1) as usize].y);
                    rlVertex2f(points[(i - 2) as usize].x, points[(i - 2) as usize].y);
                }
            }
        rlEnd();
    }
}

// Draw a color-filled rectangle
pub unsafe fn DrawRectangle(posX: i32, posY: i32, width: i32, height: i32, color: Color)
{
    DrawRectangleV(Vector2 { x: posX as f32, y: posY as f32 }, Vector2 { x: width as f32, y: height as f32 }, color);
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
    let mut topLeft = Vector2::default();
    let mut topRight = Vector2::default();
    let mut bottomLeft = Vector2::default();
    let mut bottomRight = Vector2::default();

    // Only calculate rotation if needed
    if rotation == 0.0
    {
        let x = rec.x - origin.x;
        let y = rec.y - origin.y;
        topLeft = Vector2 { x, y };
        topRight = Vector2 { x: x + rec.width, y };
        bottomLeft = Vector2 { x, y: y + rec.height };
        bottomRight = Vector2 { x: x + rec.width, y: y + rec.height };
    }
    else
    {
        let sinRotation = (rotation*DEG2RAD).sin();
        let cosRotation = (rotation*DEG2RAD).cos();
        let x = rec.x;
        let y = rec.y;
        let dx = -origin.x;
        let dy = -origin.y;

        topLeft.x = x + dx*cosRotation - dy*sinRotation;
        topLeft.y = y + dx*sinRotation + dy*cosRotation;

        topRight.x = x + (dx + rec.width)*cosRotation - dy*sinRotation;
        topRight.y = y + (dx + rec.width)*sinRotation + dy*cosRotation;

        bottomLeft.x = x + dx*cosRotation - (dy + rec.height)*sinRotation;
        bottomLeft.y = y + dx*sinRotation + (dy + rec.height)*cosRotation;

        bottomRight.x = x + (dx + rec.width)*cosRotation - (dy + rec.height)*sinRotation;
        bottomRight.y = y + (dx + rec.width)*sinRotation + (dy + rec.height)*cosRotation;
    }

    let vertices = [topLeft, bottomLeft, bottomRight, topRight];
    DrawTexturedQuad(vertices, [color; 4]);
}

// Draw a vertical-gradient-filled rectangle
pub unsafe fn DrawRectangleGradientV(posX: i32, posY: i32, width: i32, height: i32, top: Color, bottom: Color)
{
    DrawRectangleGradientEx(Rectangle { x: posX as f32, y: posY as f32, width: width as f32, height: height as f32 }, top, bottom, bottom, top);
}

// Draw a horizontal-gradient-filled rectangle
pub unsafe fn DrawRectangleGradientH(posX: i32, posY: i32, width: i32, height: i32, left: Color, right: Color)
{
    DrawRectangleGradientEx(Rectangle { x: posX as f32, y: posY as f32, width: width as f32, height: height as f32 }, left, left, right, right);
}

// Draw a gradient-filled rectangle with custom vertex colors, counter-clockwise color order
pub unsafe fn DrawRectangleGradientEx(rec: Rectangle, col1: Color, col2: Color, col3: Color, col4: Color)
{
    DrawTexturedQuad([
        Vector2 { x: rec.x, y: rec.y },
        Vector2 { x: rec.x, y: rec.y + rec.height },
        Vector2 { x: rec.x + rec.width, y: rec.y + rec.height },
        Vector2 { x: rec.x + rec.width, y: rec.y }
    ], [col1, col2, col3, col4]);
}

// Draw rectangle outline
// WARNING: All Draw*Lines() functions use RL_LINES for drawing,
// it implies flushing the current batch and changing draw mode to RL_LINES
// but it solves another issue: https://github.com/raysan5/raylib/issues/3884
pub unsafe fn DrawRectangleLines(posX: i32, posY: i32, width: i32, height: i32, color: Color)
{
    let mat = rlGetMatrixTransform();
    let xOffset = 0.5/mat.m0;
    let yOffset = 0.5/mat.m5;

    rlBegin(RL_LINES);
        rlColor4ub(color.r, color.g, color.b, color.a);
        rlVertex2f(posX as f32 + xOffset, posY as f32 + yOffset);
        rlVertex2f(posX as f32 + width as f32 - xOffset, posY as f32 + yOffset);

        rlVertex2f(posX as f32 + width as f32 - xOffset, posY as f32 + yOffset);
        rlVertex2f(posX as f32 + width as f32 - xOffset, posY as f32 + height as f32 - yOffset);

        rlVertex2f(posX as f32 + width as f32 - xOffset, posY as f32 + height as f32 - yOffset);
        rlVertex2f(posX as f32 + xOffset, posY as f32 + height as f32 - yOffset);

        rlVertex2f(posX as f32 + xOffset, posY as f32 + height as f32 - yOffset);
        rlVertex2f(posX as f32 + xOffset, posY as f32 + yOffset);
    rlEnd();
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
        let top = Rectangle { x: rec.x, y: rec.y, width: rec.width, height: thick };
        let bottom = Rectangle { x: rec.x, y: rec.y - thick + rec.height, width: rec.width, height: thick };
        let left = Rectangle { x: rec.x, y: rec.y + thick, width: thick, height: rec.height - thick*2.0 };
        let right = Rectangle { x: rec.x - thick + rec.width, y: rec.y + thick, width: thick, height: rec.height - thick*2.0 };

        DrawRectangleRec(top, color);
        DrawRectangleRec(bottom, color);
        DrawRectangleRec(left, color);
        DrawRectangleRec(right, color);
    }
    else
    {
        thick *= -1.0;

        let top = Rectangle { x: rec.x - thick, y: rec.y - thick, width: rec.width + thick*2.0, height: thick };
        let bottom = Rectangle { x: rec.x - thick, y: rec.y + rec.height, width: rec.width + thick*2.0, height: thick };
        let left = Rectangle { x: rec.x - thick, y: rec.y, width: thick, height: rec.height };
        let right = Rectangle { x: rec.x + rec.width, y: rec.y, width: thick, height: rec.height };

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
    let radius = if rec.width > rec.height { (rec.height*roundness)/2.0 } else { (rec.width*roundness)/2.0 };
    if radius <= 0.0 { return; }

    // Calculate number of segments to use for the corners
    if segments < 4
    {
        // Calculate the maximum angle between segments based on the error rate (usually 0.5f)
        let th = (2.0*(1.0 - SMOOTH_CIRCLE_ERROR_RATE/radius).powi(2) - 1.0).acos();
        segments = ((2.0*PI/th)/4.0).ceil() as i32;
        if segments <= 0 { segments = 4; }
    }

    let mut points = Vec::with_capacity((segments*4 + 1) as usize);
    let centers = [
        Vector2 { x: rec.x + radius, y: rec.y + radius },
        Vector2 { x: rec.x + rec.width - radius, y: rec.y + radius },
        Vector2 { x: rec.x + rec.width - radius, y: rec.y + rec.height - radius },
        Vector2 { x: rec.x + radius, y: rec.y + rec.height - radius }
    ];
    let angles = [180.0, 270.0, 0.0, 90.0];
    for k in 0..4
    {
        for i in 0..=segments
        {
            let angle = (angles[k] + 90.0*i as f32/segments as f32)*DEG2RAD;
            points.push(Vector2 { x: centers[k].x + angle.cos()*radius, y: centers[k].y + angle.sin()*radius });
        }
    }
    let center = Vector2 { x: rec.x + rec.width/2.0, y: rec.y + rec.height/2.0 };
    let mut fan = Vec::with_capacity(points.len() + 2);
    fan.push(center);
    fan.extend_from_slice(&points);
    fan.push(points[0]);
    DrawTriangleFan(&fan, fan.len() as i32, color);
}

// Draw rectangle with rounded edges
pub unsafe fn DrawRectangleRoundedLines(rec: Rectangle, mut roundness: f32, mut segments: i32, color: Color)
{
    // Not a rounded rectangle
    if roundness <= 0.0
    {
        DrawRectangleLines(rec.x as i32, rec.y as i32, rec.width as i32, rec.height as i32, color);
        return;
    }

    if roundness >= 1.0 { roundness = 1.0; }
    let radius = if rec.width > rec.height { (rec.height*roundness)/2.0 } else { (rec.width*roundness)/2.0 };
    if radius <= 0.0 { return; }

    if segments < 4
    {
        let th = (2.0*(1.0 - SMOOTH_CIRCLE_ERROR_RATE/radius).powi(2) - 1.0).acos();
        segments = ((2.0*PI/th)/4.0).ceil() as i32;
        if segments <= 0 { segments = 4; }
    }

    let centers = [
        Vector2 { x: rec.x + radius + 0.5, y: rec.y + radius + 0.5 },
        Vector2 { x: rec.x + rec.width - radius - 0.5, y: rec.y + radius + 0.5 },
        Vector2 { x: rec.x + rec.width - radius - 0.5, y: rec.y + rec.height - radius - 0.5 },
        Vector2 { x: rec.x + radius + 0.5, y: rec.y + rec.height - radius - 0.5 }
    ];
    let angles = [180.0, 270.0, 0.0, 90.0];

    rlBegin(RL_LINES);
        rlColor4ub(color.r, color.g, color.b, color.a);
        for k in 0..4
        {
            let mut angle = angles[k];
            for _ in 0..segments
            {
                rlVertex2f(centers[k].x + (DEG2RAD*angle).cos()*radius, centers[k].y + (DEG2RAD*angle).sin()*radius);
                rlVertex2f(centers[k].x + (DEG2RAD*(angle + 90.0/segments as f32)).cos()*radius, centers[k].y + (DEG2RAD*(angle + 90.0/segments as f32)).sin()*radius);
                angle += 90.0/segments as f32;
            }
        }
        let point = [
            Vector2 { x: centers[0].x, y: rec.y + 0.5 }, Vector2 { x: centers[1].x, y: rec.y + 0.5 },
            Vector2 { x: rec.x + rec.width - 0.5, y: centers[1].y }, Vector2 { x: rec.x + rec.width - 0.5, y: centers[2].y },
            Vector2 { x: centers[2].x, y: rec.y + rec.height - 0.5 }, Vector2 { x: centers[3].x, y: rec.y + rec.height - 0.5 },
            Vector2 { x: rec.x + 0.5, y: centers[3].y }, Vector2 { x: rec.x + 0.5, y: centers[0].y }
        ];
        for i in (0..8).step_by(2)
        {
            rlVertex2f(point[i].x, point[i].y);
            rlVertex2f(point[i + 1].x, point[i + 1].y);
        }
    rlEnd();
}

// Draw rectangle with rounded edges outline with line thickness
pub unsafe fn DrawRectangleRoundedLinesEx(mut rec: Rectangle, roundness: f32, segments: i32, mut thick: f32, color: Color)
{
    // Not a rounded rectangle
    if roundness <= 0.0
    {
        DrawRectangleLinesEx(rec, thick, color);
        return;
    }

    let mut actualRoundness = roundness;
    if actualRoundness >= 1.0 { actualRoundness = 1.0; }

    let radius = if rec.width > rec.height { rec.height*actualRoundness/2.0 } else { rec.width*actualRoundness/2.0 };
    if radius <= 0.0 { return; }

    let innerRadius;
    let outerRadius;
    if thick >= 0.0
    {
        outerRadius = radius;
        innerRadius = (outerRadius - thick).max(0.0);
    }
    else
    {
        thick *= -1.0;
        rec.x -= thick;
        rec.y -= thick;
        rec.width += thick*2.0;
        rec.height += thick*2.0;
        innerRadius = radius;
        outerRadius = innerRadius + thick;
    }

    let centers = [
        Vector2 { x: rec.x + outerRadius, y: rec.y + outerRadius },
        Vector2 { x: rec.x + rec.width - outerRadius, y: rec.y + outerRadius },
        Vector2 { x: rec.x + rec.width - outerRadius, y: rec.y + rec.height - outerRadius },
        Vector2 { x: rec.x + outerRadius, y: rec.y + rec.height - outerRadius }
    ];

    DrawRing(centers[0], innerRadius, outerRadius, 180.0, 270.0, segments, color);
    DrawRing(centers[1], innerRadius, outerRadius, 270.0, 360.0, segments, color);
    DrawRing(centers[2], innerRadius, outerRadius, 0.0, 90.0, segments, color);
    DrawRing(centers[3], innerRadius, outerRadius, 90.0, 180.0, segments, color);

    DrawRectangleRec(Rectangle { x: rec.x + outerRadius, y: rec.y, width: rec.width - outerRadius*2.0, height: thick }, color);
    DrawRectangleRec(Rectangle { x: rec.x + outerRadius, y: rec.y + rec.height - thick, width: rec.width - outerRadius*2.0, height: thick }, color);
    DrawRectangleRec(Rectangle { x: rec.x, y: rec.y + outerRadius, width: thick, height: rec.height - outerRadius*2.0 }, color);
    DrawRectangleRec(Rectangle { x: rec.x + rec.width - thick, y: rec.y + outerRadius, width: thick, height: rec.height - outerRadius*2.0 }, color);
}

// Draw a polygon of n sides
pub unsafe fn DrawPoly(center: Vector2, mut sides: i32, radius: f32, rotation: f32, color: Color)
{
    if sides < 3 { sides = 3; }
    let mut centralAngle = rotation*DEG2RAD;
    let angleStep = 360.0/sides as f32*DEG2RAD;

    let mut points = Vec::with_capacity((sides + 2) as usize);
    points.push(center);
    for _ in 0..=sides
    {
        points.push(Vector2 { x: center.x + centralAngle.cos()*radius, y: center.y + centralAngle.sin()*radius });
        centralAngle += angleStep;
    }
    DrawTriangleFan(&points, points.len() as i32, color);
}

// Draw a polygon outline of n sides
pub unsafe fn DrawPolyLines(center: Vector2, mut sides: i32, radius: f32, rotation: f32, color: Color)
{
    if sides < 3 { sides = 3; }
    let mut centralAngle = rotation*DEG2RAD;
    let angleStep = 360.0/sides as f32*DEG2RAD;

    rlBegin(RL_LINES);
        for _ in 0..sides
        {
            rlColor4ub(color.r, color.g, color.b, color.a);
            rlVertex2f(center.x + centralAngle.cos()*radius, center.y + centralAngle.sin()*radius);
            rlVertex2f(center.x + (centralAngle + angleStep).cos()*radius, center.y + (centralAngle + angleStep).sin()*radius);
            centralAngle += angleStep;
        }
    rlEnd();
}

pub unsafe fn DrawPolyLinesEx(center: Vector2, mut sides: i32, radius: f32, rotation: f32, mut thick: f32, color: Color)
{
    if sides < 3 { sides = 3; }
    let mut centralAngle = rotation*DEG2RAD;
    let exteriorAngle = 360.0/sides as f32*DEG2RAD;
    let apothem = radius*(DEG2RAD*180.0/sides as f32).cos();

    let outerRadius;
    let innerRadius;
    if thick >= 0.0
    {
        outerRadius = radius;
        innerRadius = 0.0f32.max(radius - thick*(radius/apothem));
    }
    else
    {
        thick *= -1.0;
        outerRadius = radius + thick*(radius/apothem);
        innerRadius = radius;
    }

    for _ in 0..sides
    {
        let nextAngle = centralAngle + exteriorAngle;
        DrawTexturedQuad([
            Vector2 { x: center.x + centralAngle.cos()*outerRadius, y: center.y + centralAngle.sin()*outerRadius },
            Vector2 { x: center.x + centralAngle.cos()*innerRadius, y: center.y + centralAngle.sin()*innerRadius },
            Vector2 { x: center.x + nextAngle.cos()*innerRadius, y: center.y + nextAngle.sin()*innerRadius },
            Vector2 { x: center.x + nextAngle.cos()*outerRadius, y: center.y + nextAngle.sin()*outerRadius }
        ], [color; 4]);
        centralAngle = nextAngle;
    }
}

// Draw a color-filled circle
pub unsafe fn DrawCircle(centerX: i32, centerY: i32, radius: f32, color: Color)
{
    DrawCircleV(Vector2 { x: centerX as f32, y: centerY as f32 }, radius, color);
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
            rlVertex2f(center.x + (DEG2RAD*(i + 10) as f32).cos()*radius, center.y + (DEG2RAD*(i + 10) as f32).sin()*radius);
            rlColor4ub(outer.r, outer.g, outer.b, outer.a);
            rlVertex2f(center.x + (DEG2RAD*i as f32).cos()*radius, center.y + (DEG2RAD*i as f32).sin()*radius);
        }
    rlEnd();
}

// Draw a piece of a circle
pub unsafe fn DrawCircleSector(center: Vector2, mut radius: f32, mut startAngle: f32, mut endAngle: f32, mut segments: i32, color: Color)
{
    if startAngle == endAngle { return; }
    if radius <= 0.0 { radius = 0.1; }  // Avoid div by zero

    // Function expects (endAngle > startAngle)
    if endAngle < startAngle
    {
        std::mem::swap(&mut startAngle, &mut endAngle);
    }

    // Drawing a whole circle, things get weird without limiting the circle to 360 degrees
    if endAngle - startAngle >= 360.0 { endAngle = startAngle + 360.0; }

    segments = GetCircleSegments(radius, startAngle, endAngle, segments);
    let stepLength = (endAngle - startAngle)/segments as f32;
    let mut points = Vec::with_capacity((segments + 2) as usize);
    points.push(center);
    for i in 0..=segments
    {
        let angle = (startAngle + stepLength*i as f32)*DEG2RAD;
        points.push(Vector2 { x: center.x + angle.cos()*radius, y: center.y + angle.sin()*radius });
    }
    DrawTriangleFan(&points, points.len() as i32, color);
}

// Draw a piece of a circle outlines
pub unsafe fn DrawCircleSectorLines(center: Vector2, mut radius: f32, mut startAngle: f32, mut endAngle: f32, mut segments: i32, color: Color)
{
    if startAngle == endAngle { return; }
    if radius <= 0.0 { radius = 0.1; }  // Avoid div by zero issue

    // Function expects (endAngle > startAngle)
    if endAngle < startAngle
    {
        std::mem::swap(&mut startAngle, &mut endAngle);
    }

    let mut showCapLines = true;
    // Drawing a whole circle, things get weird without limiting the circle to 360 degrees
    if endAngle - startAngle >= 360.0
    {
        showCapLines = false;
        endAngle = startAngle + 360.0;
    }

    segments = GetCircleSegments(radius, startAngle, endAngle, segments);
    let stepLength = (endAngle - startAngle)/segments as f32;
    let mut angle = startAngle;

    rlBegin(RL_LINES);
        rlColor4ub(color.r, color.g, color.b, color.a);
        if showCapLines
        {
            rlVertex2f(center.x, center.y);
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*radius, center.y + (DEG2RAD*angle).sin()*radius);
        }

        for _ in 0..segments
        {
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*radius, center.y + (DEG2RAD*angle).sin()*radius);
            rlVertex2f(center.x + (DEG2RAD*(angle + stepLength)).cos()*radius, center.y + (DEG2RAD*(angle + stepLength)).sin()*radius);
            angle += stepLength;
        }

        if showCapLines
        {
            rlVertex2f(center.x, center.y);
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*radius, center.y + (DEG2RAD*angle).sin()*radius);
        }
    rlEnd();
}

// Draw a piece of a circle outlines with thickness
pub unsafe fn DrawCircleSectorLinesEx(center: Vector2, radius: f32, startAngle: f32, endAngle: f32, segments: i32, thick: f32, color: Color)
{
    DrawRing(center, radius - thick, radius, startAngle, endAngle, segments, color);

    if (endAngle - startAngle).abs() < 360.0
    {
        let start = startAngle*DEG2RAD;
        let end = endAngle*DEG2RAD;
        let capRadius = if thick >= 0.0 { radius } else { radius - thick };
        DrawLineEx(center, Vector2 { x: center.x + start.cos()*capRadius, y: center.y + start.sin()*capRadius }, thick.abs(), color);
        DrawLineEx(center, Vector2 { x: center.x + end.cos()*capRadius, y: center.y + end.sin()*capRadius }, thick.abs(), color);
    }
}

// Draw circle outline
pub unsafe fn DrawCircleLines(centerX: i32, centerY: i32, radius: f32, color: Color)
{
    DrawCircleLinesV(Vector2 { x: centerX as f32, y: centerY as f32 }, radius, color);
}

// Draw circle outline (Vector version)
pub unsafe fn DrawCircleLinesV(center: Vector2, radius: f32, color: Color)
{
    rlBegin(RL_LINES);
        rlColor4ub(color.r, color.g, color.b, color.a);

        // NOTE: Circle outline is drawn pixel by pixel every degree (0 to 360)
        for i in (0..360).step_by(10)
        {
            rlVertex2f(center.x + (DEG2RAD*i as f32).cos()*radius, center.y + (DEG2RAD*i as f32).sin()*radius);
            rlVertex2f(center.x + (DEG2RAD*(i + 10) as f32).cos()*radius, center.y + (DEG2RAD*(i + 10) as f32).sin()*radius);
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
    DrawEllipseV(Vector2 { x: centerX as f32, y: centerY as f32 }, radiusH, radiusV, color);
}

// Draw ellipse (Vector version)
pub unsafe fn DrawEllipseV(center: Vector2, radiusH: f32, radiusV: f32, color: Color)
{
    rlBegin(RL_TRIANGLES);
        for i in (0..360).step_by(10)
        {
            rlColor4ub(color.r, color.g, color.b, color.a);
            rlVertex2f(center.x, center.y);
            rlVertex2f(center.x + (DEG2RAD*(i + 10) as f32).cos()*radiusH, center.y + (DEG2RAD*(i + 10) as f32).sin()*radiusV);
            rlVertex2f(center.x + (DEG2RAD*i as f32).cos()*radiusH, center.y + (DEG2RAD*i as f32).sin()*radiusV);
        }
    rlEnd();
}

// Draw ellipse outline
pub unsafe fn DrawEllipseLines(centerX: i32, centerY: i32, radiusH: f32, radiusV: f32, color: Color)
{
    DrawEllipseLinesV(Vector2 { x: centerX as f32, y: centerY as f32 }, radiusH, radiusV, color);
}

// Draw ellipse outline
pub unsafe fn DrawEllipseLinesV(center: Vector2, radiusH: f32, radiusV: f32, color: Color)
{
    rlBegin(RL_LINES);
        for i in (0..360).step_by(10)
        {
            rlColor4ub(color.r, color.g, color.b, color.a);
            rlVertex2f(center.x + (DEG2RAD*(i + 10) as f32).cos()*radiusH, center.y + (DEG2RAD*(i + 10) as f32).sin()*radiusV);
            rlVertex2f(center.x + (DEG2RAD*i as f32).cos()*radiusH, center.y + (DEG2RAD*i as f32).sin()*radiusV);
        }
    rlEnd();
}

// Draw ellipse outline with thickness
pub unsafe fn DrawEllipseLinesEx(center: Vector2, radiusH: f32, radiusV: f32, thick: f32, color: Color)
{
    let mut outerRadiusH = radiusH;
    let mut innerRadiusH = radiusH - thick;
    let mut outerRadiusV = radiusV;
    let mut innerRadiusV = radiusV - thick;

    if thick >= 0.0
    {
        // Just a filled-in ellipse
        if (innerRadiusH <= 0.0) || (innerRadiusV <= 0.0)
        {
            DrawEllipseV(center, radiusH, radiusV, color);
            return;
        }
    }
    else
    {
        std::mem::swap(&mut outerRadiusH, &mut innerRadiusH);
        std::mem::swap(&mut outerRadiusV, &mut innerRadiusV);
    }

    for i in (0..360).step_by(10)
    {
        let angle = DEG2RAD*i as f32;
        let nextAngle = DEG2RAD*(i + 10) as f32;
        DrawTexturedQuad([
            Vector2 { x: center.x + angle.cos()*innerRadiusH, y: center.y + angle.sin()*innerRadiusV },
            Vector2 { x: center.x + nextAngle.cos()*innerRadiusH, y: center.y + nextAngle.sin()*innerRadiusV },
            Vector2 { x: center.x + nextAngle.cos()*outerRadiusH, y: center.y + nextAngle.sin()*outerRadiusV },
            Vector2 { x: center.x + angle.cos()*outerRadiusH, y: center.y + angle.sin()*outerRadiusV }
        ], [color; 4]);
    }
}

// Draw ring
pub unsafe fn DrawRing(center: Vector2, mut innerRadius: f32, mut outerRadius: f32, mut startAngle: f32, mut endAngle: f32, mut segments: i32, color: Color)
{
    if startAngle == endAngle { return; }

    // Function expects (outerRadius > innerRadius)
    if outerRadius < innerRadius
    {
        std::mem::swap(&mut outerRadius, &mut innerRadius);
        if outerRadius <= 0.0 { outerRadius = 0.1; }
    }

    // Function expects (endAngle > startAngle)
    if endAngle < startAngle
    {
        std::mem::swap(&mut startAngle, &mut endAngle);
    }

    // Drawing a whole circle, things get weird without limiting the circle to 360 degrees
    if endAngle - startAngle >= 360.0 { endAngle = startAngle + 360.0; }
    segments = GetCircleSegments(outerRadius, startAngle, endAngle, segments);

    // Not a ring
    if innerRadius <= 0.0
    {
        DrawCircleSector(center, outerRadius, startAngle, endAngle, segments, color);
        return;
    }

    let stepLength = (endAngle - startAngle)/segments as f32;
    let mut angle = startAngle;
    for _ in 0..segments
    {
        let nextAngle = angle + stepLength;
        DrawTexturedQuad([
            Vector2 { x: center.x + (DEG2RAD*angle).cos()*outerRadius, y: center.y + (DEG2RAD*angle).sin()*outerRadius },
            Vector2 { x: center.x + (DEG2RAD*angle).cos()*innerRadius, y: center.y + (DEG2RAD*angle).sin()*innerRadius },
            Vector2 { x: center.x + (DEG2RAD*nextAngle).cos()*innerRadius, y: center.y + (DEG2RAD*nextAngle).sin()*innerRadius },
            Vector2 { x: center.x + (DEG2RAD*nextAngle).cos()*outerRadius, y: center.y + (DEG2RAD*nextAngle).sin()*outerRadius }
        ], [color; 4]);
        angle = nextAngle;
    }
}

// Draw ring outline
pub unsafe fn DrawRingLines(center: Vector2, mut innerRadius: f32, mut outerRadius: f32, mut startAngle: f32, mut endAngle: f32, mut segments: i32, color: Color)
{
    if startAngle == endAngle { return; }
    if outerRadius < innerRadius
    {
        std::mem::swap(&mut outerRadius, &mut innerRadius);
        if outerRadius <= 0.0 { outerRadius = 0.1; }
    }
    if endAngle < startAngle { std::mem::swap(&mut startAngle, &mut endAngle); }

    let mut showCapLines = true;
    if endAngle - startAngle >= 360.0
    {
        showCapLines = false;
        endAngle = startAngle + 360.0;
    }
    segments = GetCircleSegments(outerRadius, startAngle, endAngle, segments);

    if innerRadius <= 0.0
    {
        DrawCircleSectorLines(center, outerRadius, startAngle, endAngle, segments, color);
        return;
    }

    let stepLength = (endAngle - startAngle)/segments as f32;
    let mut angle = startAngle;
    rlBegin(RL_LINES);
        rlColor4ub(color.r, color.g, color.b, color.a);
        if showCapLines
        {
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*outerRadius, center.y + (DEG2RAD*angle).sin()*outerRadius);
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*innerRadius, center.y + (DEG2RAD*angle).sin()*innerRadius);
        }
        for _ in 0..segments
        {
            let nextAngle = angle + stepLength;
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*outerRadius, center.y + (DEG2RAD*angle).sin()*outerRadius);
            rlVertex2f(center.x + (DEG2RAD*nextAngle).cos()*outerRadius, center.y + (DEG2RAD*nextAngle).sin()*outerRadius);
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*innerRadius, center.y + (DEG2RAD*angle).sin()*innerRadius);
            rlVertex2f(center.x + (DEG2RAD*nextAngle).cos()*innerRadius, center.y + (DEG2RAD*nextAngle).sin()*innerRadius);
            angle = nextAngle;
        }
        if showCapLines
        {
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*outerRadius, center.y + (DEG2RAD*angle).sin()*outerRadius);
            rlVertex2f(center.x + (DEG2RAD*angle).cos()*innerRadius, center.y + (DEG2RAD*angle).sin()*innerRadius);
        }
    rlEnd();
}

// Draw ring outline with line thickness
pub unsafe fn DrawRingLinesEx(center: Vector2, innerRadius: f32, outerRadius: f32, startAngle: f32, endAngle: f32, segments: i32, thick: f32, color: Color)
{
    if thick >= 0.0
    {
        DrawRing(center, (innerRadius - thick).max(0.0), innerRadius, startAngle, endAngle, segments, color);
        DrawRing(center, outerRadius - thick, outerRadius, startAngle, endAngle, segments, color);
    }
    else
    {
        DrawRing(center, (innerRadius + thick).max(0.0), innerRadius, startAngle, endAngle, segments, color);
        DrawRing(center, outerRadius, outerRadius - thick, startAngle, endAngle, segments, color);
    }

    if (endAngle - startAngle).abs() < 360.0
    {
        let start = startAngle*DEG2RAD;
        let end = endAngle*DEG2RAD;
        DrawLineEx(
            Vector2 { x: center.x + start.cos()*innerRadius, y: center.y + start.sin()*innerRadius },
            Vector2 { x: center.x + start.cos()*outerRadius, y: center.y + start.sin()*outerRadius }, thick.abs(), color);
        DrawLineEx(
            Vector2 { x: center.x + end.cos()*innerRadius, y: center.y + end.sin()*innerRadius },
            Vector2 { x: center.x + end.cos()*outerRadius, y: center.y + end.sin()*outerRadius }, thick.abs(), color);
    }
}

// Draw spline: Linear, minimum 2 points
pub unsafe fn DrawSplineLinear(points: &[Vector2], pointCount: i32, thick: f32, color: Color)
{
    if pointCount < 2 { return; }

    for i in 0..pointCount - 1
    {
        DrawSplineSegmentLinear(points[i as usize], points[(i + 1) as usize], thick, color);
    }
}

// Draw spline: B-Spline, minimum 4 points
pub unsafe fn DrawSplineBasis(points: &[Vector2], pointCount: i32, thick: f32, color: Color)
{
    if pointCount < 4 { return; }

    for i in 0..pointCount - 3
    {
        DrawSplineSegmentBasis(points[i as usize], points[(i + 1) as usize], points[(i + 2) as usize], points[(i + 3) as usize], thick, color);
    }
}

// Draw spline: Catmull-Rom, minimum 4 points
pub unsafe fn DrawSplineCatmullRom(points: &[Vector2], pointCount: i32, thick: f32, color: Color)
{
    if pointCount < 4 { return; }

    for i in 0..pointCount - 3
    {
        DrawSplineSegmentCatmullRom(points[i as usize], points[(i + 1) as usize], points[(i + 2) as usize], points[(i + 3) as usize], thick, color);
    }
}

// Draw spline: Quadratic Bezier, minimum 3 points (1 control point): [p1, c2, p3, c4...]
pub unsafe fn DrawSplineBezierQuadratic(points: &[Vector2], pointCount: i32, thick: f32, color: Color)
{
    if pointCount >= 3
    {
        for i in (0..pointCount - 2).step_by(2) { DrawSplineSegmentBezierQuadratic(points[i as usize], points[(i + 1) as usize], points[(i + 2) as usize], thick, color); }

        // Cap circle drawing at the end of every segment
        //for (int i = 2; i < pointCount - 2; i += 2) DrawCircleV(points[i], thick/2.0f, color);
    }
}

// Draw spline: Cubic Bezier, minimum 4 points (2 control points): [p1, c2, c3, p4, c5, c6...]
pub unsafe fn DrawSplineBezierCubic(points: &[Vector2], pointCount: i32, thick: f32, color: Color)
{
    if pointCount >= 4
    {
        for i in (0..pointCount - 3).step_by(3) { DrawSplineSegmentBezierCubic(points[i as usize], points[(i + 1) as usize], points[(i + 2) as usize], points[(i + 3) as usize], thick, color); }

        // Cap circle drawing at the end of every segment
        //for (int i = 3; i < pointCount - 3; i += 3) DrawCircleV(points[i], thick/2.0f, color);
    }
}

// Draw spline segment: Linear, 2 points
pub unsafe fn DrawSplineSegmentLinear(p1: Vector2, p2: Vector2, thick: f32, color: Color)
{
    // NOTE: For the linear spline no subdivisions are used, only a single quad
    DrawLineEx(p1, p2, thick, color);
}

// Draw spline segment: B-Spline, 4 points
pub unsafe fn DrawSplineSegmentBasis(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thick: f32, color: Color)
{
    let mut splinePoints = [Vector2 { x: 0.0, y: 0.0 }; (SPLINE_SEGMENT_DIVISIONS + 1) as usize];
    for i in 0..=SPLINE_SEGMENT_DIVISIONS
    {
        splinePoints[i as usize] = GetSplinePointBasis(p1, p2, p3, p4, i as f32/SPLINE_SEGMENT_DIVISIONS as f32);
    }
    DrawSplineSampled(&splinePoints, thick, color);
}

// Draw spline segment: Catmull-Rom, 4 points
pub unsafe fn DrawSplineSegmentCatmullRom(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, thick: f32, color: Color)
{
    let mut splinePoints = [Vector2 { x: 0.0, y: 0.0 }; (SPLINE_SEGMENT_DIVISIONS + 1) as usize];
    for i in 0..=SPLINE_SEGMENT_DIVISIONS
    {
        splinePoints[i as usize] = GetSplinePointCatmullRom(p1, p2, p3, p4, i as f32/SPLINE_SEGMENT_DIVISIONS as f32);
    }
    DrawSplineSampled(&splinePoints, thick, color);
}

// Draw spline segment: Quadratic Bezier, 2 points, 1 control point
pub unsafe fn DrawSplineSegmentBezierQuadratic(p1: Vector2, c2: Vector2, p3: Vector2, thick: f32, color: Color)
{
    let mut splinePoints = [Vector2 { x: 0.0, y: 0.0 }; (SPLINE_SEGMENT_DIVISIONS + 1) as usize];
    for i in 0..=SPLINE_SEGMENT_DIVISIONS
    {
        splinePoints[i as usize] = GetSplinePointBezierQuadratic(p1, c2, p3, i as f32/SPLINE_SEGMENT_DIVISIONS as f32);
    }
    DrawSplineSampled(&splinePoints, thick, color);
}

// Draw spline segment: Cubic Bezier, 2 points, 2 control points
pub unsafe fn DrawSplineSegmentBezierCubic(p1: Vector2, c2: Vector2, c3: Vector2, p4: Vector2, thick: f32, color: Color)
{
    let mut splinePoints = [Vector2 { x: 0.0, y: 0.0 }; (SPLINE_SEGMENT_DIVISIONS + 1) as usize];
    for i in 0..=SPLINE_SEGMENT_DIVISIONS
    {
        splinePoints[i as usize] = GetSplinePointBezierCubic(p1, c2, c3, p4, i as f32/SPLINE_SEGMENT_DIVISIONS as f32);
    }
    DrawSplineSampled(&splinePoints, thick, color);
}

// Get spline point for a given t [0.0f .. 1.0f], Linear
pub fn GetSplinePointLinear(startPos: Vector2, endPos: Vector2, t: f32) -> Vector2
{
    let mut point = Vector2::default();

    point.x = startPos.x*(1.0 - t) + endPos.x*t;
    point.y = startPos.y*(1.0 - t) + endPos.y*t;

    return point;
}

// Get spline point for a given t [0.0f .. 1.0f], B-Spline
pub fn GetSplinePointBasis(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, t: f32) -> Vector2
{
    let mut point = Vector2::default();

    let mut a = [0.0; 4];
    let mut b = [0.0; 4];

    a[0] = (-p1.x + 3.0*p2.x - 3.0*p3.x + p4.x)/6.0;
    a[1] = (3.0*p1.x - 6.0*p2.x + 3.0*p3.x)/6.0;
    a[2] = (-3.0*p1.x + 3.0*p3.x)/6.0;
    a[3] = (p1.x + 4.0*p2.x + p3.x)/6.0;

    b[0] = (-p1.y + 3.0*p2.y - 3.0*p3.y + p4.y)/6.0;
    b[1] = (3.0*p1.y - 6.0*p2.y + 3.0*p3.y)/6.0;
    b[2] = (-3.0*p1.y + 3.0*p3.y)/6.0;
    b[3] = (p1.y + 4.0*p2.y + p3.y)/6.0;

    point.x = a[3] + t*(a[2] + t*(a[1] + t*a[0]));
    point.y = b[3] + t*(b[2] + t*(b[1] + t*b[0]));

    return point;
}

// Get spline point for a given t [0.0f .. 1.0f], Catmull-Rom
pub fn GetSplinePointCatmullRom(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2, t: f32) -> Vector2
{
    let mut point = Vector2::default();

    let q0 = (-t*t*t) + (2.0*t*t) + (-t);
    let q1 = (3.0*t*t*t) + (-5.0*t*t) + 2.0;
    let q2 = (-3.0*t*t*t) + (4.0*t*t) + t;
    let q3 = t*t*t - t*t;

    point.x = 0.5*((p1.x*q0) + (p2.x*q1) + (p3.x*q2) + (p4.x*q3));
    point.y = 0.5*((p1.y*q0) + (p2.y*q1) + (p3.y*q2) + (p4.y*q3));

    return point;
}

// Get spline point for a given t [0.0f .. 1.0f], Quadratic Bezier
pub fn GetSplinePointBezierQuadratic(startPos: Vector2, controlPos: Vector2, endPos: Vector2, t: f32) -> Vector2
{
    let mut point = Vector2::default();

    let a = (1.0 - t).powi(2);
    let b = 2.0*(1.0 - t)*t;
    let c = t.powi(2);

    point.y = a*startPos.y + b*controlPos.y + c*endPos.y;
    point.x = a*startPos.x + b*controlPos.x + c*endPos.x;

    return point;
}

// Get spline point for a given t [0.0f .. 1.0f], Cubic Bezier
pub fn GetSplinePointBezierCubic(startPos: Vector2, startControlPos: Vector2, endControlPos: Vector2, endPos: Vector2, t: f32) -> Vector2
{
    let mut point = Vector2::default();

    let a = (1.0 - t).powi(3);
    let b = 3.0*(1.0 - t).powi(2)*t;
    let c = 3.0*(1.0 - t)*t.powi(2);
    let d = t.powi(3);

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
    let mut collision = false;

    if (point.x >= rec.x) && (point.x < (rec.x + rec.width)) && (point.y >= rec.y) && (point.y < (rec.y + rec.height)) { collision = true; }

    return collision;
}

// Check if point is inside circle
pub fn CheckCollisionPointCircle(point: Vector2, center: Vector2, radius: f32) -> bool
{
    let mut collision = false;

    let distanceSquared = (point.x - center.x)*(point.x - center.x) + (point.y - center.y)*(point.y - center.y);

    if distanceSquared <= radius*radius { collision = true; }

    return collision;
}

// Check if point is inside a triangle defined by three points (p1, p2, p3)
pub fn CheckCollisionPointTriangle(point: Vector2, p1: Vector2, p2: Vector2, p3: Vector2) -> bool
{
    let mut collision = false;

    let alpha = ((p2.y - p3.y)*(point.x - p3.x) + (p3.x - p2.x)*(point.y - p3.y)) /
                ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y));

    let beta = ((p3.y - p1.y)*(point.x - p3.x) + (p1.x - p3.x)*(point.y - p3.y)) /
               ((p2.y - p3.y)*(p1.x - p3.x) + (p3.x - p2.x)*(p1.y - p3.y));

    let gamma = 1.0 - alpha - beta;

    if (alpha > 0.0) && (beta > 0.0) && (gamma > 0.0) { collision = true; }

    return collision;
}

// Check if point is within a polygon described by array of vertices
// NOTE: Based on http://jeffreythompson.org/collision-detection/poly-point.php
pub fn CheckCollisionPointPoly(point: Vector2, points: &[Vector2], pointCount: i32) -> bool
{
    let mut collision = false;

    if pointCount > 2
    {
        let mut j = pointCount - 1;
        for i in 0..pointCount
        {
            if ((points[i as usize].y > point.y) != (points[j as usize].y > point.y)) &&
               (point.x < (points[j as usize].x - points[i as usize].x)*(point.y - points[i as usize].y)/(points[j as usize].y - points[i as usize].y) + points[i as usize].x)
            {
                collision = !collision;
            }
            j = i;
        }
    }

    return collision;
}

// Check collision between two rectangles
pub fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle) -> bool
{
    let mut collision = false;

    if (rec1.x < (rec2.x + rec2.width) && (rec1.x + rec1.width) > rec2.x) &&
       (rec1.y < (rec2.y + rec2.height) && (rec1.y + rec1.height) > rec2.y) { collision = true; }

    return collision;
}

// Check collision between two circles
pub fn CheckCollisionCircles(center1: Vector2, radius1: f32, center2: Vector2, radius2: f32) -> bool
{
    let dx = center2.x - center1.x;      // X distance between centers
    let dy = center2.y - center1.y;      // Y distance between centers

    let distanceSquared = dx*dx + dy*dy; // Distance between centers squared
    let radiusSum = radius1 + radius2;

    let collision = distanceSquared <= radiusSum*radiusSum;

    return collision;
}

// Check collision between circle and rectangle
// NOTE: Reviewed version to take into account corner limit case
pub fn CheckCollisionCircleRec(center: Vector2, radius: f32, rec: Rectangle) -> bool
{
    let mut collision = false;

    let recCenterX = rec.x + rec.width/2.0;
    let recCenterY = rec.y + rec.height/2.0;

    let dx = (center.x - recCenterX).abs();
    let dy = (center.y - recCenterY).abs();

    if (dx <= (rec.width/2.0 + radius)) && (dy <= (rec.height/2.0 + radius))
    {
        if dx <= rec.width/2.0 { collision = true; }
        else if dy <= rec.height/2.0 { collision = true; }
        else
        {
            let cornerDistanceSq = (dx - rec.width/2.0)*(dx - rec.width/2.0) +
                (dy - rec.height/2.0)*(dy - rec.height/2.0);

            collision = cornerDistanceSq <= radius*radius;
        }
    }

    return collision;
}

// Check the collision between two lines defined by two points each, returns collision point by reference
// REF: https://en.wikipedia.org/wiki/Line-line_intersection#Given_two_points_on_each_line_segment
pub fn CheckCollisionLines(startPos1: Vector2, endPos1: Vector2, startPos2: Vector2, endPos2: Vector2, collisionPoint: Option<&mut Vector2>) -> bool
{
    let mut collision = false;

    let rx = endPos1.x - startPos1.x;
    let ry = endPos1.y - startPos1.y;
    let sx = endPos2.x - startPos2.x;
    let sy = endPos2.y - startPos2.y;

    let div = rx*sy - ry*sx;

    if div.abs() >= f32::EPSILON
    {
        let s12x = startPos2.x - startPos1.x;
        let s12y = startPos2.y - startPos1.y;

        let t = (s12x*sy - s12y*sx)/div;
        let u = (s12x*ry - s12y*rx)/div;

        if (0.0 <= t) && (t <= 1.0) && (0.0 <= u) && (u <= 1.0)
        {
            if let Some(collisionPoint) = collisionPoint
            {
                collisionPoint.x = startPos1.x + t*rx;
                collisionPoint.y = startPos1.y + t*ry;
            }

            collision = true;
        }
    }

    return collision;
}

// Check if point belongs to line created between two points [p1] and [p2] with defined margin in pixels [threshold]
pub fn CheckCollisionPointLine(point: Vector2, p1: Vector2, p2: Vector2, threshold: i32) -> bool
{
    let mut collision = false;

    let dxc = point.x - p1.x;
    let dyc = point.y - p1.y;
    let dxl = p2.x - p1.x;
    let dyl = p2.y - p1.y;
    let cross = dxc*dyl - dyc*dxl;

    if cross.abs() < threshold as f32*dxl.abs().max(dyl.abs())
    {
        if dxl.abs() >= dyl.abs() { collision = if dxl > 0.0 { (p1.x <= point.x) && (point.x <= p2.x) } else { (p2.x <= point.x) && (point.x <= p1.x) }; }
        else { collision = if dyl > 0.0 { (p1.y <= point.y) && (point.y <= p2.y) } else { (p2.y <= point.y) && (point.y <= p1.y) }; }
    }

    return collision;
}

// Check if circle collides with a line created between two points [p1] and [p2]
pub fn CheckCollisionCircleLine(center: Vector2, radius: f32, p1: Vector2, p2: Vector2) -> bool
{
    let mut collision = false;

    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;

    if (dx.abs() + dy.abs()) <= f32::EPSILON
    {
        collision = CheckCollisionCircles(p1, 0.0, center, radius);
    }
    else
    {
        let lengthSQ = dx*dx + dy*dy;
        let mut dotProduct = (((center.x - p1.x)*(p2.x - p1.x)) + ((center.y - p1.y)*(p2.y - p1.y)))/lengthSQ;

        if dotProduct > 1.0 { dotProduct = 1.0; }
        else if dotProduct < 0.0 { dotProduct = 0.0; }

        let dx2 = (p1.x - dotProduct*dx) - center.x;
        let dy2 = (p1.y - dotProduct*dy) - center.y;
        let distanceSQ = dx2*dx2 + dy2*dy2;

        if distanceSQ <= radius*radius { collision = true; }
    }

    return collision;
}

// Get collision rectangle for two rectangles collision
pub fn GetCollisionRec(rec1: Rectangle, rec2: Rectangle) -> Rectangle
{
    let mut overlap = Rectangle::default();

    let left = if rec1.x > rec2.x { rec1.x } else { rec2.x };
    let right1 = rec1.x + rec1.width;
    let right2 = rec2.x + rec2.width;
    let right = if right1 < right2 { right1 } else { right2 };
    let top = if rec1.y > rec2.y { rec1.y } else { rec2.y };
    let bottom1 = rec1.y + rec1.height;
    let bottom2 = rec2.y + rec2.height;
    let bottom = if bottom1 < bottom2 { bottom1 } else { bottom2 };

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
    let result;

    t /= 0.5*d;
    if t < 1.0 { result = 0.5*c*t*t*t + b; }
    else
    {
        t -= 2.0;
        result = 0.5*c*(t*t*t + 2.0) + b;
    }

    return result;
}

fn GetCircleSegments(radius: f32, startAngle: f32, endAngle: f32, mut segments: i32) -> i32
{
    let minSegments = ((endAngle - startAngle)/90.0).ceil() as i32;
    if segments < minSegments
    {
        let th = (2.0*(1.0 - SMOOTH_CIRCLE_ERROR_RATE/radius).powi(2) - 1.0).acos();
        segments = ((endAngle - startAngle)*(2.0*PI/th)/360.0).ceil() as i32;
        if segments <= 0 { segments = minSegments; }
    }
    return segments;
}

unsafe fn DrawTexturedQuad(vertices: [Vector2; 4], colors: [Color; 4])
{
    rlSetTexture(GetShapesTexture().id);
    let shapeRect = GetShapesTextureRectangle();

    rlBegin(RL_QUADS);
        rlNormal3f(0.0, 0.0, 1.0);
        for i in 0..4
        {
            rlColor4ub(colors[i].r, colors[i].g, colors[i].b, colors[i].a);
            match i
            {
                0 => rlTexCoord2f(shapeRect.x/texShapes.width as f32, shapeRect.y/texShapes.height as f32),
                1 => rlTexCoord2f(shapeRect.x/texShapes.width as f32, (shapeRect.y + shapeRect.height)/texShapes.height as f32),
                2 => rlTexCoord2f((shapeRect.x + shapeRect.width)/texShapes.width as f32, (shapeRect.y + shapeRect.height)/texShapes.height as f32),
                _ => rlTexCoord2f((shapeRect.x + shapeRect.width)/texShapes.width as f32, shapeRect.y/texShapes.height as f32)
            }
            rlVertex2f(vertices[i].x, vertices[i].y);
        }
    rlEnd();
    rlSetTexture(0);
}

unsafe fn DrawSplineSampled(points: &[Vector2], thick: f32, color: Color)
{
    for i in 0..points.len() - 1
    {
        DrawLineEx(points[i], points[i + 1], thick, color);
    }
}
