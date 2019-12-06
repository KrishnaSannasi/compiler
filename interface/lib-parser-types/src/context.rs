
use lib_arena::{Arena, local::LocalArena, sync::SyncArena};
use super::{HAst, Let, Expr};

pub type LocalContext<A, B, C> =
    Context<LocalArena<A>, LocalArena<B>, LocalArena<C>>;
    
pub type SyncContext<A, B, C> =
    Context<SyncArena<A>, SyncArena<B>, SyncArena<C>>;

#[derive(Default)]
pub struct Context<A, B, C> {
    pub high_ast: A,
    pub let_node: B,
    pub expr: C,
}

#[derive(Clone, Copy)]
pub struct ContextRef<'input, 'hacx> {
    high_ast: &'hacx dyn Arena<Value = HAst<'input, 'hacx>>,
    let_node: &'hacx dyn Arena<Value = Let<'input, 'hacx>>,
    expr: &'hacx dyn Arena<Value = Expr<'input, 'hacx>>,
}

pub trait ContextOverload<'ctx, A> {
    #[allow(clippy::mut_from_ref)]
    fn alloc(self, value: A) -> &'ctx mut A;
}

impl<A, B, C> Context<A, B, C> {
    pub fn as_ref<'input, 'hacx>(&'hacx self) -> ContextRef<'input, 'hacx>
    where
        A: Arena<Value = HAst<'input, 'hacx>>,
        B: Arena<Value = Let<'input, 'hacx>>,
        C: Arena<Value = Expr<'input, 'hacx>>,
    {
        ContextRef {
            high_ast: &self.high_ast,
            let_node: &self.let_node,
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

impl<'input, 'hacx> ContextOverload<'hacx, HAst<'input, 'hacx>> for ContextRef<'input, 'hacx> {
    #[inline]
    #[allow(clippy::mut_from_ref)]
    fn alloc(self, value: HAst<'input, 'hacx>) -> &'hacx mut HAst<'input, 'hacx> {
        self.high_ast.alloc(value)
    }
}

impl<'input, 'hacx> ContextOverload<'hacx, Let<'input, 'hacx>> for ContextRef<'input, 'hacx> {
    #[inline]
    #[allow(clippy::mut_from_ref)]
    fn alloc(self, value: Let<'input, 'hacx>) -> &'hacx mut Let<'input, 'hacx> {
        self.let_node.alloc(value)
    }
}

impl<'input, 'hacx> ContextOverload<'hacx, Expr<'input, 'hacx>> for ContextRef<'input, 'hacx> {
    #[inline]
    #[allow(clippy::mut_from_ref)]
    fn alloc(self, value: Expr<'input, 'hacx>) -> &'hacx mut Expr<'input, 'hacx> {
        self.expr.alloc(value)
    }
}