use std::fmt;

use crate::common::{Idx, IdxVec};
use crate::idx_ty;

pub mod builder;

idx_ty! {
    pub struct Local { .. }
}

impl fmt::Debug for Local {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "%{}", self.index())
    }
}

idx_ty! {
    pub struct DefId { .. }
}

impl fmt::Debug for DefId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@{}", self.index())
    }
}

idx_ty! {
    pub struct BasicBlock { .. }
}

impl fmt::Debug for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ".bb{}", self.index())
    }
}

pub struct Body {
    blocks: IdxVec<BasicBlock, BasicBlockData>,
    locals: IdxVec<Local, ()>,
}

pub struct BasicBlockData {
    instructions: Vec<Instruction>,
    terminator: Option<Terminator>,
}

pub enum Terminator {
    Return,
    Unreachable,
    Goto(BasicBlock),
    Call {
        target: Operand,
        args: Vec<Operand>,
        rvp: Option<Lvalue>,
        goto: BasicBlock,
    },
    Switch {
        source: Operand,
        cases: Vec<(u64, BasicBlock)>,
    },
}

pub enum Instruction {
    Assign(Box<(Lvalue, Rvalue)>),
    Nop,
}

pub enum Operand {
    Copy(Lvalue),
    Move(Lvalue),
    Constant(Box<Constant>),
}

pub struct Lvalue {
    root: Local,
    projections: Vec<Projection>,
}

pub enum Projection {
    Deref,
}

pub enum Rvalue {
    Operand(Operand),
    AddressOf(Lvalue),
}

pub enum Constant {
    Scalar(u64),
    Global(DefId),
}
