use std::collections::HashMap;
use crate::ast::Stmt;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Func(Function),
    None,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Default)]
pub struct Env {
    vars: HashMap<String, Value>,
}

impl Env {
    pub fn set(&mut self, name: String, val: Value) {
        self.vars.insert(name, val);
    }

    pub fn get(&self, name: &str) -> Value {
        self.vars.get(name).cloned().unwrap_or(Value::None)
    }
}
