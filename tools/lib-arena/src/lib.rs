#![feature(const_fn)]
pub mod local;
pub mod sync;

use std::rc::Rc;
use std::sync::Arc;

pub trait Arena {
    type Value;

    #[allow(clippy::mut_from_ref)]
    fn alloc(&self, value: Self::Value) -> &mut Self::Value;
}

impl<A: ?Sized + Arena> Arena for &A {
    type Value = A::Value;

    fn alloc(&self, value: Self::Value) -> &mut Self::Value {
        A::alloc(self, value)
    }
}

impl<A: ?Sized + Arena> Arena for &mut A {
    type Value = A::Value;

    fn alloc(&self, value: Self::Value) -> &mut Self::Value {
        A::alloc(self, value)
    }
}

impl<A: ?Sized + Arena> Arena for Box<A> {
    type Value = A::Value;

    fn alloc(&self, value: Self::Value) -> &mut Self::Value {
        A::alloc(self, value)
    }
}

impl<A: ?Sized + Arena> Arena for Rc<A> {
    type Value = A::Value;

    fn alloc(&self, value: Self::Value) -> &mut Self::Value {
        A::alloc(self, value)
    }
}

impl<A: ?Sized + Arena> Arena for Arc<A> {
    type Value = A::Value;

    fn alloc(&self, value: Self::Value) -> &mut Self::Value {
        A::alloc(self, value)
    }
}