use common::{
    ast::{
        ast_struct::ASTNode, data_type::DataType, syntax_element::SyntaxElement
    }, 
    error::ErrorType
};

use crate::symbol_table_struct::{SymbolInfo, SymbolTable, SymbolTableStack, SymbolValue};

impl SymbolTableStack {
    /// Adds a new variable into the current scope
    pub fn sym_table_init(&mut self, init_node: &ASTNode) -> Result<(), Vec<ErrorType>> {

    }    

}