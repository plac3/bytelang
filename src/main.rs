#![allow(dead_code, unused_imports, unused_variables, unused_mut)]

//static linkage
mod compiler;
mod utility;

//namespacing
use compiler::lexical_analysis::Lexer;
use compiler::syntax_analysis::Parser;

fn main() {
    let source = r#"10 200 "urmom" "#;
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.execute();
    println!("HELLO {:?}", tokens);

    /*
    let mut parser = Parser::new();
    parser.execute();

    println!("{:?}", tokens);
    */
}
