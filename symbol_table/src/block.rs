//! This file contains symbol table stack generation functions for nodes that create
//! code blocks.

use common::{
    ast::{ast_struct::ASTNode, data_type::DataType, syntax_element::SyntaxElement},
    error::ErrorType,
};

use crate::core::{SymbolInfo, SymbolTable, SymbolValue, SymbolTableStack};

impl SymbolTableStack {
    /// Processes function declarations in the symbol table stack
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

    /// Processes struct declarations in the symbol table stack
    /// 
    /// # Parameters
    ///
    /// - `struct_node`: A reference to an `ASTNode` containing a struct definition.
    /// 
    /// # Returns
    ///
    /// Returns a `Result` with Ok indicating the process was successful or Err containing a 
    /// vector if errors if there were any.
    pub fn sym_table_struct(&mut self, struct_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        unimplemented!();
    }

    /// Processes enum declarations in the symbol table stack
    /// 
    /// # Parameters
    ///
    /// - `enum_node`: A reference to an `ASTNode` containing an enum definition.
    /// 
    /// # Returns
    ///
    /// Returns a `Result` with Ok indicating the process was successful or Err containing a 
    /// vector if errors if there were any.
    pub fn sym_table_enum(&mut self, enum_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        unimplemented!();
    }

    /// Processes for loop declarations in the symbol table stack
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

    /// Processes while loop declarations in the symbol table stack
    /// 
    /// # Parameters
    ///
    /// - `while_node`: A reference to an `ASTNode` containing a while loop definition.
    /// 
    /// # Returns
    ///
    /// Returns a `Result` with Ok indicating the process was successful or Err containing a 
    /// vector if errors if there were any.
    pub fn sym_table_while(&mut self, while_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        unimplemented!();
    }

    /// Processes do-while loop declarations in the symbol table stack
    /// 
    /// # Parameters
    ///
    /// - `do_while_node`: A reference to an `ASTNode` containing a do while loop definition.
    /// 
    /// # Returns
    ///
    /// Returns a `Result` with Ok indicating the process was successful or Err containing a 
    /// vector if errors if there were any.
    pub fn sym_table_do_while(&mut self, do_while_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        unimplemented!();
    }

    /// Processes switch statements in the symbol table stack
    /// 
    /// # Parameters
    ///
    /// - `switch_node`: A reference to an `ASTNode` containing a switch statement definition.
    /// 
    /// # Returns
    ///
    /// Returns a `Result` with Ok indicating the process was successful or Err containing a 
    /// vector if errors if there were any.
    pub fn sym_table_switch(&mut self, switch_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        unimplemented!();
    }

    /// Processes if-else statements in the symbol table stack
    /// 
    /// # Parameters
    ///
    /// - `if_else_node`: A reference to an `ASTNode` containing an if else statement definition.
    /// 
    /// # Returns
    ///
    /// Returns a `Result` with Ok indicating the process was successful or Err containing a 
    /// vector if errors if there were any.
    pub fn sym_table_if_else(&mut self, if_else_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        unimplemented!();
}
