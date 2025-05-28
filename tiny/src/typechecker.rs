use crate::value_object::ast::{
    Stmt,
    Expr,
    BinaryOperation
};
use crate::value_object::tiny_type::TinyType;

pub enum TypeCheckError {
    CondMustBeBool,
    TernaryReturnsTypeMustBeSame,
    UndefinedOperation
}

pub struct TypeChecker {}

impl TypeChecker {
    pub fn typecheck(ast: Stmt) -> Result<Option<TinyType>, TypeCheckError> {
        match ast {
            Stmt::Expr(expr) => Ok(Some(Self::typecheck_expr(expr)?))
        }
    }

    fn typecheck_expr(expr: Expr) -> Result<TinyType, TypeCheckError> {
        match expr {
            Expr::Bool(_) => Ok(TinyType::Bool),
            Expr::Float(_) => Ok(TinyType::Float),
            Expr::Int(_) => Ok(TinyType::Int),
            Expr::If {cond, thn, els} => {
                let cond: TinyType = Self::typecheck_expr(*cond)?;
                if cond != TinyType::Bool {
                    return Err(TypeCheckError::CondMustBeBool);
                }
                let thn: TinyType = Self::typecheck_expr(*thn)?;
                let els: TinyType = Self::typecheck_expr(*els)?;
                if thn != els {
                    return Err(TypeCheckError::TernaryReturnsTypeMustBeSame);
                }

                Ok(thn)
            }
            Expr::BinOp(op) => {
                match *op.clone() {
                    BinaryOperation::Add { left, right } => {
                        let left: TinyType = Self::typecheck_expr(*left)?;
                        let right: TinyType = Self::typecheck_expr(*right)?;
                        match (left, right) {
                            (TinyType::Float, TinyType::Float) | (TinyType::Int, TinyType::Float) | (TinyType::Float, TinyType::Int) => {
                                Ok(TinyType::Float)
                            }
                            (TinyType::Int, TinyType::Int) => Ok(TinyType::Int),
                            _ => Err(TypeCheckError::UndefinedOperation)
                        }
                    }
                }
            }
        }
    }
}