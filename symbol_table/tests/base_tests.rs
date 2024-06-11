//! Base Case Testing for STS
use common::ast::{
    ast_struct::ASTNode, data_type::DataType, syntax_element::SyntaxElement, ast_struct::AST
};
use symbol_table::core::{SymbolTable, SymbolTableStack};

/*
Complete the following tests as defined. Make sure to also create combination tests 
in combination_tests.rs, boundry case tests in edge_tests.rs, and error handling tests
in error_tests.rs. Make enough tests to be sure the STS is comprehensively tested. 
*/

fn setup_ast_node(element: SyntaxElement, children: Vec<ASTNode>) -> ASTNode {
    unimplemented!();
}

fn setup_symbol_table_stack() -> SymbolTableStack {
    unimplemented!();
}

fn print_stack_state(stack: &SymbolTableStack, msg: &str) {
    println!("{}:\n{}\n", msg, stack.to_string());
}


/// Write a test to assure that an STS with an empty function declaration functions properly. 
#[test]
fn test_empty_function() {
    unimplemented!();
}

/// Write a test to assure that an STS with an empty function declaration 
/// that includes parameters functions properly. 
#[test]
fn test_empty_function_with_parameter() {
    unimplemented!();
}

/// Write a test to assure an STS with an enum declaration without variants functions properly. 
#[test]
fn test_enum_declaration() {
    unimplemented!();
}

/// Write a test to assure an STS with an enum declaration with variants functions properly. 
#[test]
fn test_enum_with_variants() {
    unimplemented!();
}

/// Write a test to assure an STS with a for loop declaration functions properly. 
#[test]
fn test_for_loop() {
    unimplemented!();
}

/// Write a test to assure an STS with a while loop declaration functions properly. 
#[test]
fn test_while_loop() {
    unimplemented!();
}

/// Write a test to assure an STS with a do-while loop declaration functions properly. 
#[test]
fn test_do_while_loop() {
    unimplemented!();
}

/// Write a test to assure an STS with an if-else block declaration functions properly. 
#[test]
fn test_if_else_statement() {
    unimplemented!();
}

/// Write a test to assure an STS with a switch statement with 3 cases functions properly. 
#[test]
fn test_switch_statement() {
    unimplemented!();
}

/// Write a test to assure an STS with a switch statement without cases functions properly. 
#[test]
fn test_switch_statement_cases() {
    unimplemented!();
}

/// Write a test to assure an STS with an itialized variables functions properly. 
#[test]
fn test_initialization() {
    unimplemented!();
}

/// Write a test to assure an STS with a binary expression functions properly. 
#[test]
fn test_binary_expression_initialization() {
    unimplemented!();
}

/// Write a test to assure an STS with a unary expression functions properly.
#[test]
fn test_unary_expression_initialization() {
    unimplemented!();
}


