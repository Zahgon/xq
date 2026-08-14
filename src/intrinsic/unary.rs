use xq_lang::ast::UnaryOp;

use crate::{
    vm::{bytecode::NamedFn0, QueryExecutionError},
    Value,
};

pub(crate) fn unary(operator: &UnaryOp) -> NamedFn0 { panic!("STUB: not implemented") }

fn unary_plus(value: Value) -> Result<Value, QueryExecutionError> { panic!("STUB: not implemented") }

fn unary_minus(value: Value) -> Result<Value, QueryExecutionError> { panic!("STUB: not implemented") }
