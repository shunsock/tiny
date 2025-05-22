use crate::ast::{BinaryOperation, Expr, Stmt};
use crate::opcode::OpCode;
use crate::tiny_object::TinyObject;

#[derive(Debug)]
pub enum CompileError {
    UnsupportedExpr,
}

pub fn compile_error_to_message(e: CompileError) -> String {
    match e {
        CompileError::UnsupportedExpr => {
            "unsupported expression encountered during compilation".to_string()
        }
    }
}

pub(crate) struct Compiler {
    code: Vec<OpCode>,
}

impl Compiler {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    pub fn compile_stmt(&mut self, stmt: Stmt) -> Result<Vec<OpCode>, CompileError> {
        match stmt {
            Stmt::Expr(expr) => {
                self.compile_expr(expr)?;
                self.code.push(OpCode::Pop);
            }
        }
        Ok(self.code.clone())
    }

    fn compile_expr(&mut self, expr: Expr) -> Result<(), CompileError> {
        match expr {
            Expr::Int(n) => {
                self.code.push(OpCode::Push(TinyObject::Int(n)));
                Ok(())
            }
            Expr::BinOp(boxed_op) => self.compile_binop(*boxed_op),
            Expr::If { cond, thn, els } => self.compile_if(*cond, *thn, *els),
        }
    }

    fn compile_binop(&mut self, op: BinaryOperation) -> Result<(), CompileError> {
        match op {
            BinaryOperation::Add { left, right } => {
                self.compile_expr(*left)?;
                self.compile_expr(*right)?;
                self.code.push(OpCode::Add);
                Ok(())
            }
        }
    }

    fn compile_if(&mut self, cond: Expr, thn: Expr, els: Expr) -> Result<(), CompileError> {
        self.compile_expr(cond)?; // [cond_cmds]
        let jump_if_false_pos: usize = self.code.len();
        self.code.push(OpCode::JumpIfFalse(0)); // [cond_cmds, JumpIfFalse(0)]
        self.compile_expr(thn)?; // [cond_cmds, JumpIfFalse(0), thn_cmds ]
        let jump_pos: usize = self.code.len();
        self.code.push(OpCode::Jump(0)); // [cond_cmds, JumpIfFalse(0), thn_cmds, Jump(0)]
        let else_start: usize = self.code.len();
        self.compile_expr(els)?; // [cond_cmds, JumpIfFalse(0), thn_cmds, Jump(0), els_cmds]
        let end: usize = self.code.len();
        self.code[jump_if_false_pos] = OpCode::JumpIfFalse(else_start); // [cond_cmds, JumpIfFalse(els_start), thn_cmds, Jump(0), els_cmds]
        self.code[jump_pos] = OpCode::Jump(end); // [cond_cmds, JumpIfFalse(els_start), thn_cmds, Jump(end), els_cmds]
        Ok(())
    }
}
