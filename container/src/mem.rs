use core::ops::Range;
use core::ptr::NonNull;
#[derive(Clone, Debug)]
pub struct Span {
    start: Address,
    end: Address,
}

impl Span {
    pub fn new(start: impl Into<Address>, end: impl Into<Address>) -> Self {
        Span {
            start: start.into(),
            end: end.into(),
        }
    }

    pub fn empty() -> Self {
        Span {
            start: Address::empty(),
            end: Address::empty(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.start.is_empty() && self.end.is_empty()
    }

    pub fn get_start_end(&self) -> Option<(Address, Address)> {
        if self.is_empty() {
            None
        } else {
            Some((self.start, self.end))
        }
    }
}

impl Default for Span {
    fn default() -> Self {
        Span::empty()
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Address(NonNull<u8>);

impl Address {
    //FIXME: Here can be a problem that it'll panic if the pointer is null
    pub fn new(ptr: *mut u8) -> Self {
        Self(NonNull::new(ptr).unwrap())
    }
    pub fn empty() -> Self {
        Self(NonNull::dangling())
    }
    pub fn is_empty(&self) -> bool {
        self.0 == NonNull::dangling()
    }
    pub fn as_ptr(&self) -> *mut u8 {
        self.0.as_ptr()
    }
    pub fn offset(&self, count: isize) -> Self {
        Self(NonNull::new(unsafe { self.0.as_ptr().offset(count) }).unwrap())
    }

    pub fn add(&self, count: usize) -> Self {
        Self(NonNull::new(unsafe { self.0.as_ptr().add(count) }).unwrap())
    }

    pub fn sub(&self, count: usize) -> Self {
        Self(NonNull::new(unsafe { self.0.as_ptr().sub(count) }).unwrap())
    }

    pub fn distance(&self, other: &Self) -> isize {
        (self.as_ptr() as isize) - (other.as_ptr() as isize)
    }
}

impl From<Address> for *mut u8 {
    fn from(address: Address) -> *mut u8 {
        address.as_ptr()
    }
}

impl From<*mut u8> for Address {
    fn from(ptr: *mut u8) -> Self {
        Self::new(ptr)
    }
}
