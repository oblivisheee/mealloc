use super::types::ContainerID;
use core::ops::Deref;
use tinyvec::{array_vec, Array, ArrayVec};

const BLOCK_INDEXER_CAPACITY: usize = 30 * 1024;

pub struct BlockIndexer {
    blocks: ArrayVec<BlockArray>,
}

impl BlockIndexer {
    pub fn new() -> Self {
        BlockIndexer {
            blocks: ArrayVec::default(),
        }
    }

    pub fn find_free_blocks(&self, num_blocks_needed: usize) -> Option<usize> {
        for i in 0..self.blocks.len() - num_blocks_needed + 1 {
            if self.blocks[i..i + num_blocks_needed]
                .iter()
                .all(|b| !b.status())
            {
                return Some(i);
            }
        }
        None
    }

    pub fn allocate_blocks(&mut self, start_index: usize, num_blocks: usize, id: ContainerID) {
        for i in start_index..start_index + num_blocks {
            self.blocks[i].alloc(id.clone());
        }
    }

    pub fn deallocate_blocks(&mut self, id: &ContainerID) -> Option<(usize, usize)> {
        let mut start_index = None;
        let mut count = 0;

        for (i, block) in self.blocks.iter_mut().enumerate() {
            if let Some(block_id) = block.container_id() {
                if &block_id == id {
                    block.dealloc();
                    if start_index.is_none() {
                        start_index = Some(i);
                    }
                    count += 1;
                }
            }
        }

        start_index.map(|index| (index, count))
    }
}

pub struct BlockArray([Block; BLOCK_INDEXER_CAPACITY]);

impl Array for BlockArray {
    type Item = Block;
    const CAPACITY: usize = BLOCK_INDEXER_CAPACITY;

    fn as_slice(&self) -> &[Self::Item] {
        &self.0
    }

    fn as_slice_mut(&mut self) -> &mut [Self::Item] {
        &mut self.0
    }

    fn default() -> Self {
        BlockArray(core::array::from_fn(|_| Block::new()))
    }
}

impl AsRef<[Block]> for BlockArray {
    fn as_ref(&self) -> &[Block] {
        &self.0
    }
}

impl AsMut<[Block]> for BlockArray {
    fn as_mut(&mut self) -> &mut [Block] {
        &mut self.0
    }
}

impl Deref for BlockIndexer {
    type Target = ArrayVec<BlockArray>;

    fn deref(&self) -> &Self::Target {
        &self.blocks
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

impl Default for Block {
    fn default() -> Self {
        Block::new()
    }
}
