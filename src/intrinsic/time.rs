use std::rc::Rc;

use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};
use time_fmt::{
    format::format_zoned_offset_date_time,
    parse::{parse_date_time_maybe_with_zone, TimeZoneSpecifier},
};
use time_tz::{system::get_timezone, Offset, TimeZone};

use crate::{
    number::PrimitiveReal,
    value::RcString,
    vm::{QueryExecutionError, Result},
    Array, Value,
};

fn try_unwrap_number(value: &Value) -> Option<PrimitiveReal> { panic!("STUB: not implemented") }

fn try_unwrap_string(value: &Value) -> Option<RcString> { panic!("STUB: not implemented") }

fn try_unwrap_array(value: &Value) -> Option<Rc<Array>> { panic!("STUB: not implemented") }

fn timestamp_to_time(timestamp: PrimitiveReal) -> Result<OffsetDateTime> { panic!("STUB: not implemented") }

fn time_to_timestamp(dt: &OffsetDateTime) -> Value { panic!("STUB: not implemented") }

fn time_to_array(dt: &OffsetDateTime) -> Value { panic!("STUB: not implemented") }

fn try_array_to_time(value: &Value) -> Option<Result<PrimitiveDateTime>> { panic!("STUB: not implemented") }

pub(crate) fn format_time(context: Value, format: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn format_time_local(context: Value, format: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn parse_time(context: Value, format: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn fromdateiso8601(context: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn gm_time(context: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn gm_time_local(context: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn mk_time(context: Value) -> Result<Value> { panic!("STUB: not implemented") }

pub(crate) fn now(_: Value) -> Result<Value> { panic!("STUB: not implemented") }
