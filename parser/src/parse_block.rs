//! Contains functions which turn a stream of tokens representing a block or blocks of code into a corresponding abstract syntax tree. 

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
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(tokens);
    /// let block = parser.parse_block()?;
    /// ```
    pub fn parse_block(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        self.consume_token(Token::LBRACKET)?; 
        let mut block_exp = ASTNode::new(SyntaxElement::BlockExpression);

        let mut children: Vec<ASTNode> = Vec::new();

        while self.get_current() < self.get_input().len() && self.get_input().get(self.get_current()) != Some(&Token::RBRACKET) {
            match self.parse_router() {
                Ok(Some(expr_node)) => {
                    children.push(expr_node);
                }
                Ok(None) => {}
                _ => panic!("parse_block parse problem")
            }
        }
        if self.get_input().get(self.get_current()) == Some(&Token::RBRACKET) {
            self.consume_token(Token::RBRACKET)?;
        } else {
            panic!("failed to reach stop token")
        }
        block_exp.add_children(children);

        Ok(Some(block_exp))
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
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(tokens);
    /// let initialization = parser.parse_initialization()?;
    /// ```
    pub fn parse_initialization(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        todo!();
    }

    fn parse_initialization_with_value(&mut self, var_node: ASTNode) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        todo!();
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
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(tokens);
    /// let if_statement = parser.parse_if_statement()?;
    /// ```
    pub fn parse_if_statement(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                Some(Token::IF) => {
                    self.consume_token(Token::IF)?;
                    self.consume_token(Token::LPAREN)?;
                    
                    let condition: ASTNode = match self.parse_router() {
                        Ok(Some(value)) => {value}
                        _ => panic!("if statement panic")
                    };
                    self.consume_token(Token::RPAREN)?;

                    let mut condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
                    condition_node.add_child(condition);

                    let mut if_node: ASTNode = ASTNode::new(SyntaxElement::IfStatement);
                    if_node.add_child(condition_node);

                    match self.parse_router() {
                        Ok(Some(node)) => {
                            if_node.add_child(node);
                        }
                        _ => {
                            panic!("Missing then branch")
                        }
                    }


                    if let Some(Token::ELSE) = self.get_input().get(self.get_current()) {
                        self.consume_token(Token::ELSE)?;
                        match self.parse_router() {
                            Ok(Some(node)) => {
                                if_node.add_child(node);
                            }
                            _ => {
                                panic!("Missing else block exp")
                            }
                        }
                    };

                    return Ok(Some(if_node));
                }
                _ => panic!("Problem parsing in if statement"),
            }
        } panic!("Problem parsing if statement 2")
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
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(tokens);
    /// let for_loop = parser.parse_for_loop()?;
    /// ```
    pub fn parse_for_loop(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        todo()!
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
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(tokens);
    /// let while_loop = parser.parse_while_loop()?;
    /// ```
    pub fn parse_while_loop(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        todo()!
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
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(tokens);
    /// let do_while_loop = parser.parse_do_while_loop()?;
    /// ```
    pub fn parse_do_while_loop(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        todo()!
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
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(tokens);
    /// let switch_statement = parser.parse_switch_statement()?;
    /// ```
    pub fn parse_switch_statement(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        todo!();
    }
}
