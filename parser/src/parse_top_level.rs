//! Contains functions for parsing top level expressions, like functions and structs.

use common::{ 
    ast::{
        ast_struct::ASTNode, data_type::DataType, syntax_element::SyntaxElement
    }, error::ErrorType
};

use lexer::token::Token;

use crate::parser_core::Parser;

impl Parser {
    /// Parses a function declaration. This method expects tokens for the function's name (identifier),
    /// return type, parameters, and function body. The resulting AST will include a `FunctionDeclaration`
    /// node containing the function's identifier, parameters, return type, and body.
    ///
    /// # Parameters
    ///
    /// - `identifier_node`: An `ASTNode` representing the function's identifier.
    /// - `return_type_node`: An `ASTNode` representing the function's return type.
    ///
    /// # Returns
    ///
    /// Returns an `Option<ASTNode>` containing the parsed function declaration node, or an error `Vec<ErrorType>` if parsing fails.
    ///
    /// # Errors
    ///
    /// - Returns an error if there is a failure in token consumption or block parsing.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(tokens);
    /// let identifier_node = ASTNode::new(SyntaxElement::Identifier("my_function".to_string()));
    /// let return_type_node = ASTNode::new(SyntaxElement::Type(DataType::Void));
    /// let result = parser.parse_function_declaration(identifier_node, return_type_node);
    /// ```
    pub fn parse_function_declaration(&mut self, identifier_node: ASTNode, return_type_node: ASTNode) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        todo!();
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
    /// - Returns an error if there is a failure in token consumption or if the expected tokens are not found.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(tokens);
    /// let result = parser.parse_enum_declaration();
    /// ```
    pub fn parse_enum_declaration(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        todo!();
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
    /// - Returns an error if there is a failure in token consumption or if the expected tokens are not found.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new();
    /// let result = parser.parse_struct_declaration();
    /// ```
    pub fn parse_struct_declaration(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        todo!();
    }
}
