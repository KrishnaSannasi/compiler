
use std::collections::HashSet;
use std::alloc::{Layout, alloc, handle_alloc_error};
use std::{ptr, fmt};
use std::hash::{Hasher, Hash};

use parking_lot::RwLock;
use once_cell::sync::OnceCell;

macro_rules! static_assert {
    ($a:expr) => {
        const _: () = [()][(!$a) as usize];
    }
}

static_assert! { std::mem::align_of::<u8>() < std::mem::align_of::<usize>() }
static_assert! { std::mem::align_of::<ThinStr>() == std::mem::align_of::<usize>() }
static_assert! { std::mem::size_of::<ThinStr>() == std::mem::size_of::<usize>() }

type Interner = RwLock<HashSet<Box<ThinStrInner>>>;
static INTERNER: OnceCell<Interner> = OnceCell::INIT;

pub fn intern(value: &str) -> ThinStr {
    let interner = INTERNER.get_or_init(Default::default);
    let data = interner.read();

    if let Some(value) = data.get(value) {
        ThinStr {
            ptr: ptr::NonNull::from(&**value).cast(),
        }
    } else {
        drop(data);

        intern_slow(interner, value)
    }
}

#[cold]
fn intern_slow(interner: &Interner, value: &str) -> ThinStr {
    unsafe {
        let len = value.len() + std::mem::size_of::<usize>();
        let len = (len + std::mem::align_of::<usize>() - 1) / std::mem::align_of::<usize>() * std::mem::align_of::<usize>();

        let layout = Layout::from_size_align_unchecked(
            len,
            std::mem::align_of::<usize>()
        );

        let ptr = ptr::NonNull::new(alloc(layout));

        let ptr = match ptr {
            Some(ptr) => ptr,
            None => handle_alloc_error(layout)
        };

        ptr.cast::<usize>().as_ptr().write(value.len());

        ptr::copy_nonoverlapping(
            value.as_ptr(),
            ptr.cast::<u8>().as_ptr().add(std::mem::size_of::<usize>()),
            value.len()
        );

        let interned_value: *mut [u8] = std::slice::from_raw_parts_mut(
            ptr.cast::<u8>().as_ptr(),
            value.len()
        );
        
        // this allocation is guarded to align of usize
        #[allow(clippy::cast_ptr_alignment)]
        let interned_value: *mut ThinStrInner = interned_value as _;
        let interned_value = Box::from_raw(interned_value);
        let value = ThinStr {
            ptr: ptr::NonNull::from(&*interned_value).cast(),
        };
        
        interner
            .write()
            .insert(interned_value);

        value
    }
}

#[repr(C)]
#[derive(Debug, Eq)]
struct ThinStrInner {
    len: usize,
    data: str
}

impl std::borrow::Borrow<str> for Box<ThinStrInner> {
    fn borrow(&self) -> &str {
        &self.data
    }
}

impl PartialEq for ThinStrInner {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Hash for ThinStrInner {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state)
    }
}

#[derive(Clone, Copy, Eq)]
pub struct ThinStr {
    ptr: ptr::NonNull<()>,
}

impl fmt::Debug for ThinStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.to_str(), f)
    }
}

impl fmt::Display for ThinStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.to_str(), f)
    }
}

impl fmt::Pointer for ThinStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&self.ptr, f)
    }
}

impl PartialEq<ThinStr> for ThinStr {
    fn eq(&self, other: &ThinStr) -> bool {
        std::ptr::eq(self.ptr.as_ptr(), other.ptr.as_ptr())
    }
}

impl Hash for ThinStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.ptr.as_ptr(), state)
    }
}

impl std::borrow::Borrow<str> for ThinStr {
    fn borrow(&self) -> &str {
        self.to_str()
    }
}

impl AsRef<str> for ThinStr {
    fn as_ref(&self) -> &str {
        self.to_str()
    }
}

impl AsRef<ThinStr> for ThinStr {
    fn as_ref(&self) -> &ThinStr {
        self
    }
}

impl ThinStr {
    pub fn to_str(self) -> &'static str {
        unsafe {
            let len = *self.ptr.cast::<usize>().as_ptr();
        
            let slice = std::slice::from_raw_parts(
                self.ptr.cast::<u8>().as_ptr().add(std::mem::size_of::<usize>()),
                len
            );

            std::str::from_utf8_unchecked(slice)
        }
    }
}

#[test]
fn simple_intern() {
    let w = intern("bar");
    let x = intern("foo");
    let y = intern("foo");
    let z = intern("foo");

    assert_eq!(x, y);
    assert_eq!(x, z);
    assert_eq!(y, z);
    
    assert_ne!(x, w);
    assert_ne!(y, w);
    assert_ne!(z, w);
}
