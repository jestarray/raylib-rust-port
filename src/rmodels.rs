use crate::types::{Color, Vector3};
use crate::rlgl::{self, RL_LINES, RL_QUADS, RL_TRIANGLES};

pub fn draw_cube(position: Vector3, width: f32, height: f32, length: f32, color: Color) {
    let x = position.x;
    let y = position.y;
    let z = position.z;

    unsafe {
        rlgl::rlSetTexture(rlgl::RLGL.State.defaultTextureId);
        rlgl::rlBegin(RL_QUADS);
        rlgl::rlColor4ub(color.r, color.g, color.b, color.a);

        // Front Face
        rlgl::rlVertex3f(x - width/2.0, y - height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y - height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y + height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y + height/2.0, z + length/2.0);

        // Back Face
        rlgl::rlVertex3f(x - width/2.0, y - height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y + height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y + height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y - height/2.0, z - length/2.0);

        // Top Face
        rlgl::rlVertex3f(x - width/2.0, y + height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y + height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y + height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y + height/2.0, z - length/2.0);

        // Bottom Face
        rlgl::rlVertex3f(x - width/2.0, y - height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y - height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y - height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y - height/2.0, z + length/2.0);

        // Right face
        rlgl::rlVertex3f(x + width/2.0, y - height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y + height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y + height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y - height/2.0, z + length/2.0);

        // Left face
        rlgl::rlVertex3f(x - width/2.0, y - height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y - height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y + height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y + height/2.0, z - length/2.0);
        rlgl::rlEnd();
    }
}

pub fn draw_cube_wires(position: Vector3, width: f32, height: f32, length: f32, color: Color) {
    let x = position.x;
    let y = position.y;
    let z = position.z;

    unsafe {
        rlgl::rlSetTexture(rlgl::RLGL.State.defaultTextureId);
        rlgl::rlBegin(RL_LINES);
        rlgl::rlColor4ub(color.r, color.g, color.b, color.a);

        // Front Face
        rlgl::rlVertex3f(x - width/2.0, y - height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y - height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y - height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y + height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y + height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y + height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y + height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y - height/2.0, z + length/2.0);

        // Back Face
        rlgl::rlVertex3f(x - width/2.0, y - height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y - height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y - height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y + height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y + height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y + height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y + height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y - height/2.0, z - length/2.0);

        // Top Face
        rlgl::rlVertex3f(x - width/2.0, y + height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y + height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y + height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y + height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y + height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y + height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y + height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y + height/2.0, z + length/2.0);

        // Bottom Face
        rlgl::rlVertex3f(x - width/2.0, y - height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y - height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y - height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y - height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y - height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y - height/2.0, z - length/2.0);
        rlgl::rlVertex3f(x - width/2.0, y - height/2.0, z + length/2.0);
        rlgl::rlVertex3f(x + width/2.0, y - height/2.0, z + length/2.0);
        rlgl::rlEnd();
    }
}

pub fn draw_grid(slices: i32, spacing: f32) {
    let half_slices = slices / 2;

    unsafe {
        rlgl::rlBegin(RL_LINES);
        rlgl::rlColor4ub(128, 128, 128, 255);

        for i in -half_slices..=half_slices {
            rlgl::rlVertex3f(i as f32 * spacing, 0.0, -half_slices as f32 * spacing);
            rlgl::rlVertex3f(i as f32 * spacing, 0.0, half_slices as f32 * spacing);

            rlgl::rlVertex3f(-half_slices as f32 * spacing, 0.0, i as f32 * spacing);
            rlgl::rlVertex3f(half_slices as f32 * spacing, 0.0, i as f32 * spacing);
        }
        rlgl::rlEnd();
    }
}
