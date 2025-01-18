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
                        } else if special_chars.contains(&c.to_string()) {
                            err = true;
                            writeln!(io::stderr(), "[line {}] Error: Unexpected character: {}", line_number_index + 1,  c.to_string()).unwrap();
                        }
                    }
                }
                println!("EOF  null")
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }

            process::exit(65);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
