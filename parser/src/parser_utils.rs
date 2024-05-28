//! Contains useful helper functions for the parser.

use lexer::token::Token;

use crate::parser_core::Parser;

impl Parser {
    /// Converts an operator token to its corresponding character representation.
    ///
    /// # Parameters
    ///
    /// - `token`: A reference to a `Token` that represents an operator.
    ///
    /// # Returns
    ///
    /// Returns a `char` representing the operator corresponding to the provided token.
    ///
    /// # Panics
    ///
    /// Panics if the provided token is not a recognized operator token (`PLUS`, `DASH`, `ASTERISK`, `FSLASH`).
    ///
    /// # Examples
    ///
    /// ```
    /// use lexer::token::Token;
    /// use parser::Parser;
    /// let parser = Parser::new(vec![/* tokens */]);
    /// let plus_char = parser.operator_to_char(&Token::PLUS);
    /// assert_eq!(plus_char, '+');
    /// let dash_char = parser.operator_to_char(&Token::DASH);
    /// assert_eq!(dash_char, '-');
    /// ```
    pub fn operator_to_char(&self, token: &Token) -> char {
        match token {
            Token::PLUS => '+',
            Token::DASH => '-',
            Token::ASTERISK => '*',
            Token::FSLASH => '/',
            _ => panic!("not an operator")
        }
    }
}
