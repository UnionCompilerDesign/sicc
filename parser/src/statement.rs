//! Contains functions for parsing expressions, such as unary expressions and variable assignments.

use common::{ 
    error::ErrorType,
    ast::{
        ast_struct::ASTNode, 
        syntax_element::SyntaxElement,
    },
};
use lexer::token::Token;
use crate::core::Parser;

impl Parser {
    /// Parses a unary expression. 
    /// Specifically handles DASH and EXCLAMATIONPOINT tokens, as returns corresponding AST with a top-level 
    /// 'SyntaxElement::UnaryExpression' ASTNode.
    ///
    /// # Returns
    ///
    /// Returns an `Option<ASTNode>` representing the parsed unary expression, or an error
    /// `Vec<ErrorType>` if parsing fails.
    ///
    /// # Errors
    ///
    /// - Returns an error if parsing of the unary expression fails.
    ///
    pub fn parse_unary_expression(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        unimplemented!();
    }
    
    /// Parses a variable reassignment. Handles assignment to literals, expressions, and other identifiers.
    /// Creates a top level 'SyntaxElement::Assignment' ASTNode, with children representing the identifier and
    /// its new AssignedValue. Is called by 'Parser::parse_identifier', which fullfills the `name_chars` parameter.
    ///
    /// # Parameters
    ///
    /// - `name_chars`: A vector of characters representing the name of the variable to be reassigned.
    ///
    /// # Returns
    ///
    /// Returns an `Option<ASTNode>` representing the parsed assignment, or an error
    /// `Vec<ErrorType>` if parsing fails.
    ///
    /// # Errors
    ///
    /// - Returns an error if parsing of the assignment fails.
    ///
    pub fn parse_assignment(&mut self, name_chars: Vec<char>) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        unimplemented!();
    }

    /// Entry point for the parsing of a binary expression.
    ///
    /// # Returns
    /// 
    /// * `Ok(Some(ASTNode))` - if the binary expression was successfully parsed.
    /// * `Ok(None)` - if there was no binary expression to parse.
    /// * `Err(Vec<ErrorType>)` - if there were errors encountered during parsing.
    ///
    /// # Errors
    ///
    /// - Returns an error if parsing of the assignment fails.
    ///
    pub fn parse_binary_expression(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        unimplemented!();
    }
}