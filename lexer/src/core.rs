//! This file drives the lexing process, which takes an input string and breaks it up into lexemes (tokens).

use crate::token::Token;
use common::error::ErrorType;

/// The `Lexer` struct models the process of lexical analysis.
/// 
/// At initialization, it takes a string input, a starting position, and the current character.
///
/// # Fields
/// * `input` - A vector of characters representing the source code to be lexed.
/// * `position` - The current position within the input vector.
/// * `current` - The current character being analyzed by the lexer.
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current: char,
}

impl Lexer {
    /// Initializes the lexer. 
    /// 
    /// # Parameters
    /// * `input` - A vector of characters that represents the source code to be lexed. 
    fn new(input: Vec<char>) -> Self {
        let _ = input;
        unimplemented!()
    }

    /// Lexically analyzes the given input string and returns a vector of tokens or a vector of errors.
    ///
    /// # Parameters
    /// * `input` - A string slice representing the source code to be lexed.
    ///
    /// # Returns
    /// * `Ok(Vec<Token>)` - A vector of tokens if the input is successfully lexed without errors.
    /// * `Err(Vec<ErrorType>)` - A vector of error types if any issues occur during lexing, such as unrecognized tokens.
    ///
    /// # Errors
    /// This function may return errors if it encounters characters that do not conform the expected token or character types.
    pub fn lex(input: &str) -> Result<Vec<Token>, Vec<ErrorType>> {
        let _ = input;
        unimplemented!()
    }
}