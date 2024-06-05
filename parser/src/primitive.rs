//! Contains functions for parsing individual tokens, such as identifiers and protected keywords.

use common::{ 
    ast::{
        ast_struct::ASTNode, data_type::DataType, syntax_element::SyntaxElement
    }, error::ErrorType
};
use lexer::token::Token;
use crate::core::Parser;

impl Parser {
    /// Parses a primitive value token into an AST node representing a literal value.
    ///
    /// # Returns
    ///
    /// Returns an `Option<ASTNode>` containing the literal value, or an error `Vec<ErrorType>` if parsing fails.
    ///
    /// # Errors
    ///
    /// * Returns an error if the current token is not a `NUMBER` or if there is a failure in token consumption.
    pub fn parse_primitive(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        unimplemented!();
    }

    /// Parses an identifier token into an AST node or an assignment if an equal sign follows the identifier.
    /// This method expects a token of type `IDENTIFIER`.
    ///
    /// # Returns
    ///
    /// Returns an `Option<ASTNode>` containing either the identifier or the assignment node, or an error `Vec<ErrorType>` if parsing fails.
    ///
    /// # Errors
    ///
    /// * Returns an error if the current token is not an `IDENTIFIER` or if there is a failure in token consumption or assignment parsing.
    pub fn parse_identifier(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        unimplemented!();
    }

    /// Parses a variable name from an identifier token and returns it as a string.
    /// This method expects a token of type `IDENTIFIER`.
    ///
    /// # Returns
    ///
    /// Returns a `String` representing the variable name, or an error `Vec<ErrorType>` if parsing fails.
    ///
    /// # Errors
    ///
    /// * Returns an error if the current token is not an `IDENTIFIER` or if there is a failure in token consumption.
    pub fn parse_variable_name(&mut self) -> Result<String, Vec<ErrorType>> {
        unimplemented!();
    }

    /// Parses a protected keyword into the corresponding AST node. Supported keywords include `BREAK`, `CONTINUE`, and `RETURN`.
    /// This method also handles the `EOF` and `SEMICOLON` tokens appropriately.
    ///
    /// # Returns
    ///
    /// Returns an `Option<ASTNode>` containing the parsed keyword node, or an error `Vec<ErrorType>` if parsing fails.
    ///
    /// # Errors
    ///
    /// * Returns an error if the current token is not a recognized keyword or if there is a failure in token consumption or value parsing.
    pub fn parse_protected_keyword(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        unimplemented!();
    }

    /// Consumes a type token and returns the corresponding `DataType` enum value. Supported types include
    /// `TINTEGER`, `TBOOLEAN`, `TDOUBLE`, `TFLOAT`, `TCHAR`, `TVOID`, `TSIGN`, `TUSIGN`, `TSIGNINT`, and `TLONG`.
    ///
    /// # Returns
    ///
    /// Returns a `DataType` representing the type of the token, or an error `ErrorType` if parsing fails.
    ///
    /// # Errors
    ///
    /// * Returns an error if the current token is not a recognized type token or if there is a failure in token consumption.
    pub fn parse_type(&mut self) -> Result<DataType, ErrorType> {
        unimplemented!();
    }
}