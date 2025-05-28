use crate::value_object::ast::{BinaryOperation, Expr, Stmt};
use crate::value_object::tiny_type::TinyType;

pub enum TypeCheckError {
    CondMustBeBool,
    TernaryReturnsTypeMustBeSame,
    UndefinedOperation,
}

pub fn typecheck_error_to_message(e: TypeCheckError) -> String {
    match e {
        TypeCheckError::TernaryReturnsTypeMustBeSame => {
            "ternary return type must be same".to_string()
        }
        TypeCheckError::CondMustBeBool => "condition value must be bool".to_string(),
        TypeCheckError::UndefinedOperation => "you are trying undefined operation".to_string(),
    }
}

pub struct TypeChecker {}

impl TypeChecker {
    pub fn typecheck(ast: Stmt) -> Result<Option<TinyType>, TypeCheckError> {
        match ast {
            Stmt::Expr(expr) => Ok(Some(Self::typecheck_expr(expr)?)),
        }
    }

    fn typecheck_expr(expr: Expr) -> Result<TinyType, TypeCheckError> {
        match expr {
            Expr::Bool(_) => Ok(TinyType::Bool),
            Expr::Float(_) => Ok(TinyType::Float),
            Expr::Int(_) => Ok(TinyType::Int),
            Expr::If { cond, thn, els } => Ok(Self::typecheck_if(*cond, *thn, *els)?),
            Expr::BinOp(op) => Ok(Self::typecheck_binop(*op)?),
        }
    }

    fn typecheck_if(cond: Expr, thn: Expr, els: Expr) -> Result<TinyType, TypeCheckError> {
        let cond: TinyType = Self::typecheck_expr(cond)?;
        if cond != TinyType::Bool {
            return Err(TypeCheckError::CondMustBeBool);
        }
        let thn: TinyType = Self::typecheck_expr(thn)?;
        let els: TinyType = Self::typecheck_expr(els)?;
        if thn != els {
            return Err(TypeCheckError::TernaryReturnsTypeMustBeSame);
        }

        Ok(thn)
    }

    fn typecheck_binop(op: BinaryOperation) -> Result<TinyType, TypeCheckError> {
        match op {
            BinaryOperation::Add { left, right } => {
                let left: TinyType = Self::typecheck_expr(*left)?;
                let right: TinyType = Self::typecheck_expr(*right)?;
                match (left, right) {
                    (TinyType::Float, TinyType::Float)
                    | (TinyType::Int, TinyType::Float)
                    | (TinyType::Float, TinyType::Int) => Ok(TinyType::Float),
                    (TinyType::Int, TinyType::Int) => Ok(TinyType::Int),
                    _ => Err(TypeCheckError::UndefinedOperation),
                }
            }
        }
    }
}
