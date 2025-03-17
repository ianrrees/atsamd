#![no_std]
//! Library for boards using the SAMD21

#[cfg(feature="feather_m0")]
mod feather_m0;
#[cfg(feature="feather_m0")]
pub use feather_m0::*;

#[cfg(feature="metro_m0")]
mod metro_m0;
#[cfg(feature="metro_m0")]
pub use metro_m0::*;
