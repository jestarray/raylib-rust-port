#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod external {
    include!(concat!(env!("OUT_DIR"), "/external_bindings.rs"));
}

pub mod math;
pub mod types;
pub mod core;
pub mod rlgl;
pub mod rtextures;
pub mod rshapes;
pub mod rtext;
pub mod rcamera;
pub mod rmodels;
