/*
    Lexer

    TODO: Make generic lex_rule trait, which we can apply depending on the current lookahead
    TODO: Make trie out of token symbols, for efficient symbol matching
*/

#![allow(dead_code, unused_imports, unused_variables, unused_mut)]

//mod
pub mod token;

//pub use
pub use token::Token;

use std::char;
//std
use std::iter::{Iterator, Peekable};
use std::str::CharIndices;

//crate
use crate::utility::peek::{self, Peek};

pub struct Lexer<'a> {
    //a peekable iterator of char indicies can be used to find and return subslices of the original source code
    iter: Peek<3, CharIndices<'a>>,
    source: &'a str,
    start: usize,
    cur: usize,
}

impl<'a> Lexer<'a> {
    fn next(&mut self) -> Option<char> {
        let (i, c) = self.iter.next()?;
        self.cur = i;
        Some(c)
    }

    fn peek(&mut self, n: usize) -> Option<char> {
        self.iter.peek(n).map(|(_, c)| c).copied()
    }

    fn consume_buffer(&mut self) -> &'a str {
        &self.source[self.start..=self.cur]
    }

    fn consume_while(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(c) = self.peek(0) {
            if !predicate(c) {
                break;
            }
            self.next();
        }
    }

    fn consume_while_tok(&mut self, predicate: impl Fn(char) -> bool) -> &'a str {
        self.consume_while(predicate);
        self.consume_buffer()
    }

    pub fn execute(mut self) -> Vec<Token<'a>> {
        // create a vector to store the tokens
        let mut tokens = Vec::new();

        // loop through the iterator
        while let Some(chr) = self.next() {
            self.start = self.cur;

            //identifier
            let token = if chr.is_alphabetic() {
                let identifier = self.consume_while_tok(|c| c.is_alphabetic());

                if let Some(token) = Token::get_keyword(identifier) {
                    token
                } else {
                    Token::Identifier(identifier)
                }
                //is it a keyword? nani
            } else if chr.is_numeric() {
                Token::Number(self.consume_while_tok(|c| c.is_numeric()))
            } else if chr == '"' || chr == '\'' {
                self.consume_while(|c| c != chr);

                //if there's a closing quote, consume buffer, otherwise eof
                if let Some(_) = self.next() {
                    Token::String(self.consume_buffer())
                } else {
                    break;
                }
            } else {
                continue;
            };

            // symbols here pls
            tokens.push(token);
        }

        tokens // gangsta nation
    }

    pub fn new(source: &'a str) -> Lexer<'a> {
        Self {
            iter: Peek::from(source.char_indices()),
            source,
            start: 0,
            cur: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let source = r#"local testicals true false 123 'string' "#;

        let lexer = Lexer::new(&source);
        let tokens = lexer.execute();

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0], Token::Local);
        assert_eq!(tokens[1], Token::Identifier("testicals"));
        assert_eq!(tokens[2], Token::True);
        assert_eq!(tokens[3], Token::False);
        assert_eq!(tokens[4], Token::Number("123"));
        assert_eq!(tokens[5], Token::String("string"));
    }
}
