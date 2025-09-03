use minsk::{eval, lexer::Token, parser::Parser};

fn main() {
    let tokens = vec![
        Token::Number(1),
        Token::Plus,
        Token::Number(2),
        Token::Star,
        Token::Number(3),
        Token::EndOfFile,
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let result = eval::evaluate(ast);

    println!("result = {result}");
}
