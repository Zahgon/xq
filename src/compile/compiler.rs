use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::{Debug, Formatter},
    rc::Rc,
    slice::from_ref,
};

use itertools::Itertools;
use thiserror::Error;
use xq_lang::ast::{
    self, BinaryArithmeticOp, BinaryOp, BindPattern, ConstantPrimitive, FuncArg, FuncDef,
    Identifier, ObjectBindPatternEntry, Query, StringFragment, Suffix, Term, UpdateOp,
};

use crate::{
    data_structure::PHashMap,
    intrinsic,
    module_loader::{ModuleLoadError, ModuleLoader},
    value::Array,
    vm::{
        bytecode::{ClosureAddress, NamedFn0, NamedFn1, NamedFn2},
        Address, ByteCode, Program, ScopeId, ScopedSlot,
    },
    Number, Value,
};

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Use of unknown variable `{0:}`")]
    UnknownVariable(Identifier),
    #[error("Use of unknown function `{0:}` that takes {1:} arguments")]
    UnknownFunction(Identifier, usize),
    #[error("Bind pattern has the same variable `{0:}`")]
    SameVariableInPattern(Identifier),
    #[error("Unknown label `{0:}`")]
    UnknownLabel(Identifier),
    #[error(transparent)]
    ModuleLoadError(#[from] ModuleLoadError),
    #[error("Unknown string formatter `{0:?}`")]
    UnknownStringFormatter(Identifier),
}

type Result<T, E = CompileError> = std::result::Result<T, E>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct FunctionIdentifier(pub(crate) Identifier, pub(crate) usize);
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct DeclaredFunction {
    
    address: Address,
    
    tail_call_discard_frame: Address,
    
    tail_call_preserve_frame: Address,
    
    arg_types: Vec<ArgType>,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum ArgType {
    
    Closure,
    
    Value,
}
#[derive(Clone)]
enum FunctionLike {
    Function(DeclaredFunction),
    Closure(ScopedSlot),
    Intrinsic(ByteCode, Vec<ArgType>),
    ManuallyImplemented(
        &'static str,
        fn(&mut Compiler, &[Query], Address) -> Result<Address>,
    ),
}
impl Debug for FunctionLike {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct PlaceHolder(Address);

struct CodeEmitter {
    code: Vec<ByteCode>,
}

impl CodeEmitter {
    fn new() -> Self { panic!("STUB: not implemented") }

    fn address(&self) -> Address { panic!("STUB: not implemented") }

    fn output(&self) -> Address { panic!("STUB: not implemented") }

    fn backtrack(&self) -> Address { panic!("STUB: not implemented") }

    fn follow_jump(&self, mut address: Address) -> Address { panic!("STUB: not implemented") }

    fn jump_or_follow(&mut self, address: Address) { panic!("STUB: not implemented") }

    fn get_next_op(&self, next: Address) -> &ByteCode { panic!("STUB: not implemented") }

    fn emit_normal_op(&mut self, code: ByteCode, next: Address) -> Address { panic!("STUB: not implemented") }

    fn emit_terminal_op(&mut self, code: ByteCode) -> Address { panic!("STUB: not implemented") }

    fn emit_constant_primitive<V>(&mut self, value: V, next: Address) -> Address
    where
        V: Into<ConstantPrimitive>,
    { panic!("STUB: not implemented") }

    fn emit_constant<V>(&mut self, value: V, next: Address) -> Address
    where
        V: Into<Value>,
    { panic!("STUB: not implemented") }

    fn emit_push<V>(&mut self, value: V, next: Address) -> Address
    where
        V: Into<Value>,
    { panic!("STUB: not implemented") }

    fn emit_fork(&mut self, fork_pc: Address, next: Address) -> Address { panic!("STUB: not implemented") }

    fn emit_terminal_placeholder(&mut self) -> (Address, PlaceHolder) { panic!("STUB: not implemented") }

    fn replace_placeholder(&mut self, placeholder: PlaceHolder, code: ByteCode) { panic!("STUB: not implemented") }
}

#[derive(Debug, Clone)]
struct Scope {
    id: ScopeId,
    next_variable_slot_id: usize,
    next_closure_slot_id: usize,
    next_label_slot_id: usize,
    functions: PHashMap<FunctionIdentifier, FunctionLike>,
    variables: PHashMap<Identifier, ScopedSlot>,
    labels: PHashMap<Identifier, ScopedSlot>,
    leaked_scope_ids: Rc<RefCell<HashSet<ScopeId>>>,
}

impl Scope {
    fn new(id: ScopeId) -> Self { panic!("STUB: not implemented") }

    fn nested(id: ScopeId, previous: &Self) -> Self { panic!("STUB: not implemented") }

    fn require_slot(&self) -> bool { panic!("STUB: not implemented") }

    fn has_slot_leaked_scope(&self) -> bool { panic!("STUB: not implemented") }

    fn allocate_variable(&mut self) -> ScopedSlot { panic!("STUB: not implemented") }

    fn register_variable(&mut self, name: Identifier) -> ScopedSlot { panic!("STUB: not implemented") }

    fn register_function(&mut self, name: Identifier, function: DeclaredFunction) { panic!("STUB: not implemented") }

    fn register_closure(&mut self, name: Identifier) -> ScopedSlot { panic!("STUB: not implemented") }

    fn allocate_label(&mut self) -> ScopedSlot { panic!("STUB: not implemented") }

    fn register_label(&mut self, name: Identifier) -> ScopedSlot { panic!("STUB: not implemented") }

    fn lookup_variable(&self, name: &Identifier) -> Option<&ScopedSlot> { panic!("STUB: not implemented") }

    fn lookup_function(&self, identifier: &FunctionIdentifier) -> Option<&FunctionLike> { panic!("STUB: not implemented") }

    fn lookup_label(&self, name: &Identifier) -> Option<&ScopedSlot> { panic!("STUB: not implemented") }
}

pub struct Compiler {
    emitter: CodeEmitter,
    next_scope_id: ScopeId,
    scope_stack: Vec<Scope>,
}

struct SavedScope(Scope);

trait Compile {
    fn compile(&self, compiler: &mut Compiler, next: Address) -> Result<Address>;
}

impl Compile for Value {
    fn compile(&self, compiler: &mut Compiler, next: Address) -> Result<Address> { panic!("STUB: not implemented") }
}

impl Compile for Query {
    fn compile(&self, compiler: &mut Compiler, next: Address) -> Result<Address> { panic!("STUB: not implemented") }
}

impl Compile for Term {
    fn compile(&self, compiler: &mut Compiler, next: Address) -> Result<Address> { panic!("STUB: not implemented") }
}

impl Compile for Box<Query> {
    fn compile(&self, compiler: &mut Compiler, next: Address) -> Result<Address> { panic!("STUB: not implemented") }
}

impl<F> Compile for F
where
    F: Fn(&mut Compiler, Address) -> Result<Address>,
{
    fn compile(&self, compiler: &mut Compiler, next: Address) -> Result<Address> { panic!("STUB: not implemented") }
}

impl Default for Compiler {
    fn default() -> Self { panic!("STUB: not implemented") }
}

impl Compiler {
    pub fn new() -> Self { panic!("STUB: not implemented") }

    fn current_scope(&self) -> &Scope { panic!("STUB: not implemented") }

    fn current_scope_mut(&mut self) -> &mut Scope { panic!("STUB: not implemented") }

    fn save_scope(&mut self) -> SavedScope { panic!("STUB: not implemented") }

    fn restore_scope(&mut self, save: SavedScope) { panic!("STUB: not implemented") }

    fn allocate_variable(&mut self) -> ScopedSlot { panic!("STUB: not implemented") }

    fn register_variable(&mut self, name: Identifier) -> ScopedSlot { panic!("STUB: not implemented") }

    fn register_closure(&mut self, name: Identifier) -> ScopedSlot { panic!("STUB: not implemented") }

    fn register_function(&mut self, name: Identifier, function: DeclaredFunction) { panic!("STUB: not implemented") }

    fn enter_scope(&mut self) -> ScopeId { panic!("STUB: not implemented") }

    fn current_scope_require_slot(&self) -> bool { panic!("STUB: not implemented") }

    fn exit_scope(&mut self, id: ScopeId) -> (usize, usize, usize) { panic!("STUB: not implemented") }

    fn exit_scope_and_emit_new_frame(&mut self, id: ScopeId, next: Address) -> Address { panic!("STUB: not implemented") }

    fn exit_global_scope_and_emit_new_frame(&mut self, next: Address) -> Address { panic!("STUB: not implemented") }

    fn lookup_variable(&self, name: &Identifier) -> Result<&ScopedSlot> { panic!("STUB: not implemented") }

    fn lookup_compilable_intrinsic(function: &FunctionIdentifier) -> Option<FunctionLike> { panic!("STUB: not implemented") }

    fn lookup_function(&self, function: &FunctionIdentifier) -> Result<FunctionLike> { panic!("STUB: not implemented") }

    fn compile_function_inner(&mut self, args: &[FuncArg], body: &Query) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_closure(&mut self, closure: &Query) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_funcdef(&mut self, func: &FuncDef) -> Result<()> { panic!("STUB: not implemented") }

    fn compile_func_call_args(
        &mut self,
        args: &[Query],
        types: &[ArgType],
        mut next: Address,
    ) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_func_call(
        &mut self,
        function: FunctionLike,
        args: &[Query],
        next: Address,
    ) -> Result<Address> { panic!("STUB: not implemented") }

    fn lookup_and_compile_func_call(
        &mut self,
        name: Identifier,
        args: &[Query],
        next: Address,
    ) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_try<T: Compile, U: Compile>(
        &mut self,
        body: &T,
        catch: Option<&U>,
        next: Address,
    ) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_if<T: Compile, U: Compile, V: Compile>(
        &mut self,
        cond: &T,
        positive: &U,
        negative: Option<&V>,
        next: Address,
    ) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_index<T: Compile, U: Compile>(
        &mut self,
        body: &T,
        index: &U,
        next: Address,
    ) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_slice<T: Compile, U: Compile, V: Compile>(
        &mut self,
        body: &T,
        start: Option<&U>,
        end: Option<&V>,
        next: Address,
    ) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_iterate<T: Compile>(&mut self, body: &T, next: Address) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_term_suffix(
        &mut self,
        term: &Term,
        suffix: &Suffix,
        next: Address,
    ) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_object_entry(
        &mut self,
        context: ScopedSlot,
        key: &Query,
        value: &Option<Query>,
        next: Address,
    ) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_object(
        &mut self,
        kvs: &[(Query, Option<Query>)],
        mut next: Address,
    ) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_bind<T: Compile>(
        &mut self,
        source: &Term,
        patterns: &[BindPattern],
        body: &T,
        next: Address,
    ) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_string<T: Compile>(
        &mut self,
        fragments: &[StringFragment],
        stringifier: T,
        mut next: Address,
    ) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_term(&mut self, term: &ast::Term, next: Address) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_query(&mut self, query: &ast::Query, next: Address) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_without_path_tracking<T: Compile>(
        &mut self,
        body: &T,
        next: Address,
    ) -> Result<Address> { panic!("STUB: not implemented") }

    fn compile_prelude(&mut self, ast: &ast::Program) -> Result<()> { panic!("STUB: not implemented") }

    pub fn compile<M: ModuleLoader>(
        &mut self,
        ast: &ast::Program,
        module_loader: &M,
    ) -> Result<Program> { panic!("STUB: not implemented") }
}
