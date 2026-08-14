#![allow(clippy::uninlined_format_args)]
use derive_more::Display;
use itertools::Itertools;

use crate::data_structure::undo::Undo;

#[derive(Debug, Eq, PartialEq)]
pub struct UStackToken(usize);

#[derive(Debug)]
struct UndoSpanInfo {
    id: UStackToken,
    undo_start: usize,
    stack_initial: usize,
    stack_bottom: usize,
}

#[derive(Debug, Display)]
#[display(fmt = "{}", stack)]
pub struct UStack<T> {
    stack: Vec<T>,
    undoes: Vec<T>,
    spans: Vec<UndoSpanInfo>,
    current: UndoSpanInfo,
    next_token_id: usize,
}

impl<T> Default for UStack<T> {
    fn default() -> Self { panic!("STUB: not implemented") }
}

impl<T> UStack<T> {
    fn new() -> Self { panic!("STUB: not implemented") }
}

impl<T> Undo for UStack<T> {
    type UndoToken = UStackToken;

    fn save(&mut self) -> Self::UndoToken { panic!("STUB: not implemented") }

    fn undo(&mut self, token: Self::UndoToken) { panic!("STUB: not implemented") }
}

impl<T> UStack<T> {
    pub fn is_empty(&self) -> bool { panic!("STUB: not implemented") }

    pub fn len(&self) -> usize { panic!("STUB: not implemented") }

    pub fn top(&self) -> Option<&T> { panic!("STUB: not implemented") }

    pub fn push(&mut self, value: T) { panic!("STUB: not implemented") }

    pub fn pop_and_discard(&mut self) -> bool { panic!("STUB: not implemented") }

    pub fn iter(&self) -> std::slice::Iter<'_, T> { panic!("STUB: not implemented") }

    pub fn into_vec(self) -> Vec<T> { panic!("STUB: not implemented") }
}

impl<T: Clone> UStack<T> {
    pub fn pop(&mut self) -> Option<T> { panic!("STUB: not implemented") }

    pub fn dup(&mut self) { panic!("STUB: not implemented") }

    pub fn top_mut(&mut self) -> Option<&mut T> { panic!("STUB: not implemented") }

    pub fn swap(&mut self) { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_simple() {
        let mut s = UStack::new();
        let token = s.save();
        s.push(1);
        s.undo(token);
        assert_eq!(s.pop(), None);

        s.push(1);
        s.push(2);
        assert_eq!(s.iter().cloned().collect_vec(), vec![1, 2]);
        let token = s.save();
        assert_eq!(s.pop(), Some(2));
        s.push(3);
        s.undo(token);
        assert_eq!(s.iter().cloned().collect_vec(), vec![1, 2]);
        s.push(3);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
    }

    #[test]
    fn test_simple2() {
        let mut s = UStack::new();
        s.push(0);
        let token1 = s.save();
        s.pop();
        s.push(1);
        let _token2 = s.save();
        assert_eq!(s.pop(), Some(1));
        s.undo(token1);
        assert_eq!(s.iter().cloned().collect_vec(), vec![0]);
    }
}
