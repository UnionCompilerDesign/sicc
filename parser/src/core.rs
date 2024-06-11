//! The driver for the parsing process, uses the method of recursive descent to systematically iterate through 
//! tokens and routes to appropriate helper methods in the parser to construct an abstract syntax tree.
                                 
use common::{ 
    ast::{
        ast_struct::{ASTNode, AST}, 
        syntax_element::SyntaxElement,
    }, error::ErrorType
};
use lexer::token::Token;

/// The `Parser` struct models the process of parsing.
/// 
/// At initialization, it takes an input as a vector of tokens.
///
/// # Fields
/// * `input` - A vector of tokens from the output of the lexer representing the source code to be parsed.
/// * `current` - The current token being considered by the parser.
pub struct Parser {
    input: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Creates a new `Parser` instance with the given input tokens.
    ///
    /// This initializer sets up a `Parser` by accepting a vector of tokens and initializing the
    /// current token index to 0.
    ///
    /// # Parameters
    ///
    /// * `input`: A vector of `Token` representing the sequence of tokens to be parsed.
    ///
    /// # Returns
    ///
    /// Returns a new `Parser` instance ready to parse the provided tokens.
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
    /// * `input`: A vector of `Token` representing the input to be parsed.
    ///
    /// # Returns
    ///
    /// Returns a `Result<AST, Vec<ErrorType>>` containing the constructed AST if successful, 
    /// or a vector of `ErrorType` if there are parsing errors.
    ///
    /// # Errors
    ///
    /// * Returns a vector of errors if there are issues during parsing, such as unexpected tokens.
    ///
    /// # Examples
    ///
    /// ```
    /// use lexer::token::Token;
    /// use parser::core::Parser;
    /// let tokens: Vec<Token> = vec![/* tokens */];
    /// let ast = Parser::parse(tokens);
    /// ```
    pub fn parse(input: Vec<Token>) -> Result<AST, Vec<ErrorType>> {
        unimplemented!();
    }  

    /// Entry point to the main parsing logic. Routes the current token to the appropriate parsing method based on token type.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<ASTNode>, Vec<ErrorType>>` containing the parsed AST node or errors encountered during parsing.
    ///
    /// # Errors
    ///
    /// * Returns a vector of errors if there are issues during parsing, such as unexpected tokens or parsing failures.
    pub fn parse_router(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        unimplemented!();
    }
}