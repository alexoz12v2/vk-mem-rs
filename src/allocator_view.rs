use crate::{Allocator, AllocatorView};
use alloc::sync::Arc;
use core::ops::Deref;

pub trait AsAllocatorView {
    fn as_allocator_view(&self) -> AllocatorView;
}

impl AsAllocatorView for AllocatorView {
    fn as_allocator_view(&self) -> AllocatorView {
        *self
    }
}

impl AsAllocatorView for Arc<Allocator> {
    fn as_allocator_view(&self) -> AllocatorView {
        *(self.deref().deref())
    }
}

impl AsAllocatorView for &Allocator {
    fn as_allocator_view(&self) -> AllocatorView {
        ***self
    }
}

impl AsAllocatorView for Allocator {
    fn as_allocator_view(&self) -> AllocatorView {
        *(self.deref())
    }
}
