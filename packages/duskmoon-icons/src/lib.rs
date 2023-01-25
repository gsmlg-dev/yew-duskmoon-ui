#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use wasm_bindgen::prelude::*;

#[cfg(feature = "mdi")]
pub mod mdi;
#[cfg(feature = "mdi")]
#[doc(hidden)]
pub use mdi::*;

#[cfg(feature = "bsi")]
pub mod bsi;
#[cfg(feature = "bsi")]
#[doc(hidden)]
pub use bsi::*;
