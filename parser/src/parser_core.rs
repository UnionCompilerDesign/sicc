/*
Converts tokens into an AST and creates a symbol table stack
*/
                                 
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

    /// Parses an input of tokens into an AST, or returns a vector of errors
    pub fn parse(input: Vec<Token>) -> Result<AST, Vec<ErrorType>> {
        let mut parser = Parser::new(input);
        let mut root_children: Vec<ASTNode> = Vec::new();  
        let mut errors: Vec<ErrorType> = Vec::new();

        while parser.get_current() < parser.get_input().len() {
            match parser.parse_router() { 
                Ok(Some(node)) => {
                    if node.get_element() == SyntaxElement::BinaryExpression  { 
                        root_children.pop();
                    }
                    if node.get_element() != SyntaxElement::NoExpression {
                        root_children.push(node);  
                    }
                }
                Ok(None) => {}
                Err(error_types) => {
                    errors.extend(error_types);
                }
            } 
        }

        let mut root: ASTNode = ASTNode::new(SyntaxElement::TopLevelExpression);
        root.add_children(root_children);
        if errors.is_empty() {
            return Ok(AST::new(root));
        }
        Err(errors)
    }  

    /// Steps the current token position back by 1.
    pub fn step_current_back(&mut self) {
        self.current -= 1;
    }

    /// Gets the current input vector
    pub fn get_input(&mut self) -> Vec<Token> {
        self.input.clone()
    }

    /// Gets the current position in the input vector
    pub fn get_current(&mut self) -> usize {
        self.current.clone()
    }

    /// Consumes a token if the expected token matches the token
    pub fn consume_token(&mut self, expected_token: Token) -> Result<(), ErrorType> {
        if let Some(token) = self.get_input().get(self.get_current()) {
            if *token == expected_token {
                self.current += 1;
                Ok(())
            } else {
                panic!("What is this? This is not the right token. Try again. Expected: {:?}, Actual: {:?}", expected_token, *token)
            }
        } else {
            panic!("You tried to consume a token that doesn't exist? Tsk tsk")
        }
    }
    
    /// Peeks at the token that's next (self.current + 1)
    pub fn peek_token(&mut self) -> Option<Token> {
        if self.get_current() < self.get_input().len() {
            self.get_input().get(self.get_current() + 1).cloned()
        } 
        else {
            None
        }
    }

    /// Peeks at the token that's before this one (self.current - 1)
    pub fn peek_last_token(&mut self) -> Option<Token> {
        if self.get_current() > 0 {
            self.get_input().get(self.get_current() - 1).cloned()
        } 
        else {
            None
        }
    }


    fn is_unary_minus(&mut self) -> bool {
        if self.current == 0 || // First token
            matches!(self.peek_last_token(), Some(Token::LPAREN) | Some(Token::COMMA) | None | Some(Token::PLUS) | Some(Token::DASH) | Some(Token::ASTERISK) | Some(Token::FSLASH)) {
            true // It's unary if it's the first token or follows an operator or an open parenthesis
        } 
        else {
            false // Otherwise its binary
        }
    }    

    /// Steps the current token position forward.
    pub fn step_current_forward(&mut self) {
        self.current += 1;
    }

    /// Returns a null expression. Used to ignore tokens that aren't legal on their own but may be part of a larger expression.
    pub fn no_expression(&mut self) -> Result<Option<ASTNode>, Vec<ErrorType>> {
        self.step_current_forward();
        Ok(Some(ASTNode::new(SyntaxElement::NoExpression)))

    }


    /// Entry point to the main parsing logic. Serves as a way to match the current token type to the file/expression we want to parse
    // TODO Need to actually return errors here
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
