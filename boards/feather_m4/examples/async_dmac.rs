//! This example shows a safe API to
//! execute a memory-to-memory DMA transfer

#![no_std]
#![no_main]

use feather_m4 as bsp;
include!("../../examples/m4-async_dmac.rs");