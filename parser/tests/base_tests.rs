//! This file contains basic tests for the parser, most often ensuring correct parsing of individual tokens.

use common::ast::{
    core::{ASTNode, AST}, data_type::DataType, node_type::NodeType
};
use lexer::token::Token;
use parser::core::Parser;

/// Tests that an empty input generates an AST with only a TopLevelExpression node.
#[test]
fn test_empty_input() { 
    let tokens: Vec<Token> = vec![];
    let ast = Parser::parse(tokens).expect("Failed to parse");
    assert_eq!(ast.get_root().get_node_type(), NodeType::TopLevelExpression);
    assert!(ast.get_root().get_children().is_empty());
}

/// Tests that an input with only an EOF token generates an AST with only a TopLevelExpression node.
#[test]
fn test_eof() { 
    let tokens: Vec<Token> = vec![Token::EOF];
    let ast = Parser::parse(tokens).expect("Failed to parse");
    assert_eq!(ast.get_root().get_node_type(), NodeType::TopLevelExpression);
    assert!(ast.get_root().get_children().is_empty());
}

/// Tests that a single number token yields a Literal syntax element in the AST.
#[test]
fn test_number_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::NUMBER(vec!['2', '3']),
    ];

    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    
    let ast = result.expect("Failed to parse");

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(ASTNode::new(NodeType::Literal("23".to_string())));
    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// Tests that a number token representing a floating point number yields a Literal syntax element in the AST.
#[test]
fn test_floating_point_number() {
    let tokens: Vec<Token> = vec![
        Token::NUMBER(vec!['2', '.', '3']),
    ];

    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    
    let ast = result.expect("Failed to parse");

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(ASTNode::new(NodeType::Literal("2.3".to_string())));
    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// Tests that an identifier token yields an Identifier syntax element in the AST.
#[test]
fn test_identifier_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['f', 'o', 'o']),
    ];

    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    
    let ast = result.expect("Failed to parse");

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(ASTNode::new(NodeType::Identifier("foo".to_string())));
    let expected_ast: AST = AST::new(top_level_expr);


    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// Tests that a break token yields a Break syntax element in the AST.
#[test]
fn test_break_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::BREAK,
        Token::SEMICOLON,
    ];

    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    
    let ast = result.expect("Failed to parse");

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(ASTNode::new(NodeType::Break));
    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// Tests that a continue token yields a Continue syntax element in the AST.
#[test]
fn test_continue_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::CONTINUE,
        Token::SEMICOLON,
    ];

    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    
    let ast = result.expect("Failed to parse");

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(ASTNode::new(NodeType::Continue));
    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// Tests that a semicolon token generates an AST with only a TopLevelExpression node.
#[test]
fn test_semicolon_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::SEMICOLON,
    ];

    let ast = Parser::parse(tokens).expect("Failed to parse");
    assert_eq!(ast.get_root().get_node_type(), NodeType::TopLevelExpression);
    assert!(ast.get_root().get_children().is_empty());
}

/// Tests that the presence of an operator creates an operator syntax element.
#[test]
fn test_operator_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['x']),
        Token::LESSTHAN,
        Token::IDENTIFIER(vec!['y']),
        Token::SEMICOLON,
    ];

    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    
    let ast = result.expect("Failed to parse");

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    
    let mut binary_expr_node = ASTNode::new(NodeType::BinaryExpression);
    let identifier_node_a: ASTNode = ASTNode::new(NodeType::Identifier("x".to_string()));
    let identifier_node_b: ASTNode = ASTNode::new(NodeType::Identifier("y".to_string()));

    binary_expr_node.add_child(identifier_node_a);
    binary_expr_node.add_child(ASTNode::new(NodeType::Operator("<".to_string())));
    binary_expr_node.add_child(identifier_node_b);

    top_level_expr.add_child(binary_expr_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// Tests that the presence of a type annotation creates a type syntax element.
#[test]
fn test_data_type_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['y']),
        Token::SEMICOLON,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut initialization_node: ASTNode = ASTNode::new(NodeType::Initialization);

    let var_id_node: ASTNode = ASTNode::new(NodeType::Identifier("y".to_string()));
    let type_node: ASTNode = ASTNode::new(NodeType::Type(DataType::Integer));

    let mut variable_node: ASTNode = ASTNode::new(NodeType::Variable);
    variable_node.add_child(var_id_node);
    variable_node.add_child(type_node);

    initialization_node.add_child(variable_node);

    let mut top_level_expr: ASTNode = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(initialization_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast);
}

/// This test checks the parser's ability to create an IfStatement syntax element
/// as the child of the TopLevelExpression node.
#[test]
fn test_if_statement_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::IF,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('x')),
        Token::RPAREN,
        Token::LBRACKET,
        Token::RETURN,
        Token::IDENTIFIER(vec!('y')),
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let if_syntax_element = ast.get_root().get_children()[0].get_node_type();

    assert_eq!(if_syntax_element, NodeType::IfStatement, "No IfStatement syntax element was found as a child of the TopLevelExpression node.");
}

/// This test checks the parser's ability to create a ForLoop syntax element
/// as the child of the TopLevelExpression node.
#[test]
fn test_for_loop_syntax_element() {
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

    let for_syntax_element = ast.get_root().get_children()[0].get_node_type();

    assert_eq!(for_syntax_element, NodeType::ForLoop, "No ForLoop syntax element was found as a child of the TopLevelExpression node.");
}

/// This test checks the parser's ability to create a WhileLoop syntax element
/// as the child of the TopLevelExpression node.
#[test]
fn test_while_loop_syntax_element() {
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

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let while_syntax_element = ast.get_root().get_children()[0].get_node_type();

    assert_eq!(while_syntax_element, NodeType::WhileLoop, "No WhileLoop syntax element was found as a child of the TopLevelExpression node.");
}

/// This test checks the parser's ability to create a DoWhileLoop syntax element
/// as the child of the TopLevelExpression node.
#[test]
fn test_do_while_loop_syntax_element() {
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

    let do_while_syntax_element = ast.get_root().get_children()[0].get_node_type();

    assert_eq!(do_while_syntax_element, NodeType::DoWhileLoop, "No DoWhileLoop syntax element was found as a child of the TopLevelExpression node.");
}

/// This test checks the parser's ability to create a Return syntax element.
#[test]
fn test_return_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::RETURN,
        Token::IDENTIFIER(vec!('x')),
        Token::SEMICOLON,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let return_syntax_element = ast.get_root().get_children()[0].get_node_type();

    assert_eq!(return_syntax_element, NodeType::Return, "No Return syntax element was found as a child of the TopLevelExpression node.");
}

/// This test checks the parser's ability to create a SwitchStatement syntax element
/// as the child of the TopLevelExpression node.
#[test]
fn test_switch_statement_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::SWITCH,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['y']),
        Token::RPAREN,
        Token::LBRACKET,        
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let switch_syntax_element = ast.get_root().get_children()[0].get_node_type();

    assert_eq!(switch_syntax_element, NodeType::SwitchStatement, "No SwitchStatement syntax element was found as a child of the TopLevelExpression node.");
}

/// This test checks the parser's ability to create a Case syntax element
/// as the child of the SwitchStatement AST node.
#[test]
fn test_case_syntax_element() {
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
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let case_syntax_element = ast.get_root().get_children()[0].get_children()[1].get_children()[0].get_node_type();

    assert_eq!(case_syntax_element, NodeType::Case, "No Case syntax element was found as a child of the SwitchStatement AST node.");
}

/// This test checks the parser's ability to create a Default syntax element
/// as the child of the SwitchStatement AST node.
#[test]
fn test_default_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::SWITCH,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['y']),
        Token::RPAREN,
        Token::LBRACKET, 
        Token::DEFAULT,
        Token::COLON,
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['8']),
        Token::SEMICOLON,     
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let default_syntax_element = ast.get_root().get_children()[0].get_children()[1].get_children()[0].get_node_type();

    assert_eq!(default_syntax_element, NodeType::Default, "No Default syntax element was found as a child of the SwitchStatement AST node.");
}

/// This test checks the parser's ability to create an Assignment syntax element
/// as the child of the TopLevelExpression AST node.
#[test]
fn test_assignment_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['y']),
        Token::EQUAL,
        Token::NUMBER(vec!['8']),
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let assignment_syntax_element = ast.get_root().get_children()[0].get_node_type();

    assert_eq!(assignment_syntax_element, NodeType::Assignment, "No Assignment syntax element was found as a child of the TopLevelExpression AST node.");
}

/// This test checks the parser's ability to create an Initialization syntax element
/// as the child of the TopLevelExpression AST node.
#[test]
fn test_initialization_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['y']),
        Token::EQUAL,
        Token::NUMBER(vec!['8']),
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let initialization_syntax_element = ast.get_root().get_children()[0].get_node_type();

    assert_eq!(initialization_syntax_element, NodeType::Initialization, "No Initialization syntax element was found as a child of the TopLevelExpression AST node.");
}

/// This test checks the parser's ability to create a FunctionDeclaration syntax element
/// as the child of the TopLevelExpression AST node.
#[test]
fn test_function_declaration_syntax_element() {
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

    let func_syntax_element = ast.get_root().get_children()[0].get_node_type();

    assert_eq!(func_syntax_element, NodeType::FunctionDeclaration, "No FunctionDeclaration syntax element was found as a child of the TopLevelExpression AST node.");
}

/// This test checks the parser's ability to create a StructDeclaration syntax element
/// as the child of the TopLevelExpression AST node.
#[test]
fn test_struct_declaration_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::STRUCT,
        Token::IDENTIFIER(vec!['M', 'y', 'S', 't', 'r', 'u', 'c', 't']),
        Token::LBRACE,
        Token::RBRACE,
        Token::SEMICOLON,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let struct_syntax_element = ast.get_root().get_children()[0].get_node_type();

    assert_eq!(struct_syntax_element, NodeType::StructDeclaration, "No StructDeclaration syntax element was found as a child of the TopLevelExpression AST node.");
}

/// This test checks the parser's ability to create an EnumDeclaration syntax element
/// as the child of the TopLevelExpression AST node.
#[test]
fn test_enum_declaration_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::ENUM,
        Token::IDENTIFIER(vec!['M', 'y', 'E', 'n', 'u', 'm']),
        Token::LBRACE,
        Token::RBRACE,
        Token::SEMICOLON,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let enum_syntax_element = ast.get_root().get_children()[0].get_node_type();

    assert_eq!(enum_syntax_element, NodeType::EnumDeclaration, "No EnumDeclaration syntax element was found as a child of the TopLevelExpression AST node.");
}

/// This test checks the parser's ability to create a BlockExpression syntax element
/// as the child of the TopLevelExpression AST node.
#[test]
fn test_block_expression_declaration_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::LBRACKET,
        Token::RBRACKET,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let block_expression_syntax_element = ast.get_root().get_children()[0].get_node_type();

    assert_eq!(block_expression_syntax_element, NodeType::BlockExpression, "No BlockExpression syntax element was found as a child of the TopLevelExpression AST node.");
}

/// This test checks the parser's ability to create a Condition syntax element
/// as the child of an IfStatement AST node.
#[test]
fn test_condition_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::IF,
        Token::LPAREN,
        Token::IDENTIFIER(vec!('x')),
        Token::RPAREN,
        Token::LBRACKET,
        Token::RETURN,
        Token::IDENTIFIER(vec!('y')),
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let condition_syntax_element = ast.get_root().get_children()[0].get_children()[0].get_node_type();
    
    assert_eq!(condition_syntax_element, NodeType::Condition, "No Condition syntax element was found as a child of the IfStatement node.");
}
/// This test checks the parser's ability to create a Variant syntax element
/// as the child of the EnumDeclaration AST node.
#[test]
fn test_variant_syntax_element() {
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

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let variant_syntax_element = ast.get_root().get_children()[0].get_children()[1].get_node_type();

    assert_eq!(variant_syntax_element, NodeType::Variant, "No Variant syntax element was found as a child of the EnumDeclaration AST node.");
}

/// This test checks the parser's ability to create an AssignedValue syntax element
/// as the child of an Initialization AST node.
#[test]
fn test_assigned_value_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['y']),
        Token::EQUAL,
        Token::NUMBER(vec!['8']),
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let assigned_value_syntax_element = ast.get_root().get_children()[0].get_children()[1].get_node_type();

    assert_eq!(assigned_value_syntax_element, NodeType::AssignedValue, "No AssignedValue syntax element was found as a child of the Initialization AST node.");
}

/// This test checks the parser's ability to create a Field syntax element
/// as the child of the StructDeclaration AST node.
#[test]
fn test_field_syntax_element() {
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

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let struct_field_syntax_element = ast.get_root().get_children()[0].get_children()[1].get_node_type();

    assert_eq!(struct_field_syntax_element, NodeType::Field, "No Field syntax element was found as a child of the StructDeclaration AST node.");
}

/// This test checks the parser's ability to create a Parameter syntax element
/// as the child of a FunctionDeclaration AST node.
#[test]
fn test_parameter_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::TBOOLEAN,
        Token::IDENTIFIER(vec!['c', 'a', 'l', 'c', 'u', 'l', 'a', 't', 'e']),
        Token::LPAREN,
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['x']),
        Token::RPAREN,
        Token::LBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let parameter_syntax_element = ast.get_root().get_children()[0].get_children()[1].get_node_type();

    assert_eq!(parameter_syntax_element, NodeType::Parameter, "No Parameter syntax element was found as a child of the FunctionDeclaration AST node.");
}

/// This test checks the parser's ability to create a Variable syntax element
/// as the child of an Initialization AST node.
#[test]
fn test_variable_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['y']),
        Token::EQUAL,
        Token::NUMBER(vec!['8']),
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let variable_syntax_element = ast.get_root().get_children()[0].get_children()[0].get_node_type();

    assert_eq!(variable_syntax_element, NodeType::Variable, "No Variable syntax element was found as a child of the Initialization AST node.");
}

/// This tests checks the parser's abillity to create a BinaryExpression syntax element
/// as the child of the TopLevelExpression AST Node.
#[test]
fn test_bin_exp_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['x']),
        Token::LESSTHAN,
        Token::IDENTIFIER(vec!['y']),
        Token::SEMICOLON,
    ];
    
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let bin_exp_syntax_element = ast.get_root().get_children()[0].get_node_type();

    assert_eq!(bin_exp_syntax_element, NodeType::BinaryExpression, "No BinaryExpression syntax element was found as a child of the TopLevelExpression AST node.");
}

/// This tests checks the parser's abillity to create a UnaryExpression syntax element
/// as the child of the TopLevelExpression AST Node.
#[test]
fn test_unary_exp_syntax_element() {
    let tokens: Vec<Token> = vec![
        Token::DASH,
        Token::IDENTIFIER(vec!['y']),
        Token::SEMICOLON,
    ];
    
    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let unary_exp_syntax_element = ast.get_root().get_children()[0].get_node_type();

    assert_eq!(unary_exp_syntax_element, NodeType::UnaryExpression, "No UnaryExpression syntax element was found as a child of the TopLevelExpression AST node.");
}

/// This test checks the parser's ability to create a LoopInitializer and LoopIncrement syntax element
/// as the child of the ForLoop AST node.
#[test]
fn test_for_loop_elements() {
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

    let loop_init = ast.get_root().get_children()[0].get_children()[0].get_node_type();

    assert_eq!(loop_init, NodeType::LoopInitializer, "No LoopInitializer syntax element was found as a child of the ForLoop node.");

    let loop_increment = ast.get_root().get_children()[0].get_children()[2].get_node_type();

    assert_eq!(loop_increment, NodeType::LoopIncrement, "No LoopIncrement syntax element was found as a child of the ForLoop node.");
}