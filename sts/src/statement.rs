//! This file contains symbol table stack generation functions for nodes within
//! code blocks.

use common::{
    ast::core::ASTNode, 
    error::ErrorType
};
use crate::core::SymbolTableStack;

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
    pub fn sym_table_init(&mut self, node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        let _ = node;
        unimplemented!();
    }    

}
