#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    Pipe,
    RedirectOut,
    RedirectIn,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut buf = String::new();

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' => {
                if !buf.is_empty() {
                    tokens.push(Token::Word(buf.clone()));
                    buf.clear();
                }
            }
            '"' => {
                let mut quoted = String::new();
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next == '"' {
                        break;
                    }
                    quoted.push(next);
                }
                tokens.push(Token::Word(quoted));
            }
            '|' => {
                if !buf.is_empty() {
                    tokens.push(Token::Word(buf.clone()));
                    buf.clear();
                }
                tokens.push(Token::Pipe);
            }
            '>' => {
                if !buf.is_empty() {
                    tokens.push(Token::Word(buf.clone()));
                    buf.clear();
                }
                tokens.push(Token::RedirectOut);
            }
            '<' => {
                if !buf.is_empty() {
                    tokens.push(Token::Word(buf.clone()));
                    buf.clear();
                }
                tokens.push(Token::RedirectIn);
            }
            _ => buf.push(c),
        }
    }

    if !buf.is_empty() {
        tokens.push(Token::Word(buf));
    }

    tokens
}
