
use lib_arena::{Arena, local::LocalArena, sync::SyncArena};
use super::{HAst, Let, Expr, Assign};

pub type LocalContext<A, B, C, D> =
    Context<LocalArena<A>, LocalArena<B>, LocalArena<C>, LocalArena<D>>;
    
pub type SyncContext<A, B, C, D> =
    Context<SyncArena<A>, SyncArena<B>, SyncArena<C>, SyncArena<D>>;

#[derive(Default)]
pub struct Context<A, B, C, D> {
    pub high_ast: A,
    pub node_let: B,
    pub node_assign: C,
    pub expr: D,
}

#[derive(Clone, Copy)]
pub struct ContextRef<'input, 'hacx> {
    high_ast: &'hacx dyn Arena<Value = HAst<'input, 'hacx>>,
    node_let: &'hacx dyn Arena<Value = Let<'input, 'hacx>>,
    node_assign: &'hacx dyn Arena<Value = Assign<'input, 'hacx>>,
    expr: &'hacx dyn Arena<Value = Expr<'input, 'hacx>>,
}

pub trait ContextOverload<'ctx, A> {
    #[allow(clippy::mut_from_ref)]
    fn alloc(self, value: A) -> &'ctx mut A;
}

impl<A, B, C, D> Context<A, B, C, D> {
    pub fn as_ref<'input, 'hacx>(&'hacx self) -> ContextRef<'input, 'hacx>
    where
        A: Arena<Value = HAst<'input, 'hacx>>,
        B: Arena<Value = Let<'input, 'hacx>>,
        C: Arena<Value = Assign<'input, 'hacx>>,
        D: Arena<Value = Expr<'input, 'hacx>>,
    {
        ContextRef {
            high_ast: &self.high_ast,
            node_let: &self.node_let,
            node_assign: &self.node_assign,
            expr: &self.expr,
        }
    }
}

impl<'input, 'hacx> ContextRef<'input, 'hacx> {
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub fn alloc<V>(self, value: V) -> &'hacx mut V
    where
        Self: ContextOverload<'hacx, V>
    {
        ContextOverload::alloc(self, value)
    }
}

macro_rules! overload_set {
    ($($field:ident => $type:ident),* $(,)?) => {$(
        impl<'input, 'hacx> ContextOverload<'hacx, $type<'input, 'hacx>> for ContextRef<'input, 'hacx> {
            #[inline]
            #[allow(clippy::mut_from_ref)]
            fn alloc(self, value: $type<'input, 'hacx>) -> &'hacx mut $type<'input, 'hacx> {
                self.$field.alloc(value)
            }
        }
    )*};
}

overload_set! {
    high_ast => HAst,
    node_let => Let,
    node_assign => Assign,
    expr => Expr,
}
