//! Edge Case Testing for STS
use common::ast::{
    ast_struct::ASTNode, data_type::DataType, syntax_element::SyntaxElement, ast_struct::AST
};
use symbol_table::core::{SymbolTable, SymbolTableStack};

/*
Thoroughly test the functionality of generating SymbolTableStack's for different boundary scenarios for
different SymbolElements. 
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