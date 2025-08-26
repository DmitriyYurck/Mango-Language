use crate::ast::*;
use crate::env::{Env, Value, Function};

pub fn eval(stmts: &[Stmt], env: &mut Env) -> Value {
    let mut last = Value::None;
    for stmt in stmts {
        last = eval_stmt(stmt, env);
    }
    last
}

fn eval_stmt(stmt: &Stmt, env: &mut Env) -> Value {
    match stmt {
        Stmt::Expr(e) => eval_expr(e, env),
        Stmt::Let(name, expr) => {
            let val = eval_expr(expr, env);
            env.set(name.clone(), val.clone());
            val
        }
        Stmt::Print(expr) => {
             let val = eval_expr(expr, env);
             match val {
                 Value::String(s) => println!("{}", s),
                 other => println!("{:?}", other),
             }
             Value::None
        }
        Stmt::If(cond, then_branch, else_branch) => {
            let cond_val = eval_expr(cond, env);
            match cond_val {
                Value::Bool(true) => eval(then_branch, env),
                Value::Bool(false) => eval(else_branch, env),
                _ => panic!("Condition must be boolean"),
            }
        }
        Stmt::Func(name, params, body) => {
            let func = Value::Func(Function {
                params: params.clone(),
                body: body.clone(),
            });
            env.set(name.clone(), func.clone());
            func
        }
        Stmt::Return(expr) => eval_expr(expr, env),
    }
}

fn eval_expr(expr: &Expr, env: &mut Env) -> Value {
    match expr {
        Expr::Number(n) => Value::Number(*n),
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Var(name) => env.get(name),
        Expr::Binary(left, op, right) => {
            let l = eval_expr(left, env);
            let r = eval_expr(right, env);
            match (l, r) {
                (Value::Number(a), Value::Number(b)) => match op.as_str() {
                    "+" => Value::Number(a + b),
                    "-" => Value::Number(a - b),
                    "*" => Value::Number(a * b),
                    "/" => Value::Number(a / b),
                    ">" => Value::Bool(a > b),
                    "<" => Value::Bool(a < b),
                    "=" => Value::Bool((a - b).abs() < std::f64::EPSILON),
                    _ => panic!("Unknown operator {}", op),
                },
                (Value::Bool(a), Value::Bool(b)) => match op.as_str() {
                    "=" => Value::Bool(a == b),
                    _ => panic!("Unsupported boolean operator {}", op),
                },
                _ => panic!("Type mismatch in binary expression"),
            }
        }
        Expr::Call(name, args) => {
            let func_val = env.get(name);
            if let Value::Func(func) = func_val {
                let mut local_env = Env::default();
                for (param, arg_expr) in func.params.iter().zip(args.iter()) {
                    let arg_val = eval_expr(arg_expr, env);
                    local_env.set(param.clone(), arg_val);
                }
                eval(&func.body, &mut local_env)
            } else {
                panic!("{} is not a function", name);
            }
        }
        Expr::String(s) => Value::String(s.clone()),
        Expr::Block(stmts) => {
            let mut local_env = Env::default();
            eval(stmts, &mut local_env)
        }
    }
}
