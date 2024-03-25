#[derive(Debug)]
pub enum Token<'a> {
    Plus, 
    Minus,

    Number(&'a str)
}