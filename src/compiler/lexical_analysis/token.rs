use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]

pub enum Token<'a> {
    //
    Identifier(&'a str), // variable names, function names, etc.
    Number(&'a str),     // integer literals, float literals, etc.
    String(&'a str),     // string literals, character literals, etc.

    //Keywords
    Local,
    True,
    False,

    //Symbols
    Comma,
}

impl<'a> Token<'a> {
    pub fn get_keyword(identifier: &'a str) -> Option<Token<'a>> {
        match identifier {
            "true" => Some(Token::True),
            "false" => Some(Token::False),
            "local" => Some(Token::Local),
            _ => None,
        }
    }
}
