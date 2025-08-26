use crate::ast::*;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn next(&mut self) -> Token {
        let tok = self.peek().clone();
        self.pos += 1;
        tok
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.peek() != &Token::EOF {
            stmts.push(self.parse_stmt());
        }
        stmts
    }

    fn parse_stmt(&mut self) -> Stmt {
        match self.peek() {
            Token::Keyword(k) if k == "let" => {
                self.next();
                if let Token::Ident(name) = self.next() {
                    self.expect('=');
                    let expr = self.parse_expr();
                    Stmt::Let(name, expr)
                } else {
                    panic!("Expected identifier after 'let'");
                }
            }
            Token::Keyword(k) if k == "print" => {
                self.next();
                let expr = self.parse_expr();
                Stmt::Print(expr)
            }
            Token::Keyword(k) if k == "if" => {
                self.next();
                let cond = self.parse_expr();
                let then_branch = if let Token::Symbol('{') = self.peek() {
                    self.next();
                    self.parse_block()
                } else {
                    vec![self.parse_stmt()]
                };
                let else_branch = match self.peek() {
                    Token::Keyword(k) if k == "else" => {
                        self.next();
                        if let Token::Keyword(k2) = self.peek() {
                            if k2 == "if" {
                                vec![self.parse_stmt()]
                            } else if let Token::Symbol('{') = self.peek() {
                                self.next();
                                self.parse_block()
                            } else {
                                vec![self.parse_stmt()]
                            }
                        } else if let Token::Symbol('{') = self.peek() {
                            self.next();
                            self.parse_block()
                        } else {
                            vec![self.parse_stmt()]
                        }
                    }
                    _ => vec![],
                };
                Stmt::If(cond, then_branch, else_branch)
            }
            Token::Keyword(k) if k == "func" => {
                self.next();
                let name = if let Token::Ident(n) = self.next() {
                    n
                } else {
                    panic!("Expected function name after 'func'");
                };
                self.expect('(');
                let mut params = Vec::new();
                while let Token::Ident(p) = self.peek() {
                    params.push(p.clone());
                    self.next();
                    if self.peek() == &Token::Symbol(',') {
                        self.next();
                    }
                }
                self.expect(')');
                self.expect('{');
                let body = self.parse_block();
                Stmt::Func(name, params, body)
            }
            Token::Keyword(k) if k == "return" => {
                self.next();
                let expr = self.parse_expr();
                Stmt::Return(expr)
            }
            _ => Stmt::Expr(self.parse_expr()),
        }
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.peek() != &Token::Symbol('}') {
            stmts.push(self.parse_stmt());
        }
        self.expect('}');
        stmts
    }

    fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_primary();
        while let Token::Symbol(op) = self.peek() {
            if "+-*/><=!".contains(*op) {
                let op = *op;
                self.next();
                let right = self.parse_primary();
                left = Expr::Binary(Box::new(left), op.to_string(), Box::new(right));
            } else {
                break;
            }
        }
        left
    }

    fn parse_primary(&mut self) -> Expr {
        match self.next() {
            Token::Number(n) => Expr::Number(n),
            Token::Bool(b) => Expr::Bool(b),
            Token::Symbol('{') => {
                let block = self.parse_block();
                Expr::Block(block)
            }
            Token::String(s) => Expr::String(s.clone()),
            Token::Ident(name) => {
                if self.peek() == &Token::Symbol('(') {
                    self.next();
                    let mut args = Vec::new();
                    while self.peek() != &Token::Symbol(')') {
                        args.push(self.parse_expr());
                        if self.peek() == &Token::Symbol(',') {
                            self.next();
                        }
                    }
                    self.expect(')');
                    Expr::Call(name, args)
                } else {
                    Expr::Var(name)
                }
            }
            token => panic!("Unexpected token: {:?}", token),
        }
    }

    fn expect(&mut self, expected: char) {
        match self.next() {
            Token::Symbol(c) if c == expected => {}
            other => panic!("Expected '{}', but found {:?}", expected, other),
        }
    }
}
