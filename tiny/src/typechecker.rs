use crate::value_object::ast::Stmt;

pub enum TypeCheckError {}

pub struct TypeChecker {}

impl TypeChecker {
    fn typecheck(ast: Stmt) -> Result<(), TypeCheckError> {
        Ok(())
    }
}