//static linkage
mod compiler; 
mod utility;

//namespacing
use compiler::lexical_analysis::Lexer;
use compiler::syntax_analysis::Parser;

fn main() {
    let source = String::from("local a = 1");

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.execute();

    let mut parser = Parser::new();
    parser.execute();

    println!("{:?}", tokens);
}
