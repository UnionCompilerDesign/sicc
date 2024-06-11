use common::ast::{
    core::{ASTNode, AST}, data_type::DataType, node_type::NodeType
};
use lexer::token::Token;
use parser::core::Parser;

/// This test verifies the parser's ability to handle nested switch statements.
/// It checks that the parser correctly constructs the AST for a switch statement
/// that contains another switch statement within one of its cases.
#[test]
fn test_nested_switch_statements() {

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

    let mut inner_switch = ASTNode::new(NodeType::SwitchStatement);

    {
        let mut assignedval_node1 = ASTNode::new(NodeType::AssignedValue);
        assignedval_node1.add_child(ASTNode::new(NodeType::Literal("1".to_string())));

        let mut var_node1 = ASTNode::new(NodeType::Variable);
        var_node1.add_child(ASTNode::new(NodeType::Identifier("z".to_string())));
        var_node1.add_child(ASTNode::new(NodeType::Type(DataType::Integer)));

        let mut initialization_node1 = ASTNode::new(NodeType::Initialization);
        initialization_node1.add_child(var_node1);
        initialization_node1.add_child(assignedval_node1);

        let mut case1_block = ASTNode::new(NodeType::BlockExpression);
        case1_block.add_child(initialization_node1);

        let mut case1 = ASTNode::new(NodeType::Case);
        case1.add_child(ASTNode::new(NodeType::Literal("1".to_string())));
        case1.add_child(case1_block);
        
        let mut assignedval_node2 = ASTNode::new(NodeType::AssignedValue);
        assignedval_node2.add_child(ASTNode::new(NodeType::Literal("2".to_string())));
        
        let mut var_node2 = ASTNode::new(NodeType::Variable);
        var_node2.add_child(ASTNode::new(NodeType::Identifier("z".to_string())));
        var_node2.add_child(ASTNode::new(NodeType::Type(DataType::Integer)));

        let mut initialization_node2 = ASTNode::new(NodeType::Initialization);
        initialization_node2.add_child(var_node2);
        initialization_node2.add_child(assignedval_node2);

        let mut case2_block = ASTNode::new(NodeType::BlockExpression);
        case2_block.add_child(initialization_node2);

        let mut case2 = ASTNode::new(NodeType::Case);
        case2.add_child(ASTNode::new(NodeType::Literal("2".to_string())));
        case2.add_child(case2_block);

        let mut cases_block_node = ASTNode::new(NodeType::BlockExpression);
        cases_block_node.add_child(case1);
        cases_block_node.add_child(case2);

        let identifier_node = ASTNode::new(NodeType::Identifier("x".to_string()));

        inner_switch.add_child(identifier_node);
        inner_switch.add_child(cases_block_node);
    }

    let mut case1_block = ASTNode::new(NodeType::BlockExpression);
    case1_block.add_child(inner_switch);

    let mut case1 = ASTNode::new(NodeType::Case);
    case1.add_child(ASTNode::new(NodeType::Literal("1".to_string())));
    case1.add_child(case1_block);
        
    let mut assignedval_node2 = ASTNode::new(NodeType::AssignedValue);
    assignedval_node2.add_child(ASTNode::new(NodeType::Literal("3".to_string())));

    let mut var_node2 = ASTNode::new(NodeType::Variable);
    var_node2.add_child(ASTNode::new(NodeType::Identifier("z".to_string())));
    var_node2.add_child(ASTNode::new(NodeType::Type(DataType::Integer)));

    let mut initialization_node2 = ASTNode::new(NodeType::Initialization);
    initialization_node2.add_child(var_node2);
    initialization_node2.add_child(assignedval_node2);

    let mut case2_block = ASTNode::new(NodeType::BlockExpression);
    case2_block.add_child(initialization_node2);

    let mut case2 = ASTNode::new(NodeType::Case);
    case2.add_child(ASTNode::new(NodeType::Literal("2".to_string())));
    case2.add_child(case2_block);

    let mut cases_block_node = ASTNode::new(NodeType::BlockExpression);
    cases_block_node.add_child(case1);
    cases_block_node.add_child(case2);

    let identifier_node = ASTNode::new(NodeType::Identifier("y".to_string()));

    let mut switch_statement_node = ASTNode::new(NodeType::SwitchStatement);
    switch_statement_node.add_child(identifier_node);
    switch_statement_node.add_child(cases_block_node);

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(switch_statement_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// This test verifies the parser's handling of nested binary expressions with mixed operators.
/// Specifically, it checks that the parser correctly parses and constructs the AST for an expression
/// involving addition and multiplication with proper precedence and grouping.
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

    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");
    let ast = result.expect("Failed to parse");

    let mut multiplication_node = ASTNode::new(NodeType::BinaryExpression);
    multiplication_node.add_child(ASTNode::new(NodeType::Identifier("b".to_string())));
    multiplication_node.add_child(ASTNode::new(NodeType::Operator("*".to_string())));
    multiplication_node.add_child(ASTNode::new(NodeType::Identifier("c".to_string())));

    let mut addition_node = ASTNode::new(NodeType::BinaryExpression);
    addition_node.add_child(ASTNode::new(NodeType::Identifier("a".to_string())));
    addition_node.add_child(ASTNode::new(NodeType::Operator("+".to_string())));
    addition_node.add_child(multiplication_node);

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(addition_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// This test verifies the parser's ability to handle nested if statements.
/// It checks that the parser correctly constructs the AST for an outer if statement
/// that contains another if statement within its then branch.
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

    let mut outer_if_node: ASTNode = ASTNode::new(NodeType::IfStatement);

    let mut outer_condition_node: ASTNode = ASTNode::new(NodeType::Condition);
    let outer_inner_condition_node = ASTNode::new(NodeType::Identifier("x".to_string()));
    outer_condition_node.add_child(outer_inner_condition_node);

    let mut outer_then_branch_node: ASTNode = ASTNode::new(NodeType::BlockExpression);
    let mut inner_if_node: ASTNode = ASTNode::new(NodeType::IfStatement);

    let mut inner_condition_node: ASTNode = ASTNode::new(NodeType::Condition);
    let inner_inner_condition_node = ASTNode::new(NodeType::Identifier("y".to_string()));
    inner_condition_node.add_child(inner_inner_condition_node);

    let mut inner_then_branch_node: ASTNode = ASTNode::new(NodeType::BlockExpression);
    let mut return_node: ASTNode = ASTNode::new(NodeType::Return);
    let mut assigned_value_node: ASTNode = ASTNode::new(NodeType::AssignedValue);

    let return_value_node: ASTNode = ASTNode::new(NodeType::Identifier("z".to_string()));
    assigned_value_node.add_child(return_value_node);

    return_node.add_child(assigned_value_node);
    inner_then_branch_node.add_child(return_node);

    inner_if_node.add_child(inner_condition_node);
    inner_if_node.add_child(inner_then_branch_node);

    outer_then_branch_node.add_child(inner_if_node);

    outer_if_node.add_child(outer_condition_node);
    outer_if_node.add_child(outer_then_branch_node);

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(outer_if_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST for nested if statements.");
}

/// This test verifies the parser's handling of an if statement inside a while loop.
/// It checks that the parser correctly constructs the AST for a while loop that contains
/// an if statement within its body.
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

    let mut while_loop_node: ASTNode = ASTNode::new(NodeType::WhileLoop);

    let mut while_condition_node: ASTNode = ASTNode::new(NodeType::Condition);
    let while_condition_inner_node = ASTNode::new(NodeType::Identifier("x".to_string()));
    while_condition_node.add_child(while_condition_inner_node);

    let mut while_then_branch_node: ASTNode = ASTNode::new(NodeType::BlockExpression);
    let mut if_node: ASTNode = ASTNode::new(NodeType::IfStatement);

    let mut if_condition_node: ASTNode = ASTNode::new(NodeType::Condition);
    let if_condition_inner_node = ASTNode::new(NodeType::Identifier("y".to_string()));
    if_condition_node.add_child(if_condition_inner_node);

    let mut if_then_branch_node: ASTNode = ASTNode::new(NodeType::BlockExpression);
    let mut return_node: ASTNode = ASTNode::new(NodeType::Return);
    let mut assigned_value_node: ASTNode = ASTNode::new(NodeType::AssignedValue);

    let return_value_node: ASTNode = ASTNode::new(NodeType::Identifier("z".to_string()));
    assigned_value_node.add_child(return_value_node);

    return_node.add_child(assigned_value_node);
    if_then_branch_node.add_child(return_node);

    if_node.add_child(if_condition_node);
    if_node.add_child(if_then_branch_node);

    while_then_branch_node.add_child(if_node);

    while_loop_node.add_child(while_condition_node);
    while_loop_node.add_child(while_then_branch_node);

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(while_loop_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST for if statements inside loops.");
}

/// This test verifies the parser's ability to handle nested if-else statements.
/// It checks that the parser correctly constructs the AST for an outer if-else statement
/// that contains another if-else statement within its then branch.
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

    let mut outer_if_node: ASTNode = ASTNode::new(NodeType::IfStatement);

    let mut outer_condition_node: ASTNode = ASTNode::new(NodeType::Condition);
    let outer_inner_condition_node = ASTNode::new(NodeType::Identifier("a".to_string()));
    outer_condition_node.add_child(outer_inner_condition_node);

    let mut outer_then_branch_node: ASTNode = ASTNode::new(NodeType::BlockExpression);
    let mut inner_if_node: ASTNode = ASTNode::new(NodeType::IfStatement);

    let mut inner_condition_node: ASTNode = ASTNode::new(NodeType::Condition);
    let inner_inner_condition_node = ASTNode::new(NodeType::Identifier("b".to_string()));
    inner_condition_node.add_child(inner_inner_condition_node);

    let mut inner_then_branch_node: ASTNode = ASTNode::new(NodeType::BlockExpression);
    let mut return_node_c: ASTNode = ASTNode::new(NodeType::Return);
    let mut assigned_value_node_c: ASTNode = ASTNode::new(NodeType::AssignedValue);

    let return_value_node_c: ASTNode = ASTNode::new(NodeType::Identifier("c".to_string()));
    assigned_value_node_c.add_child(return_value_node_c);

    return_node_c.add_child(assigned_value_node_c);
    inner_then_branch_node.add_child(return_node_c);

    let mut inner_else_branch_node: ASTNode = ASTNode::new(NodeType::BlockExpression);
    let mut return_node_d: ASTNode = ASTNode::new(NodeType::Return);
    let mut assigned_value_node_d: ASTNode = ASTNode::new(NodeType::AssignedValue);

    let return_value_node_d: ASTNode = ASTNode::new(NodeType::Identifier("d".to_string()));
    assigned_value_node_d.add_child(return_value_node_d);

    return_node_d.add_child(assigned_value_node_d);
    inner_else_branch_node.add_child(return_node_d);

    inner_if_node.add_child(inner_condition_node);
    inner_if_node.add_child(inner_then_branch_node);
    inner_if_node.add_child(inner_else_branch_node);

    outer_then_branch_node.add_child(inner_if_node);

    outer_if_node.add_child(outer_condition_node);
    outer_if_node.add_child(outer_then_branch_node);

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(outer_if_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST for nested if-else statements.");
}

/// This test verifies the parser's handling of a function with nested if statements.
/// It checks that the parser correctly constructs the AST for a function that contains
/// nested if statements within its body.
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

    let mut function_node: ASTNode = ASTNode::new(NodeType::FunctionDeclaration);

    let function_name_node = ASTNode::new(NodeType::Identifier("foo".to_string()));
    function_node.add_child(function_name_node);
    let return_type_node: ASTNode = ASTNode::new(NodeType::Type(DataType::Void));
    function_node.add_child(return_type_node);

    let mut function_body_node: ASTNode = ASTNode::new(NodeType::BlockExpression);
    let mut outer_if_node: ASTNode = ASTNode::new(NodeType::IfStatement);

    let mut outer_condition_node: ASTNode = ASTNode::new(NodeType::Condition);
    let outer_inner_condition_node = ASTNode::new(NodeType::Identifier("a".to_string()));
    outer_condition_node.add_child(outer_inner_condition_node);

    let mut outer_then_branch_node: ASTNode = ASTNode::new(NodeType::BlockExpression);
    let mut inner_if_node: ASTNode = ASTNode::new(NodeType::IfStatement);

    let mut inner_condition_node: ASTNode = ASTNode::new(NodeType::Condition);
    let inner_inner_condition_node = ASTNode::new(NodeType::Identifier("b".to_string()));
    inner_condition_node.add_child(inner_inner_condition_node);

    let mut inner_then_branch_node: ASTNode = ASTNode::new(NodeType::BlockExpression);
    let mut return_node: ASTNode = ASTNode::new(NodeType::Return);
    let mut assigned_value_node: ASTNode = ASTNode::new(NodeType::AssignedValue);

    let return_value_node: ASTNode = ASTNode::new(NodeType::Identifier("c".to_string()));
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

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(function_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST for a function with nested if statements.");
}

/// This test verifies the parser's handling of a function that contains nested loops.
/// It checks that the parser correctly constructs the AST for a function with a for loop
/// that contains a while loop within its body.
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

    let mut function_node: ASTNode = ASTNode::new(NodeType::FunctionDeclaration);

    let function_name_node = ASTNode::new(NodeType::Identifier("foo".to_string()));
    function_node.add_child(function_name_node);
    let return_type_node: ASTNode = ASTNode::new(NodeType::Type(DataType::Void));
    function_node.add_child(return_type_node);

    let mut function_body_node: ASTNode = ASTNode::new(NodeType::BlockExpression);
    let mut for_loop_node: ASTNode = ASTNode::new(NodeType::ForLoop);

    let mut initializer_node: ASTNode = ASTNode::new(NodeType::LoopInitializer);
    let mut initialization_node: ASTNode = ASTNode::new(NodeType::Assignment);
    let variable_node = ASTNode::new(NodeType::Identifier("i".to_string()));
    let value_node = ASTNode::new(NodeType::Literal("0".to_string()));
    initialization_node.add_child(variable_node);
    initialization_node.add_child(value_node);
    initializer_node.add_child(initialization_node);

    let mut condition_node: ASTNode = ASTNode::new(NodeType::Condition);
    let mut condition_expression_node: ASTNode = ASTNode::new(NodeType::BinaryExpression);
    let left_operand_node = ASTNode::new(NodeType::Identifier("i".to_string()));
    let operator_node = ASTNode::new(NodeType::Operator("<".to_string()));
    let right_operand_node = ASTNode::new(NodeType::Literal("1".to_string()));
    condition_expression_node.add_child(left_operand_node);
    condition_expression_node.add_child(operator_node);
    condition_expression_node.add_child(right_operand_node);
    condition_node.add_child(condition_expression_node);

    let mut increment_node: ASTNode = ASTNode::new(NodeType::LoopIncrement);
    let mut assignment_node: ASTNode = ASTNode::new(NodeType::Assignment);
    let variable_node = ASTNode::new(NodeType::Identifier("i".to_string()));
    let mut increment_expression_node: ASTNode = ASTNode::new(NodeType::BinaryExpression);
    let left_operand_node = ASTNode::new(NodeType::Identifier("i".to_string()));
    let operator_node = ASTNode::new(NodeType::Operator("+".to_string()));
    let right_operand_node = ASTNode::new(NodeType::Literal("1".to_string()));
    increment_expression_node.add_child(left_operand_node);
    increment_expression_node.add_child(operator_node);
    increment_expression_node.add_child(right_operand_node);
    assignment_node.add_child(variable_node);
    assignment_node.add_child(increment_expression_node);
    increment_node.add_child(assignment_node);

    let mut then_branch_node: ASTNode = ASTNode::new(NodeType::BlockExpression);
    let mut while_loop_node: ASTNode = ASTNode::new(NodeType::WhileLoop);

    let mut while_condition_node: ASTNode = ASTNode::new(NodeType::Condition);
    let while_condition_inner_node = ASTNode::new(NodeType::Identifier("j".to_string()));
    while_condition_node.add_child(while_condition_inner_node);

    let mut while_then_branch_node: ASTNode = ASTNode::new(NodeType::BlockExpression);
    let mut return_node: ASTNode = ASTNode::new(NodeType::Return);
    let mut assigned_value_node: ASTNode = ASTNode::new(NodeType::AssignedValue);

    let return_value_node: ASTNode = ASTNode::new(NodeType::Identifier("k".to_string()));
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

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(function_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST for a function with C-like for loop.");
}

/// Tests the abillity to construct a nested block expression. 
///
/// Tokens represent the statement '{{}};'.
#[test]
fn test_nested_block_expression_parsing() {
    let tokens: Vec<Token> = vec![
        Token::LBRACKET,
        Token::LBRACKET,
        Token::RBRACKET,
        Token::RBRACKET,
    ];

    let result = Parser::parse(tokens);
    assert!(result.is_ok(), "Parser should successfully parse the expression without errors.");

    let ast = result.expect("Failed to parse");

    let mut block_exp_1 = ASTNode::new(NodeType::BlockExpression);

    block_exp_1.add_child(ASTNode::new(NodeType::BlockExpression));

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);

    top_level_expr.add_child(block_exp_1);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// Test the parsing of nested block expressions.
/// 
/// The input source code includes nested blocks `{ { } { } }`.
#[test]
fn test_nested_block_expressions_parsing2() {
    let tokens: Vec<Token> = vec![
        Token::LBRACKET,
        Token::LBRACKET,
        Token::RBRACKET,
        Token::LBRACKET,
        Token::RBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let inner_block1 = ASTNode::new(NodeType::BlockExpression);

    let inner_block2 = ASTNode::new(NodeType::BlockExpression);

    let mut outer_block = ASTNode::new(NodeType::BlockExpression);
    outer_block.add_child(inner_block1);
    outer_block.add_child(inner_block2);

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(outer_block);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// Test the parsing of deeply nested block expressions.
/// 
/// The input source code includes deeply nested blocks `{ { { { } } } }`.
#[test]
fn test_deeply_nested_block_expressions_parsing() {
    let tokens: Vec<Token> = vec![
        Token::LBRACKET,
        Token::LBRACKET,
        Token::LBRACKET,
        Token::LBRACKET,
        Token::RBRACKET,
        Token::RBRACKET,
        Token::RBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let inner_most_block = ASTNode::new(NodeType::BlockExpression);
    
    let mut level3_block = ASTNode::new(NodeType::BlockExpression);
    level3_block.add_child(inner_most_block);
    
    let mut level2_block = ASTNode::new(NodeType::BlockExpression);
    level2_block.add_child(level3_block);
    
    let mut level1_block = ASTNode::new(NodeType::BlockExpression);
    level1_block.add_child(level2_block);

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(level1_block);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// Test the parsing of nested block expressions with statements inside.
/// 
/// The input source code includes nested blocks with variable declarations inside: 
/// `{ { int x = 1; } { int y = 2; } }`.
#[test]
fn test_nested_block_expressions_with_statements_parsing() {
    let tokens: Vec<Token> = vec![
        Token::LBRACKET,
        
        Token::LBRACKET,
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['1']),
        Token::SEMICOLON,
        Token::RBRACKET,
        
        Token::LBRACKET,
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['y']),
        Token::EQUAL,
        Token::NUMBER(vec!['2']),
        Token::SEMICOLON,
        Token::RBRACKET,
        
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut assignedval_node1 = ASTNode::new(NodeType::AssignedValue);
    assignedval_node1.add_child(ASTNode::new(NodeType::Literal("1".to_string())));

    let mut var_node1 = ASTNode::new(NodeType::Variable);
    var_node1.add_child(ASTNode::new(NodeType::Identifier("x".to_string())));
    var_node1.add_child(ASTNode::new(NodeType::Type(DataType::Integer)));

    let mut initialization_node1 = ASTNode::new(NodeType::Initialization);
    initialization_node1.add_child(var_node1);
    initialization_node1.add_child(assignedval_node1);

    let mut inner_block1 = ASTNode::new(NodeType::BlockExpression);
    inner_block1.add_child(initialization_node1);

    let mut assignedval_node2 = ASTNode::new(NodeType::AssignedValue);
    assignedval_node2.add_child(ASTNode::new(NodeType::Literal("2".to_string())));

    let mut var_node2 = ASTNode::new(NodeType::Variable);
    var_node2.add_child(ASTNode::new(NodeType::Identifier("y".to_string())));
    var_node2.add_child(ASTNode::new(NodeType::Type(DataType::Integer)));

    let mut initialization_node2 = ASTNode::new(NodeType::Initialization);
    initialization_node2.add_child(var_node2);
    initialization_node2.add_child(assignedval_node2);

    let mut inner_block2 = ASTNode::new(NodeType::BlockExpression);
    inner_block2.add_child(initialization_node2);

    let mut outer_block = ASTNode::new(NodeType::BlockExpression);
    outer_block.add_child(inner_block1);
    outer_block.add_child(inner_block2);

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(outer_block);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// Test the parsing of a switch statement with a nested if statement.
/// 
/// The input source code includes a switch statement that switches on a variable `z`.
/// The switch statement contains a case block (`case 1`) with an if statement nested inside it.
/// The if statement checks a variable `x` and contains a return statement.
#[test]
fn test_switch_with_nested_if_statement_parsing() {

    let tokens: Vec<Token> = vec![
        Token::SWITCH,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['z']),
        Token::RPAREN,
        Token::LBRACKET,
        
        Token::CASE,
        Token::NUMBER(vec!['1']),
        Token::COLON,
        
        Token::IF,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['x']),
        Token::RPAREN,
        Token::LBRACKET,
        Token::RETURN,
        Token::IDENTIFIER(vec!['x']),
        Token::SEMICOLON,
        Token::RBRACKET,
        
        Token::BREAK,
        Token::SEMICOLON,
        
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut condition_node = ASTNode::new(NodeType::Condition);
    let inner_condition_node = ASTNode::new(NodeType::Identifier("x".to_string()));
    condition_node.add_child(inner_condition_node);

    let mut then_branch_node = ASTNode::new(NodeType::BlockExpression);
    let mut return_node = ASTNode::new(NodeType::Return);
    let mut assigned_value_node = ASTNode::new(NodeType::AssignedValue);
    let return_value_node = ASTNode::new(NodeType::Identifier("x".to_string()));
    assigned_value_node.add_child(return_value_node);

    return_node.add_child(assigned_value_node);
    then_branch_node.add_child(return_node);

    let mut if_statement_node = ASTNode::new(NodeType::IfStatement);
    if_statement_node.add_child(condition_node);
    if_statement_node.add_child(then_branch_node);

    let mut case1_block = ASTNode::new(NodeType::BlockExpression);
    case1_block.add_child(if_statement_node);
    case1_block.add_child(ASTNode::new(NodeType::Break));

    let mut case1 = ASTNode::new(NodeType::Case);
    case1.add_child(ASTNode::new(NodeType::Literal("1".to_string())));
    case1.add_child(case1_block);

    let mut cases_block_node = ASTNode::new(NodeType::BlockExpression);
    cases_block_node.add_child(case1);

    let identifier_node = ASTNode::new(NodeType::Identifier("z".to_string()));

    let mut switch_statement_node = ASTNode::new(NodeType::SwitchStatement);
    switch_statement_node.add_child(identifier_node);
    switch_statement_node.add_child(cases_block_node);

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(switch_statement_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// Test the parsing of a switch statement with a nested if-else statement.
/// 
/// The input source code includes a switch statement that switches on a variable `a`.
/// The switch statement contains a case block (`case 1`) with an if-else statement nested inside it.
/// The if statement checks a variable `b` and contains a return statement in both the if and else branches.
#[test]
fn test_switch_with_nested_if_else_statement_parsing() {

    let tokens: Vec<Token> = vec![
        Token::SWITCH,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['a']),
        Token::RPAREN,
        Token::LBRACKET,
        
        Token::CASE,
        Token::NUMBER(vec!['1']),
        Token::COLON,
        
        Token::IF,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['b']),
        Token::RPAREN,
        Token::LBRACKET,
        Token::RETURN,
        Token::NUMBER(vec!['1']),
        Token::SEMICOLON,
        Token::RBRACKET,
        
        Token::ELSE,
        Token::LBRACKET,
        Token::RETURN,
        Token::NUMBER(vec!['0']),
        Token::SEMICOLON,
        Token::RBRACKET,
        
        Token::BREAK,
        Token::SEMICOLON,
        
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut condition_node = ASTNode::new(NodeType::Condition);
    let inner_condition_node = ASTNode::new(NodeType::Identifier("b".to_string()));
    condition_node.add_child(inner_condition_node);

    let mut then_branch_node = ASTNode::new(NodeType::BlockExpression);
    let mut return_node_if = ASTNode::new(NodeType::Return);
    let mut assigned_value_node_if = ASTNode::new(NodeType::AssignedValue);
    let return_value_node_if = ASTNode::new(NodeType::Literal("1".to_string()));
    assigned_value_node_if.add_child(return_value_node_if);

    return_node_if.add_child(assigned_value_node_if);
    then_branch_node.add_child(return_node_if);

    let mut else_branch_node = ASTNode::new(NodeType::BlockExpression);
    let mut return_node_else = ASTNode::new(NodeType::Return);
    let mut assigned_value_node_else = ASTNode::new(NodeType::AssignedValue);
    let return_value_node_else = ASTNode::new(NodeType::Literal("0".to_string()));
    assigned_value_node_else.add_child(return_value_node_else);

    return_node_else.add_child(assigned_value_node_else);
    else_branch_node.add_child(return_node_else);

    let mut if_statement_node = ASTNode::new(NodeType::IfStatement);
    if_statement_node.add_child(condition_node);
    if_statement_node.add_child(then_branch_node);
    if_statement_node.add_child(else_branch_node);

    let mut case1_block = ASTNode::new(NodeType::BlockExpression);
    case1_block.add_child(if_statement_node);
    case1_block.add_child(ASTNode::new(NodeType::Break));

    let mut case1 = ASTNode::new(NodeType::Case);
    case1.add_child(ASTNode::new(NodeType::Literal("1".to_string())));
    case1.add_child(case1_block);

    let mut cases_block_node = ASTNode::new(NodeType::BlockExpression);
    cases_block_node.add_child(case1);

    let identifier_node = ASTNode::new(NodeType::Identifier("a".to_string()));

    let mut switch_statement_node = ASTNode::new(NodeType::SwitchStatement);
    switch_statement_node.add_child(identifier_node);
    switch_statement_node.add_child(cases_block_node);

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(switch_statement_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// Test the parsing of an if statement with a nested switch statement.
/// 
/// The input source code includes an if statement that checks a variable `x`.
/// The if statement contains a switch statement that switches on a variable `y`.
/// The switch statement contains one case block (`case 1`) and a default block.
#[test]
fn test_if_with_nested_switch_statement_parsing() {
    let tokens: Vec<Token> = vec![
        Token::IF,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['x']),
        Token::RPAREN,
        Token::LBRACKET,
        
        Token::SWITCH,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['y']),
        Token::RPAREN,
        Token::LBRACKET,
        
        Token::CASE,
        Token::NUMBER(vec!['1']),
        Token::COLON,
        Token::BREAK,
        Token::SEMICOLON,
        
        Token::DEFAULT,
        Token::COLON,
        Token::BREAK,
        Token::SEMICOLON,
        
        Token::RBRACKET,
        
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut case1_block = ASTNode::new(NodeType::BlockExpression);
    case1_block.add_child(ASTNode::new(NodeType::Break));

    let mut case1 = ASTNode::new(NodeType::Case);
    case1.add_child(ASTNode::new(NodeType::Literal("1".to_string())));
    case1.add_child(case1_block);

    let mut default_block = ASTNode::new(NodeType::BlockExpression);
    default_block.add_child(ASTNode::new(NodeType::Break));

    let mut default_case = ASTNode::new(NodeType::Default);
    default_case.add_child(default_block);

    let mut cases_block_node = ASTNode::new(NodeType::BlockExpression);
    cases_block_node.add_child(case1);
    cases_block_node.add_child(default_case);

    let identifier_node = ASTNode::new(NodeType::Identifier("y".to_string()));

    let mut switch_statement_node = ASTNode::new(NodeType::SwitchStatement);
    switch_statement_node.add_child(identifier_node);
    switch_statement_node.add_child(cases_block_node);

    let mut condition_node = ASTNode::new(NodeType::Condition);
    let inner_condition_node = ASTNode::new(NodeType::Identifier("x".to_string()));
    condition_node.add_child(inner_condition_node);

    let mut then_branch_node = ASTNode::new(NodeType::BlockExpression);
    then_branch_node.add_child(switch_statement_node);

    let mut if_statement_node = ASTNode::new(NodeType::IfStatement);
    if_statement_node.add_child(condition_node);
    if_statement_node.add_child(then_branch_node);

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(if_statement_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// Test the parsing of an if statement with a nested switch statement with multiple cases.
/// 
/// The input source code includes an if statement that checks a variable `a`.
/// The if statement contains a switch statement that switches on a variable `b`.
/// The switch statement contains two case blocks (`case 1` and `case 2`).
#[test]
fn test_if_with_nested_switch_with_multiple_cases_parsing() {
    let tokens: Vec<Token> = vec![
        Token::IF,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['a']),
        Token::RPAREN,
        Token::LBRACKET,
        
        Token::SWITCH,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['b']),
        Token::RPAREN,
        Token::LBRACKET,
        
        Token::CASE,
        Token::NUMBER(vec!['1']),
        Token::COLON,
        Token::BREAK,
        Token::SEMICOLON,
        
        Token::CASE,
        Token::NUMBER(vec!['2']),
        Token::COLON,
        Token::BREAK,
        Token::SEMICOLON,
        
        Token::RBRACKET,
        
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut case1_block = ASTNode::new(NodeType::BlockExpression);
    case1_block.add_child(ASTNode::new(NodeType::Break));

    let mut case1 = ASTNode::new(NodeType::Case);
    case1.add_child(ASTNode::new(NodeType::Literal("1".to_string())));
    case1.add_child(case1_block);

    let mut case2_block = ASTNode::new(NodeType::BlockExpression);
    case2_block.add_child(ASTNode::new(NodeType::Break));

    let mut case2 = ASTNode::new(NodeType::Case);
    case2.add_child(ASTNode::new(NodeType::Literal("2".to_string())));
    case2.add_child(case2_block);

    let mut cases_block_node = ASTNode::new(NodeType::BlockExpression);
    cases_block_node.add_child(case1);
    cases_block_node.add_child(case2);

    let identifier_node = ASTNode::new(NodeType::Identifier("b".to_string()));

    let mut switch_statement_node = ASTNode::new(NodeType::SwitchStatement);
    switch_statement_node.add_child(identifier_node);
    switch_statement_node.add_child(cases_block_node);

    let mut condition_node = ASTNode::new(NodeType::Condition);
    let inner_condition_node = ASTNode::new(NodeType::Identifier("a".to_string()));
    condition_node.add_child(inner_condition_node);

    let mut then_branch_node = ASTNode::new(NodeType::BlockExpression);
    then_branch_node.add_child(switch_statement_node);

    let mut if_statement_node = ASTNode::new(NodeType::IfStatement);
    if_statement_node.add_child(condition_node);
    if_statement_node.add_child(then_branch_node);

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(if_statement_node);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}

/// Test the parsing of mixed statements and nested block expressions.
/// 
/// The input source code includes nested blocks and variable declarations: 
/// `{ int a = 0; { int b = 1; { int c = 2; } } int d = 3; }`.
#[test]
fn test_mixed_statements_and_nested_blocks_parsing() {
    let tokens: Vec<Token> = vec![
        Token::LBRACKET,
        
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['a']),
        Token::EQUAL,
        Token::NUMBER(vec!['0']),
        Token::SEMICOLON,
        
        Token::LBRACKET,
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['b']),
        Token::EQUAL,
        Token::NUMBER(vec!['1']),
        Token::SEMICOLON,
        
        Token::LBRACKET,
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['c']),
        Token::EQUAL,
        Token::NUMBER(vec!['2']),
        Token::SEMICOLON,
        Token::RBRACKET,
        
        Token::RBRACKET,
        
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['d']),
        Token::EQUAL,
        Token::NUMBER(vec!['3']),
        Token::SEMICOLON,
        
        Token::RBRACKET,
        Token::EOF,
    ];

    let ast: AST = Parser::parse(tokens).expect("Failed to parse");

    let mut assignedval_node_a = ASTNode::new(NodeType::AssignedValue);
    assignedval_node_a.add_child(ASTNode::new(NodeType::Literal("0".to_string())));

    let mut var_node_a = ASTNode::new(NodeType::Variable);
    var_node_a.add_child(ASTNode::new(NodeType::Identifier("a".to_string())));
    var_node_a.add_child(ASTNode::new(NodeType::Type(DataType::Integer)));

    let mut initialization_node_a = ASTNode::new(NodeType::Initialization);
    initialization_node_a.add_child(var_node_a);
    initialization_node_a.add_child(assignedval_node_a);

    let mut assignedval_node_b = ASTNode::new(NodeType::AssignedValue);
    assignedval_node_b.add_child(ASTNode::new(NodeType::Literal("1".to_string())));

    let mut var_node_b = ASTNode::new(NodeType::Variable);
    var_node_b.add_child(ASTNode::new(NodeType::Identifier("b".to_string())));
    var_node_b.add_child(ASTNode::new(NodeType::Type(DataType::Integer)));

    let mut initialization_node_b = ASTNode::new(NodeType::Initialization);
    initialization_node_b.add_child(var_node_b);
    initialization_node_b.add_child(assignedval_node_b);

    let mut assignedval_node_c = ASTNode::new(NodeType::AssignedValue);
    assignedval_node_c.add_child(ASTNode::new(NodeType::Literal("2".to_string())));

    let mut var_node_c = ASTNode::new(NodeType::Variable);
    var_node_c.add_child(ASTNode::new(NodeType::Identifier("c".to_string())));
    var_node_c.add_child(ASTNode::new(NodeType::Type(DataType::Integer)));

    let mut initialization_node_c = ASTNode::new(NodeType::Initialization);
    initialization_node_c.add_child(var_node_c);
    initialization_node_c.add_child(assignedval_node_c);

    let mut inner_block2 = ASTNode::new(NodeType::BlockExpression);
    inner_block2.add_child(initialization_node_c);

    let mut inner_block1 = ASTNode::new(NodeType::BlockExpression);
    inner_block1.add_child(initialization_node_b);
    inner_block1.add_child(inner_block2);

    let mut assignedval_node_d = ASTNode::new(NodeType::AssignedValue);
    assignedval_node_d.add_child(ASTNode::new(NodeType::Literal("3".to_string())));

    let mut var_node_d = ASTNode::new(NodeType::Variable);
    var_node_d.add_child(ASTNode::new(NodeType::Identifier("d".to_string())));
    var_node_d.add_child(ASTNode::new(NodeType::Type(DataType::Integer)));

    let mut initialization_node_d = ASTNode::new(NodeType::Initialization);
    initialization_node_d.add_child(var_node_d);
    initialization_node_d.add_child(assignedval_node_d);

    let mut outer_block = ASTNode::new(NodeType::BlockExpression);
    outer_block.add_child(initialization_node_a);
    outer_block.add_child(inner_block1);
    outer_block.add_child(initialization_node_d);

    let mut top_level_expr = ASTNode::new(NodeType::TopLevelExpression);
    top_level_expr.add_child(outer_block);

    let expected_ast: AST = AST::new(top_level_expr);

    assert_eq!(ast, expected_ast, "The parsed AST does not match the expected AST.");
}