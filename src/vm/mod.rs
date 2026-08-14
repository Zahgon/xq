#[allow(dead_code)]
pub mod bytecode;
#[allow(dead_code)]
pub mod error;
pub mod machine;

pub(crate) use bytecode::{ByteCode, Program};
pub use error::{QueryExecutionError, Result};

use crate::Value;

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Address(pub(crate) usize);

impl Address {
    fn next(&mut self) { panic!("STUB: not implemented") }

    pub(crate) fn get_next(&self) -> Self { panic!("STUB: not implemented") }
}

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct ScopeId(pub(crate) usize);

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct ScopedSlot(pub(crate) ScopeId, pub(crate) usize);
