//! This file contains basic tests for the parser, most often ensuring correct parsing of individual tokens.

use common::ast::{
    ast_struct::{ASTNode, AST}, data_type::DataType, syntax_element::SyntaxElement
};
use std::env;

use lexer::token::Token;
use parser::parser_core::Parser;

/// cargo test --test base_tests
/// panic!("{:?}", self.get_input().get(self.get_current()));

/// println!("{:#?}", ast);
/// println!("{:#?}", expected_ast);

/// An empty input should generate an ast with only a TopLevelExpression node.
#[test]
fn test_empty_input() { 
    let tokens: Vec<Token> = vec![];
    let ast = Parser::parse(tokens).expect("Failed to parse");
    assert_eq!(ast.get_root().get_element(), SyntaxElement::TopLevelExpression);
    assert!(ast.get_root().get_children().is_empty());
}

/// An input with only an EOF token should generate an ast with only a TopLevelExpression node.
#[test]
fn test_EOF() { 
    let tokens: Vec<Token> = vec![Token::EOF];
    let ast = Parser::parse(tokens).expect("Failed to parse");
    assert_eq!(ast.get_root().get_element(), SyntaxElement::TopLevelExpression);
    assert!(ast.get_root().get_children().is_empty());
}

/// A number token should yield a literal syntax element.
#[test]
fn test_number() {
    let tokens: Vec<Token> = vec![
        Token::NUMBER(vec!['2', '3']),
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    
    // ast
    let ast = result.expect("Failed to parse");

    // expected_ast
    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(ASTNode::new(SyntaxElement::Literal("23".to_string())));
    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// A number token representing a floating point number should yield a literal syntax element.
#[test]
fn test_floating_point_number() {
    let tokens: Vec<Token> = vec![
        Token::NUMBER(vec!['2', '.', '3']),
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    
    // ast
    let ast = result.expect("Failed to parse");

    // expected_ast
    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(ASTNode::new(SyntaxElement::Literal("2.3".to_string())));
    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// An idenifier token should yield an identifier syntax element.
#[test]
fn test_identifier() {
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['f', 'o', 'o']),
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    
    // ast
    let ast = result.expect("Failed to parse");

    // expected_ast
    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(ASTNode::new(SyntaxElement::Identifier("foo".to_string())));
    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// A string literal token should yield a literal syntax element. [TODO]
// #[test]
// fn test_string_literal() {
//     let tokens: Vec<Token> = vec![
//         Token::STRINGLITERAL(vec!['t', 'e', 's', 't', '5']),
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    
//     // ast
//     let ast = result.expect("Failed to parse");

//     // expected_ast
//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
//     top_level_expr.add_child(ASTNode::new(SyntaxElement::Literal("test5".to_string())));
//     let expected_ast: AST = AST::new(top_level_expr);

//     println!("{:#?}", ast);
//     println!("{:#?}", expected_ast);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

/// A char token should yield a literal syntax element. [TODO]
// #[test]
// fn test_string_literal() {
//     let tokens: Vec<Token> = vec![
//         Token::CHAR('a'),
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    
//     // ast
//     let ast = result.expect("Failed to parse");

//     // expected_ast
//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
//     top_level_expr.add_child(ASTNode::new(SyntaxElement::Literal("a".to_string())));
//     let expected_ast: AST = AST::new(top_level_expr);

//     println!("{:#?}", ast);
//     println!("{:#?}", expected_ast);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

/// An break token should yield an break syntax element.
#[test]
fn test_break() {
    let tokens: Vec<Token> = vec![
        Token::BREAK,
        Token::SEMICOLON,
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    
    // ast
    let ast = result.expect("Failed to parse");

    // expected_ast
    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(ASTNode::new(SyntaxElement::Break));
    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// A continue token should yield an continue syntax element.
#[test]
fn test_continue() {
    let tokens: Vec<Token> = vec![
        Token::CONTINUE,
        Token::SEMICOLON,
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    
    // ast
    let ast = result.expect("Failed to parse");

    // expected_ast
    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(ASTNode::new(SyntaxElement::Continue));
    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// A semicolon token should generate an ast with only a TopLevelExpression node.
#[test]
fn test_semicolon() {
    let tokens: Vec<Token> = vec![
        Token::SEMICOLON,
    ];

    let tokens: Vec<Token> = vec![Token::EOF];
    let ast = Parser::parse(tokens).expect("Failed to parse");
    assert_eq!(ast.get_root().get_element(), SyntaxElement::TopLevelExpression);
    assert!(ast.get_root().get_children().is_empty());
}
