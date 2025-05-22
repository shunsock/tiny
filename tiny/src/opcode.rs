use crate::tiny_object::TinyObject;

#[derive(Debug, Clone)]
pub enum OpCode {
    Push(TinyObject),
    Add,
    JumpIfFalse(usize),
    Jump(usize),
    Pop,
}
