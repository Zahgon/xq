use std::fmt::{Debug, Formatter};

use crate::{
    vm::{Address, Result, ScopeId, ScopedSlot},
    Value,
};

#[derive(Clone, Eq, PartialEq)]
pub struct NamedFunction<F: Clone> {
    pub name: &'static str,
    pub func: F,
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct ClosureAddress(pub(crate) Address);

pub type NamedFn0 = NamedFunction<fn(Value) -> Result<Value>>;
pub type NamedFn1 = NamedFunction<fn(Value, Value) -> Result<Value>>;
pub type NamedFn2 = NamedFunction<fn(Value, Value, Value) -> Result<Value>>;
pub type NamedFn3 = NamedFunction<fn(Value, Value, Value, Value) -> Result<Value>>;

impl<F: Clone> Debug for NamedFunction<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ByteCode {
    
    Unreachable,
    
    PlaceHolder,
    
    Nop,
    
    Push(Value),
    
    Pop,
    
    Dup,
    
    Swap,
    
    Const(Value),
    
    Load(ScopedSlot),
    
    Store(ScopedSlot),
    
    PushClosure(ClosureAddress),
    
    StoreClosure(ScopedSlot),
    
    AppendObject,
    
    Append(ScopedSlot),

    Index,
    
    Slice {
        start: bool,
        end: bool,
    },
    
    Each,
    
    Access,

    EnterPathTracking,
    ExitPathTracking,
    EnterNonPathTracking,
    ExitNonPathTracking,

    Fork {
        fork_pc: Address,
    },
    
    ForkTryBegin {
        catch_pc: Option<Address>,
    },
    
    ForkTryEnd,
    
    ForkAlt {
        fork_pc: Address,
    },
    
    ForkLabel(ScopedSlot),
    
    Break(ScopedSlot),
    
    Backtrack,
    
    Jump(Address),
    
    JumpUnless(Address),
    
    CallClosure(ScopedSlot),
    
    Call(Address),
    
    TailCall(Address),
    
    CallChainRet(Address),
    
    TailCallClosure(ScopedSlot),
    
    NewFrame {
        id: ScopeId,
        variable_cnt: usize,
        closure_cnt: usize,
        label_cnt: usize,
    },
    
    Ret,
    
    Output,
    
    Input,

    Intrinsic0(NamedFn0),
    
    Intrinsic1(NamedFn1),
    
    Intrinsic2(NamedFn2),
    
    Intrinsic3(NamedFn3),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub(crate) code: Vec<ByteCode>,
    pub(crate) entry_point: Address,
}

impl Program {
    pub(crate) fn fetch_code(&self, pc: Address) -> Option<&ByteCode> { panic!("STUB: not implemented") }
}
