#![no_std]

pub mod container;
pub mod mem;

use core::alloc::GlobalAlloc;

pub struct Allocator {}

//unsafe impl GlobalAlloc for Allocator {}
