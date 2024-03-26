#[derive(Debug)]
pub enum Token<'a> {
    Add, 
    Subtract,
    Multiply,
    Divide,
    Module,

    Identifier,
    Equal,
    Name,

    Number(&'a str)
}