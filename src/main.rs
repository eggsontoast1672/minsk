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

fn main() {
    let source = "1 + 2 * 3";
    the_whole_kitchen_sink(source);
}
