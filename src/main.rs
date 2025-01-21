use std::{env, process};
use std::fs;
use std::io::{self, Write};

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
            let mut err = false;
            let mut before = "".to_string();
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                let special_chars = &vec!["$".to_string(), "#".to_string(), "@".to_string(), "^".to_string(), "%".to_string()];
                for (line_number_index, l) in file_contents.lines().enumerate() {
                    for c in l.chars() {
                        if "=" != c.to_string() && before == "=" {
                            println!("EQUAL = null");
                        } else if "=" != c.to_string() && before == "!" {
                            println!("BANG ! null");
                        } else if "=" != c.to_string() && before == "<" {
                            println!("LESS < null")
                        } else if "=" != c.to_string() && before == ">" { 
                            println!("GREATER > null")
                        } else if "/" != c.to_string() && before == "/" {
                            println!("SLASH / null")
                        }

                        if "(" == c.to_string() {
                            println!("LEFT_PAREN ( null")
                        } else if  ")" == c.to_string() {
                            println!("RIGHT_PAREN ) null")
                        } else if "{" == c.to_string() {
                            println!("LEFT_BRACE {{ null")
                        } else if "}" == c.to_string() {
                            println!("RIGHT_BRACE }} null")
                        } else if "*" == c.to_string() {
                            println!("STAR * null")
                        } else if "." == c.to_string() {
                            println!("DOT . null")
                        } else if "," == c.to_string() {
                            println!("COMMA , null")
                        } else if "+" == c.to_string() {
                            println!("PLUS + null")
                        } else if "-" == c.to_string() {
                            println!("MINUS - null")
                        } else if ";" == c.to_string() {
                            println!("SEMICOLON ; null")
                        } else if before == "=" && "=" == c.to_string() {
                            println!("EQUAL_EQUAL == null");
                            before = "".to_string();
                            continue;
                        } else if before == "!" && "=" == c.to_string() {
                            println!("BANG_EQUAL != null");
                            before = "".to_string();
                            continue;
                        } else if before == "<" && "=" == c.to_string() {
                            println!("LESS_EQUAL <= null");
                            before = "".to_string();
                            continue;
                        } else if before == ">" && "=" == c.to_string() {
                            println!("GREATER_EQUAL >= null");
                            before = "".to_string();
                            continue;
                        } else if "/" == before && "/" == c.to_string() {
                            before = "".to_string();
                            break;
                        } else if before.len() > 0 {
                            if let Some(first_char) = before.chars().nth(0) {
                                if first_char == '"' && c.to_string() == '"'.to_string() {
                                    println!("STRING \"{}\" {}", &before[1..], &before[1..]);
                                    before = "".to_string();
                                    continue;
                                }else if first_char != '"' && c.to_string() == '"'.to_string() {
                                    before = '"'.to_string();
                                    continue;
                                } else if first_char == '"' && c.to_string() != '"'.to_string() {
                                    before = before + c.to_string().as_str();
                                    continue;
                                }
                            }
                        }
                        if special_chars.contains(&c.to_string()) {
                            err = true;
                            writeln!(io::stderr(), "[line {}] Error: Unexpected character: {}", line_number_index + 1,  c.to_string()).unwrap();
                        }
                        if c.to_string() != " " {
                            before = c.to_string()
                        }
                    }

                    if let Some(first_char) = before.chars().nth(0) {
                        if first_char == '"' {
                            err = true;
                            writeln!(io::stderr(), "[line {}] Error: Unterminated string.", line_number_index + 1).unwrap();
                        }
                    }
                }
                if before == "=" {
                    println!("EQUAL = null");
                }else if before == "!" {
                    println!("BANG ! null")
                }else if before == ">" {
                    println!("GREATER > null")
                } else if before == "<" {
                    println!("LESS < null")
                } else if before == "/" {
                    println!("SLASH / null")
                }

                println!("EOF  null")
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }

            if err {
                process::exit(65);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}


