//! This file contains combination tests for the parser, which tests sequences of tokens that represent common programming concepts, such as statements, loops, expressions, etc.

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

/// --- EXPRESSION SECTION --- ///

#[test]
fn test_basic_binary_expr() {
    // (-A * -B)
    let tokens: Vec<Token> = vec![
        Token::LPAREN,
        Token::DASH,
        Token::IDENTIFIER(vec!['a']),
        Token::ASTERISK,
        Token::DASH,
        Token::IDENTIFIER(vec!['b']),
        Token::RPAREN,
        Token::EOF
        
    ];
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut binary_expr_node = ASTNode::new(SyntaxElement::BinaryExpression);
    let mut unary_expr_node_1: ASTNode = ASTNode::new(SyntaxElement::UnaryExpression);
    let mut unary_expr_node_2: ASTNode = ASTNode::new(SyntaxElement::UnaryExpression);

    let identifier_node_a: ASTNode = ASTNode::new(SyntaxElement::Identifier("a".to_string()));
    let identifier_node_b: ASTNode = ASTNode::new(SyntaxElement::Identifier("b".to_string()));
    unary_expr_node_1.add_child(ASTNode::new(SyntaxElement::Operator("-".to_string())));
    unary_expr_node_1.add_child(identifier_node_a);
    unary_expr_node_2.add_child(ASTNode::new(SyntaxElement::Operator("-".to_string())));
    unary_expr_node_2.add_child(identifier_node_b);

    binary_expr_node.add_child(unary_expr_node_1);
    binary_expr_node.add_child(ASTNode::new(SyntaxElement::Operator("*".to_string())));
    binary_expr_node.add_child(unary_expr_node_2);


    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(binary_expr_node);

    let expected_ast: AST = AST::new(top_level_expr);
    println!("{}", ast);
    assert_eq!(ast, expected_ast);

    let tokens_2: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['a']),
        Token::PLUS,
        Token::NUMBER(vec!['5']),
        Token::EOF
        
    ];
    // A + B
    let ast_2: AST = Parser::parse(tokens_2).expect("Failed to parse");

    let mut binary_expr_node_2 = ASTNode::new(SyntaxElement::BinaryExpression);

    let identifier_node_a_2: ASTNode = ASTNode::new(SyntaxElement::Identifier("a".to_string()));
    let identifier_node_b_2: ASTNode = ASTNode::new(SyntaxElement::Literal('5'.to_string()));

    binary_expr_node_2.add_child(identifier_node_a_2);
    binary_expr_node_2.add_child(ASTNode::new(SyntaxElement::Operator("+".to_string())));
    binary_expr_node_2.add_child(identifier_node_b_2);


    let mut top_level_expr_2 = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr_2.add_child(binary_expr_node_2);

    let expected_ast_2: AST = AST::new(top_level_expr_2);
    assert_eq!(ast_2, expected_ast_2);

    let tokens_3: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['a']),
        Token::DASH,
        Token::NUMBER(vec!['5']),
        Token::EOF
        
    ];
    // A - B
    let ast_3: AST = Parser::parse(tokens_3).expect("Failed to parse");

    let mut binary_expr_node_3 = ASTNode::new(SyntaxElement::BinaryExpression);

    let identifier_node_a_3: ASTNode = ASTNode::new(SyntaxElement::Identifier("a".to_string()));
    let identifier_node_b_3: ASTNode = ASTNode::new(SyntaxElement::Literal('5'.to_string()));

    binary_expr_node_3.add_child(identifier_node_a_3);
    binary_expr_node_3.add_child(ASTNode::new(SyntaxElement::Operator("-".to_string())));
    binary_expr_node_3.add_child(identifier_node_b_3);


    let mut top_level_expr_3 = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr_3.add_child(binary_expr_node_3);

    let expected_ast_3: AST = AST::new(top_level_expr_3);

    assert_eq!(ast_3, expected_ast_3);

    let tokens_4: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['a']),
        Token::FSLASH,
        Token::NUMBER(vec!['5']),
        Token::EOF
        
    ];
    // A / B
    let ast_4: AST = Parser::parse(tokens_4).expect("Failed to parse");

    let mut binary_expr_node_4 = ASTNode::new(SyntaxElement::BinaryExpression);

    let identifier_node_a_4: ASTNode = ASTNode::new(SyntaxElement::Identifier("a".to_string()));
    let identifier_node_b_4: ASTNode = ASTNode::new(SyntaxElement::Literal('5'.to_string()));

    binary_expr_node_4.add_child(identifier_node_a_4);
    binary_expr_node_4.add_child(ASTNode::new(SyntaxElement::Operator("/".to_string())));
    binary_expr_node_4.add_child(identifier_node_b_4);


    let mut top_level_expr_4 = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr_4.add_child(binary_expr_node_4);

    let expected_ast_4: AST = AST::new(top_level_expr_4);


    assert_eq!(ast_4, expected_ast_4);
}

#[test]
fn test_compound_binary_expr() {
    // A * B + C
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['a']),
        Token::ASTERISK,
        Token::IDENTIFIER(vec!['b']),
        Token::PLUS,
        Token::IDENTIFIER(vec!['c']),
        Token::EOF
        
    ];
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");
    let mut binary_expr_node: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);

    let identifier_node_a: ASTNode = ASTNode::new(SyntaxElement::Identifier("a".to_string()));
    let identifier_node_b: ASTNode = ASTNode::new(SyntaxElement::Identifier("b".to_string()));

    binary_expr_node.add_child(identifier_node_a);
    binary_expr_node.add_child(ASTNode::new(SyntaxElement::Operator("*".to_string())));
    binary_expr_node.add_child(identifier_node_b);

    let mut top_binary_expr_node: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);
    top_binary_expr_node.add_child(binary_expr_node);
    top_binary_expr_node.add_child(ASTNode::new(SyntaxElement::Operator("+".to_string())));
    top_binary_expr_node.add_child(ASTNode::new(SyntaxElement::Identifier("c".to_string())));

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(top_binary_expr_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);

    // A + B * C + D
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['a']),
        Token::PLUS,
        Token::IDENTIFIER(vec!['b']),
        Token::ASTERISK,
        Token::IDENTIFIER(vec!['c']),
        Token::PLUS,
        Token::IDENTIFIER(vec!['d']),
        Token::EOF
        
    ];
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut binary_expr_node: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);

    let identifier_node_a: ASTNode = ASTNode::new(SyntaxElement::Identifier("a".to_string()));
    let identifier_node_b: ASTNode = ASTNode::new(SyntaxElement::Identifier("b".to_string()));
    let identifier_node_c: ASTNode = ASTNode::new(SyntaxElement::Identifier("c".to_string()));
    let identifier_node_d: ASTNode = ASTNode::new(SyntaxElement::Identifier("d".to_string()));

    binary_expr_node.add_child(identifier_node_b);
    binary_expr_node.add_child(ASTNode::new(SyntaxElement::Operator("*".to_string())));
    binary_expr_node.add_child(identifier_node_c);

    let mut binary_expr_node_2: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);

    binary_expr_node_2.add_child(identifier_node_a);
    binary_expr_node_2.add_child(ASTNode::new(SyntaxElement::Operator("+".to_string())));
    binary_expr_node_2.add_child(binary_expr_node);


    let mut top_binary_expr_node: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);
    top_binary_expr_node.add_child(binary_expr_node_2);
    top_binary_expr_node.add_child(ASTNode::new(SyntaxElement::Operator("+".to_string())));
    top_binary_expr_node.add_child(identifier_node_d);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(top_binary_expr_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);


    // A * B + C / D % E - F
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['a']),
        Token::ASTERISK,
        Token::IDENTIFIER(vec!['b']),
        Token::PLUS,
        Token::IDENTIFIER(vec!['c']),
        Token::FSLASH,
        Token::IDENTIFIER(vec!['d']),
        Token::PERCENT,
        Token::IDENTIFIER(vec!['e']),
        Token::DASH,
        Token::IDENTIFIER(vec!['f']),
        Token::EOF
        
    ];
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");
    let identifier_node_a: ASTNode = ASTNode::new(SyntaxElement::Identifier("a".to_string()));
    let identifier_node_b: ASTNode = ASTNode::new(SyntaxElement::Identifier("b".to_string()));
    let identifier_node_c: ASTNode = ASTNode::new(SyntaxElement::Identifier("c".to_string()));
    let identifier_node_d: ASTNode = ASTNode::new(SyntaxElement::Identifier("d".to_string()));
    let identifier_node_e: ASTNode = ASTNode::new(SyntaxElement::Identifier("e".to_string()));
    let identifier_node_f: ASTNode = ASTNode::new(SyntaxElement::Identifier("f".to_string()));

    let mut binary_expr_node: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);

    binary_expr_node.add_child(identifier_node_c);
    binary_expr_node.add_child(ASTNode::new(SyntaxElement::Operator("/".to_string())));
    binary_expr_node.add_child(identifier_node_d);


    let mut binary_expr_node_2: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);

    binary_expr_node_2.add_child(binary_expr_node);
    binary_expr_node_2.add_child(ASTNode::new(SyntaxElement::Operator("%".to_string())));
    binary_expr_node_2.add_child(identifier_node_e);

    let mut binary_expr_node_3: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);

    binary_expr_node_3.add_child(identifier_node_a);
    binary_expr_node_3.add_child(ASTNode::new(SyntaxElement::Operator("*".to_string())));
    binary_expr_node_3.add_child(identifier_node_b);

    let mut binary_expr_node_4: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);

    binary_expr_node_4.add_child(binary_expr_node_3);
    binary_expr_node_4.add_child(ASTNode::new(SyntaxElement::Operator("+".to_string())));
    binary_expr_node_4.add_child(binary_expr_node_2);

    let mut top_binary_expr_node: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);

    top_binary_expr_node.add_child(binary_expr_node_4);
    top_binary_expr_node.add_child(ASTNode::new(SyntaxElement::Operator("-".to_string())));
    top_binary_expr_node.add_child(identifier_node_f);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(top_binary_expr_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);

    // (-A * B) + C / D % -E - F
    let tokens: Vec<Token> = vec![

        Token::LPAREN,
        Token::DASH,
        Token::IDENTIFIER(vec!['a']),
        Token::ASTERISK,
        Token::IDENTIFIER(vec!['b']),
        Token::RPAREN,
        Token::PLUS,
        Token::IDENTIFIER(vec!['c']),
        Token::FSLASH,
        Token::IDENTIFIER(vec!['d']),
        Token::PERCENT,
        Token::DASH,
        Token::IDENTIFIER(vec!['e']),
        Token::DASH,
        Token::IDENTIFIER(vec!['f']),
        Token::EOF
        
    ];
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");
    let identifier_node_a: ASTNode = ASTNode::new(SyntaxElement::Identifier("a".to_string()));
    let identifier_node_b: ASTNode = ASTNode::new(SyntaxElement::Identifier("b".to_string()));
    let identifier_node_c: ASTNode = ASTNode::new(SyntaxElement::Identifier("c".to_string()));
    let identifier_node_d: ASTNode = ASTNode::new(SyntaxElement::Identifier("d".to_string()));
    let identifier_node_e: ASTNode = ASTNode::new(SyntaxElement::Identifier("e".to_string()));
    let identifier_node_f: ASTNode = ASTNode::new(SyntaxElement::Identifier("f".to_string()));

    let mut binary_expr_node: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);

    binary_expr_node.add_child(identifier_node_c);
    binary_expr_node.add_child(ASTNode::new(SyntaxElement::Operator("/".to_string())));
    binary_expr_node.add_child(identifier_node_d);


    let mut binary_expr_node_2: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);

    binary_expr_node_2.add_child(binary_expr_node);
    binary_expr_node_2.add_child(ASTNode::new(SyntaxElement::Operator("%".to_string())));
    let mut unary_expr_node_1: ASTNode = ASTNode::new(SyntaxElement::UnaryExpression);
    unary_expr_node_1.add_child(ASTNode::new(SyntaxElement::Operator("-".to_string())));
    unary_expr_node_1.add_child(identifier_node_e);
    binary_expr_node_2.add_child(unary_expr_node_1);

    let mut binary_expr_node_3: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);
    let mut unary_expr_node_2: ASTNode = ASTNode::new(SyntaxElement::UnaryExpression);
    unary_expr_node_2.add_child(ASTNode::new(SyntaxElement::Operator("-".to_string())));
    unary_expr_node_2.add_child(identifier_node_a);
    binary_expr_node_3.add_child(unary_expr_node_2);
    binary_expr_node_3.add_child(ASTNode::new(SyntaxElement::Operator("*".to_string())));
    binary_expr_node_3.add_child(identifier_node_b);

    let mut binary_expr_node_4: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);

    binary_expr_node_4.add_child(binary_expr_node_3);
    binary_expr_node_4.add_child(ASTNode::new(SyntaxElement::Operator("+".to_string())));
    binary_expr_node_4.add_child(binary_expr_node_2);

    let mut top_binary_expr_node: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);

    top_binary_expr_node.add_child(binary_expr_node_4);
    top_binary_expr_node.add_child(ASTNode::new(SyntaxElement::Operator("-".to_string())));
    top_binary_expr_node.add_child(identifier_node_f);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(top_binary_expr_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);

}

#[test]
fn test_unary_expression() {
    let tokens: Vec<Token> = vec![
        Token::DASH,
        Token::IDENTIFIER(vec!['A']),
        Token::EOF
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    // ast
    let ast = result.expect("Failed to parse");

    // expected_ast
    let mut unary_expr_node = ASTNode::new(SyntaxElement::UnaryExpression);
    unary_expr_node.add_child(ASTNode::new(SyntaxElement::Operator("-".to_string())));
    unary_expr_node.add_child(ASTNode::new(SyntaxElement::Identifier("A".to_string())));
    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(unary_expr_node);
    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

#[test]
fn test_binary_expr_with_unary_negation() {
    let tokens: Vec<Token> = vec![
        Token::DASH,
        Token::NUMBER(vec!['5']),
        Token::DASH, 
        Token::NUMBER(vec!['3']),
        Token::EOF,
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    let ast = result.expect("Failed to parse");

    // Construct the expected AST for the expression `-5 - 3`
    let mut unary_expr_node = ASTNode::new(SyntaxElement::UnaryExpression);
    unary_expr_node.add_child(ASTNode::new(SyntaxElement::Operator("-".to_string())));
    unary_expr_node.add_child(ASTNode::new(SyntaxElement::Literal("5".to_string())));

    let int_node = ASTNode::new(SyntaxElement::Literal("3".to_string()));

    let mut binary_expr_node = ASTNode::new(SyntaxElement::BinaryExpression);
    binary_expr_node.add_child(unary_expr_node);
    binary_expr_node.add_child(ASTNode::new(SyntaxElement::Operator("-".to_string())));
    binary_expr_node.add_child(int_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(binary_expr_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// This tests the increment operator (++) [TODO]
// #[test]
// fn test_increment_operator() {
//     let tokens: Vec<Token> = vec![
//         Token::IDENTIFIER(vec!['x']),
//         Token::PLUSPLUS,
//         Token::SEMICOLON,
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
//     let ast = result.expect("Failed to parse");

//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);

//     let expected_ast: AST = AST::new(top_level_expr);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

/// This tests the decrement operator (--) [TODO]
// #[test]
// fn test_increment_operator() {
//     let tokens: Vec<Token> = vec![
//         Token::IDENTIFIER(vec!['x']),
//         Token::MINUSMINUS,
//         Token::SEMICOLON,
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
//     let ast = result.expect("Failed to parse");

//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);

//     let expected_ast: AST = AST::new(top_level_expr);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

/// --- ASSIGNMENT SECTION --- ///

/// This tests an variable assignment to a number.
#[test]
fn test_assignment_to_number() {
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL, 
        Token::NUMBER(vec!['3']),
        Token::SEMICOLON,
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    let ast = result.expect("Failed to parse");

    // Construct the expected AST for the expression `x = 3;`
    let mut assignment_node = ASTNode::new(SyntaxElement::Assignment);
    assignment_node.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));
    assignment_node.add_child(ASTNode::new(SyntaxElement::Literal("3".to_string())));

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(assignment_node);

    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// This tests an variable assignment to a binary addition expression.
#[test]
fn test_assignment_to_addition_expression() {
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['3']),
        Token::PLUS,
        Token::NUMBER(vec!['4']),
        Token::SEMICOLON,
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    let ast = result.expect("Failed to parse");

    // Construct the expected AST for the expression `x = 3 + 4;`
    let mut assignment_node = ASTNode::new(SyntaxElement::Assignment);
    assignment_node.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));

    let mut binary_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
    binary_expression_node.add_child(ASTNode::new(SyntaxElement::Literal("3".to_string())));
    binary_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("+".to_string())));
    binary_expression_node.add_child(ASTNode::new(SyntaxElement::Literal("4".to_string())));

    assignment_node.add_child(binary_expression_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(assignment_node);

    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// This tests an variable assignment to a binary multiplication expression.
#[test]
fn test_assignment_to_multiplication_expression() {
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['y']),
        Token::EQUAL,
        Token::NUMBER(vec!['5']),
        Token::ASTERISK,
        Token::NUMBER(vec!['6']),
        Token::SEMICOLON,
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    let ast = result.expect("Failed to parse");

    // Construct the expected AST for the expression `y = 5 * 6;`
    let mut assignment_node = ASTNode::new(SyntaxElement::Assignment);
    assignment_node.add_child(ASTNode::new(SyntaxElement::Identifier("y".to_string())));

    let mut binary_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
    binary_expression_node.add_child(ASTNode::new(SyntaxElement::Literal("5".to_string())));
    binary_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("*".to_string())));
    binary_expression_node.add_child(ASTNode::new(SyntaxElement::Literal("6".to_string())));

    assignment_node.add_child(binary_expression_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(assignment_node);

    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// This tests an variable assignment to a binary division expression.
#[test]
fn test_assignment_to_division_expression() {
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['z']),
        Token::EQUAL,
        Token::NUMBER(vec!['8']),
        Token::FSLASH,
        Token::NUMBER(vec!['2']),
        Token::SEMICOLON,
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    let ast = result.expect("Failed to parse");

    // Construct the expected AST for the expression `z = 8 / 2;`
    let mut assignment_node = ASTNode::new(SyntaxElement::Assignment);
    assignment_node.add_child(ASTNode::new(SyntaxElement::Identifier("z".to_string())));

    let mut binary_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
    binary_expression_node.add_child(ASTNode::new(SyntaxElement::Literal("8".to_string())));
    binary_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("/".to_string())));
    binary_expression_node.add_child(ASTNode::new(SyntaxElement::Literal("2".to_string())));

    assignment_node.add_child(binary_expression_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(assignment_node);

    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// This tests an variable assignment to a binary subtraction expression.
#[test]
fn test_assignment_to_subtraction_expression() {
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['w']),
        Token::EQUAL,
        Token::NUMBER(vec!['1','0']),
        Token::DASH,
        Token::NUMBER(vec!['4']),
        Token::SEMICOLON,
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    let ast = result.expect("Failed to parse");

    // Construct the expected AST for the expression `w = 10 - 4;`
    let mut assignment_node = ASTNode::new(SyntaxElement::Assignment);
    assignment_node.add_child(ASTNode::new(SyntaxElement::Identifier("w".to_string())));

    let mut binary_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
    binary_expression_node.add_child(ASTNode::new(SyntaxElement::Literal("10".to_string())));
    binary_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("-".to_string())));
    binary_expression_node.add_child(ASTNode::new(SyntaxElement::Literal("4".to_string())));

    assignment_node.add_child(binary_expression_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(assignment_node);

    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// This tests an variable assignment to a binary addition expression with parentheses.
#[test]
fn test_assignment_to_parenthesized_addition_expression() {
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['a']),
        Token::EQUAL,
        Token::LPAREN,
        Token::NUMBER(vec!['3']),
        Token::PLUS,
        Token::NUMBER(vec!['4']),
        Token::RPAREN,
        Token::ASTERISK,
        Token::NUMBER(vec!['2']),
        Token::SEMICOLON,
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    let ast = result.expect("Failed to parse");

    // Construct the expected AST for the expression `a = (3 + 4) * 2;`
    let mut assignment_node = ASTNode::new(SyntaxElement::Assignment);
    assignment_node.add_child(ASTNode::new(SyntaxElement::Identifier("a".to_string())));

    let mut binary_expression_node_outer = ASTNode::new(SyntaxElement::BinaryExpression);
    let mut binary_expression_node_inner = ASTNode::new(SyntaxElement::BinaryExpression);
    
    binary_expression_node_inner.add_child(ASTNode::new(SyntaxElement::Literal("3".to_string())));
    binary_expression_node_inner.add_child(ASTNode::new(SyntaxElement::Operator("+".to_string())));
    binary_expression_node_inner.add_child(ASTNode::new(SyntaxElement::Literal("4".to_string())));
    
    binary_expression_node_outer.add_child(binary_expression_node_inner);
    binary_expression_node_outer.add_child(ASTNode::new(SyntaxElement::Operator("*".to_string())));
    binary_expression_node_outer.add_child(ASTNode::new(SyntaxElement::Literal("2".to_string())));

    assignment_node.add_child(binary_expression_node_outer);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(assignment_node);

    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// This tests an variable assignment to a complex binary expression with multiple parentheses.
#[test]
fn test_assignment_to_complex_expression() {
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['b']),
        Token::EQUAL,
        Token::LPAREN,
        Token::NUMBER(vec!['1']),
        Token::PLUS,
        Token::NUMBER(vec!['2']),
        Token::RPAREN,
        Token::ASTERISK,
        Token::LPAREN,
        Token::NUMBER(vec!['3']),
        Token::DASH,
        Token::NUMBER(vec!['4']),
        Token::RPAREN,
        Token::SEMICOLON,
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    let ast = result.expect("Failed to parse");

    // Construct the expected AST for the expression `b = (1 + 2) * (3 - 4);`
    let mut assignment_node = ASTNode::new(SyntaxElement::Assignment);
    assignment_node.add_child(ASTNode::new(SyntaxElement::Identifier("b".to_string())));

    let mut binary_expression_node_outer = ASTNode::new(SyntaxElement::BinaryExpression);
    let mut binary_expression_node_inner_left = ASTNode::new(SyntaxElement::BinaryExpression);
    let mut binary_expression_node_inner_right = ASTNode::new(SyntaxElement::BinaryExpression);

    binary_expression_node_inner_left.add_child(ASTNode::new(SyntaxElement::Literal("1".to_string())));
    binary_expression_node_inner_left.add_child(ASTNode::new(SyntaxElement::Operator("+".to_string())));
    binary_expression_node_inner_left.add_child(ASTNode::new(SyntaxElement::Literal("2".to_string())));
    
    binary_expression_node_inner_right.add_child(ASTNode::new(SyntaxElement::Literal("3".to_string())));
    binary_expression_node_inner_right.add_child(ASTNode::new(SyntaxElement::Operator("-".to_string())));
    binary_expression_node_inner_right.add_child(ASTNode::new(SyntaxElement::Literal("4".to_string())));

    binary_expression_node_outer.add_child(binary_expression_node_inner_left);
    binary_expression_node_outer.add_child(ASTNode::new(SyntaxElement::Operator("*".to_string())));
    binary_expression_node_outer.add_child(binary_expression_node_inner_right);

    assignment_node.add_child(binary_expression_node_outer);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(assignment_node);

    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// This tests an variable assignment to a binary division expression with nested parentheses.
#[test]
fn test_assignment_to_nested_parentheses_expression() {
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['c']),
        Token::EQUAL,
        Token::LPAREN,
        Token::LPAREN,
        Token::NUMBER(vec!['7']),
        Token::PLUS,
        Token::NUMBER(vec!['8']),
        Token::RPAREN,
        Token::ASTERISK,
        Token::NUMBER(vec!['2']),
        Token::RPAREN,
        Token::FSLASH,
        Token::NUMBER(vec!['3']),
        Token::SEMICOLON,
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    let ast = result.expect("Failed to parse");

    // Construct the expected AST for the expression `c = ((7 + 8) * 2) / 3;`
    let mut assignment_node = ASTNode::new(SyntaxElement::Assignment);
    assignment_node.add_child(ASTNode::new(SyntaxElement::Identifier("c".to_string())));

    let mut binary_expression_node_outer = ASTNode::new(SyntaxElement::BinaryExpression);
    let mut binary_expression_node_middle = ASTNode::new(SyntaxElement::BinaryExpression);
    let mut binary_expression_node_inner = ASTNode::new(SyntaxElement::BinaryExpression);

    binary_expression_node_inner.add_child(ASTNode::new(SyntaxElement::Literal("7".to_string())));
    binary_expression_node_inner.add_child(ASTNode::new(SyntaxElement::Operator("+".to_string())));
    binary_expression_node_inner.add_child(ASTNode::new(SyntaxElement::Literal("8".to_string())));
    
    binary_expression_node_middle.add_child(binary_expression_node_inner);
    binary_expression_node_middle.add_child(ASTNode::new(SyntaxElement::Operator("*".to_string())));
    binary_expression_node_middle.add_child(ASTNode::new(SyntaxElement::Literal("2".to_string())));

    binary_expression_node_outer.add_child(binary_expression_node_middle);
    binary_expression_node_outer.add_child(ASTNode::new(SyntaxElement::Operator("/".to_string())));
    binary_expression_node_outer.add_child(ASTNode::new(SyntaxElement::Literal("3".to_string())));

    assignment_node.add_child(binary_expression_node_outer);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(assignment_node);

    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// --- INITIALIZATION SECTION --- ///

#[test]
fn test_initialization_parsing_no_value() {
    let tokens = vec![
        Token::TBOOLEAN,
        Token::IDENTIFIER(vec!['x']),
        Token::SEMICOLON,
        Token::EOF,
    ];
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut initialization_node: ASTNode = ASTNode::new(SyntaxElement::Initialization);

    let var_id_node: ASTNode = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    let type_node: ASTNode = ASTNode::new(SyntaxElement::Type(DataType::Boolean));

    let mut variable_node: ASTNode = ASTNode::new(SyntaxElement::Variable);
    variable_node.add_child(var_id_node);
    variable_node.add_child(type_node);

    initialization_node.add_child(variable_node);

    let mut top_level_expr: ASTNode = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(initialization_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);
}

#[test]
fn test_initialization_parsing_int() {
    let tokens = vec![
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['1']),
        Token::SEMICOLON,
        Token::EOF,
    ];
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut initialization_node: ASTNode = ASTNode::new(SyntaxElement::Initialization);

    let var_id_node: ASTNode = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    let type_node: ASTNode = ASTNode::new(SyntaxElement::Type(DataType::Integer));

    let mut variable_node: ASTNode = ASTNode::new(SyntaxElement::Variable);
    variable_node.add_child(var_id_node);
    variable_node.add_child(type_node);

    let value_node: ASTNode = ASTNode::new(SyntaxElement::Literal("1".to_string()));
    let mut assigned_value_node: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);
    assigned_value_node.add_child(value_node);

    initialization_node.add_child(variable_node);
    initialization_node.add_child(assigned_value_node);

    let mut top_level_expr: ASTNode = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(initialization_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);
}

#[test]
fn test_initialization_parsing_long() {
    let tokens = vec![
        Token::TLONG,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['1']),
        Token::SEMICOLON,
        Token::EOF,
    ];
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut initialization_node: ASTNode = ASTNode::new(SyntaxElement::Initialization);

    let var_id_node: ASTNode = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    let type_node: ASTNode = ASTNode::new(SyntaxElement::Type(DataType::Long));

    let mut variable_node: ASTNode = ASTNode::new(SyntaxElement::Variable);
    variable_node.add_child(var_id_node);
    variable_node.add_child(type_node);

    let value_node: ASTNode = ASTNode::new(SyntaxElement::Literal("1".to_string()));
    let mut assigned_value_node: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);
    assigned_value_node.add_child(value_node);

    initialization_node.add_child(variable_node);
    initialization_node.add_child(assigned_value_node);

    let mut top_level_expr: ASTNode = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(initialization_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);
}

/// --- STRUCT SECTION --- ///

/// This tests a struct declaration without fields.
#[test]
fn test_struct_declaration_empty() {
    let tokens: Vec<Token> = vec![
        Token::STRUCT,
        Token::IDENTIFIER(vec!['M', 'y', 'S', 't', 'r', 'u', 'c', 't']),
        Token::LBRACE,
        Token::RBRACE,
        Token::SEMICOLON,
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the struct declaration without errors.");
    let ast = result.expect("Failed to parse");

    // Construct the expected AST for the struct declaration `struct MyStruct {};`
    let mut struct_node = ASTNode::new(SyntaxElement::StructDeclaration);
    struct_node.add_child(ASTNode::new(SyntaxElement::Identifier("MyStruct".to_string())));

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(struct_node);

    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// This tests a struct declaration with fields.
#[test]
fn test_struct_declaration_with_fields() {
    let tokens: Vec<Token> = vec![
        Token::STRUCT,
        Token::IDENTIFIER(vec!['M', 'y', 'S', 't', 'r', 'u', 'c', 't']),
        Token::LBRACE,
        Token::IDENTIFIER(vec!['a']),
        Token::COLON,
        Token::TINTEGER,
        Token::COMMA,
        Token::IDENTIFIER(vec!['b']),
        Token::COLON,
        Token::TBOOLEAN,
        Token::RBRACE,
        Token::SEMICOLON,
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the struct declaration without errors.");
    let ast = result.expect("Failed to parse");

    // Construct the expected AST for the struct declaration `struct MyStruct { a: int, b: bool };`
    let mut struct_node = ASTNode::new(SyntaxElement::StructDeclaration);
    struct_node.add_child(ASTNode::new(SyntaxElement::Identifier("MyStruct".to_string())));

    let mut field_a = ASTNode::new(SyntaxElement::Field);
    field_a.add_child(ASTNode::new(SyntaxElement::Literal("a".to_string())));
    field_a.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));
    
    let mut field_b = ASTNode::new(SyntaxElement::Field);
    field_b.add_child(ASTNode::new(SyntaxElement::Literal("b".to_string())));
    field_b.add_child(ASTNode::new(SyntaxElement::Type(DataType::Boolean)));

    struct_node.add_child(field_a);
    struct_node.add_child(field_b);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(struct_node);

    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// This tests an assignment to a struct field.
// #[test]
// fn test_assignment_to_struct_field() {
//     let tokens: Vec<Token> = vec![
//         Token::IDENTIFIER(vec!['m', 'y', 'S', 't', 'r', 'u', 'c', 't']),
//         Token::DOT,
//         Token::IDENTIFIER(vec!['a']),
//         Token::EQUAL,
//         Token::NUMBER(vec!['1', '0']),
//         Token::SEMICOLON,
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
//     let ast = result.expect("Failed to parse");

//     // Construct the expected AST for the expression `myStruct.a = 10;`
//     let mut assignment_node = ASTNode::new(SyntaxElement::Assignment);
    
//     let mut field_access_node = ASTNode::new(SyntaxElement::Field);
//     field_access_node.add_child(ASTNode::new(SyntaxElement::Identifier("myStruct".to_string())));
//     field_access_node.add_child(ASTNode::new(SyntaxElement::Identifier("a".to_string())));
    
//     assignment_node.add_child(field_access_node);
//     assignment_node.add_child(ASTNode::new(SyntaxElement::Literal("10".to_string())));

//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
//     top_level_expr.add_child(assignment_node);

//     let expected_ast: AST = AST::new(top_level_expr);

//     println!("{:#?}", ast);
//     println!("{:#?}", expected_ast);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

/// This tests an assignment to a struct field with a binary expression.
// #[test]
// fn test_assignment_to_struct_field_with_binary_expression() {
//     let tokens: Vec<Token> = vec![
//         Token::IDENTIFIER(vec!['m', 'y', 'S', 't', 'r', 'u', 'c', 't']),
//         Token::DOT,
//         Token::IDENTIFIER(vec!['b']),
//         Token::EQUAL,
//         Token::NUMBER(vec!['5']),
//         Token::PLUS,
//         Token::NUMBER(vec!['3']),
//         Token::SEMICOLON,
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
//     let ast = result.expect("Failed to parse");

//     // Construct the expected AST for the expression `myStruct.b = 5 + 3;`
//     let mut assignment_node = ASTNode::new(SyntaxElement::Assignment);
    
//     let mut field_access_node = ASTNode::new(SyntaxElement::Field);
//     field_access_node.add_child(ASTNode::new(SyntaxElement::Identifier("myStruct".to_string())));
//     field_access_node.add_child(ASTNode::new(SyntaxElement::Identifier("b".to_string())));
    
//     let mut binary_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Literal("5".to_string())));
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("+".to_string())));
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Literal("3".to_string())));

//     assignment_node.add_child(field_access_node);
//     assignment_node.add_child(binary_expression_node);

//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
//     top_level_expr.add_child(assignment_node);

//     let expected_ast: AST = AST::new(top_level_expr);

//     println!("{:#?}", ast);
//     println!("{:#?}", expected_ast);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

/// --- ENUM SECTION --- ///

/// This tests an enum declaration without variants.
#[test]
fn test_enum_declaration_empty() {
    let tokens: Vec<Token> = vec![
        Token::ENUM,
        Token::IDENTIFIER(vec!['M', 'y', 'E', 'n', 'u', 'm']),
        Token::LBRACE,
        Token::RBRACE,
        Token::SEMICOLON,
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the enum declaration without errors.");
    let ast = result.expect("Failed to parse");

    // Construct the expected AST for the enum declaration `enum MyEnum {};`
    let mut enum_node = ASTNode::new(SyntaxElement::EnumDeclaration);
    enum_node.add_child(ASTNode::new(SyntaxElement::Identifier("MyEnum".to_string())));

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(enum_node);

    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// This tests an enum declaration with variants.
#[test]
fn test_enum_declaration_with_variants() {
    let tokens: Vec<Token> = vec![
        Token::ENUM,
        Token::IDENTIFIER(vec!['C', 'o', 'l', 'o', 'r']),
        Token::LBRACE,
        Token::IDENTIFIER(vec!['R', 'e', 'd']),
        Token::COMMA,
        Token::IDENTIFIER(vec!['G', 'r', 'e', 'e', 'n']),
        Token::COMMA,
        Token::IDENTIFIER(vec!['B', 'l', 'u', 'e']),
        Token::RBRACE,
        Token::SEMICOLON,
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the enum declaration without errors.");
    let ast = result.expect("Failed to parse");

    // Construct the expected AST for the enum declaration `enum Color { Red, Green, Blue };`
    let mut enum_node = ASTNode::new(SyntaxElement::EnumDeclaration);
    enum_node.add_child(ASTNode::new(SyntaxElement::Identifier("Color".to_string())));

    let mut variant_red = ASTNode::new(SyntaxElement::Variant);
    variant_red.add_child(ASTNode::new(SyntaxElement::Identifier("Red".to_string())));
    
    let mut variant_green = ASTNode::new(SyntaxElement::Variant);
    variant_green.add_child(ASTNode::new(SyntaxElement::Identifier("Green".to_string())));

    let mut variant_blue = ASTNode::new(SyntaxElement::Variant);
    variant_blue.add_child(ASTNode::new(SyntaxElement::Identifier("Blue".to_string())));

    enum_node.add_child(variant_red);
    enum_node.add_child(variant_green);
    enum_node.add_child(variant_blue);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(enum_node);

    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// --- FUNCTION SECTION --- ///

#[test]
fn test_single_function_declaration() {
    let tokens: Vec<Token> = vec![
        Token::TVOID,
        Token::IDENTIFIER(vec!['m', 'y', '_', 'f', 'u', 'n', 'c']),
        Token::LPAREN,
        Token::RPAREN,
        Token::LBRACKET,
        
        Token::RBRACKET,
        Token::EOF,
    ];
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut function_declaration_node = ASTNode::new(SyntaxElement::FunctionDeclaration);

    let identifier_node: ASTNode = ASTNode::new(SyntaxElement::Identifier("my_func".to_string()));
    let block_expression_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);

    function_declaration_node.add_child(identifier_node);

    let return_type_node: ASTNode = ASTNode::new(SyntaxElement::Type(DataType::Void));
    function_declaration_node.add_child(return_type_node);

    function_declaration_node.add_child(block_expression_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(function_declaration_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);
}

#[test]
fn test_function_with_parameters_and_return_type() {
    let tokens: Vec<Token> = vec![
        Token::TBOOLEAN,
        Token::IDENTIFIER(vec!['c', 'a', 'l', 'c', 'u', 'l', 'a', 't', 'e']),
        Token::LPAREN,
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['x']),
        Token::COMMA,
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['y']),
        Token::RPAREN,
        Token::LBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut function_declaration_node = ASTNode::new(SyntaxElement::FunctionDeclaration);
    let identifier_node = ASTNode::new(SyntaxElement::Identifier("calculate".to_string()));
    function_declaration_node.add_child(identifier_node);

    let mut parameter_x_node = ASTNode::new(SyntaxElement::Parameter);
    parameter_x_node.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));
    parameter_x_node.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));

    let mut parameter_y_node = ASTNode::new(SyntaxElement::Parameter);
    parameter_y_node.add_child(ASTNode::new(SyntaxElement::Identifier("y".to_string())));
    parameter_y_node.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));

    function_declaration_node.add_child(parameter_x_node);
    function_declaration_node.add_child(parameter_y_node);

    let return_type_node: ASTNode = ASTNode::new(SyntaxElement::Type(DataType::Boolean));
    function_declaration_node.add_child(return_type_node);

    let block_expression_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);

    function_declaration_node.add_child(block_expression_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(function_declaration_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);
}

#[test]
fn test_function_with_body() {
    let tokens: Vec<Token> = vec![
        Token::TVOID,
        Token::IDENTIFIER(vec!['t', 'e', 's', 't']),
        Token::LPAREN,
        Token::RPAREN,
        Token::LBRACKET,
        
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['1']),
        Token::SEMICOLON,
        
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut function_declaration_node: ASTNode = ASTNode::new(SyntaxElement::FunctionDeclaration);
    let identifier_node = ASTNode::new(SyntaxElement::Identifier("test".to_string()));
    function_declaration_node.add_child(identifier_node);

    let mut block_expression_node = ASTNode::new(SyntaxElement::BlockExpression);
    let mut initialization_node = ASTNode::new(SyntaxElement::Initialization);
    let mut assigned_value_node: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);
    let mut variable_node: ASTNode = ASTNode::new(SyntaxElement::Variable);

    let variable_id_node: ASTNode = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    let type_node: ASTNode = ASTNode::new(SyntaxElement::Type(DataType::Integer));
    let value_node: ASTNode = ASTNode::new(SyntaxElement::Literal("1".to_string()));

    variable_node.add_child(variable_id_node);
    variable_node.add_child(type_node);

    assigned_value_node.add_child(value_node);

    initialization_node.add_child(variable_node);
    initialization_node.add_child(assigned_value_node);

    block_expression_node.add_child(initialization_node);

    let return_type_node: ASTNode = ASTNode::new(SyntaxElement::Type(DataType::Void));
    function_declaration_node.add_child(return_type_node);

    function_declaration_node.add_child(block_expression_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(function_declaration_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);
}

#[test]
fn test_function_with_if_else_statement() {
    let tokens: Vec<Token> = vec![
        Token::TBOOLEAN,
        Token::IDENTIFIER(vec!['f', 'o', 'o']),
        Token::LPAREN,
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['a']),
        Token::COMMA,
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['b']),
        Token::RPAREN,
        Token::LBRACKET,
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
        Token::LBRACKET,
        Token::RETURN,
        Token::IDENTIFIER(vec!['x']),
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut function_declaration_node = ASTNode::new(SyntaxElement::FunctionDeclaration);
    let identifier_node = ASTNode::new(SyntaxElement::Identifier("foo".to_string()));
    function_declaration_node.add_child(identifier_node);

    let mut parameter_a_node = ASTNode::new(SyntaxElement::Parameter);
    parameter_a_node.add_child(ASTNode::new(SyntaxElement::Identifier("a".to_string())));
    parameter_a_node.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));
    function_declaration_node.add_child(parameter_a_node);

    let mut parameter_b_node = ASTNode::new(SyntaxElement::Parameter);
    parameter_b_node.add_child(ASTNode::new(SyntaxElement::Identifier("b".to_string())));
    parameter_b_node.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));
    function_declaration_node.add_child(parameter_b_node);

    let return_type_node = ASTNode::new(SyntaxElement::Type(DataType::Boolean));
    function_declaration_node.add_child(return_type_node);

    let mut if_statement_node = ASTNode::new(SyntaxElement::IfStatement);
    let mut condition_node = ASTNode::new(SyntaxElement::Condition);
    let inner_condition_node = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    condition_node.add_child(inner_condition_node);

    let mut then_branch_node = ASTNode::new(SyntaxElement::BlockExpression);
    let mut then_return_node = ASTNode::new(SyntaxElement::Return);
    let mut then_return_value = ASTNode::new(SyntaxElement::AssignedValue);
    let then_return_value_node = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    then_return_value.add_child(then_return_value_node);

    then_return_node.add_child(then_return_value);
    then_branch_node.add_child(then_return_node);

    let mut else_branch_node = ASTNode::new(SyntaxElement::BlockExpression);
    let mut else_return_node = ASTNode::new(SyntaxElement::Return);
    let mut else_return_value: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);
    let else_return_value_node = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    else_return_value.add_child(else_return_value_node);

    else_return_node.add_child(else_return_value);
    else_branch_node.add_child(else_return_node);

    if_statement_node.add_child(condition_node);
    if_statement_node.add_child(then_branch_node);
    if_statement_node.add_child(else_branch_node);

    let mut block_expression_node = ASTNode::new(SyntaxElement::BlockExpression);
    block_expression_node.add_child(if_statement_node);

    function_declaration_node.add_child(block_expression_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(function_declaration_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// --- CONTROL FLOW SECTION --- ///

#[test]
fn test_for_loop_parsing() {
    env::set_var("RUST_BACKTRACE", "1");

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
        Token::LBRACKET,
        Token::BREAK,
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::EOF,
    ];
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut for_loop_node: ASTNode = ASTNode::new(SyntaxElement::ForLoop);

    let mut init_node: ASTNode = ASTNode::new(SyntaxElement::LoopInitializer);
    let mut assignment_node: ASTNode = ASTNode::new(SyntaxElement::Assignment);
    assignment_node.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));
    assignment_node.add_child(ASTNode::new(SyntaxElement::Literal("0".to_string())));
    init_node.add_child(assignment_node);

    let mut condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let mut bin_exp_node: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);
    bin_exp_node.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));
    bin_exp_node.add_child(ASTNode::new(SyntaxElement::Operator("<".to_string())));
    bin_exp_node.add_child(ASTNode::new(SyntaxElement::Literal("1".to_string())));
    condition_node.add_child(bin_exp_node);

    let mut increment_node: ASTNode = ASTNode::new(SyntaxElement::LoopIncrement);
    let mut assignment_node2: ASTNode = ASTNode::new(SyntaxElement::Assignment);
    assignment_node2.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));

    let mut bin_exp_node2: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);
    bin_exp_node2.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));
    bin_exp_node2.add_child(ASTNode::new(SyntaxElement::Operator("+".to_string())));
    bin_exp_node2.add_child(ASTNode::new(SyntaxElement::Literal("1".to_string())));
    assignment_node2.add_child(bin_exp_node2);
    
    increment_node.add_child(assignment_node2);

    let mut body_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let break_node: ASTNode = ASTNode::new(SyntaxElement::Break);

    body_node.add_child(break_node);

    for_loop_node.add_child(init_node);
    for_loop_node.add_child(condition_node);
    for_loop_node.add_child(increment_node);
    for_loop_node.add_child(body_node);

    let mut top_level_expr: ASTNode = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(for_loop_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);
}

#[test]
fn test_while_loop_parsing() {
    let tokens: Vec<Token> = vec![
        Token::WHILE,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('x')),
        Token::RPAREN,
        Token::LBRACKET,
        Token::BREAK,
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::EOF,
    ];
    let actual_ast = Parser::parse(tokens).expect("Failed to parse");

    let mut while_loop_node = ASTNode::new(SyntaxElement::WhileLoop);
    
    let mut condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let condition_value_node = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    condition_node.add_child(condition_value_node);

    let mut body_node = ASTNode::new(SyntaxElement::BlockExpression);
    let break_node = ASTNode::new(SyntaxElement::Break);

    body_node.add_child(break_node);

    while_loop_node.add_child(condition_node); 
    while_loop_node.add_child(body_node); 

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(while_loop_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(actual_ast, expected_ast);
}

#[test]
fn test_do_while_loop_parsing() {
    let tokens: Vec<Token> = vec![
        Token::DO,
        Token::LBRACKET,
        Token::BREAK,
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::WHILE,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('x')),
        Token::RPAREN,
        Token::SEMICOLON,
        Token::EOF,
    ];
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut do_while_loop_node = ASTNode::new(SyntaxElement::DoWhileLoop);
    
    let mut body_node = ASTNode::new(SyntaxElement::BlockExpression);
    let break_node = ASTNode::new(SyntaxElement::Break);
    body_node.add_child(break_node);

    let mut condition_node = ASTNode::new(SyntaxElement::Condition);
    let condition_value_node = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    condition_node.add_child(condition_value_node);

    do_while_loop_node.add_child(body_node); 
    do_while_loop_node.add_child(condition_node); 

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(do_while_loop_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast)
}

#[test]
fn test_if_statement_parsing() {
    let tokens: Vec<Token> = vec![
        Token::IF,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('x')),
        Token::RPAREN,
        Token::LBRACKET,
        Token::RETURN,
        Token::IDENTIFIER(vec!('x')),
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut if_statement_node: ASTNode = ASTNode::new(SyntaxElement::IfStatement);

    let mut condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let inner_condition_node = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    condition_node.add_child(inner_condition_node);


    let mut then_branch_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let mut return_node: ASTNode = ASTNode::new(SyntaxElement::Return);
    let mut assigned_value_node: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);

    let return_value_node: ASTNode = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    assigned_value_node.add_child(return_value_node);

    return_node.add_child(assigned_value_node);
    then_branch_node.add_child(return_node);

    if_statement_node.add_child(condition_node);
    if_statement_node.add_child(then_branch_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(if_statement_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

#[test]
fn test_switch_statement_parsing() {

    /*
    let input = "switch (y) {
        case 1:
            let x: Integer = 6;
            break;
        case 2:
            let x: Integer = 7;
            break;
        default:
            let x: Integer = 8;
            break;
    }";
    */

    let tokens: Vec<Token> = vec![
        Token::SWITCH,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['y']),
        Token::RPAREN,
        Token::LBRACKET,
        
        Token::CASE,
        Token::NUMBER(vec!['1']),
        Token::COLON,
        
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['6']),
        Token::SEMICOLON,
        
        Token::BREAK,
        Token::SEMICOLON,
        
        Token::CASE,
        Token::NUMBER(vec!['2']),
        Token::COLON,
        
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['7']),
        Token::SEMICOLON,
        
        Token::BREAK,
        Token::SEMICOLON,
        
        Token::DEFAULT,
        Token::COLON,
        
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['8']),
        Token::SEMICOLON,
        
        Token::BREAK,
        Token::SEMICOLON,
        
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    // Case 1

    let mut assignedval_node1 = ASTNode::new(SyntaxElement::AssignedValue);
    assignedval_node1.add_child(ASTNode::new(SyntaxElement::Literal("6".to_string())));

    let mut var_node1 = ASTNode::new(SyntaxElement::Variable);
    var_node1.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));
    var_node1.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));

    let mut initialization_node1 = ASTNode::new(SyntaxElement::Initialization);
    initialization_node1.add_child(var_node1);
    initialization_node1.add_child(assignedval_node1);

    let mut case1_block = ASTNode::new(SyntaxElement::BlockExpression);
    case1_block.add_child(initialization_node1);
    case1_block.add_child(ASTNode::new(SyntaxElement::Break));

    let mut case1 = ASTNode::new(SyntaxElement::Case);
    case1.add_child(ASTNode::new(SyntaxElement::Literal("1".to_string())));
    case1.add_child(case1_block);
    
    // Case 2
    
    let mut assignedval_node2 = ASTNode::new(SyntaxElement::AssignedValue);
    assignedval_node2.add_child(ASTNode::new(SyntaxElement::Literal("7".to_string())));

    let mut var_node2 = ASTNode::new(SyntaxElement::Variable);
    var_node2.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));
    var_node2.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));

    let mut initialization_node2 = ASTNode::new(SyntaxElement::Initialization);
    initialization_node2.add_child(var_node2);
    initialization_node2.add_child(assignedval_node2);

    let mut case2_block = ASTNode::new(SyntaxElement::BlockExpression);
    case2_block.add_child(initialization_node2);
    case2_block.add_child(ASTNode::new(SyntaxElement::Break));

    let mut case2 = ASTNode::new(SyntaxElement::Case);
    case2.add_child(ASTNode::new(SyntaxElement::Literal("2".to_string())));
    case2.add_child(case2_block);

    // Default
    
    let mut assignedval_node3 = ASTNode::new(SyntaxElement::AssignedValue);
    assignedval_node3.add_child(ASTNode::new(SyntaxElement::Literal("8".to_string())));
 
    let mut var_node3 = ASTNode::new(SyntaxElement::Variable);
    var_node3.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));
    var_node3.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));

    let mut initialization_node3 = ASTNode::new(SyntaxElement::Initialization);
    initialization_node3.add_child(var_node3);
    initialization_node3.add_child(assignedval_node3);

    let mut case3_block = ASTNode::new(SyntaxElement::BlockExpression);
    case3_block.add_child(initialization_node3);
    case3_block.add_child(ASTNode::new(SyntaxElement::Break));

    let mut case3 = ASTNode::new(SyntaxElement::Default);
    case3.add_child(case3_block);

    let mut cases_block_node = ASTNode::new(SyntaxElement::BlockExpression);
    cases_block_node.add_child(case1);
    cases_block_node.add_child(case2);
    cases_block_node.add_child(case3);

    let identifier_node = ASTNode::new(SyntaxElement::Identifier("y".to_string()));

    let mut switch_statement_node = ASTNode::new(SyntaxElement::SwitchStatement);
    switch_statement_node.add_child(identifier_node);
    switch_statement_node.add_child(cases_block_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(switch_statement_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

#[test]
fn test_switch_statement_parsing_without_break_statements() {
    /*
    let input = "switch (y) {
      case 1:
          let x: Integer = 1;
      case 2:
          let x: Integer = 2;
    }";
    */

    let tokens: Vec<Token> = vec![
        Token::SWITCH,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['y']),
        Token::RPAREN,
        Token::LBRACKET,
        
        Token::CASE,
        Token::NUMBER(vec!['1']),
        Token::COLON,
        
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['1']),
        Token::SEMICOLON,
        
        Token::CASE,
        Token::NUMBER(vec!['2']),
        Token::COLON,
        
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['2']),
        Token::SEMICOLON,
        
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    // Case 1

    let mut assignedval_node1 = ASTNode::new(SyntaxElement::AssignedValue);
    assignedval_node1.add_child(ASTNode::new(SyntaxElement::Literal("1".to_string())));

    let mut var_node1 = ASTNode::new(SyntaxElement::Variable);
    var_node1.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));
    var_node1.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));

    let mut initialization_node1 = ASTNode::new(SyntaxElement::Initialization);
    initialization_node1.add_child(var_node1);
    initialization_node1.add_child(assignedval_node1);

    let mut case1_block = ASTNode::new(SyntaxElement::BlockExpression);
    case1_block.add_child(initialization_node1);

    let mut case1 = ASTNode::new(SyntaxElement::Case);
    case1.add_child(ASTNode::new(SyntaxElement::Literal("1".to_string())));
    case1.add_child(case1_block);
    
    // Case 2
    
    let mut assignedval_node2 = ASTNode::new(SyntaxElement::AssignedValue);
    assignedval_node2.add_child(ASTNode::new(SyntaxElement::Literal("2".to_string())));

    
    let mut var_node2 = ASTNode::new(SyntaxElement::Variable);
    var_node2.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));
    var_node2.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));

    let mut initialization_node2 = ASTNode::new(SyntaxElement::Initialization);
    initialization_node2.add_child(var_node2);
    initialization_node2.add_child(assignedval_node2);

    let mut case2_block = ASTNode::new(SyntaxElement::BlockExpression);
    case2_block.add_child(initialization_node2);

    let mut case2 = ASTNode::new(SyntaxElement::Case);
    case2.add_child(ASTNode::new(SyntaxElement::Literal("2".to_string())));
    case2.add_child(case2_block);


    let mut cases_block_node = ASTNode::new(SyntaxElement::BlockExpression);
    cases_block_node.add_child(case1);
    cases_block_node.add_child(case2);

    let identifier_node = ASTNode::new(SyntaxElement::Identifier("y".to_string()));

    let mut switch_statement_node = ASTNode::new(SyntaxElement::SwitchStatement);
    switch_statement_node.add_child(identifier_node);
    switch_statement_node.add_child(cases_block_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(switch_statement_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

// /// --- Boolean and Comparison Tests --- ///

// /// This tests a simple boolean expression using logical AND.
// #[test]
// fn test_logical_and_expression() {
//     let tokens: Vec<Token> = vec![
//         Token::IDENTIFIER(vec!['a']),
//         Token::ANDAND,
//         Token::IDENTIFIER(vec!['b']),
//         Token::SEMICOLON,
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
//     let ast = result.expect("Failed to parse");

//     // Construct the expected AST for the expression `a && b;`
//     let mut binary_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("a".to_string())));
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("&&".to_string())));
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("b".to_string())));

//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
//     top_level_expr.add_child(binary_expression_node);

//     let expected_ast: AST = AST::new(top_level_expr);

//     println!("{:#?}", ast);
//     println!("{:#?}", expected_ast);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

// /// This tests a simple comparison expression using equality.
// #[test]
// fn test_equality_expression() {
//     let tokens: Vec<Token> = vec![
//         Token::IDENTIFIER(vec!['x']),
//         Token::EQUALEQUAL,
//         Token::NUMBER(vec!['1', '0']),
//         Token::SEMICOLON,
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
//     let ast = result.expect("Failed to parse");

//     // Construct the expected AST for the expression `x == 10;`
//     let mut binary_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("==".to_string())));
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Literal("10".to_string())));

//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
//     top_level_expr.add_child(binary_expression_node);

//     let expected_ast: AST = AST::new(top_level_expr);

//     println!("{:#?}", ast);
//     println!("{:#?}", expected_ast);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

// /// This tests a complex boolean expression using logical OR and comparison.
// #[test]
// fn test_logical_or_comparison_expression() {
//     let tokens: Vec<Token> = vec![
//         Token::IDENTIFIER(vec!['a']),
//         Token::GREATERTHAN,
//         Token::NUMBER(vec!['5']),
//         Token::BARBAR,
//         Token::IDENTIFIER(vec!['b']),
//         Token::LESSTHAN,
//         Token::NUMBER(vec!['1', '0']),
//         Token::SEMICOLON,
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
//     let ast = result.expect("Failed to parse");

//     // Construct the expected AST for the expression `a > 5 || b < 10;`
//     let mut or_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
    
//     let mut greater_than_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     greater_than_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("a".to_string())));
//     greater_than_expression_node.add_child(ASTNode::new(SyntaxElement::Operator(">".to_string())));
//     greater_than_expression_node.add_child(ASTNode::new(SyntaxElement::Literal("5".to_string())));

//     let mut less_than_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     less_than_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("b".to_string())));
//     less_than_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("<".to_string())));
//     less_than_expression_node.add_child(ASTNode::new(SyntaxElement::Literal("10".to_string())));

//     or_expression_node.add_child(greater_than_expression_node);
//     or_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("||".to_string())));
//     or_expression_node.add_child(less_than_expression_node);

//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
//     top_level_expr.add_child(or_expression_node);

//     let expected_ast: AST = AST::new(top_level_expr);

//     println!("{:#?}", ast);
//     println!("{:#?}", expected_ast);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

// /// This tests a comparison expression with logical NOT.
// #[test]
// fn test_logical_not_comparison_expression() {
//     let tokens: Vec<Token> = vec![
//         Token::EXCLAMATIONPOINT,
//         Token::IDENTIFIER(vec!['x']),
//         Token::NOTEQUAL,
//         Token::NUMBER(vec!['0']),
//         Token::SEMICOLON,
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
//     let ast = result.expect("Failed to parse");

//     // Construct the expected AST for the expression `!x != 0;`
//     let mut not_expression_node = ASTNode::new(SyntaxElement::UnaryExpression);
//     not_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("!".to_string())));
    
//     let mut not_equal_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     not_equal_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));
//     not_equal_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("!=".to_string())));
//     not_equal_expression_node.add_child(ASTNode::new(SyntaxElement::Literal("0".to_string())));

//     not_expression_node.add_child(not_equal_expression_node);

//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
//     top_level_expr.add_child(not_expression_node);

//     let expected_ast: AST = AST::new(top_level_expr);

//     println!("{:#?}", ast);
//     println!("{:#?}", expected_ast);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

// /// --- Bitwise Operator Section --- ///

// /// This tests a simple bitwise AND expression.
// #[test]
// fn test_bitwise_and_expression() {
//     let tokens: Vec<Token> = vec![
//         Token::IDENTIFIER(vec!['a']),
//         Token::AMPERSAND,
//         Token::IDENTIFIER(vec!['b']),
//         Token::SEMICOLON,
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
//     let ast = result.expect("Failed to parse");

//     // Construct the expected AST for the expression `a & b;`
//     let mut binary_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("a".to_string())));
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("&".to_string())));
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("b".to_string())));

//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
//     top_level_expr.add_child(binary_expression_node);

//     let expected_ast: AST = AST::new(top_level_expr);

//     println!("{:#?}", ast);
//     println!("{:#?}", expected_ast);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

// /// This tests a simple bitwise OR expression.
// #[test]
// fn test_bitwise_or_expression() {
//     let tokens: Vec<Token> = vec![
//         Token::IDENTIFIER(vec!['x']),
//         Token::BAR,
//         Token::IDENTIFIER(vec!['y']),
//         Token::SEMICOLON,
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
//     let ast = result.expect("Failed to parse");

//     // Construct the expected AST for the expression `x | y;`
//     let mut binary_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("|".to_string())));
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("y".to_string())));

//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
//     top_level_expr.add_child(binary_expression_node);

//     let expected_ast: AST = AST::new(top_level_expr);

//     println!("{:#?}", ast);
//     println!("{:#?}", expected_ast);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

// /// This tests a simple bitwise XOR expression.
// #[test]
// fn test_bitwise_xor_expression() {
//     let tokens: Vec<Token> = vec![
//         Token::IDENTIFIER(vec!['a']),
//         Token::CARET,
//         Token::IDENTIFIER(vec!['b']),
//         Token::SEMICOLON,
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
//     let ast = result.expect("Failed to parse");

//     // Construct the expected AST for the expression `a ^ b;`
//     let mut binary_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("a".to_string())));
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("^".to_string())));
//     binary_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("b".to_string())));

//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
//     top_level_expr.add_child(binary_expression_node);

//     let expected_ast: AST = AST::new(top_level_expr);

//     println!("{:#?}", ast);
//     println!("{:#?}", expected_ast);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

// /// This tests a bitwise NOT expression.
// #[test]
// fn test_bitwise_not_expression() {
//     let tokens: Vec<Token> = vec![
//         Token::TILDE,
//         Token::IDENTIFIER(vec!['x']),
//         Token::SEMICOLON,
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
//     let ast = result.expect("Failed to parse");

//     // Construct the expected AST for the expression `~x;`
//     let mut unary_expression_node = ASTNode::new(SyntaxElement::UnaryExpression);
//     unary_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("~".to_string())));
//     unary_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));

//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
//     top_level_expr.add_child(unary_expression_node);

//     let expected_ast: AST = AST::new(top_level_expr);

//     println!("{:#?}", ast);
//     println!("{:#?}", expected_ast);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

// /// This tests a complex bitwise expression with multiple operators.
// #[test]
// fn test_complex_bitwise_expression() {
//     let tokens: Vec<Token> = vec![
//         Token::IDENTIFIER(vec!['a']),
//         Token::AMPERSAND,
//         Token::IDENTIFIER(vec!['b']),
//         Token::BAR,
//         Token::IDENTIFIER(vec!['c']),
//         Token::CARET,
//         Token::IDENTIFIER(vec!['d']),
//         Token::SEMICOLON,
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
//     let ast = result.expect("Failed to parse");

//     // Construct the expected AST for the expression `a & b | c ^ d;`
//     let mut xor_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     xor_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("c".to_string())));
//     xor_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("^".to_string())));
//     xor_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("d".to_string())));

//     let mut or_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     or_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("b".to_string())));
//     or_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("|".to_string())));
//     or_expression_node.add_child(xor_expression_node);

//     let mut and_expression_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     and_expression_node.add_child(ASTNode::new(SyntaxElement::Identifier("a".to_string())));
//     and_expression_node.add_child(ASTNode::new(SyntaxElement::Operator("&".to_string())));
//     and_expression_node.add_child(or_expression_node);

//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
//     top_level_expr.add_child(and_expression_node);

//     let expected_ast: AST = AST::new(top_level_expr);

//     println!("{:#?}", ast);
//     println!("{:#?}", expected_ast);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }