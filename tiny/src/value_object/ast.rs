#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    If {
        cond: Box<Expr>,
        thn: Box<Expr>,
        els: Box<Expr>,
    },
    Int(i32),
    Bool(bool),
    Float(f32),
    BinOp(Box<BinaryOperation>),
}

#[derive(Debug, Clone)]
pub enum BinaryOperation {
    Add { left: Box<Expr>, right: Box<Expr> },
}
