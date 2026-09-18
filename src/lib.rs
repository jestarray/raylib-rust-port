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

pub mod external {
    include!(concat!(env!("OUT_DIR"), "/external_bindings.rs"));
}

pub mod math;
pub mod rcamera;
pub mod rcore;
pub mod rcore_desktop_sdl;
pub mod rlgl;
pub mod rmodels;
pub mod rshapes;
pub mod rtext;
pub mod rtextures;
pub mod types;
