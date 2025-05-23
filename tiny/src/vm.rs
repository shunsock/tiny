use crate::value_object::opcode::OpCode;
use crate::value_object::tiny_object::TinyObject;

#[derive(Debug)]
pub enum RuntimeError {
    StackUnderflow,
    InvalidJump,
    InvalidOperation,
}

pub fn runtime_error_to_message(e: RuntimeError) -> String {
    match e {
        RuntimeError::StackUnderflow => {
            "stack underflow: not enough values on the stack".to_string()
        }
        RuntimeError::InvalidJump => "invalid jump: jump target is out of bounds".to_string(),
        RuntimeError::InvalidOperation => {
            "invalid operation: jump target is out of bounds".to_string()
        }
    }
}

pub struct VM {
    pub stack: Vec<TinyObject>,
    pc: usize,
    code: Vec<OpCode>,
}

impl VM {
    pub fn new(code: Vec<OpCode>) -> Self {
        Self {
            stack: Vec::new(),
            pc: 0,
            code,
        }
    }

    pub fn run(&mut self) -> Result<Option<TinyObject>, RuntimeError> {
        while self.pc < self.code.len() {
            match self.code[self.pc].clone() {
                OpCode::Push(obj) => {
                    self.stack.push(obj);
                    self.pc += 1;
                }
                OpCode::Add => {
                    let b = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    match (a, b) {
                        (TinyObject::Int(a), TinyObject::Int(b)) => {
                            self.stack.push(TinyObject::Int(a + b));
                        }
                        (TinyObject::Float(a), TinyObject::Int(b)) => {
                            self.stack.push(TinyObject::Float(a + b as f32));
                        }
                        (TinyObject::Int(a), TinyObject::Float(b)) => {
                            self.stack.push(TinyObject::Float(a as f32 + b));
                        }
                        (TinyObject::Float(a), TinyObject::Float(b)) => {
                            self.stack.push(TinyObject::Float(a + b));
                        }
                        _ => return Err(RuntimeError::InvalidOperation),
                    }
                    self.pc += 1;
                }
                OpCode::JumpIfFalse(target) => {
                    let cond: TinyObject = self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    if Self::evaluate_condition(cond)? == false {
                        if target > self.code.len() {
                            return Err(RuntimeError::InvalidJump);
                        }
                        self.pc = target;
                    } else {
                        self.pc += 1;
                    }
                }
                OpCode::Jump(target) => {
                    if target > self.code.len() {
                        return Err(RuntimeError::InvalidJump);
                    }
                    self.pc = target;
                }
                OpCode::Pop => {
                    self.stack.pop().ok_or(RuntimeError::StackUnderflow)?;
                    self.pc += 1;
                }
            }
        }

        Ok(self.stack.last().cloned())
    }

    fn evaluate_condition(obj: TinyObject) -> Result<bool, RuntimeError> {
        match obj {
            TinyObject::Int(n) => Ok(n > 0),
            TinyObject::Bool(b) => Ok(b),
            _ => Err(RuntimeError::InvalidOperation),
        }
    }
}
