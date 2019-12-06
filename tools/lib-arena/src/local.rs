use std::cell::UnsafeCell;

pub struct LocalArena<T> {
    data: UnsafeCell<Vec<Vec<T>>>
}

impl<T> LocalArena<T> {
    pub const fn new() -> Self {
        Self {
            data: UnsafeCell::new(Vec::new())
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn alloc(&self, value: T) -> &mut T {
        let last = self.ensure_space();        

        let data = unsafe {
            &mut *self.data.get()
        };

        let slab = unsafe { data.get_unchecked_mut(last) };

        let len = slab.len();

        slab.push(value);

        unsafe { slab.get_unchecked_mut(len) }
    }

    #[inline]
    fn alloc_if_empty(&self) {
        let data = unsafe {
            &*self.data.get()
        };

        if data.is_empty() {
            self.alloc_slab(16);
        }
    }

    #[inline]
    fn ensure_space(&self) -> usize {
        self.alloc_if_empty();
        
        let data = unsafe {
            &*self.data.get()
        };

        let mut last = data.len() - 1;

        let slab = unsafe { data.get_unchecked(last) };
        
        let capacity = slab.capacity();

        if capacity == slab.len() {
            self.alloc_slab(capacity * 2);
            last += 1;
        }

        last
    }

    #[cold]
    fn alloc_slab(&self, capacity: usize) {
        let data = unsafe {
            &mut *self.data.get()
        };
        
        data.push(Vec::with_capacity(capacity));
    }
}

impl<T> super::Arena for LocalArena<T> {
    type Value = T;

    fn alloc(&self, value: T) -> &mut T {
        self.alloc(value)
    }
}

#[test]
fn simple() {
    let arena = LocalArena::new();

    for i in 0..100 {
        assert_eq!(*arena.alloc(i), i);
    }
}