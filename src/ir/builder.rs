use crate::common::{Idx, IdxVec};

use std::ops::{Deref, DerefMut};

use super::*;

pub struct BodyBuilder {
    body: Body,
}

pub struct BlockBuilder<'b> {
    body: &'b mut BodyBuilder,
    basic_block: BasicBlock,
}

pub struct CallBuilder<'b, 'c> {
    block: &'c mut BlockBuilder<'b>,
    target: Operand,
    goto: BasicBlock,
    args: Vec<Operand>,
    rvp: Option<Lvalue>,
}
pub struct SwitchBuilder<'b, 's> {
    block: &'s mut BlockBuilder<'b>,
    source: Operand,
    cases: Vec<(u64, BasicBlock)>,
}

impl BodyBuilder {
    fn new(args: usize) -> BodyBuilder {
        let mut locals = IdxVec::new();
        locals.push(());
        for _ in 0..args {
            locals.push(());
        }
        BodyBuilder {
            body: Body {
                blocks: IdxVec::new(),
                locals,
            },
        }
    }

    pub fn rvp(&self) -> Local {
        Local::new(0)
    }

    pub fn arg(&self, n: usize) -> Local {
        Local::new(1 + n)
    }

    pub fn fresh_local(&mut self) -> Local {
        self.body.locals.push(())
    }

    pub fn fresh_block(&mut self) -> BasicBlock {
        self.body.blocks.push(BasicBlockData {
            instructions: vec![],
            terminator: None,
        })
    }

    pub fn builder_for_block<'b>(&'b mut self, basic_block: BasicBlock) -> BlockBuilder<'b> {
        BlockBuilder::new(self, basic_block)
    }

    pub fn build(self) -> Body {
        self.body
    }
}

impl<'b> BlockBuilder<'b> {
    fn new(body: &'b mut BodyBuilder, basic_block: BasicBlock) -> BlockBuilder<'b> {
        BlockBuilder { body, basic_block }
    }

    #[inline]
    fn block_data_mut(&mut self) -> &mut BasicBlockData {
        &mut self.body.body.blocks[self.basic_block]
    }

    pub fn nop(&mut self) {
        self.block_data_mut().instructions.push(Instruction::Nop)
    }

    pub fn assign(&mut self, lvalue: Lvalue, rvalue: Rvalue) {
        self.block_data_mut()
            .instructions
            .push(Instruction::Assign(Box::new((lvalue, rvalue))))
    }

    pub fn ret(mut self) -> BasicBlock {
        self.block_data_mut().terminator = Some(Terminator::Return);
        self.basic_block
    }

    pub fn unreachable(mut self) -> BasicBlock {
        self.block_data_mut().terminator = Some(Terminator::Unreachable);
        self.basic_block
    }

    pub fn goto(mut self, target: BasicBlock) -> BasicBlock {
        self.block_data_mut().terminator = Some(Terminator::Goto(target));
        self.basic_block
    }

    pub fn call(
        mut self,
        target: Operand,
        goto: BasicBlock,
        builder: impl FnOnce(&mut CallBuilder),
    ) -> BasicBlock {
        let mut call_builder = CallBuilder::new(&mut self, target, goto);
        builder(&mut call_builder);
        self.block_data_mut().terminator = Some(call_builder.build());
        self.basic_block
    }
}

impl<'b, 'c> CallBuilder<'b, 'c> {
    fn new(
        block: &'c mut BlockBuilder<'b>,
        target: Operand,
        goto: BasicBlock,
    ) -> CallBuilder<'b, 'c> {
        CallBuilder {
            block,
            target,
            goto,
            args: vec![],
            rvp: None,
        }
    }

    pub fn arg(&mut self, arg: Operand) {
        self.args.push(arg)
    }

    pub fn extend_args<I: IntoIterator<Item = Operand>>(&mut self, args: I) {
        self.args.extend(args)
    }

    pub fn set_rvp(&mut self, rvp: Lvalue) {
        self.rvp = Some(rvp)
    }

    fn build(self) -> Terminator {
        Terminator::Call {
            target: self.target,
            args: self.args,
            rvp: self.rvp,
            goto: self.goto,
        }
    }
}

impl<'b, 's> SwitchBuilder<'b, 's> {
    fn new(block: &'s mut BlockBuilder<'b>, source: Operand) -> SwitchBuilder<'b, 's> {
        SwitchBuilder {
            block,
            source,
            cases: vec![],
        }
    }

    pub fn case(&mut self, case: u64, target: BasicBlock) {
        self.cases.push((case, target))
    }

    pub fn extend_cases<I: IntoIterator<Item = (u64, BasicBlock)>>(&mut self, cases: I) {
        self.cases.extend(cases)
    }

    fn build(self) -> Terminator {
        Terminator::Switch {
            source: self.source,
            cases: self.cases,
        }
    }
}

impl<'b> Deref for BlockBuilder<'b> {
    type Target = BodyBuilder;

    fn deref(&self) -> &Self::Target {
        self.body
    }
}

impl<'b> DerefMut for BlockBuilder<'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.body
    }
}

impl<'b, 'c> Deref for CallBuilder<'b, 'c> {
    type Target = BlockBuilder<'b>;

    fn deref(&self) -> &Self::Target {
        self.block
    }
}

impl<'b, 'c> DerefMut for CallBuilder<'b, 'c> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.block
    }
}

impl<'b, 's> Deref for SwitchBuilder<'b, 's> {
    type Target = BlockBuilder<'b>;

    fn deref(&self) -> &Self::Target {
        self.block
    }
}

impl<'b, 's> DerefMut for SwitchBuilder<'b, 's> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.block
    }
}
