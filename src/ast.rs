#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Bool(bool),
    Var(String),
    Binary(Box<Expr>, String, Box<Expr>),
    Call(String, Vec<Expr>),
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Let(String, Expr),
    Print(Expr),
    If(Expr, Vec<Stmt>, Vec<Stmt>),
    Func(String, Vec<String>, Vec<Stmt>),
    Return(Expr),
}
