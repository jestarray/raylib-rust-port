#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

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
