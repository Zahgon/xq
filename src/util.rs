use std::{cell::RefCell, rc::Rc};

pub(crate) fn make_owned<T: Clone>(v: Rc<T>) -> T { panic!("STUB: not implemented") }

pub struct SharedIterator<I>(Rc<RefCell<I>>);
impl<I> From<I> for SharedIterator<I> {
    fn from(it: I) -> Self { panic!("STUB: not implemented") }
}
impl<I> Clone for SharedIterator<I> {
    fn clone(&self) -> Self { panic!("STUB: not implemented") }
}
impl<T, I: Iterator<Item = T>> Iterator for SharedIterator<I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> { panic!("STUB: not implemented") }
}
