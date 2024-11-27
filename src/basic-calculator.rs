use core::panic;
use std::str::FromStr;

#[allow(unused_variables)]
fn main() {
    let s = "1 + 1".to_string();
    let s = "2-1 + 2".to_string();
    let s = "(1+(4+5+2)-3)+(6+8)".to_string();

    println!("Result -> {}", calculate(s));
}

fn calculate(s: String) -> i32 {
    let mut tokens = TokenVec::from_str(&s).unwrap();
    tokens.calculate()
}

#[derive(Debug)]
enum Token {
    Add,
    Subs,
    Number(i32),
    Wrap(TokenVec),
}

impl FromStr for Token {
    type Err = &'static str;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        s = s.trim();

        if s.starts_with("(") && s.ends_with(")") {
            return Ok(Token::Wrap(TokenVec::from_str(s).unwrap()));
        }

        match s {
            "+" => Ok(Token::Add),
            "-" => Ok(Token::Subs),
            _ => s
                .parse::<i32>()
                .map(Token::Number)
                .map_err(|_| "Invalid token!"),
        }
    }
}

#[derive(Debug, Default)]
struct TokenVec(Vec<Token>);

impl TokenVec {
    fn calculate(&mut self) -> i32 {
        self.unwrap();

        let mut op: Option<&Token> = None;
        let mut acc = 0;
        for token in &self.0 {
            match token {
                Token::Number(n) if op.is_none() => acc = *n,
                Token::Number(n) => match op.unwrap() {
                    Token::Add => acc += n,
                    Token::Subs => acc -= n,
                    _ => panic!("Shit!"),
                },
                Token::Add | Token::Subs => {
                    op.replace(token);
                }
                _ => (),
            }
        }
        acc
    }

    fn push(&mut self, token: Token) {
        self.0.push(token);
    }

    fn unwrap(&mut self) {
        self.0.iter_mut().for_each(|t| {
            if let Token::Wrap(tv) = t {
                *t = Token::Number(tv.calculate())
            }
        });
    }
}

impl FromStr for TokenVec {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(" ", "");
        let mut root_tokens = TokenVec::default();

        let mut pointer = &mut root_tokens;
        let mut wraps: Vec<TokenVec> = Vec::new();
        let mut acc_num = String::new();
        for char in s.chars() {
            if !char.is_numeric() && !acc_num.is_empty() {
                pointer.push(Token::from_str(&acc_num).unwrap());
                acc_num.clear();
            }

            match char {
                '+' | '-' => pointer.push(Token::from_str(&char.to_string()).unwrap()),
                '0'..='9' => acc_num.push(char),
                '(' => {
                    wraps.push(TokenVec::default());
                    pointer = wraps.last_mut().unwrap();
                }
                ')' => {
                    let wrap = wraps.pop().unwrap();
                    pointer = if let Some(w) = wraps.last_mut() {
                        w
                    } else {
                        &mut root_tokens
                    };

                    pointer.push(Token::Wrap(wrap));
                }
                _ => return Err("Not valid expresion!"),
            }
        }

        if !acc_num.is_empty() {
            root_tokens.push(Token::from_str(&acc_num).unwrap());
        }

        Ok(root_tokens)
    }
}
