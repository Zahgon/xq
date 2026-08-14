use std::{
    cell::{RefCell, RefMut},
    iter::Fuse,
    rc::Rc,
};

use itertools::Itertools;
use thiserror::Error;

use crate::{
    data_structure::{
        undo::{
            stack::{UStack, UStackToken},
            Undo,
        },
        PStack, PVector,
    },
    intrinsic,
    util::make_owned,
    vm::{
        bytecode::{ClosureAddress, NamedFunction},
        error::QueryExecutionError,
        Address, ByteCode, Program, Result, ScopeId, ScopedSlot, Value,
    },
    Array, InputError,
};

#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub(crate) enum ProgramError {
    #[error("tried to pop an empty stack")]
    PopEmptyStack,
    #[error("tried to use an uninitialized frame")]
    UninitializedFrame,
    #[error("tried to load from an uninitialized slot of a frame")]
    UnknownSlot,
    #[error("tried to load from an uninitialized slot of a frame")]
    UninitializedSlot,
    #[error("tried to pop a frame but there was no frame to pop")]
    PopEmptyFrame,
}

#[derive(Debug, Clone)]
pub(crate) enum PathElement {
    Array(isize),
    Object(Rc<String>),
    Any(Value),
}

impl From<PathElement> for Value {
    fn from(elem: PathElement) -> Self { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone)]
enum PathValueIterator {
    Array {
        array: Rc<Array>,
        next: usize,
    },
    Object {
        sorted_elements: Rc<Vec<(Rc<String>, Value)>>,
        next: usize,
    },
}

impl Iterator for PathValueIterator {
    type Item = (PathElement, Value);

    fn next(&mut self) -> Option<Self::Item> { panic!("STUB: not implemented") }
}

type Frames = PVector<Option<Frame>>;
type Closure = (ClosureAddress, Frames);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LabelId(usize);

#[derive(Debug, Clone)]
struct Frame {
    slots: Rc<RefCell<Vec<Option<Value>>>>,
    closure_slots: Rc<RefCell<Vec<Option<Closure>>>>,
    label_slots: Rc<RefCell<Vec<Option<LabelId>>>>,
}

impl Frame {
    fn new(variable_cnt: usize, closure_cnt: usize, label_cnt: usize) -> Self { panic!("STUB: not implemented") }
}

#[derive(Debug)]
pub struct Machine {
    program: Rc<Program>,
}

#[derive(Debug)]
struct Environment {
    next_label_id: usize,
    forks: Vec<(<State as Undo>::UndoToken, OnFork)>,
}

#[derive(Debug)]
struct State {
    pc: Address,
    stack: UStack<Value>,

    frames: Frames,
    frame_stack: UStack<(Address, Frames, bool)>, 
    closure_stack: UStack<Closure>,

    paths: UStack<Option<(Value, Value, PStack<PathElement>)>>, 

    iterators: UStack<PathValueIterator>,
}

#[derive(Debug, Clone)]
enum OnFork {
    Nop,
    IgnoreError,
    CatchError,
    SkipCatch,
    TryAlternative,
    CatchLabel(LabelId),
    Iterate,
    IterateContext,
}

impl Environment {
    fn new(state: <State as Undo>::UndoToken) -> Self { panic!("STUB: not implemented") }

    fn push_fork(&mut self, state: &mut State, on_fork: OnFork, mut new_pc: Address) { panic!("STUB: not implemented") }

    fn pop_fork(&mut self) -> Option<(<State as Undo>::UndoToken, OnFork)> { panic!("STUB: not implemented") }

    fn gen_label_id(&mut self) -> LabelId { panic!("STUB: not implemented") }
}

impl State {
    fn new(pc: Address) -> Self { panic!("STUB: not implemented") }

    fn top(&mut self) -> Option<&Value> { panic!("STUB: not implemented") }

    fn push(&mut self, item: Value) { panic!("STUB: not implemented") }

    fn check_origin_of_path(&mut self, origin: &Value) -> Result<()> { panic!("STUB: not implemented") }

    fn push_with_path_element(&mut self, item: Value, path_elem: PathElement) { panic!("STUB: not implemented") }

    fn push_with_path(&mut self, item: Value, path_elements: Vec<PathElement>) { panic!("STUB: not implemented") }

    fn pop(&mut self) -> Value { panic!("STUB: not implemented") }

    fn dup(&mut self) { panic!("STUB: not implemented") }

    fn swap(&mut self) { panic!("STUB: not implemented") }

    fn enter_path_tracking(&mut self, base_value: Value) { panic!("STUB: not implemented") }

    fn exit_tracked_path(&mut self) -> (Value, PStack<PathElement>) { panic!("STUB: not implemented") }

    fn enter_non_path_tracking(&mut self) { panic!("STUB: not implemented") }

    fn exit_non_path_tracking(&mut self) { panic!("STUB: not implemented") }

    fn push_closure(&mut self, closure: ClosureAddress) { panic!("STUB: not implemented") }

    fn pop_closure(&mut self) -> Closure { panic!("STUB: not implemented") }

    fn push_iterator(&mut self, iter: PathValueIterator) { panic!("STUB: not implemented") }

    fn top_iterator(&mut self) -> Option<&mut PathValueIterator> { panic!("STUB: not implemented") }

    fn slot(&mut self, scoped_slot: &ScopedSlot) -> RefMut<'_, Option<Value>> { panic!("STUB: not implemented") }

    fn closure_slot(&mut self, scoped_slot: &ScopedSlot) -> RefMut<'_, Option<Closure>> { panic!("STUB: not implemented") }

    fn label_slot(&mut self, scoped_slot: &ScopedSlot) -> RefMut<'_, Option<LabelId>> { panic!("STUB: not implemented") }

    #[allow(clippy::too_many_arguments)]
    fn push_frame(
        &mut self,
        context_frames: Option<Frames>,
        scope_id: ScopeId,
        variable_cnt: usize,
        closure_cnt: usize,
        label_cnt: usize,
        chain_ret: bool,
        return_address: Address,
    ) { panic!("STUB: not implemented") }

    fn pop_frame(&mut self) -> Address { panic!("STUB: not implemented") }

    fn current_return_address(&self) -> Address { panic!("STUB: not implemented") }
}

#[derive(Debug)]
struct StateToken {
    pc: Address,
    stack: UStackToken,

    frames: Frames,
    frame_stack: UStackToken,
    closure_stack: UStackToken,

    paths: UStackToken,

    iterators: UStackToken,
}

impl Undo for State {
    type UndoToken = StateToken;

    fn save(&mut self) -> Self::UndoToken { panic!("STUB: not implemented") }

    fn undo(&mut self, token: Self::UndoToken) { panic!("STUB: not implemented") }
}

impl Machine {
    pub fn new(program: Program) -> Self { panic!("STUB: not implemented") }

    pub fn start<
        C: Iterator<Item = Result<Value, InputError>>,
        I: Iterator<Item = Result<Value, InputError>>,
    >(
        &mut self,
        context: C,
        input: I,
    ) -> ResultIterator<C, I> { panic!("STUB: not implemented") }
}

pub struct ResultIterator<
    C: Iterator<Item = Result<Value, InputError>>,
    I: Iterator<Item = Result<Value, InputError>>,
> {
    program: Rc<Program>,
    env: Environment,
    state: State,
    context: Fuse<C>,
    input: Fuse<I>,
}

impl<
        C: Iterator<Item = Result<Value, InputError>>,
        I: Iterator<Item = Result<Value, InputError>>,
    > Iterator for ResultIterator<C, I>
{
    type Item = Result<Value>;

    fn next(&mut self) -> Option<Self::Item> { panic!("STUB: not implemented") }
}

fn run_code(
    program: &Program,
    state: &mut State,
    env: &mut Environment,
    context: &mut impl Iterator<Item = Result<Value, InputError>>,
    input: &mut impl Iterator<Item = Result<Value, InputError>>,
) -> Option<Result<Value>> { panic!("STUB: not implemented") }
