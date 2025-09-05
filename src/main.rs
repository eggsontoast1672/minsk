use std::io::Write;

use minsk::{
    ast, eval,
    lexer::{Lexer, Token, TokenKind},
    parser::Parser,
    repl::{self, Color},
};

pub fn print_error(source: &str, offender: Token, c: char) {
    let mut line_index = 0;
    let mut column_index = 0;
    for (index, c) in source.char_indices() {
        // The index should never be greater, since `offset.start` is exactly at the beginning of
        // some unicode character.
        if index >= offender.start {
            break;
        }

        column_index += 1;
        if c == '\n' {
            line_index += 1;
            column_index = 0;
        }
    }

    let line = source.lines().nth(line_index).unwrap();

    repl::set_text_color(Color::White);
    print!("stdin:{}:{}:", line_index + 1, column_index + 1);

    repl::set_text_color(Color::Red);
    print!(" error:");

    repl::set_text_color(Color::White);
    println!(" unrecognized token '{c}'");

    repl::reset_text_color();
    println!("{:5} | {line}", line_index + 1);
    print!("      | ");
    for _ in 0..column_index {
        print!(" ");
    }

    repl::set_text_color(Color::Green);
    print!("^");

    repl::reset_text_color();
    println!("\n1 error generated.");
}

fn the_whole_kitchen_sink(source: &str) {
    let tokens: Vec<_> = Lexer::from_source(source).collect();
    for token in &tokens {
        if let TokenKind::Invalid(c) = token.kind {
            print_error(source, token.clone(), c);
            return;
        }
    }

    let ast = match Parser::new(tokens.clone()).parse() {
        Ok(expr) => expr,
        Err(error) => {
            println!("{error}");
            return;
        }
    };

    let result = eval::evaluate(ast.clone());

    println!("Source: {source}");

    println!("\nTokens:\n[");
    for token in &tokens {
        println!("    {token:?},");
    }
    println!("]");

    println!("\nAST:");
    ast::pretty_print(&ast);

    println!("\nResult: {result}");
}

fn get_line(prompt: &str) -> Option<String> {
    print!("{prompt}");
    std::io::stdout().flush().unwrap();
    std::io::stdin().lines().next().map(|l| l.unwrap())
}

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    println!("Interactive prompt for Minsk version {version}");

    while let Some(line) = get_line(">>> ") {
        the_whole_kitchen_sink(&line);
    }
}
