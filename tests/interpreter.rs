use interpreter::ast::*;
use interpreter::env::{Env, Value, Function};
use interpreter::interpreter::eval;

#[test]
fn test_addition() {
    let stmt = Stmt::Let("x".into(), Expr::Binary(
        Box::new(Expr::Number(2.0)),
        "+".into(),
        Box::new(Expr::Number(3.0)),
    ));
    let mut env = Env::default();
    let result = eval(&[stmt], &mut env);
    assert_eq!(env.get("x"), Value::Number(5.0));
    assert_eq!(result, Value::Number(5.0));
}
