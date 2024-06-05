//! This file contains tests for error handling, making sure that the parser returns the correct errors when given invalid token streams.

use common::{ 
    error::ErrorType,
};
use lexer::token::Token;
use parser::core::Parser;


/// This test checks that the parser returns a SyntaxError when a block does not have a closing bracket.
#[test]
fn test_unclosed_block() {
    let tokens: Vec<Token> = vec![
        Token::LBRACKET,
    ];

    let e = Parser::parse(tokens).unwrap_err();
    assert!(matches!(e[0], ErrorType::SyntaxError { .. }));
}

/// This test checks that the parser returns a SyntaxError when a variable is being initialized but no value after the equals sign is gived.
#[test]
fn test_malformed_initialization() {
    let tokens: Vec<Token> = vec![
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::SEMICOLON,
    ];

    let e = Parser::parse(tokens).unwrap_err();
    assert!(matches!(e[0], ErrorType::SyntaxError { .. }));
}

/// This test checks that the parser returns a SyntaxError if an if statement with an else condition fails to include a block expression afterwards.
#[test]
fn test_malformed_if_statement() {
    let tokens: Vec<Token> = vec![
        Token::IF,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['x']),
        Token::RPAREN,
        Token::LBRACKET,
        Token::RETURN,
        Token::IDENTIFIER(vec!['x']),
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::ELSE,
        Token::SEMICOLON,
        Token::EOF,
    ];

    let e = Parser::parse(tokens).unwrap_err();

    assert!(matches!(e[0], ErrorType::SyntaxError { .. }));
}

/// This test checks that the parser returns a SyntaxError if a for loop fails to include a body,
#[test]
fn test_malformed_for_loop() {
    let tokens: Vec<Token> = vec![
        Token::FOR,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('x')), 
        Token::EQUAL,
        Token::NUMBER(vec!('0')),
        Token::SEMICOLON,
        Token::IDENTIFIER(vec!('x')), 
        Token::LESSTHAN,
        Token::NUMBER(vec!('1')),
        Token::SEMICOLON,
        Token::IDENTIFIER(vec!('x')), 
        Token::EQUAL,
        Token::IDENTIFIER(vec!('x')), 
        Token::PLUS,
        Token::NUMBER(vec!('1')),
        Token::RPAREN,
        Token::BREAK,
        Token::SEMICOLON,
        Token::EOF,
    ];

    let e = Parser::parse(tokens).unwrap_err();

    assert!(matches!(e[0], ErrorType::SyntaxError { .. }));
}

/// This test checks that the parser returns a SyntaxError if a for loop fails to include any initialization, condition, or increment.
#[test]
fn test_malformed_for_loop_condition() {
    let tokens: Vec<Token> = vec![
        Token::FOR,
        Token::LPAREN,
        Token::RPAREN,
        Token::BREAK,
        Token::SEMICOLON,
        Token::EOF,
    ];


    let e = Parser::parse(tokens).unwrap_err();

    assert!(matches!(e[0], ErrorType::SyntaxError { .. }));
}

/// This test checks that the parser returns a SyntaxError if a while loop fails to include a condition,
#[test]
fn test_malformed_while_loop() {
    let tokens: Vec<Token> = vec![
        Token::WHILE,
        Token::LPAREN,
        Token::SEMICOLON,
        Token::RPAREN,
        Token::LBRACKET,
        Token::BREAK,
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::EOF,
    ];

    let e = Parser::parse(tokens).unwrap_err();

    assert!(matches!(e[0], ErrorType::SyntaxError { .. }));
}

/// This test checks that the parser returns a SyntaxError if a do while loop fails to include a body.
#[test]
fn test_malformed_do_while_loop() {
    let tokens: Vec<Token> = vec![
        Token::DO,
        Token::WHILE,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('x')),
        Token::RPAREN,
        Token::SEMICOLON,
        Token::EOF,
    ];

    let e = Parser::parse(tokens).unwrap_err();

    assert!(matches!(e[0], ErrorType::SyntaxError { .. }));
}

/// This test checks that the parser returns a SyntaxError if a switch statement does not have a body.
#[test]
fn test_malformed_switch_statement() {
    let tokens: Vec<Token> = vec![
        Token::SWITCH,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['y']),
        Token::RPAREN,
        
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['8']),
        Token::SEMICOLON,
        
        Token::BREAK,
        Token::SEMICOLON,
        Token::EOF,
    ];

    let e = Parser::parse(tokens).unwrap_err();

    assert!(matches!(e[0], ErrorType::SyntaxError { .. }));
}

/// This test checks that the parser retutns a SyntaxError if a function declaration does not have a body.
#[test]
fn test_malformed_function_declaration() {
    let tokens: Vec<Token> = vec![
        Token::TVOID,
        Token::IDENTIFIER(vec!['m', 'y', '_', 'f', 'u', 'n', 'c']),
        Token::LPAREN,
        Token::RPAREN,
        Token::EOF,
    ];

    let e = Parser::parse(tokens).unwrap_err();

    assert!(matches!(e[0], ErrorType::SyntaxError { .. }));
}

/// This test checks that the parser retutns a SyntaxError if a enum declaration fails to include a body.
#[test]
fn test_malformed_enum_declaration() {
    let tokens: Vec<Token> = vec![
        Token::ENUM,
        Token::IDENTIFIER(vec!['C', 'o', 'l', 'o', 'r']),
        Token::SEMICOLON,
    ];

    let e = Parser::parse(tokens).unwrap_err();

    assert!(matches!(e[0], ErrorType::SyntaxError { .. }));
}

/// This test checks that the parser retutns a SyntaxError if a struct declaration fails to include a body.
#[test]
fn test_malformed_struct_declaration() {
    let tokens: Vec<Token> = vec![
        Token::STRUCT,
        Token::IDENTIFIER(vec!['C', 'o', 'l', 'o', 'r']),
        Token::SEMICOLON,
    ];

    let e = Parser::parse(tokens).unwrap_err();

    assert!(matches!(e[0], ErrorType::SyntaxError { .. }));
}

/// This test checks that the parser retutns a SyntaxError if an assignment fails to include a value to assign.
#[test]
fn test_malformed_assignment() {
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['C', 'o', 'l', 'o', 'r']),
        Token::EQUAL,
    ];

    let e = Parser::parse(tokens).unwrap_err();

    assert!(matches!(e[0], ErrorType::SyntaxError { .. }));
}
