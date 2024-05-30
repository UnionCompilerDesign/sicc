use common::{
    ast::{
        ast_struct::{ASTNode, AST}, 
        syntax_element::SyntaxElement
    }, 
    error::ErrorType
};

use crate::symbol_table_struct::{SymbolTable, SymbolTableStack, SymbolInfo, SymbolValue};

impl SymbolTableStack {

    /// Drives the symbol table stack generation process returns back the original ast and the generated symbol table stack, else errors
    pub fn gen_sym_table_stack(ast: AST) -> Result<(AST, SymbolTableStack), Vec<ErrorType>> {

    }

    /// Routes the proper top level expression for an ASTNode
    pub fn sym_table_stack_router(&mut self, node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        
    }
}