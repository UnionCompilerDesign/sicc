//! This file contains symbol table stack generation functions for nodes within
//! code blocks.

use common::{
    ast::{
        ast_struct::ASTNode, data_type::DataType, syntax_element::SyntaxElement
    }, 
    error::ErrorType
};
use crate::core::{SymbolInfo, SymbolTable, SymbolTableStack, SymbolValue};

impl SymbolTableStack {
    /// Processes an initialization node in the symbol table stack.
    /// 
    /// # Parameters
    ///
    /// - `init_node`: A reference to an `ASTNode` containing an initialization.
    /// 
    /// # Returns
    ///
    /// Returns a `Result` with Ok indicating the process was successful or Err containing a 
    /// vector if errors if there were any.
    pub fn sym_table_init(&mut self, init_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        unimplemented!();
    }    

}
