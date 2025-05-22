#[derive(Debug, Clone)]
pub enum OpCode {
    PushInt(i32),
    Add,
    JumpIfFalse(usize),
    Jump(usize),
    Pop,
}
