#![no_std]
mod allocator;
mod mem;
mod types;
use smallvec::{Array, SmallVec};
use types::Container;
/// Maximum number of containers that can be stored in the stack.
const MAX_CONTAINERS_STACK: usize = 128;

struct ContainerArray([Container; MAX_CONTAINERS_STACK]);

unsafe impl Array for ContainerArray {
    type Item = Container;
    fn size() -> usize {
        MAX_CONTAINERS_STACK
    }
}

/// Structure that controls memory of containers.
pub struct Conductor {
    containers: SmallVec<ContainerArray>,
    allocator: Option<allocator::Allocator>,
}
impl Conductor {
    pub fn new() -> Self {
        Self {
            containers: SmallVec::new(),
            allocator: None,
        }
    }
    pub fn load(&mut self, block_size: usize, memory_size: usize) {
        if self.allocator.is_some() {
            panic!("Allocator already loaded");
        }
        self.allocator = Some(allocator::Allocator::new(block_size, memory_size));
    }
    pub fn create_container(&mut self, memory_size: usize) -> Option<&Container> {
        if let Some(allocator) = &mut self.allocator {
            let mut container = Container::new(memory_size);
            let id = container.id().clone();
            if let Some((addr_start, addr_end)) = allocator.allocate(id, memory_size) {
                container.set_span(addr_start, addr_end);
                self.containers.push(container);
                return Some(self.containers.last().unwrap());
            }
        } else {
            panic!("Allocator not loaded");
        }
        None
    }
    pub fn remove_container(&mut self, id: &types::ContainerID) {
        if let Some(allocator) = &mut self.allocator {
            for (index, container) in self.containers.iter().enumerate() {
                if container.id() == id {
                    allocator.deallocate(id);
                    self.containers.remove(index);
                    break;
                }
            }
        } else {
            panic!("Allocator not loaded");
        }
    }
}

#[cfg(test)]
#[macro_use]
extern crate std;
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_conductor() {
        println!("PASSED");
        let mut conductor = Conductor::new();
        println!("PASSED");
        conductor.load(10 * 1024, 1024 * 1024);
        println!("PASSED");
        let container_id = conductor.create_container(1024).unwrap().id().clone();
        assert_eq!(conductor.containers.len(), 1);
        conductor.remove_container(&container_id);
        assert_eq!(conductor.containers.len(), 0);
    }
}
