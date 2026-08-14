use std::collections::HashSet;

use num::{Float, ToPrimitive, Zero};
use xq_lang::ast::BinaryArithmeticOp;

use crate::{
    intrinsic::string,
    vm::{bytecode::NamedFn1, QueryExecutionError},
    Number, Value,
};

pub(crate) fn binary(operator: &BinaryArithmeticOp) -> NamedFn1 { panic!("STUB: not implemented") }

fn add(lhs: Value, rhs: Value) -> Result<Value, QueryExecutionError> { panic!("STUB: not implemented") }

fn subtract(lhs: Value, rhs: Value) -> Result<Value, QueryExecutionError> { panic!("STUB: not implemented") }

fn multiply(lhs: Value, rhs: Value) -> Result<Value, QueryExecutionError> { panic!("STUB: not implemented") }

fn divide(lhs: Value, rhs: Value) -> Result<Value, QueryExecutionError> { panic!("STUB: not implemented") }

fn modulo(lhs: Value, rhs: Value) -> Result<Value, QueryExecutionError> { panic!("STUB: not implemented") }

fn number_to_i64(n: Number) -> i64 { panic!("STUB: not implemented") }
