use std::{env, process};
use std::fs;
use std::io::{Write};
use std::str::FromStr;
use std::str::Chars;
use std::fs::File;
use std::io::{self, Read};




struct Lexer<'a> {
    input: Chars<'a>,
    current: Option<char>,
}

#[derive(Debug)]
enum Token {
    Number(f64),
    Identifier(String),
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    LeftParentheses,
    RightParentheses,
    LeftBrace,
    RightBrace,
    Star,
    Comma,
    Plus,
    Minus,
    SemiColumn,
    String(String),
    Slash
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            current: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        self.current = self.input.next();
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        match self.current {
            Some(c) if c.is_digit(10) => Some(self.read_number()),
            Some(c) if c.is_alphabetic() => Some(self.read_identifier()),
            Some(c) if c == '"' => Some(self.read_string()),
            Some(c) if c == '=' =>  {
                self.read_char();
                if self.current == Some('=') {
                    self.read_char();
                    Some(Token::EqualEqual)
                } else {
                    Some(Token::Equal)
                }
            },
            Some(c) if c == '<' =>  {
                self.read_char();
                if self.current == Some('=') {
                    self.read_char();
                    Some(Token::LessEqual)
                } else {
                    Some(Token::Less)
                }
            },
            Some(c) if c == '!' =>  {
                self.read_char();
                if self.current == Some('=') {
                    self.read_char();
                    Some(Token::BangEqual)
                } else {
                    Some(Token::Bang)
                }
            },
            Some(c) if c == '>' =>  {
                self.read_char();
                if self.current == Some('=') {
                    self.read_char();
                    Some(Token::GreaterEqual)
                } else {
                    Some(Token::Greater)
                }
            },
            Some(c) if c == '/' =>  {
                self.read_char();
                Some(Token::Slash)
            },
            Some(c) if c == '(' =>  {
                self.read_char();
                Some(Token::LeftParentheses)
            },
            Some(c) if c == ')' =>  {
                self.read_char();
                Some(Token::RightParentheses)
            },
            Some(c) if c == '{' =>  {
                self.read_char();
                Some(Token::LeftBrace)
            },
            Some(c) if c == '}' =>  {
                self.read_char();
                Some(Token::RightParentheses)
            },
            Some(c) if c == '*' =>  {
                self.read_char();
                Some(Token::Star)
            },
            Some(c) if c == ',' =>  {
                self.read_char();
                Some(Token::Comma)
            },
            Some(c) if c == '+' =>  {
                self.read_char();
                Some(Token::Plus)
            },
            Some(c) if c == '-' =>  {
                self.read_char();
                Some(Token::Minus)
            },
            Some(c) if c == ';' =>  {
                self.read_char();
                Some(Token::SemiColumn)
            },
            None => None,
            _ => {
                self.read_char();
                None
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current {
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        while let Some(c) = self.current {
            if !c.is_digit(10) && c != '.' {
                break;
            }
            number.push(c);
            self.read_char();
        }
        Token::Number(number.parse().unwrap())
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(c) = self.current {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }
            identifier.push(c);
            self.read_char();
        }
        Token::Identifier(identifier)
    }

    fn read_string(&mut self) -> Token {
        let mut string = String::new();
        self.read_char();
        while let Some(c) = self.current {
            if c == '"' {
                self.read_char();
                break;
            }
            string.push(c);
            self.read_char();
        }
        Token::String(string)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];
    match command.as_str() {
        "tokenize" => {
            let mut file = File::open(filename);

            let mut contents = String::new();
            file.unwrap().read_to_string(&mut contents);

            let mut lexer = Lexer::new(&contents);

            while let Some(token) = lexer.next_token() {
                let token_string = match token {
                    Token::Number(n) => format!("NUMBER {} {}", n, n),
                    Token::Identifier(s) => format!("IDENTIFIER {} {}", s, s),
                    Token::Equal => "EQUAL = null".to_string(),
                    Token::EqualEqual => "EQUAL_EQUAL == null".to_string(),
                    Token::Bang => "BANG != null".to_string(),
                    Token::BangEqual => "BANG_EQUAL != null".to_string(),
                    Token::Less => "LESS < null".to_string(),
                    Token::LessEqual => "LESS_EQUAL <= null".to_string(),
                    Token::Greater => "GREATER > null".to_string(),
                    Token::GreaterEqual => "GREATER_EQUAL >= null".to_string(),
                    Token::LeftParentheses => "LEFT_PAREN ( null".to_string(),
                    Token::RightParentheses => "RIGHT_PAREN ) null".to_string(),
                    Token::LeftBrace => "LEFT_BRACE { null".to_string(),
                    Token::RightBrace => "RIGHT_BRACE } null".to_string(),
                    Token::Star => "STAR * null".to_string(),
                    Token::Comma => "COMMA , null".to_string(),
                    Token::Plus => "PLUS + null".to_string(),
                    Token::Minus => "MINUS - null".to_string(),
                    Token::SemiColumn => "SEMICOLON ; null".to_string(),
                    Token::String(s) => format!("STRING "{}" {}", s, s),
                    Token::Slash => "Slash / null".to_string(),
                };


                println!("{:?}", token_string);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
