use std::process::exit;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
    Return,
    IntLit,
    Semi
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    value: Option<String>
}

pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            input,
            position: 0,
        }
    }
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut vec: Vec<Token> = Vec::new();
        let mut buf: Vec<char> = Vec::new();

        while self.peek(1).is_some() {
            if let Some(c) = self.peek(1) {
                if c.is_alphabetic() {
                    buf.push(self.consume());
                    while let Some(c) = self.peek(1) {
                        if !c.is_alphanumeric() { break }
                        buf.push(self.consume());
                    }
                    buf.push(self.consume());
                    let char_string: String = buf.clone().into_iter().collect();

                    match char_string.as_str() {
                        "exit" => {
                            vec.push(Token {
                                token_type: TokenType::Return,
                                value: None,
                            });
                        },
                        _ => {
                            println!("Incorrect Identifier {}", char_string);
                            exit(1);
                        }
                    }
                    buf = Vec::new();
                    continue
                }
                else if c.is_digit(10) {
                    buf.push(self.consume());
                    while let Some(c) = self.peek(1) {
                        if !c.is_digit(10) { break }
                        buf.push(self.consume());
                    }
                    buf.push(self.consume());
                    let char_string: String = buf.clone().into_iter().collect();
                    vec.push(Token {
                        token_type: TokenType::IntLit,
                        value: Option::from(char_string),
                    });
                    buf = Vec::new();
                    continue
                }
                else if c == ';' {
                    self.consume();
                    vec.push(Token {
                        token_type: TokenType::Semi,
                        value: None,
                    });
                    buf = Vec::new();
                    continue
                }
                else if c.is_whitespace() {
                    self.consume();
                    continue
                }
            }
        }

        vec
    }

    fn peek(&self, ahead: usize) -> Option<char> {
        self.input.chars().nth(self.position + ahead)
    }

    fn consume(&mut self) -> char {
        let c = self.input.chars().nth(self.position).expect("LOL");
        self.position += 1;
        return c
    }
}