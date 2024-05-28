//! The driver for the parsing process, uses the method of recursive descent to systematically iterate through 
//! tokens and routes to appropriate helper methods in the parser to construct an abstract syntax tree.
                                 
use common::{ 
    ast::{
        ast_struct::{ASTNode, AST}, 
        syntax_element::SyntaxElement,
    }, error::ErrorType
};

use lexer::token::Token;

/// Parses an input of tokens into an AST   
pub struct Parser {
    input: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(input: Vec<Token>) -> Self {
        Self {
            input,
            current: 0,
        }
    }

    /// Parses an input of tokens into an AST using recursive descent parsing.
    /// Iterates through tokens and routes to appropriate helper methods to construct an AST.
    ///
    /// # Parameters
    ///
    /// - `input`: A vector of `Token` representing the input to be parsed.
    ///
    /// # Returns
    ///
    /// Returns a `Result<AST, Vec<ErrorType>>` containing the constructed AST if successful, 
    /// or a vector of `ErrorType` if there are parsing errors.
    ///
    /// # Errors
    ///
    /// - Returns a vector of errors if there are issues during parsing, such as unexpected tokens.
    ///
    /// # Examples
    ///
    /// ```
    /// let tokens: Vec<Token> = vec![/* tokens */];
    /// let ast = Parser::parse(tokens);
    /// ```
    pub fn parse(input: Vec<Token>) -> Result<AST, Vec<ErrorType>> {
        let mut parser = Parser::new(input);
        let mut root_children: Vec<ASTNode> = Vec::new();  
        let mut errors: Vec<ErrorType> = Vec::new();

        todo!();
    }  

    /// Steps the current token position back by 1.
    pub fn step_current_back(&mut self) {
        self.current -= 1;
    }

    /// Gets the current input vector.
    ///
    /// # Returns
    ///
    /// Returns a vector of `Token` representing the input.
    ///
    /// # Examples
    ///
    /// ```
    /// use parser::Parser;
    /// let parser = Parser::new(vec![/* tokens */]);
    /// let input = parser.get_input();
    /// assert!(!input.is_empty());
    /// ```
    pub fn get_input(&mut self) -> Vec<Token> {
        self.input.clone()
    }

    /// Gets the current position in the input vector.
    ///
    /// # Returns
    ///
    /// Returns the current position as a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use parser::Parser;
    /// let parser = Parser::new(vec![/* tokens */]);
    /// let current_position = parser.get_current();
    /// assert_eq!(current_position, 0);
    /// ```
    pub fn get_current(&mut self) -> usize {
        self.current.clone()
    }

    /// Consumes a token if the expected token matches the current token in the input.
    ///
    /// # Parameters
    ///
    /// - `expected_token`: The `Token` that is expected to be consumed.
    ///
    /// # Returns
    ///
    /// Returns `Result<(), ErrorType>` indicating whether the token was successfully consumed.
    ///
    /// # Errors
    ///
    /// - Returns an error if the expected token does not match the current token or if there is no token to consume.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(vec![Token::LPAREN]);
    /// let result = parser.consume_token(Token::LPAREN);
    /// ```
    pub fn consume_token(&mut self, expected_token: Token) -> Result<(), ErrorType> {
        todo!();
    }
    
    /// Peeks at the next token in the input (current position + 1).
    ///
    /// # Returns
    ///
    /// Returns an `Option<Token>` containing the next token if available, or `None` if there is no next token.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(vec![Token::LPAREN, Token::RPAREN]);
    /// let next_token = parser.peek_token();
    /// assert_eq!(next_token, Some(Token::RPAREN));
    /// ```
    pub fn peek_token(&mut self) -> Option<Token> {
        todo!();
    }

    /// Peeks at the previous token in the input (current position - 1).
    ///
    /// # Returns
    ///
    /// Returns an `Option<Token>` containing the previous token if available, or `None` if there is no previous token.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(vec![Token::LPAREN, Token::RPAREN]);
    /// parser.consume_token(Token::LPAREN).unwrap();
    /// let last_token = parser.peek_last_token();
    /// assert_eq!(last_token, Some(Token::LPAREN));
    /// ```
    pub fn peek_last_token(&mut self) -> Option<Token> {
        todo()!
    }


    fn is_unary_minus(&mut self) -> bool {
        todo!();
    }    

    /// Steps the current token position forward by 1.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(vec![/* tokens */]);
    /// parser.step_current_forward();
    /// ```
    pub fn step_current_forward(&mut self) {
        self.current += 1;
    }

    /// Returns a null expression. Used to ignore tokens that aren't legal on their own but may be part of a larger expression.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<ASTNode>, Vec<ErrorType>>` containing a `NoExpression` AST node.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(vec![/* tokens */]);
    /// let node = parser.no_expression();
    /// assert!(node.is_ok());
    /// ```
    pub fn no_expression(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        self.step_current_forward();
        Ok(Some(ASTNode::new(SyntaxElement::NoExpression)))
    }


    /// Entry point to the main parsing logic. Routes the current token to the appropriate parsing method based on token type.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<ASTNode>, Vec<ErrorType>>` containing the parsed AST node or errors encountered during parsing.
    ///
    /// # Errors
    ///
    /// - Returns a vector of errors if there are issues during parsing, such as unexpected tokens or parsing failures.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = Parser::new(vec![Token::IF]);
    /// let result = parser.parse_router();
    /// assert!(result.is_ok());
    /// ```
    pub fn parse_router(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        if self.get_current() < self.get_input().len() {
            match self.get_input().get(self.get_current()) {
                // top level expressions
                Some(Token::STRUCT) => return self.parse_struct_declaration(), 
                Some(Token::ENUM) => return self.parse_enum_declaration(),

                // statements
                Some(Token::IF) => return self.parse_if_statement(),
                Some(Token::FOR) => return self.parse_for_loop(),
                Some(Token::DO) => return self.parse_do_while_loop(), 
                Some(Token::WHILE) => return self.parse_while_loop(),
                Some(Token::IDENTIFIER(_)) => return self.parse_identifier(),
                Some(Token::SWITCH) => return self.parse_switch_statement(),

                Some(Token::DASH) => {
                    if self.is_unary_minus() {
                        return self.parse_unary_expression();
                    } else {
                        return self.parse_binary_expression();
                    }
                },
                // binary operations
                Some(Token::PLUS) | 
                Some(Token::ASTERISK) | 
                Some(Token::FSLASH) |
                Some(Token::PERCENT) |
                Some(Token::GREATERTHAN) |
                Some(Token::GREATERTHANEQUAL) |
                Some(Token::LESSTHAN) |
                Some(Token::LESSTHANEQUAL) => return self.parse_binary_expression(),

                // unary operations
                Some(Token::EXCLAMATIONPOINT) => return self.parse_unary_expression(), 

                Some(Token::LBRACKET) => return self.parse_block(),

                // data types
                Some(Token::TINTEGER) |
                Some(Token::TBOOLEAN) |
                Some(Token::TDOUBLE) |
                Some(Token::TFLOAT) |
                Some(Token::TCHAR) |
                Some(Token::TVOID) |
                Some(Token::TSIGN) |
                Some(Token::TUSIGN) |
                Some(Token::TSIGNINT) |
                Some(Token::TLONG) => return self.parse_initialization(),

                
                // base elements like primitives, and protected keywords
                Some(Token::NUMBER(_)) => return self.parse_primitive(),
                Some(Token::CTRUE) |
                Some(Token::BREAK) |
		        Some(Token::RETURN) |
                Some(Token::CONTINUE) |
                Some(Token::SEMICOLON) |
                Some(Token::EOF) => return self.parse_protected_keyword(),
                Some(Token::LPAREN) => return self.no_expression(),
                _ => panic!("Are you sure this is an expression: {:?} {:?}", self.get_input().get(self.get_current()), self.get_current()),

            }
        } else {
            panic!("You hooligan. You're out of tokens")
        }
    }
}
