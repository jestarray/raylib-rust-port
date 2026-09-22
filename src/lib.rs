#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
// raylib C style binds to unused assignments, e.g let result = 0;! But the lint unused_assignments can catch bugs
#![allow(clippy::needless_return)]
#![allow(clippy::collapsible_if)]

#[cfg(all(
    feature = "GRAPHICS_API_OPENGL_33",
    feature = "GRAPHICS_API_OPENGL_ES2"
))]
compile_error!(
    "Desktop OpenGL and OpenGL ES cannot be enabled together; use --no-default-features for ES builds"
);

// Safe, snake_case wrappers around the raw C-style bindings below. `safe` is only a
// filesystem grouping; the modules themselves are exposed at the crate root so callers
// write `use raylib::core::*;` rather than `use raylib::safe::core::*;`.
mod safe;

pub use safe::{core, handle, sdl, shapes, text, textures};

pub mod math;
pub mod rcamera;
pub mod rcolors;
pub mod rlgl;
pub mod rmodels;
pub mod types;

// The raw C-style bindings are private implementation details. Each one is reachable only
// through its safe `snake_case` wrapper, re-exported above as `raylib::core`,
// `raylib::sdl`, `raylib::shapes`, `raylib::text` and `raylib::textures`.
//
// Keeping the modules declared (rather than deleting these lines) is what lets the wrappers
// name them as `crate::rcore::...`; a private module is still nameable inside the crate but
// invisible to downstream crates, which is what keeps the `unsafe` functions out of reach.
#[allow(dead_code)] // not every raw constant/alias is consumed by the wrappers
mod rcore;
#[allow(dead_code)]
mod rsdl;
#[allow(dead_code)]
mod rshapes;
#[allow(dead_code)]
mod rtext;
#[allow(dead_code)]
mod rtextures;

#[cfg(test)]
mod tests {
    use crate::rcore::WindowData;
    use crate::types::Matrix;

    /// Kept from `tests/matrix_glam_compat.rs`, which can no longer name `WindowData` now
    /// that `rcore` is private. A crate-internal test still can.
    #[test]
    fn window_data_defaults_to_a_zero_matrix() {
        assert_eq!(WindowData::default().screenScale, Matrix::ZERO);
    }
}
