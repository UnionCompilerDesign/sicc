//! Contains functions which turn a stream of tokens representing a block or blocks of code into a corresponding abstract syntax tree. 

use common::{ 
    error::ErrorType,
    ast::core::ASTNode,
};
use crate::core::Parser;

impl Parser {
    /// Creates the children of an expression that changes scope. Used for all scope changing expressions except structs and enums.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(ASTNode))` - The parsed block expression node if successful.
    /// * `Err(Vec<ErrorType>)` - A list of errors if parsing fails.
    ///
    /// # Errors
    ///
    /// * Will return an error if a token is missing or if parsing fails at any point.
    pub fn parse_block(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        unimplemented!();
    }

    /// Parses the initialization of a variable or function. 
    /// Such a statement is characterized by a leading type annotation, representing either the type of the variable or the return type of the function.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(ASTNode))` - The parsed initialization node if successful.
    /// * `Err(Vec<ErrorType>)` - A list of errors if parsing fails.
    ///
    /// # Errors
    ///
    /// * Will return an error if a token is missing or if parsing fails at any point.
    pub fn parse_initialization(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        unimplemented!();
    }

    /// Parses an if statement. Such a statement is characterized by a leading 'Token::IF', with a subsequent condition expression and body. 
    ///
    /// # Returns
    ///
    /// * `Ok(Some(ASTNode))` - The parsed if statement node if successful.
    /// * `Err(Vec<ErrorType>)` - A list of errors if parsing fails.
    ///
    /// # Errors
    ///
    /// * Will return an error if a token is missing or if parsing fails at any point.
    pub fn parse_if_statement(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        unimplemented!();
    }

    /// Parses a for loop. Looks for a initialization, condition, and increment expressions, as well as a loop body.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(ASTNode))` - The parsed for loop node if successful.
    /// * `Err(Vec<ErrorType>)` - A list of errors if parsing fails.
    ///
    /// # Errors
    ///
    /// * Will return an error if a token is missing or if parsing fails at any point.
    pub fn parse_for_loop(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        unimplemented!();
    }
    

    /// Parses a while loop. Looks for a condition expression, and a loop body.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(ASTNode))` - The parsed while loop node if successful.
    /// * `Err(Vec<ErrorType>)` - A list of errors if parsing fails.
    ///
    /// # Errors
    ///
    /// * Will return an error if a token is missing or if parsing fails at any point.
    pub fn parse_while_loop(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        unimplemented!();
    }

    /// Parses a do while loop. Looks for a condition expression and a loop body.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(ASTNode))` - The parsed do while loop node if successful.
    /// * `Err(Vec<ErrorType>)` - A list of errors if parsing fails.
    ///
    /// # Errors
    ///
    /// * Will return an error if a token is missing or if parsing fails at any point.
    pub fn parse_do_while_loop(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        unimplemented!();
    }

    /// Parses a switch statement. Looks for an identifier to switch on, and cases.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(ASTNode))` - The parsed switch statement node if successful.
    /// * `Err(Vec<ErrorType>)` - A list of errors if parsing fails.
    ///
    /// # Errors
    ///
    /// * Will return an error if a token is missing or if parsing fails at any point.
    pub fn parse_switch_statement(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        unimplemented!();
    }

    /// Parses a function declaration. This method expects tokens for the function's name (identifier),
    /// return type, parameters, and function body. The resulting AST will include a `FunctionDeclaration`
    /// node containing the function's identifier, parameters, return type, and body.
    ///
    /// # Parameters
    ///
    /// * `identifier_node`: An `ASTNode` representing the function's identifier.
    /// * `return_type_node`: An `ASTNode` representing the function's return type.
    ///
    /// # Returns
    ///
    /// Returns an `Option<ASTNode>` containing the parsed function declaration node, or an error `Vec<ErrorType>` if parsing fails.
    ///
    /// # Errors
    ///
    /// * Returns an error if there is a failure in token consumption or block parsing.
    pub fn parse_function_declaration(&mut self, identifier_node: ASTNode, return_type_node: ASTNode) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        let _ = identifier_node;
        let _ = return_type_node;
        unimplemented!();
    }
    
    /// Parses an enum declaration. This method expects tokens for the enum name and its variants,
    /// enclosed in braces. The resulting AST will include an `EnumDeclaration` node containing the
    /// enum's name and its variants as `Variant` nodes.
    ///
    /// # Returns
    ///
    /// Returns an `Option<ASTNode>` containing the parsed enum declaration node, or an error `Vec<ErrorType>` if parsing fails.
    ///
    /// # Errors
    ///
    /// * Returns an error if there is a failure in token consumption or if the expected tokens are not found.
    pub fn parse_enum_declaration(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        unimplemented!();
    }
    
    /// Parses a struct declaration. This method expects tokens for the struct name and its fields,
    /// including field names and types, enclosed in braces. The resulting AST will include a
    /// `StructDeclaration` node containing the struct's name and its fields as `Field` nodes.
    ///
    /// # Returns
    ///
    /// Returns an `Option<ASTNode>` containing the parsed struct declaration node, or an error `Vec<ErrorType>` if parsing fails.
    ///
    /// # Errors
    ///
    /// * Returns an error if there is a failure in token consumption or if the expected tokens are not found.
    pub fn parse_struct_declaration(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        unimplemented!();
    }

}