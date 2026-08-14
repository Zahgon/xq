use std::{ops::Range, rc::Rc};

use num::{ToPrimitive, Zero};

use crate::{
    vm::{error::Result, machine::PathElement, QueryExecutionError},
    Array, Number, Object, Value,
};

fn number_to_isize(n: Number) -> isize { panic!("STUB: not implemented") }

fn parse_and_shift_index<F: Fn(Value) -> QueryExecutionError>(
    array_length: usize,
    index: &Value,
    err: F,
) -> Result<Option<usize>> { panic!("STUB: not implemented") }

pub(crate) fn index(value: Value, index: Value) -> Result<(Value, PathElement)> { panic!("STUB: not implemented") }

pub(crate) fn calculate_slice_index(
    length: usize,
    start: Option<&Value>,
    end: Option<&Value>,
) -> Result<Range<usize>> { panic!("STUB: not implemented") }

pub(crate) fn slice(
    value: Value,
    start: Option<Value>,
    end: Option<Value>,
) -> Result<(Value, PathElement)> { panic!("STUB: not implemented") }
