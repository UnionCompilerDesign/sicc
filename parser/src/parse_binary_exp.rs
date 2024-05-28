//! Contains functions for handling the parsing of binary expressions.

use common::{ 
    error::ErrorType,
    ast::{
        ast_struct::ASTNode, 
        syntax_element::SyntaxElement,
    },
};

use lexer::token::Token;
use crate::parser_core::Parser;

impl Parser {
    /// Entry point for the parsing of a binary expression.
    ///
    /// # Returns
    /// 
    /// * `Ok(Some(ASTNode))` - if the binary expression was successfully parsed.
    /// * `Ok(None)` - if there was no binary expression to parse.
    /// * `Err(Vec<ErrorType>)` - if there were errors encountered during parsing.
    pub fn parse_binary_expression(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        todo!();
    }
}


/// Returns true if the given token is associated with binary expressions, false otherwise.
///
/// # Arguments
///
/// * `token` - An optional `Token`.
///
/// # Returns
///
/// A boolean indicating whether the token is a binary expression token.
pub fn is_binexp_token(token: Option<Token>) -> bool {
    todo!();
}

/// Returns the precedence value of a given token for binary operators.
///
/// # Arguments
///
/// * `token` - An optional reference to a `Token`.
///
/// # Returns
///
/// An optional `i32` representing the precedence value of the token.
pub fn binop_precedence_token(token: Option<&Token>) -> Option<i32> {
    todo!();
}

/// Returns true if the given token is an operator, false otherwise.
///
/// # Arguments
///
/// * `token` - A reference to a `Token`.
///
/// # Returns
///
/// A boolean indicating whether the token is an operator.
pub fn token_is_operator(token: &Token) -> bool {
    todo()!;
}

/// Returns true if the given token is an operand, false otherwise.
///
/// # Arguments
///
/// * `token` - An optional `Token`.
///
/// # Returns
///
/// A boolean indicating whether the token is an operand.
pub fn token_is_operand(token: Option<Token>) -> bool {
    todo()!;
}

/// Converts a vector of tokens representing a binary expression from infix notation to prefix notation.
///
/// # Arguments
///
/// * `tokens` - A vector of `Token` representing the binary expression in infix notation.
///
/// # Returns
///
/// A vector of `Token` representing the binary expression in prefix notation.
pub fn convert_infix_to_prefix(tokens: Vec<Token>) -> Vec<Token> {pub fn convert_infix_to_prefix(tokens: Vec<Token>) -> Vec<Token> {
    todo!();
}
