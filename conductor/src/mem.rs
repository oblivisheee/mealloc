use mealloc_container::mem::Address;
use tinyvec::SliceVec;
/// Buffer for memory allocation
pub struct Buffer(pub SliceVec<'static, u8>);

impl Buffer {
    /// Create an empty buffer with no capacity
    pub fn empty() -> Self {
        Buffer(SliceVec::default())
    }

    /// Create a buffer with a given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        const SIZE: usize = 10 * 1024;
        if capacity < SIZE {
            panic!("Buffer capacity is too small");
        }
        static mut STORAGE: [u8; SIZE] = [0; SIZE];
        let slice = unsafe { &mut STORAGE[..capacity - SIZE] };

        Buffer(slice.into())
    }
    /// Create a buffer from a slice
    pub fn from_slice(slice: &'static mut [u8]) -> Self {
        Buffer(slice.into())
    }

    pub fn zero_range(&mut self, start: impl Into<Address>, end: impl Into<Address>) {
        let start_ptr = start.into().as_ptr() as usize;
        let end_ptr = end.into().as_ptr() as usize;
        if start_ptr >= end_ptr || end_ptr > self.0.len() {
            panic!("Invalid address range");
        }
        for i in start_ptr..end_ptr {
            self.0[i] = 0;
        }
    }
}

impl From<&'static mut [u8]> for Buffer {
    fn from(slice: &'static mut [u8]) -> Self {
        Buffer::from_slice(slice)
    }
}
impl core::ops::Deref for Buffer {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}
impl core::ops::DerefMut for Buffer {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}
