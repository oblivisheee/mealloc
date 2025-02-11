#![no_std]

mod container;
mod mem;

use core::alloc::GlobalAlloc;

pub struct Allocator {}

//unsafe impl GlobalAlloc for Allocator {}
