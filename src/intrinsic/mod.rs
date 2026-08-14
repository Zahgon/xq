use std::rc::Rc;

use itertools::Itertools;
use num::{Float, ToPrimitive};
use phf::phf_map;

pub(crate) use self::{
    binary::binary,
    comparator::comparator,
    index::{index, slice},
    path::{del_paths, get_path, set_path},
    string::{stringifier, text},
    unary::unary,
};
use crate::{
    compile::compiler::{ArgType, FunctionIdentifier},
    util::make_owned,
    vm::{
        bytecode::{NamedFn0, NamedFn1, NamedFn2, NamedFn3},
        error::Result,
        ByteCode, QueryExecutionError,
    },
    Array, Value,
};

mod binary;
mod comparator;
mod index;
#[macro_use]
mod math;
mod path;
mod regex;
mod string;
mod time;
mod unary;

static INTRINSICS0: phf::Map<&'static str, NamedFn0> = phf_map! {
    "error" => NamedFn0 { name: "error", func: error },
    "type" => NamedFn0 { name: "type", func: get_type },
    "length" => NamedFn0 { name: "length", func: length },
    "utf8bytelength" => NamedFn0 { name: "utf8bytelength", func: utf8_byte_length },
    "keys_unsorted" =>  NamedFn0 { name: "keys_unsorted", func: keys_unsorted },
    "keys" =>  NamedFn0 { name: "keys", func: keys },
    "sort" => NamedFn0 { name: "sort", func: sort },
    "reverse" => NamedFn0 { name: "reverse", func: reverse },
    "tostring" => NamedFn0 { name: "tostring", func: text },
    "tonumber" => NamedFn0 { name: "to_number", func: string::to_number },
    "fromjson" => NamedFn0 { name: "fromjson", func: string::from_json },
    "tojson" => NamedFn0 { name: "tojson", func: string::to_json },
    "explode" => NamedFn0 { name: "explode", func: string::explode },
    "implode" => NamedFn0 { name: "implode", func: string::implode },

    "gmtime" => NamedFn0 { name: "gmtime", func: time::gm_time },
    "localtime" => NamedFn0 { name: "localtime", func: time::gm_time_local },
    "mktime" => NamedFn0 { name: "mktime", func: time::mk_time },
    "now" => NamedFn0 { name: "now", func: time::now },
    "fromdateiso8601" => NamedFn0 { name: "fromdateiso8601", func: time::fromdateiso8601 },

    "nan" => NamedFn0 { name: "nan", func: math::nan },
    "infinite" => NamedFn0 { name: "infinite", func: math::infinite },
    "isnan" => as_math_fn!(is_nan),
    "isnormal" => as_math_fn!(is_normal),
    "isinfinite" => as_math_fn!(is_infinite),
    "floor" => as_math_fn!(floor),
    "round" => as_math_fn!(round),
    "ceil" => as_math_fn!(ceil),
    "trunc" => as_math_fn!(trunc),
    "fabs" => as_math_fn!(abs),
    "sqrt" => as_math_fn!(sqrt),
    "cbrt" => as_math_fn!(cbrt),
    "sin" => as_math_fn!(sin),
    "cos" => as_math_fn!(cos),
    "tan" => as_math_fn!(tan),
    "asin" => as_math_fn!(asin),
    "acos" => as_math_fn!(acos),
    "atan" => as_math_fn!(atan),
    "sinh" => as_math_fn!(sinh),
    "cosh" => as_math_fn!(cosh),
    "tanh" => as_math_fn!(tanh),
    "asinh" => as_math_fn!(asinh),
    "acosh" => as_math_fn!(acosh),
    "atanh" => as_math_fn!(atanh),
    "exp" => as_math_fn!(exp),
    "exp2" => as_math_fn!(exp2),
    "exp10" => as_math_fn!(exp10),
    "expm1" => as_math_fn!(exp_m1),
    "log" => as_math_fn!(ln),
    "log2" => as_math_fn!(log2),
    "log10" => as_math_fn!(log10),
};
static INTRINSICS1: phf::Map<&'static str, NamedFn1> = phf_map! {
    "error" => NamedFn1 { name: "error", func: error1 },
    "has" => NamedFn1 { name: "has", func: has },
    "in" => NamedFn1 { name: "in", func: |i, c| has(c, i) },
    "contains" => NamedFn1 { name: "contains", func: contains },
    "inside" => NamedFn1 { name: "inside", func: |i, c| contains(c, i) },
    "indices" => NamedFn1 { name: "indices", func: indices },
    "startswith" => NamedFn1 { name: "startswith", func: starts_with },
    "endswith" => NamedFn1 { name: "endswith", func: ends_with },
    "split" => NamedFn1 { name: "split", func: split1 },
    "_min_by" => NamedFn1 { name: "_min_by", func: min_by },
    "_max_by" => NamedFn1 { name: "_max_by", func: max_by },
    "_group_by" => NamedFn1 { name: "_group_by", func: group_by },
    "_unique_by" => NamedFn1 { name: "_unique_by", func: unique_by },
    "delpaths" => NamedFn1 { name: "delpaths", func: path::del_paths },
    "bsearch" => NamedFn1 { name: "bsearch", func: binary_search },
    "format" => NamedFn1 { name: "format", func: format },

    "strftime" => NamedFn1 { name: "strftime", func: time::format_time },
    "strptime" => NamedFn1 { name: "strptime", func: time::parse_time },
    "strflocaltime" => NamedFn1 { name: "strflocaltime", func: time::format_time_local },
};
static INTRINSICS2: phf::Map<&'static str, NamedFn2> = phf_map! {
    "setpath" => NamedFn2 { name: "setpath", func: path::set_path },
    "__split_match_impl" => NamedFn2 { name: "__split_match_impl", func: regex::split_match_impl },

    "fmax" => as_math_fn2!(fmax),
    "fmin" => as_math_fn2!(fmin),
    "copysign" => as_math_fn2!(copysign),
    "atan2" => as_math_fn2!(atan2),
    "hypot" => as_math_fn2!(hypot),
    "pow" => as_math_fn2!(powf),
};
static INTRINSICS3: phf::Map<&'static str, NamedFn3> = phf_map! {
    "fma" => as_math_fn3!(fma),
};

pub(crate) fn lookup_intrinsic_fn(
    FunctionIdentifier(ident, n_args): &FunctionIdentifier,
) -> Option<(ByteCode, Vec<ArgType>)> { panic!("STUB: not implemented") }

pub(crate) fn truthy(value: Value) -> bool { panic!("STUB: not implemented") }

fn error(value: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn error1(_: Value, arg: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn get_type(context: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn length(context: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn utf8_byte_length(context: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn keys_unsorted(context: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn keys(context: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn has(context: Value, index: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn contains(context: Value, element: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn sort(context: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn reverse(context: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn min_by(context: Value, keys: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn max_by(context: Value, keys: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn group_by(context: Value, keys: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn unique_by(context: Value, keys: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn indices(context: Value, s: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn starts_with(context: Value, s: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn ends_with(context: Value, s: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn split1(context: Value, s: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn binary_search(context: Value, x: Value) -> Result<Value> { panic!("STUB: not implemented") }

fn format(context: Value, s: Value) -> Result<Value> { panic!("STUB: not implemented") }
