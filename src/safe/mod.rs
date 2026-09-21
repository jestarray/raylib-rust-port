//! Safe, `snake_case` wrappers around the raw C-style bindings in the crate root.
//!
//! Each submodule mirrors one of the raw modules, exposing every one of its public
//! functions under a `snake_case` name and a signature that can be called without
//! `unsafe`:
//!
//! | Wrapper module      | Raw module    |
//! |---------------------|---------------|
//! | [`core`]            | `rcore`       |
//! | [`sdl`]             | `rsdl`        |
//! | [`shapes`]          | `rshapes`     |
//! | [`text`]            | `rtext`       |
//! | [`textures`]        | `rtextures`   |
//!
//! Every raw function has exactly one wrapper here, and the raw modules themselves are
//! private, so these submodules are the only entry points.

pub mod core;
pub mod sdl;
pub mod shapes;
pub mod text;
pub mod textures;
