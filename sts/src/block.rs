//! This file contains symbol table stack generation functions for nodes that contain code blocks.

use common::{
    ast::core::ASTNode,
    error::ErrorType,
};
use crate::core::SymbolTableStack;

impl SymbolTableStack {
    /// Processes function declarations from an AST and pushes information to the STS.
    /// 
    /// # Parameters
    ///
    /// - `fn_node`: A reference to an `ASTNode` containing a function declaration.
    /// 
    /// # Returns
    ///
    /// Returns a `Result` with Ok indicating the process was successful or Err containing a 
    /// vector if errors if there were any.
    pub fn sym_table_fn(&mut self, node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        let _ = node;
        unimplemented!();
    }

    /// Processes for loop declarations from an AST and pushes a new stack for the for loop's condition.
    /// 
    /// # Parameters
    ///
    /// - `for_node`: A reference to an `ASTNode` containing a for loop definition.
    /// 
    /// # Returns
    ///
    /// Returns a `Result` with Ok indicating the process was successful or Err containing a 
    /// vector if errors if there were any.
    pub fn sym_table_for(&mut self, node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        let _ = node;
        unimplemented!();
    }

    /// Processes block expressions from an AST by pushing a new table onto the STS. 
    /// 
    /// # Parameters
    ///
    /// - `for_node`: A reference to an `ASTNode` containing a for loop definition.
    /// 
    /// # Returns
    ///
    /// Returns a `Result` with Ok indicating the process was successful or Err containing a 
    /// vector if errors if there were any.
    pub fn sym_table_block(&mut self, node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        let _ = node;
        unimplemented!();
    }
}
