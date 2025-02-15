use core::ops::Add;

use super::{
    block::BlockIndexer,
    mem::Buffer,
    types::{Address, ContainerID},
};

/// Structure that represents the allocator
pub struct Allocator {
    block_size: usize,
    blocks: BlockIndexer,
    buf: Buffer,
}

impl Allocator {
    pub fn new(block_size: usize, memory_size: usize) -> Self {
        if block_size < 10 * 1024 {
            panic!("block_size must be at least 10KB");
        }
        let buf = Buffer::with_capacity(memory_size);

        Allocator {
            block_size,
            blocks: BlockIndexer::new(),
            buf,
        }
    }

    pub fn allocate(&mut self, id: ContainerID, size: usize) -> Option<(Address, Address)> {
        let num_blocks_needed = (size + self.block_size - 1) / self.block_size;

        if let Some(start_index) = self.blocks.find_free_blocks(num_blocks_needed) {
            self.blocks
                .allocate_blocks(start_index, num_blocks_needed, id.clone());
            let start_address =
                (self.buf.as_mut_ptr() as usize + start_index * self.block_size) as *mut u8;
            let end_address = (self.buf.as_mut_ptr() as usize
                + (start_index + num_blocks_needed - 1) * self.block_size)
                as *mut u8;
            return Some((Address::new(start_address), Address::new(end_address)));
        }
        None
    }

    pub fn deallocate(&mut self, id: &ContainerID) -> Option<(Address, Address)> {
        if let Some((start_index, total_blocks)) = self.blocks.deallocate_blocks(id) {
            let start_address =
                (self.buf.as_mut_ptr() as usize + start_index * self.block_size) as *mut u8;
            let end_address = (self.buf.as_mut_ptr() as usize
                + (start_index + total_blocks - 1) * self.block_size)
                as *mut u8;
            self.buf.zero_range(start_address, end_address);
            return Some((Address::new(start_address), Address::new(end_address)));
        }
        None
    }

    pub const fn block_size(&self) -> usize {
        self.block_size
    }

    pub fn blocks_len(&self) -> usize {
        self.blocks.len()
    }
}
