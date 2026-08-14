use std::{fmt::Formatter, ops::Neg, str::FromStr};

use cast::i32;
use derive_more::{DebugCustom, Display};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub(crate) type PrimitiveReal = f64;

#[derive(
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    DebugCustom,
    Display,
    num_derive::FromPrimitive,
    num_derive::ToPrimitive,
    num_derive::NumCast,
    num_derive::NumOps,
    num_derive::Zero,
    num_derive::One,
    num_derive::Num,
    num_derive::Float,
)]
#[debug(fmt = "{_0}")]
#[display(fmt = "{_0}")]
pub struct Number(OrderedFloat<PrimitiveReal>);

impl Number {
    pub(crate) fn to_primitive_real(self) -> PrimitiveReal { panic!("STUB: not implemented") }
}

impl FromStr for Number {
    type Err = <PrimitiveReal as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> { panic!("STUB: not implemented") }
}

impl Neg for Number {
    type Output = Self;
    fn neg(self) -> Self::Output { panic!("STUB: not implemented") }
}

impl<T> From<T> for Number
where
    PrimitiveReal: cast::From<T, Output = PrimitiveReal>,
{
    fn from(v: T) -> Self { panic!("STUB: not implemented") }
}

impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    { panic!("STUB: not implemented") }
}

impl<'de> Deserialize<'de> for Number {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    { panic!("STUB: not implemented") }
}
