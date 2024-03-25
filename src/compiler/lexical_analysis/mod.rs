/*
    Lexer

    TODO: Make generic lex_rule trait, which we can apply depending on the current lookahead
    TODO: Make trie out of token symbols, for efficient symbol matching
*/

//mod
pub mod token;

//pub use
pub use token::Token;

//std
use std::iter::{Iterator, Peekable};
use std::str::CharIndices;

//crate
use crate::utility::peek::Peek;

pub struct Lexer<'a> {
    //a peekable iterator of char indicies can be used to find and return subslices of the original source code
    iter: Peek<3, CharIndices<'a>> 
}

impl<'a> Lexer<'a> {
    pub fn execute(self) -> Vec<Token<'a>> {
        Vec::new()
    }

    pub fn new(source: &'a str) -> Lexer<'a> {
        Self { 
            iter: Peek::from(source.char_indices())
        }
    }
}