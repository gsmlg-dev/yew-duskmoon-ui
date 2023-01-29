use wasm_bindgen::prelude::*;

#[cfg(feature = "mdi")]
pub mod mdi;

#[cfg(feature = "mdi_names")]
pub mod mdi_names;

#[cfg(feature = "bsi")]
pub mod bsi;

#[cfg(feature = "bsi_names")]
pub mod bsi_names;
