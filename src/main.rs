use std::{env, process};
use std::io::{Write};
use std::str::Chars;
use std::fs::File;
use std::io::{self, Read};


// ============ Error Types ============
#[derive(Debug)]
pub enum ErrorType {
    ParseError,
    IoError,
}

// ============ Token Definition ============
#[derive(Debug, Clone)]
enum Token {
    Number(String, f64),
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
    Slash,
    Dot,
    ReservedWord(String),
}


// ============ Lexer Implementation ============
struct Lexer<'a> {
    input: Chars<'a>,
    current: Option<char>,
    line: usize,
    exit_code: i32,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            current: None,
            line: 1,
            exit_code: 0,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        self.current = self.input.next();
        if let Some('\n') = self.current {
            self.line += 1;
        }
    }

    fn is_reserved_word(word: &str) -> bool {
        matches!(word,
            "and" | "class" | "else" | "false" | "for" | "fun" | "if" | "nil" | "or" |
            "print" | "return" | "super" | "this" | "true" | "var" | "while"
        )
    }

    fn is_blacklisted(c: char) -> bool {
        matches!(c, '$' | '#' | '@' | '^' | '%')
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
        match number.parse() {
            Ok(n) => Token::Number(number, n),
            Err(e) => panic!("Failed to parse number: {}", e)
        }
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

        if Self::is_reserved_word(&identifier) {
            Token::ReservedWord(identifier)
        } else {
            Token::Identifier(identifier)
        }
    }

    fn read_string(&mut self) -> Result<Token, String> {
        let mut string = String::new();
        self.read_char(); // Skip the opening quote
        while let Some(c) = self.current {
            if c == '"' {
                self.read_char(); // Skip the closing quote
                return Ok(Token::String(string));
            }
            string.push(c);
            self.read_char();
        }
        Err(format!("Unterminated string: \"{}\"", string))
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        match self.current {
            Some(c) if Self::is_blacklisted(c) => {
                writeln!(io::stderr(), "[line {}] Error: Unexpected character: {}", self.line, c).unwrap();
                self.exit_code = 65;
                self.read_char();
                self.next_token()
            },
            Some(c) if c.is_digit(10) => Some(self.read_number()),
            Some(c) if c.is_alphabetic() || c == '_' => Some(self.read_identifier()),
            Some(c) if c == '"' => match self.read_string() {
                Ok(token) => Some(token),
                Err(_) => {
                    self.exit_code = 65;
                    writeln!(io::stderr(), "[line {}] Error: Unterminated string.", self.line).unwrap();
                    None
                }
            },
            Some(c) if c == '=' => {
                self.read_char();
                if self.current == Some('=') {
                    self.read_char();
                    Some(Token::EqualEqual)
                } else {
                    Some(Token::Equal)
                }
            },
            Some(c) if c == '<' => {
                self.read_char();
                if self.current == Some('=') {
                    self.read_char();
                    Some(Token::LessEqual)
                } else {
                    Some(Token::Less)
                }
            },
            Some(c) if c == '!' => {
                self.read_char();
                if self.current == Some('=') {
                    self.read_char();
                    Some(Token::BangEqual)
                } else {
                    Some(Token::Bang)
                }
            },
            Some(c) if c == '>' => {
                self.read_char();
                if self.current == Some('=') {
                    self.read_char();
                    Some(Token::GreaterEqual)
                } else {
                    Some(Token::Greater)
                }
            },
            Some(c) if c == '/' => {
                self.read_char();
                if self.current == Some('/') {
                    while let Some(c) = self.current {
                        if c == '\n' {
                            self.read_char();
                            break;
                        }
                        self.read_char();
                    }
                    self.next_token()
                } else {
                    Some(Token::Slash)
                }
            },
            Some('(') => { self.read_char(); Some(Token::LeftParentheses) },
            Some(')') => { self.read_char(); Some(Token::RightParentheses) },
            Some('{') => { self.read_char(); Some(Token::LeftBrace) },
            Some('}') => { self.read_char(); Some(Token::RightBrace) },
            Some('*') => { self.read_char(); Some(Token::Star) },
            Some(',') => { self.read_char(); Some(Token::Comma) },
            Some('+') => { self.read_char(); Some(Token::Plus) },
            Some('-') => { self.read_char(); Some(Token::Minus) },
            Some(';') => { self.read_char(); Some(Token::SemiColumn) },
            Some('.') => { self.read_char(); Some(Token::Dot) },
            None => None,
            _ => {
                self.read_char();
                None
            }
        }
    }
}

// ============ Parser Implementation ============
struct Parser<'a> {
    source: &'a str,
    lexer: Lexer<'a>,
    current: Option<Token>,
    prev_token_end: usize,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            lexer: Lexer::new(source),
            current: None,
            prev_token_end: 0,
        }
    }

    fn advance(&mut self) -> Result<(), ErrorType> {
        self.current = self.lexer.next_token();
        Ok(())
    }

    fn parse(&mut self) -> Result<(), ErrorType> {
        self.advance()?; // Get first token
        self.parse_statements()
    }

    fn parse_statements(&mut self) -> Result<(), ErrorType> {
        while let Some(token) = &self.current {
            match token {
                Token::ReservedWord(word) if word == "true" => {
                    writeln!(io::stdout(), "true").unwrap();
                    self.advance()?;
                },
                Token::ReservedWord(word) if word == "false" => {
                    writeln!(io::stdout(), "false").unwrap();
                    self.advance()?;
                },
                Token::ReservedWord(word) if word == "nil" => {
                    writeln!(io::stdout(), "nil").unwrap();
                    self.advance()?;
                },
                Token::Number(nStr, _) => {
                    writeln!(io::stdout(), "{}", nStr).unwrap();
                    self.advance()?;
                },
                Token::String(str) => {
                    writeln!(io::stdout(), "{}", str).unwrap();
                    self.advance()?;
                },
                Token::LeftParentheses => {
                    match self.parse_paranthesis() {
                        Ok(()) => {}, // Parenthesis was parsed successfully
                        Err(err) => return Err(err), // Propagate any parsing errors
                    }
                },
                Token::Bang => {
                    match self.parse_negation() {
                        Ok(()) => {}, // Parenthesis was parsed successfully
                        Err(err) => return Err(err), // Propagate any parsing errors
                    }
                },
                _ => {
                    self.advance()?;
                }
            }
        }

        Ok(())
    }

    fn parse_paranthesis(&mut self) -> Result<(), ErrorType> {
        self.advance()?; // consume '('
        write!(io::stdout(), "(group ").unwrap();

        // Print the current token
        match &self.current {
            Some(Token::Number(nStr, val)) => {
                write!(io::stdout(), "NUMBER {} {}", nStr, val).unwrap();
                self.advance()?;
            },
            Some(Token::String(str)) => {
                write!(io::stdout(), "STRING \"{}\"", str).unwrap();
                self.advance()?;
            },
            Some(Token::Identifier(name)) => {
                write!(io::stdout(), "IDENTIFIER {}", name).unwrap();
                self.advance()?;
            },
            Some(Token::LeftParentheses) => {
                self.parse_paranthesis()?;
            },
            Some(other_token) => {
                write!(io::stdout(), "OTHER {:?}", other_token).unwrap();
                self.advance()?;
            },
            None => {
                write!(io::stdout(), "NONE").unwrap();
                return Err(ErrorType::ParseError);
            }
        }

        // Check for closing parenthesis
        match &self.current {
            Some(Token::RightParentheses) => {
                write!(io::stdout(), ")").unwrap();
                self.advance()?; // consume ')'
                Ok(())
            },
            _ => {
                writeln!(io::stderr(), "Expected closing parenthesis").unwrap();
                Err(ErrorType::ParseError)
            }
        }
    }
    fn parse_negation(&mut self) -> Result<(), ErrorType> {
        self.advance()?; // consume '!'
        write!(io::stdout(), "(! ").unwrap();
        match &self.current {
            Some(Token::ReservedWord(word)) if word == "true" || word == "false" => {
                write!(io::stdout(), "{}", word).unwrap();
                write!(io::stdout(), "{}", ")").unwrap();
                self.advance()
            },
            _ => {
                self.advance()
            }
        }
    }
}

// ============ Main Function ============
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} <command> <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    // Read the file
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    match command.as_str() {
        "tokenize" => {
            let mut lexer = Lexer::new(&contents);
            while let Some(token) = lexer.next_token() {
                println!("{:?}", token);
            }
            if lexer.exit_code > 0 {
                process::exit(lexer.exit_code);
            }
        }
        "parse" => {
            let mut parser = Parser::new(&contents);
            match parser.parse() {
                Ok(_) => {
                    // println!("Successfully parsed program");
                }
                Err(_) => {
                    eprintln!("Error parsing program");
                    process::exit(65);
                }
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            process::exit(1);
        }
    }
}