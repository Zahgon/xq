use std::rc::Rc;

use onig::{Regex, RegexOptions, Syntax};

use crate::{
    vm::{QueryExecutionError, Result},
    Array, Object, Value,
};

fn to_string(func: &'static str, v: Value) -> Result<Rc<String>> { panic!("STUB: not implemented") }

fn compile_regex(pattern: &str, flags: &str) -> Result<(Regex, bool)> { panic!("STUB: not implemented") }

pub(crate) fn split_match_impl(context: Value, pattern: Value, flags: Value) -> Result<Value> { panic!("STUB: not implemented") }
