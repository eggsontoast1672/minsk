use std::io::Write;

use minsk::{ast, eval, lexer::Lexer, parser::Parser};

fn the_whole_kitchen_sink(source: &str) {
    println!("Source: {source}");

    let tokens: Vec<_> = Lexer::from_source(source).map(|t| t.unwrap()).collect();
    println!("\nTokens:\n[");
    for token in &tokens {
        println!("    {token:?},");
    }
    println!("]");

    let ast = Parser::new(tokens).parse();
    println!("\nAST:");
    ast::pretty_print(&ast);

    let result = eval::evaluate(ast);
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
