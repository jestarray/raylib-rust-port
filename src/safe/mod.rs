//! Safe, `snake_case` wrappers around global operations in the raw bindings.
//!
//! Methods on data types live in [`crate::types`].
//! Live global state access remains explicitly `unsafe`:
//!
//! | Wrapper module      | Raw module    |
//! |---------------------|---------------|
//! | [`core`]            | `rcore`       |
//! | [`sdl`]             | `rsdl`        |
//! | [`shapes`]          | `rshapes`     |
//! | [`text`]            | `rtext`       |
//! | [`textures`]        | `rtextures`   |
//!

pub mod core;
pub mod handle;
pub mod sdl;
pub mod shapes;
pub mod text;
pub mod textures;
