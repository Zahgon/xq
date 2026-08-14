pub mod undo;

use std::{borrow::Borrow, rc::Rc};

pub type PVector<T> = imbl::Vector<T>;
pub type PHashMap<K, V> = imbl::HashMap<K, V>;

#[derive(Clone, Debug)]
pub(crate) struct PStack<T, B = [T; 32]> {
    prev: Option<Rc<PStack<T, B>>>,
    current: sized_chunks::InlineArray<T, B>,
}

impl<T, B> Default for PStack<T, B> {
    fn default() -> Self { panic!("STUB: not implemented") }
}

#[allow(dead_code)]
impl<T: Clone, B> PStack<T, B> {
    pub fn new() -> Self { panic!("STUB: not implemented") }

    pub fn push(&mut self, value: T) { panic!("STUB: not implemented") }

    pub fn pop(&mut self) -> Option<T> { panic!("STUB: not implemented") }

    pub fn top(&self) -> Option<&T> { panic!("STUB: not implemented") }

    pub fn top_mut(&mut self) -> Option<&mut T> { panic!("STUB: not implemented") }
}

#[cfg(test)]
mod test {
    use super::PStack;

    #[test]
    fn test_stack() {
        let mut v = vec![];
        let mut s = PStack::<usize, [usize; 4]>::new();
        for i in 0..10 {
            s.push(i);
            v.push(s.clone());
        }
        for i in 0..10 {
            assert_eq!(Some(9 - i), s.pop());
        }
        assert_eq!(None, s.pop());
        for i in 0..10 {
            let mut s = v[i].clone();
            for j in 0..=i {
                assert_eq!(Some(i - j), s.pop());
            }
            assert_eq!(None, s.pop());
        }
    }
}
