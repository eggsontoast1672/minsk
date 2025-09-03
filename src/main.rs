use std::io::Write;

use minsk::{ast, eval, lexer::Lexer, parser::Parser};

fn the_whole_kitchen_sink(source: &str) {
    let tokens = match Lexer::from_source(source).collect::<Result<Vec<_>, _>>() {
        Ok(ts) => ts,
        Err(error) => {
            println!("{error}");
            return;
        }
    };

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
