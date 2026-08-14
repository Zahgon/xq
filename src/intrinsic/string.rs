use std::{borrow::Cow, fmt::Write, rc::Rc};

use itertools::Itertools;
use num::{Float, ToPrimitive};

use crate::{
    util::make_owned,
    vm::{bytecode::NamedFn0, QueryExecutionError, Result},
    Array, Number, Value,
};

pub(crate) fn to_number(value: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn stringifier(fmt: &str) -> Option<NamedFn0> { panic!("STUB: not implemented") }

pub(crate) fn explode(value: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn implode(value: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn split(lhs: &str, rhs: &str) -> Value { panic!("STUB: not implemented") }

fn stringify_inner(value: Value) -> Rc<String> { panic!("STUB: not implemented") }

pub(crate) fn text(value: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn from_json(value: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn to_json(value: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn html(value: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn uri(value: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn xsv<F>(value: Value, delim: char, add_string: F) -> Result<Value>
where
    F: Fn(&mut String, &str),
{ panic!("STUB: not implemented") }

fn csv(value: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn tsv(value: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn sh(value: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn base64(value: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn base64d(value: Value) -> Result<Value> { panic!("STUB: not implemented") }
