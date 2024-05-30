use common::{
    ast::{ast_struct::ASTNode, data_type::DataType, syntax_element::SyntaxElement}, 
    error::ErrorType
};

use crate::symbol_table_struct::{SymbolInfo, SymbolTable, SymbolValue, SymbolTableStack};

impl SymbolTableStack {
    /// Adds a function type to the current scope
    pub fn sym_table_fn(&mut self, fn_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        
        // To start your going to need the current table
        let mut current_table: SymbolTable = match self.pop() { 
            Some(table) => table, 
            // Later on, make sure to replace panic!'s, like the one below with the proper error type
            None => panic!("No symbol table on the stack."), 
        };
    }


    /// Adds an enum type to the current scope
    pub fn sym_table_enum(&mut self, enum_node: &ASTNode) -> Result<(), Vec<ErrorType>> {  
        
    }


    /// Adds a struct type to the current scope
    pub fn sym_table_struct(&mut self, struct_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        
    }

    /// Adds a switch statement to the current scope
    pub fn sym_table_switch(&mut self, switch_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        
    }
    
    

    /// Adds an if-else block to the current scope
    pub fn sym_table_if_else(&mut self, if_else_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        
    }

    /// Adds a for loop to the current scope
    pub fn sym_table_for(&mut self, for_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        
    }

    /// Adds a while loop to the current scope
    pub fn sym_table_while(&mut self, while_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        
    }
    

    /// Adds a do-while loop to the current scope
    pub fn sym_table_do_while(&mut self, do_while_node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        
    }
}
