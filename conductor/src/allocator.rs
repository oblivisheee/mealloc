extern crate alloc;
use alloc::{boxed::Box, vec};
use smallvec::{smallvec, Array, SmallVec};
use tinyvec::SliceVec;

/// Quantity of 10KB to fulfill 32GB RAM. Calculated via
///
/// 32 GB = 32 * 1024 MB * 1024 KB = 33,554,432 KB
/// 33,554,432 KB / 10 KB = 3,355,443.2
/// Rounded to 3,355,444 blocks
const BLOCKS_ARRAY_CAPACITY_32GB: usize = 3355444;

/// Set the capacity of array.
const BLOCKS_ARRAY_CAPACITY: usize = BLOCKS_ARRAY_CAPACITY_32GB;

/// Array where minimal size of block for 32GB RAM: 10KB
struct BlockArray([Block; BLOCKS_ARRAY_CAPACITY]);

unsafe impl Array for BlockArray {
    type Item = Block;
    fn size() -> usize {
        BLOCKS_ARRAY_CAPACITY
    }
}

/// Structure that represents the allocator
pub struct Allocator {
    block_size: usize,
    blocks: SmallVec<BlockArray>,
    buf: Buffer,
}
impl Allocator {
    pub fn new(block_size: usize, memory_size: usize) -> Self {
        if block_size < 10 * 1024 {
            panic!("block_size must be at least 10KB");
        }
        let blocks = smallvec![Block::new(); BLOCKS_ARRAY_CAPACITY];
        let buf = Buffer::with_capacity(memory_size);
        Allocator {
            block_size,
            blocks,
            buf,
        }
    }
}

/// Buffer for memory allocation
pub struct Buffer(pub &'static mut [u8]);

impl Buffer {
    /// Create an empty buffer with no capacity
    pub fn empty() -> Self {
        Buffer(&mut [])
    }
    /// Create a buffer with a given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let vec = vec![0; capacity].into_boxed_slice();
        Buffer(Box::leak(vec))
    }
    /// Create a buffer from a slice
    pub fn from_slice(slice: &'static mut [u8]) -> Self {
        Buffer(slice)
    }
}

impl From<&'static mut [u8]> for Buffer {
    fn from(slice: &'static mut [u8]) -> Self {
        Buffer(slice)
    }
}
impl core::ops::Deref for Buffer {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        self.0
    }
}
impl core::ops::DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.0
    }
}
fn test() {
    let v = [1, 2, 3];
    let buf = Buffer::empty();
}

/// Represents a block of memory that has 2 states: allocated or deallocated
#[derive(Clone, Copy)]
pub struct Block(pub bool);

impl Block {
    pub fn new() -> Self {
        Block(false)
    }
    pub fn alloc(&mut self) {
        self.0 = true;
    }
    pub fn dealloc(&mut self) {
        self.0 = false;
    }
}
