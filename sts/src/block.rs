//! This file contains symbol table stack generation functions for nodes that contain code blocks.

use common::{
    ast::{ast_struct::ASTNode, data_type::DataType, syntax_element::SyntaxElement},
    error::ErrorType,
};
use crate::core::{SymbolInfo, SymbolTable, SymbolValue, SymbolTableStack};

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
    pub fn sym_table_fn(&mut self, fn_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
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
    pub fn sym_table_for(&mut self, for_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
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
    pub fn sym_table_block(&mut self, _block_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        unimplemented!();
    }
}
