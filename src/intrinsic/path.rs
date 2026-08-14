use std::{ops::Range, rc::Rc};

use itertools::{repeat_n, Itertools};
use num::ToPrimitive;

use crate::{
    util::make_owned,
    value::RcString,
    vm::{error::Result, QueryExecutionError},
    Array, Object, Value,
};

pub(crate) fn get_path(context: Value, path: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn set_path(context: Value, path: Value, value: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn del_paths(context: Value, paths: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn map_to_slice_range(length: usize, index: &Object) -> Result<Range<usize>> { panic!("STUB: not implemented") }

fn get_path_rec(context: Value, path: &[Value]) -> Result<Value> { panic!("STUB: not implemented") }

fn set_path_rec(context: Value, path: &[Value], replacement: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn replace_tombstone_rec(context: Value, path: &[Value], placeholder: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn del_tombstone_rec(
    original: &Value,
    tomb_stoned: Value,
    placeholder: &RcString,
) -> Result<Option<Value>> { panic!("STUB: not implemented") }
