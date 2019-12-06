use parking_lot::RwLock;

const DEFAULT_SLAB_CAPACITY: usize = 64;

pub struct SyncArena<T> {
    data: RwLock<Vec<RwLock<Vec<T>>>>,
}

impl<T> Default for SyncArena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SyncArena<T> {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self {
            data: RwLock::new(Vec::new()),
        }
    }

    #[allow(clippy::mut_from_ref)]
    fn alloc_new_slab(&self, value: T) -> &mut T {
        let mut data = self.data.write();

        let len = data.len();
        data.push(RwLock::new(Vec::with_capacity(DEFAULT_SLAB_CAPACITY)));
        let slab = unsafe { data.get_unchecked_mut(len).get_mut() };

        slab.push(value);

        unsafe { &mut *slab.as_mut_ptr() }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn alloc(&self, value: T) -> &mut T {
        let data = self.data.read();

        if data.is_empty() {
            drop(data);

            return self.alloc_new_slab(value);
        }

        for slab in data.iter() {
            let slab = slab.try_write();

            if let Some(mut slab) = slab {
                let len = slab.len();

                if len < slab.capacity() {
                    slab.push(value);

                    return unsafe { &mut *slab.as_mut_ptr().add(len) };
                }
            }
        }

        drop(data);

        self.alloc_new_slab(value)
    }

    pub fn slab_count(&self) -> usize {
        self.data.read().len()
    }

    pub fn value_count(&self) -> usize {
        self.data.read().iter().map(|slab| slab.read().len()).sum()
    }

    pub fn slab_capacity(&self) -> usize {
        self.data.read().capacity()
    }

    pub fn value_capacity(&self) -> usize {
        self.data
            .read()
            .iter()
            .map(|slab| slab.read().capacity())
            .sum()
    }

    pub fn value_remaining_capacity(&self) -> usize {
        self.data
            .read()
            .iter()
            .map(|slab| {
                let slab = slab.read();

                slab.capacity() - slab.len()
            })
            .sum()
    }
}

impl<T> super::Arena for SyncArena<T> {
    type Value = T;

    fn alloc(&self, value: T) -> &mut T {
        self.alloc(value)
    }
}

#[test]
fn simple() {
    let arena = SyncArena::new();

    for i in 0..100 {
        assert_eq!(*arena.alloc(i), i);
    }
}

#[test]
fn stampede() {
    static ARENA: SyncArena<i32> = SyncArena::new();

    let mut threads = Vec::new();

    for i in 0..4 {
        threads.push(std::thread::spawn(move || {
            let mut ptrs = Vec::new();

            for _ in 0..10_000 {
                ptrs.push(ARENA.alloc(i))
            }

            for ptr in ptrs {
                assert_eq!(*ptr, i)
            }
        }))
    }

    for thread in threads {
        thread.join().unwrap();
    }

    dbg!(ARENA.slab_capacity());
    dbg!(ARENA.slab_count());

    dbg!(ARENA.value_capacity());
    dbg!(ARENA.value_count());
    dbg!(ARENA.value_remaining_capacity());
}
