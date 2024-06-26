pub use token::Token;

pub struct Lexeme<'a> {
    kind: Token,
    body: &'a str,
}

impl<'a> Lexeme<'a> {
    pub fn new(body: Vec<Option<&(usize, char)>>) -> Lexeme<'a> {
        //Join not work cuz option.
        let body = body.join("");
        let kind = Token::Identifier;

        Self {
            body,
            kind
        }
    }
}