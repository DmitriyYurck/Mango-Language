#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Bool(bool),
    Ident(String),
    Keyword(String),
    Symbol(char),
    EOF,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\n' | '\t' => { chars.next(); },
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_digit(10) || d == '.' {
                        num.push(d);
                        chars.next();
                    } else { break; }
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_alphanumeric() || d == '_' {
                        ident.push(d);
                        chars.next();
                    } else { break; }
                }
                match ident.as_str() {
                    "true" => tokens.push(Token::Bool(true)),
                    "false" => tokens.push(Token::Bool(false)),
                    "let" | "if" | "else" | "func" | "return" | "print" => tokens.push(Token::Keyword(ident)),
                    _ => tokens.push(Token::Ident(ident)),
                }
            },
            '+' | '-' | '*' | '/' | '=' | '(' | ')' | '{' | '}' | ',' | '>' | '<' => {
                tokens.push(Token::Symbol(c));
                chars.next();
            },
            _ => { chars.next(); },
        }
    }

    tokens.push(Token::EOF);
    tokens
}
