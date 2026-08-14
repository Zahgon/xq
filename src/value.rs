use std::{
    borrow::Borrow,
    cmp::Ordering,
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::Deref,
    rc::Rc,
    slice::SliceIndex,
};

use derive_more::{DebugCustom, Display, Index, IndexMut, IntoIterator, IsVariant, Unwrap};
use itertools::Itertools;
use num::Float;
use serde::{
    de::{Error, MapAccess, SeqAccess, Visitor},
    ser::{SerializeMap, SerializeSeq},
    Deserialize, Deserializer, Serialize, Serializer,
};

use crate::Number;

type Vector = Vec<Value>;
type Map = HashMap<RcString, Value>;
pub type RcString = Rc<String>;

#[derive(
    Clone, Eq, PartialEq, Hash, Default, DebugCustom, Display, IntoIterator, Index, IndexMut,
)]
#[debug(fmt = "{_0:?}")]
#[display(fmt = "{_0:?}")]
pub struct Array(#[into_iterator(owned, ref, ref_mut)] Vector);

#[derive(Clone, Eq, PartialEq, Default, DebugCustom, Display, IntoIterator, Index, IndexMut)]
#[debug(fmt = "{_0:?}")]
#[display(fmt = "{_0:?}")]
pub struct Object(#[into_iterator(owned, ref, ref_mut)] Map);

#[allow(clippy::derived_hash_with_manual_eq)] 
impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) { panic!("STUB: not implemented") }
}

impl Array {
    pub fn new() -> Self { panic!("STUB: not implemented") }

    pub fn with_capacity(capacity: usize) -> Self { panic!("STUB: not implemented") }

    pub fn from_vec(v: Vector) -> Self { panic!("STUB: not implemented") }

    pub fn is_empty(&self) -> bool { panic!("STUB: not implemented") }

    pub fn len(&self) -> usize { panic!("STUB: not implemented") }

    pub fn push<V>(&mut self, value: V)
    where
        V: Into<Value>,
    { panic!("STUB: not implemented") }

    pub fn extend<I: IntoIterator<Item = Value>>(&mut self, iter: I) { panic!("STUB: not implemented") }

    pub fn get<I: SliceIndex<[Value]>>(&self, index: I) -> Option<&I::Output> { panic!("STUB: not implemented") }

    pub fn remove(&mut self, index: usize) -> Value { panic!("STUB: not implemented") }

    pub fn split_off(&mut self, index: usize) -> Self { panic!("STUB: not implemented") }
}

impl Object {
    pub fn new() -> Self { panic!("STUB: not implemented") }

    pub fn with_capacity(capacity: usize) -> Self { panic!("STUB: not implemented") }

    pub fn is_empty(&self) -> bool { panic!("STUB: not implemented") }

    pub fn len(&self) -> usize { panic!("STUB: not implemented") }

    pub fn iter(&self) -> impl Iterator<Item = (&RcString, &Value)> {
        self.0.iter()
    }

    pub fn insert<K, V>(&mut self, key: K, value: V) -> Option<Value>
    where
        K: Into<RcString>,
        V: Into<Value>,
    { panic!("STUB: not implemented") }

    pub fn extend<T: IntoIterator<Item = (RcString, Value)>>(&mut self, iter: T) { panic!("STUB: not implemented") }

    pub fn keys(&self) -> impl Iterator<Item = &RcString> {
        self.0.keys()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&Value>
    where
        RcString: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    { panic!("STUB: not implemented") }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut Value>
    where
        RcString: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    { panic!("STUB: not implemented") }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<Value>
    where
        RcString: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    { panic!("STUB: not implemented") }

    pub fn entry(
        &mut self,
        key: RcString,
    ) -> std::collections::hash_map::Entry<'_, RcString, Value> { panic!("STUB: not implemented") }
}

impl FromIterator<Value> for Array {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self { panic!("STUB: not implemented") }
}

impl FromIterator<(RcString, Value)> for Object {
    fn from_iter<T: IntoIterator<Item = (RcString, Value)>>(iter: T) -> Self { panic!("STUB: not implemented") }
}

impl Deref for Array {
    type Target = [Value];

    fn deref(&self) -> &Self::Target { panic!("STUB: not implemented") }
}

#[derive(
    Clone, Eq, PartialEq, Hash, derive_more::From, derive_more::TryInto, IsVariant, Unwrap,
)]
pub enum Value {
    Null,
    Boolean(bool),
    Number(Number),
    String(RcString),
    Array(Rc<Array>),
    Object(Rc<Object>),
}

impl From<String> for Value {
    fn from(s: String) -> Self { panic!("STUB: not implemented") }
}

impl From<Array> for Value {
    fn from(arr: Array) -> Self { panic!("STUB: not implemented") }
}

impl From<Object> for Value {
    fn from(obj: Object) -> Self { panic!("STUB: not implemented") }
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { panic!("STUB: not implemented") }
}
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { panic!("STUB: not implemented") }
}

impl Value {
    pub fn number<T: Into<Number>>(n: T) -> Self { panic!("STUB: not implemented") }
    pub fn string(s: String) -> Self { panic!("STUB: not implemented") }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    { panic!("STUB: not implemented") }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    { panic!("STUB: not implemented") }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { panic!("STUB: not implemented") }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering { panic!("STUB: not implemented") }
}
