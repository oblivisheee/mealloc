use super::{
    mem::Buffer,
    types::{Address, ContainerID},
};
use smallvec::{smallvec, Array, SmallVec};

/// Quantity of 10KB to fulfill 32GB RAM. Calculated via
///
/// 32 GB = 32 * 1024 MB * 1024 KB = 33,554,432 KB
/// 33,554,432 KB / 10 KB = 3,355,443.2
/// Rounded to 3,355,444 blocks
//TODO: Fix setting block capacity of Block Array, because it causes a stack overflow
const BLOCKS_ARRAY_CAPACITY_32GB: usize = 10000; //3355444;

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
    pub fn allocate(&mut self, id: ContainerID, size: usize) -> Option<(Address, Address)> {
        let num_blocks_needed = (size + self.block_size - 1) / self.block_size;

        for i in 0..self.blocks_len() - num_blocks_needed + 1 {
            if self.blocks[i..i + num_blocks_needed]
                .iter()
                .all(|b| !b.status())
            {
                for j in i..i + num_blocks_needed {
                    self.blocks[j].alloc(id.clone());
                }
                let start_address =
                    (self.buf.as_mut_ptr() as usize + i * self.block_size) as *mut u8;
                let end_address = (self.buf.as_mut_ptr() as usize
                    + (i + num_blocks_needed - 1) * self.block_size)
                    as *mut u8;
                return Some((Address::new(start_address), Address::new(end_address)));
            }
        }
        None
    }
    pub fn deallocate(&mut self, id: &ContainerID) {
        for block in self.blocks.iter_mut() {
            if let Some(block_id) = block.container_id() {
                if &block_id == id {
                    block.dealloc();
                }
            }
        }
    }
    pub const fn block_size(&self) -> usize {
        self.block_size
    }
    pub fn blocks_len(&self) -> usize {
        self.blocks.len()
    }
}

/// Represents a block of memory that has 2 states: allocated or deallocated
#[derive(Clone)]
pub struct Block {
    status: bool,
    container_id: Option<ContainerID>,
}

impl Block {
    /// Create a new block with deallocated status
    pub fn new() -> Self {
        Block {
            status: false,
            container_id: None,
        }
    }

    /// Allocate the block
    pub fn alloc(&mut self, container_id: ContainerID) {
        self.status = true;
        self.container_id = Some(container_id);
    }
    /// Deallocate the block
    pub fn dealloc(&mut self) {
        self.status = false;
        self.container_id = None;
    }
    /// If allocated, return true, otherwise return false
    pub const fn status(&self) -> bool {
        self.status
    }
    pub fn container_id(&self) -> Option<ContainerID> {
        self.container_id.clone()
    }
}

impl core::ops::Not for Block {
    type Output = Self;
    fn not(self) -> Self::Output {
        Block {
            status: !self.status,
            container_id: self.container_id,
        }
    }
}
