use std::cmp::Ordering;

use itertools::Itertools;
use num::Float;
use xq_lang::ast::Comparator;

use crate::{vm::bytecode::NamedFn1, Value};

pub(crate) fn comparator(comparator: &Comparator) -> NamedFn1 { panic!("STUB: not implemented") }

fn compare(lhs: Value, rhs: Value) -> Option<Ordering> { panic!("STUB: not implemented") }

struct PartialValue<'a>(&'a Value);

impl<'a> PartialEq<Self> for PartialValue<'a> {
    fn eq(&self, other: &Self) -> bool { panic!("STUB: not implemented") }
}

impl<'a> PartialOrd<Self> for PartialValue<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { panic!("STUB: not implemented") }
}
