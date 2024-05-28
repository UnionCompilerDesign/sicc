use common::ast::{
    ast_struct::{ASTNode, AST}, data_type::DataType, syntax_element::SyntaxElement
};
use std::env;

use lexer::token::Token;
use parser::parser_core::Parser;

#[test]
fn test_nested_switch_statements() {

    /* 
    let input ="switch (y) {
                case 1:
                    switch(x) {
                        case 1:
                            let z: Integer = 1;
                        case 2:
                            let z: Integer = 2;
                    }
                case 2:
                    let z: Integer = 3;
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
        
        Token::SWITCH,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['x']),
        Token::RPAREN,
        Token::LBRACKET,
        
        Token::CASE,
        Token::NUMBER(vec!['1']),
        Token::COLON,
        
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['z']),
        Token::EQUAL,
        Token::NUMBER(vec!['1']),
        Token::SEMICOLON,
        
        Token::CASE,
        Token::NUMBER(vec!['2']),
        Token::COLON,
        
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['z']),
        Token::EQUAL,
        Token::NUMBER(vec!['2']),
        Token::SEMICOLON,
        
        Token::RBRACKET,
        
        Token::CASE,
        Token::NUMBER(vec!['2']),
        Token::COLON,
        
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['z']),
        Token::EQUAL,
        Token::NUMBER(vec!['3']),
        Token::SEMICOLON,
        
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut inner_switch = ASTNode::new(SyntaxElement::SwitchStatement);

    {
        // Inner Switch Case 1

        let mut assignedval_node1 = ASTNode::new(SyntaxElement::AssignedValue);
        assignedval_node1.add_child(ASTNode::new(SyntaxElement::Literal("1".to_string())));

        let mut var_node1 = ASTNode::new(SyntaxElement::Variable);
        var_node1.add_child(ASTNode::new(SyntaxElement::Identifier("z".to_string())));
        var_node1.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));

        let mut initialization_node1 = ASTNode::new(SyntaxElement::Initialization);
        initialization_node1.add_child(var_node1);
        initialization_node1.add_child(assignedval_node1);

        let mut case1_block = ASTNode::new(SyntaxElement::BlockExpression);
        case1_block.add_child(initialization_node1);

        let mut case1 = ASTNode::new(SyntaxElement::Case);
        case1.add_child(ASTNode::new(SyntaxElement::Literal("1".to_string())));
        case1.add_child(case1_block);
        
        // Inner Switch Case 2
        
        let mut assignedval_node2 = ASTNode::new(SyntaxElement::AssignedValue);
        assignedval_node2.add_child(ASTNode::new(SyntaxElement::Literal("2".to_string())));

        
        let mut var_node2 = ASTNode::new(SyntaxElement::Variable);
        var_node2.add_child(ASTNode::new(SyntaxElement::Identifier("z".to_string())));
        var_node2.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));

        let mut initialization_node2 = ASTNode::new(SyntaxElement::Initialization);
        initialization_node2.add_child(var_node2);
        initialization_node2.add_child(assignedval_node2);

        let mut case2_block = ASTNode::new(SyntaxElement::BlockExpression);
        case2_block.add_child(initialization_node2);

        let mut case2 = ASTNode::new(SyntaxElement::Case);
        case2.add_child(ASTNode::new(SyntaxElement::Literal("2".to_string())));
        case2.add_child(case2_block);

        // Synthesis of Inner switch

        let mut cases_block_node = ASTNode::new(SyntaxElement::BlockExpression);
        cases_block_node.add_child(case1);
        cases_block_node.add_child(case2);

        let identifier_node = ASTNode::new(SyntaxElement::Identifier("x".to_string()));

        inner_switch.add_child(identifier_node);
        inner_switch.add_child(cases_block_node);
    }

    // Outer Switch Case 1
    let mut case1_block = ASTNode::new(SyntaxElement::BlockExpression);
    case1_block.add_child(inner_switch);

    let mut case1 = ASTNode::new(SyntaxElement::Case);
    case1.add_child(ASTNode::new(SyntaxElement::Literal("1".to_string())));
    case1.add_child(case1_block);
    
    // Outer Switch Case 2
    
    let mut assignedval_node2 = ASTNode::new(SyntaxElement::AssignedValue);
    assignedval_node2.add_child(ASTNode::new(SyntaxElement::Literal("3".to_string())));

    let mut var_node2 = ASTNode::new(SyntaxElement::Variable);
    var_node2.add_child(ASTNode::new(SyntaxElement::Identifier("z".to_string())));
    var_node2.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));

    let mut initialization_node2 = ASTNode::new(SyntaxElement::Initialization);
    initialization_node2.add_child(var_node2);
    initialization_node2.add_child(assignedval_node2);

    let mut case2_block = ASTNode::new(SyntaxElement::BlockExpression);
    case2_block.add_child(initialization_node2);

    let mut case2 = ASTNode::new(SyntaxElement::Case);
    case2.add_child(ASTNode::new(SyntaxElement::Literal("2".to_string())));
    case2.add_child(case2_block);

    // Synthesis of outer switch

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

/// This tests a complex nested binary expression with mixed operators.
#[test]
fn test_nested_binary_expression() {
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['a']),
        Token::PLUS,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['b']),
        Token::ASTERISK,
        Token::IDENTIFIER(vec!['c']),
        Token::RPAREN,
        Token::SEMICOLON,
    ];

    // Parse the tokens into an AST
    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    let ast = result.expect("Failed to parse");

    // Construct the expected AST for the expression `a + (b * c);`
    let mut multiplication_node = ASTNode::new(SyntaxElement::BinaryExpression);
    multiplication_node.add_child(ASTNode::new(SyntaxElement::Identifier("b".to_string())));
    multiplication_node.add_child(ASTNode::new(SyntaxElement::Operator("*".to_string())));
    multiplication_node.add_child(ASTNode::new(SyntaxElement::Identifier("c".to_string())));

    let mut addition_node = ASTNode::new(SyntaxElement::BinaryExpression);
    addition_node.add_child(ASTNode::new(SyntaxElement::Identifier("a".to_string())));
    addition_node.add_child(ASTNode::new(SyntaxElement::Operator("+".to_string())));
    addition_node.add_child(multiplication_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(addition_node);

    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

// /// This tests a complex boolean expression with mixed operators.
// #[test]
// fn test_complex_boolean_expression() {
//     let tokens: Vec<Token> = vec![
//         Token::IDENTIFIER(vec!['x']),
//         Token::ANDAND,
//         Token::LPAREN,
//         Token::IDENTIFIER(vec!['y']),
//         Token::BARBAR,
//         Token::IDENTIFIER(vec!['z']),
//         Token::RPAREN,
//         Token::SEMICOLON,
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
//     let ast = result.expect("Failed to parse");

//     // Construct the expected AST for the expression `x && (y || z);`
//     let mut or_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     or_node.add_child(ASTNode::new(SyntaxElement::Identifier("y".to_string())));
//     or_node.add_child(ASTNode::new(SyntaxElement::Operator("||".to_string())));
//     or_node.add_child(ASTNode::new(SyntaxElement::Identifier("z".to_string())));

//     let mut and_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     and_node.add_child(ASTNode::new(SyntaxElement::Identifier("x".to_string())));
//     and_node.add_child(ASTNode::new(SyntaxElement::Operator("&&".to_string())));
//     and_node.add_child(or_node);

//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
//     top_level_expr.add_child(and_node);

//     let expected_ast: AST = AST::new(top_level_expr);

//     println!("{:#?}", ast);
//     println!("{:#?}", expected_ast);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

// /// This tests a nested bitwise expression with mixed operators.
// #[test]
// fn test_nested_bitwise_expression() {
//     let tokens: Vec<Token> = vec![
//         Token::IDENTIFIER(vec!['a']),
//         Token::AMPERSAND,
//         Token::LPAREN,
//         Token::IDENTIFIER(vec!['b']),
//         Token::CARET,
//         Token::IDENTIFIER(vec!['c']),
//         Token::RPAREN,
//         Token::SEMICOLON,
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
//     let ast = result.expect("Failed to parse");

//     // Construct the expected AST for the expression `a & (b ^ c);`
//     let mut xor_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     xor_node.add_child(ASTNode::new(SyntaxElement::Identifier("b".to_string())));
//     xor_node.add_child(ASTNode::new(SyntaxElement::Operator("^".to_string())));
//     xor_node.add_child(ASTNode::new(SyntaxElement::Identifier("c".to_string())));

//     let mut and_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     and_node.add_child(ASTNode::new(SyntaxElement::Identifier("a".to_string())));
//     and_node.add_child(ASTNode::new(SyntaxElement::Operator("&".to_string())));
//     and_node.add_child(xor_node);

//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
//     top_level_expr.add_child(and_node);

//     let expected_ast: AST = AST::new(top_level_expr);

//     println!("{:#?}", ast);
//     println!("{:#?}", expected_ast);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

// /// This tests a mixed bitwise and boolean expression.
// #[test]
// fn test_mixed_bitwise_boolean_expression() {
//     let tokens: Vec<Token> = vec![
//         Token::IDENTIFIER(vec!['a']),
//         Token::ANDAND,
//         Token::IDENTIFIER(vec!['b']),
//         Token::BAR,
//         Token::IDENTIFIER(vec!['c']),
//         Token::EQUALEQUAL,
//         Token::NUMBER(vec!['1']),
//         Token::SEMICOLON,
//     ];

//     // Parse the tokens into an AST
//     let result = Parser::parse(tokens);
//     assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
//     let ast = result.expect("Failed to parse");

//     // Construct the expected AST for the expression `a && b | c == 1;`
//     let mut equal_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     equal_node.add_child(ASTNode::new(SyntaxElement::Identifier("c".to_string())));
//     equal_node.add_child(ASTNode::new(SyntaxElement::Operator("==".to_string())));
//     equal_node.add_child(ASTNode::new(SyntaxElement::Literal("1".to_string())));

//     let mut or_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     or_node.add_child(ASTNode::new(SyntaxElement::Identifier("b".to_string())));
//     or_node.add_child(ASTNode::new(SyntaxElement::Operator("|".to_string())));
//     or_node.add_child(equal_node);

//     let mut and_node = ASTNode::new(SyntaxElement::BinaryExpression);
//     and_node.add_child(ASTNode::new(SyntaxElement::Identifier("a".to_string())));
//     and_node.add_child(ASTNode::new(SyntaxElement::Operator("&&".to_string())));
//     and_node.add_child(or_node);

//     let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
//     top_level_expr.add_child(and_node);

//     let expected_ast: AST = AST::new(top_level_expr);

//     println!("{:#?}", ast);
//     println!("{:#?}", expected_ast);

//     assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
// }

#[test]
fn test_nested_if_statements() {
    let tokens: Vec<Token> = vec![
        Token::IF,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('x')),
        Token::RPAREN,
        Token::LBRACKET,
        Token::IF,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('y')),
        Token::RPAREN,
        Token::LBRACKET,
        Token::RETURN,
        Token::IDENTIFIER(vec!('z')),
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut outer_if_node: ASTNode = ASTNode::new(SyntaxElement::IfStatement);

    let mut outer_condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let outer_inner_condition_node = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    outer_condition_node.add_child(outer_inner_condition_node);

    let mut outer_then_branch_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let mut inner_if_node: ASTNode = ASTNode::new(SyntaxElement::IfStatement);

    let mut inner_condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let inner_inner_condition_node = ASTNode::new(SyntaxElement::Identifier("y".to_string()));
    inner_condition_node.add_child(inner_inner_condition_node);

    let mut inner_then_branch_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let mut return_node: ASTNode = ASTNode::new(SyntaxElement::Return);
    let mut assigned_value_node: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);

    let return_value_node: ASTNode = ASTNode::new(SyntaxElement::Identifier("z".to_string()));
    assigned_value_node.add_child(return_value_node);

    return_node.add_child(assigned_value_node);
    inner_then_branch_node.add_child(return_node);

    inner_if_node.add_child(inner_condition_node);
    inner_if_node.add_child(inner_then_branch_node);

    outer_then_branch_node.add_child(inner_if_node);

    outer_if_node.add_child(outer_condition_node);
    outer_if_node.add_child(outer_then_branch_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(outer_if_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST for nested if statements.");
}

#[test]
fn test_if_inside_loop() {
    let tokens: Vec<Token> = vec![
        Token::WHILE,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('x')),
        Token::RPAREN,
        Token::LBRACKET,
        Token::IF,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('y')),
        Token::RPAREN,
        Token::LBRACKET,
        Token::RETURN,
        Token::IDENTIFIER(vec!('z')),
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut while_loop_node: ASTNode = ASTNode::new(SyntaxElement::WhileLoop);

    let mut while_condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let while_condition_inner_node = ASTNode::new(SyntaxElement::Identifier("x".to_string()));
    while_condition_node.add_child(while_condition_inner_node);

    let mut while_then_branch_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let mut if_node: ASTNode = ASTNode::new(SyntaxElement::IfStatement);

    let mut if_condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let if_condition_inner_node = ASTNode::new(SyntaxElement::Identifier("y".to_string()));
    if_condition_node.add_child(if_condition_inner_node);

    let mut if_then_branch_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let mut return_node: ASTNode = ASTNode::new(SyntaxElement::Return);
    let mut assigned_value_node: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);

    let return_value_node: ASTNode = ASTNode::new(SyntaxElement::Identifier("z".to_string()));
    assigned_value_node.add_child(return_value_node);

    return_node.add_child(assigned_value_node);
    if_then_branch_node.add_child(return_node);

    if_node.add_child(if_condition_node);
    if_node.add_child(if_then_branch_node);

    while_then_branch_node.add_child(if_node);

    while_loop_node.add_child(while_condition_node);
    while_loop_node.add_child(while_then_branch_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(while_loop_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST for if statements inside loops.");
}

#[test]
fn test_nested_if_else_statements() {
    let tokens: Vec<Token> = vec![
        Token::IF,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('a')),
        Token::RPAREN,
        Token::LBRACKET,
        Token::IF,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('b')),
        Token::RPAREN,
        Token::LBRACKET,
        Token::RETURN,
        Token::IDENTIFIER(vec!('c')),
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::ELSE,
        Token::LBRACKET,
        Token::RETURN,
        Token::IDENTIFIER(vec!('d')),
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut outer_if_node: ASTNode = ASTNode::new(SyntaxElement::IfStatement);

    let mut outer_condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let outer_inner_condition_node = ASTNode::new(SyntaxElement::Identifier("a".to_string()));
    outer_condition_node.add_child(outer_inner_condition_node);

    let mut outer_then_branch_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let mut inner_if_node: ASTNode = ASTNode::new(SyntaxElement::IfStatement);

    let mut inner_condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let inner_inner_condition_node = ASTNode::new(SyntaxElement::Identifier("b".to_string()));
    inner_condition_node.add_child(inner_inner_condition_node);

    let mut inner_then_branch_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let mut return_node_c: ASTNode = ASTNode::new(SyntaxElement::Return);
    let mut assigned_value_node_c: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);

    let return_value_node_c: ASTNode = ASTNode::new(SyntaxElement::Identifier("c".to_string()));
    assigned_value_node_c.add_child(return_value_node_c);

    return_node_c.add_child(assigned_value_node_c);
    inner_then_branch_node.add_child(return_node_c);

    let mut inner_else_branch_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let mut return_node_d: ASTNode = ASTNode::new(SyntaxElement::Return);
    let mut assigned_value_node_d: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);

    let return_value_node_d: ASTNode = ASTNode::new(SyntaxElement::Identifier("d".to_string()));
    assigned_value_node_d.add_child(return_value_node_d);

    return_node_d.add_child(assigned_value_node_d);
    inner_else_branch_node.add_child(return_node_d);

    inner_if_node.add_child(inner_condition_node);
    inner_if_node.add_child(inner_then_branch_node);
    inner_if_node.add_child(inner_else_branch_node);

    outer_then_branch_node.add_child(inner_if_node);

    outer_if_node.add_child(outer_condition_node);
    outer_if_node.add_child(outer_then_branch_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(outer_if_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST for nested if-else statements.");
}

#[test]
fn test_function_with_nested_if() {
    let tokens: Vec<Token> = vec![
        Token::TVOID,
        Token::IDENTIFIER(vec!('f', 'o', 'o')),
        Token::LPAREN,
        Token::RPAREN,
        Token::LBRACKET,
        Token::IF,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('a')),
        Token::RPAREN,
        Token::LBRACKET,
        Token::IF,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('b')),
        Token::RPAREN,
        Token::LBRACKET,
        Token::RETURN,
        Token::IDENTIFIER(vec!('c')),
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::RBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut function_node: ASTNode = ASTNode::new(SyntaxElement::FunctionDeclaration);

    let function_name_node = ASTNode::new(SyntaxElement::Identifier("foo".to_string()));
    function_node.add_child(function_name_node);
    let return_type_node: ASTNode = ASTNode::new(SyntaxElement::Type(DataType::Void));
    function_node.add_child(return_type_node);

    let mut function_body_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let mut outer_if_node: ASTNode = ASTNode::new(SyntaxElement::IfStatement);

    let mut outer_condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let outer_inner_condition_node = ASTNode::new(SyntaxElement::Identifier("a".to_string()));
    outer_condition_node.add_child(outer_inner_condition_node);

    let mut outer_then_branch_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let mut inner_if_node: ASTNode = ASTNode::new(SyntaxElement::IfStatement);

    let mut inner_condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let inner_inner_condition_node = ASTNode::new(SyntaxElement::Identifier("b".to_string()));
    inner_condition_node.add_child(inner_inner_condition_node);

    let mut inner_then_branch_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let mut return_node: ASTNode = ASTNode::new(SyntaxElement::Return);
    let mut assigned_value_node: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);

    let return_value_node: ASTNode = ASTNode::new(SyntaxElement::Identifier("c".to_string()));
    assigned_value_node.add_child(return_value_node);

    return_node.add_child(assigned_value_node);
    inner_then_branch_node.add_child(return_node);

    inner_if_node.add_child(inner_condition_node);
    inner_if_node.add_child(inner_then_branch_node);

    outer_then_branch_node.add_child(inner_if_node);

    outer_if_node.add_child(outer_condition_node);
    outer_if_node.add_child(outer_then_branch_node);

    function_body_node.add_child(outer_if_node);

    function_node.add_child(function_body_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(function_node);

    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST for a function with nested if statements.");
}

#[test]
fn test_function_with_loops() {
    let tokens: Vec<Token> = vec![
        Token::TVOID,
        Token::IDENTIFIER(vec!('f', 'o', 'o')),
        Token::LPAREN,
        Token::RPAREN,
        Token::LBRACKET,
        Token::FOR,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('i')),
        Token::EQUAL,
        Token::NUMBER(vec!('0')),
        Token::SEMICOLON,
        Token::IDENTIFIER(vec!('i')),
        Token::LESSTHAN,
        Token::NUMBER(vec!('1')),
        Token::SEMICOLON,
        Token::IDENTIFIER(vec!('i')),
        Token::EQUAL,
        Token::IDENTIFIER(vec!('i')),
        Token::PLUS,
        Token::NUMBER(vec!('1')),
        Token::RPAREN,
        Token::LBRACKET,
        Token::WHILE,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('j')),
        Token::RPAREN,
        Token::LBRACKET,
        Token::RETURN,
        Token::IDENTIFIER(vec!('k')),
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::RBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut function_node: ASTNode = ASTNode::new(SyntaxElement::FunctionDeclaration);

    let function_name_node = ASTNode::new(SyntaxElement::Identifier("foo".to_string()));
    function_node.add_child(function_name_node);
    let return_type_node: ASTNode = ASTNode::new(SyntaxElement::Type(DataType::Void));
    function_node.add_child(return_type_node);

    let mut function_body_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let mut for_loop_node: ASTNode = ASTNode::new(SyntaxElement::ForLoop);

    let mut initializer_node: ASTNode = ASTNode::new(SyntaxElement::LoopInitializer);
    let mut initialization_node: ASTNode = ASTNode::new(SyntaxElement::Assignment);
    let variable_node = ASTNode::new(SyntaxElement::Identifier("i".to_string()));
    let value_node = ASTNode::new(SyntaxElement::Literal("0".to_string()));
    initialization_node.add_child(variable_node);
    initialization_node.add_child(value_node);
    initializer_node.add_child(initialization_node);

    let mut condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let mut condition_expression_node: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);
    let left_operand_node = ASTNode::new(SyntaxElement::Identifier("i".to_string()));
    let operator_node = ASTNode::new(SyntaxElement::Operator("<".to_string()));
    let right_operand_node = ASTNode::new(SyntaxElement::Literal("1".to_string()));
    condition_expression_node.add_child(left_operand_node);
    condition_expression_node.add_child(operator_node);
    condition_expression_node.add_child(right_operand_node);
    condition_node.add_child(condition_expression_node);

    let mut increment_node: ASTNode = ASTNode::new(SyntaxElement::LoopIncrement);
    let mut assignment_node: ASTNode = ASTNode::new(SyntaxElement::Assignment);
    let variable_node = ASTNode::new(SyntaxElement::Identifier("i".to_string()));
    let mut increment_expression_node: ASTNode = ASTNode::new(SyntaxElement::BinaryExpression);
    let left_operand_node = ASTNode::new(SyntaxElement::Identifier("i".to_string()));
    let operator_node = ASTNode::new(SyntaxElement::Operator("+".to_string()));
    let right_operand_node = ASTNode::new(SyntaxElement::Literal("1".to_string()));
    increment_expression_node.add_child(left_operand_node);
    increment_expression_node.add_child(operator_node);
    increment_expression_node.add_child(right_operand_node);
    assignment_node.add_child(variable_node);
    assignment_node.add_child(increment_expression_node);
    increment_node.add_child(assignment_node);

    let mut then_branch_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let mut while_loop_node: ASTNode = ASTNode::new(SyntaxElement::WhileLoop);

    let mut while_condition_node: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let while_condition_inner_node = ASTNode::new(SyntaxElement::Identifier("j".to_string()));
    while_condition_node.add_child(while_condition_inner_node);

    let mut while_then_branch_node: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let mut return_node: ASTNode = ASTNode::new(SyntaxElement::Return);
    let mut assigned_value_node: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);

    let return_value_node: ASTNode = ASTNode::new(SyntaxElement::Identifier("k".to_string()));
    assigned_value_node.add_child(return_value_node);

    return_node.add_child(assigned_value_node);
    while_then_branch_node.add_child(return_node);

    while_loop_node.add_child(while_condition_node);
    while_loop_node.add_child(while_then_branch_node);

    then_branch_node.add_child(while_loop_node);

    for_loop_node.add_child(initializer_node);
    for_loop_node.add_child(condition_node);
    for_loop_node.add_child(increment_node);
    for_loop_node.add_child(then_branch_node);

    function_body_node.add_child(for_loop_node);

    function_node.add_child(function_body_node);

    let mut top_level_expr = ASTNode::new(SyntaxElement::TopLevelExpression);
    top_level_expr.add_child(function_node);

    let expected_ast: AST = AST::new(top_level_expr);

    println!("{:#?}", ast);
    println!("{:#?}", expected_ast);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST for a function with C-like for loop.");
}

