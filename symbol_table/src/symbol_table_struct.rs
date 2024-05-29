use std::{collections::{HashMap, HashSet}, fmt};

use common::{
    ast::{
        ast_struct::ASTNode, data_type::DataType,
    }, 
    error::ErrorType
};

/// Initialized values in a scope
#[derive(Clone)]
pub struct SymbolTable {
    values: HashMap<String, SymbolInfo>,
}

/// Types of symbol values in a symbol table
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SymbolValue {
    
}

/// Information on a symbol in a symboltable
#[derive(Clone, Debug)]
pub struct SymbolInfo {
    data_type: DataType,
    value: SymbolValue,
}

impl SymbolInfo {
    /// Creates a new symbol in a symbol table
    pub fn new(data_type: DataType, value: SymbolValue) -> Self {
        
    }

    /// Retrieves the symbol info's value
    pub fn get_value(&self) -> SymbolValue {
        
    }

    /// Retrieves the symbol info's data type
    pub fn get_data_type(&self) -> DataType {
        
    }
}

impl SymbolTable {
    /// Creates a new symbol table
    pub fn new() -> Self {
        
    }

    /// Adds a new symbol info to the table
    pub fn add(&mut self, name: String, info: SymbolInfo) {
        
    }

    /// Retrieves a value from the symbol table, else None
    pub fn get(&self, name: &str) -> Option<&SymbolInfo> {
        
    }

    /// Checks if the symbol table is empty
    pub fn is_empty(&self) -> bool {
        
    }
}


impl fmt::Debug for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_map();
        for (key, value) in &self.values {
            builder.entry(&key, &value);
        }
        builder.finish()
    }
}

/// A stack of symbol tables, used to represent different levels of scope
#[derive(Clone)]
pub struct SymbolTableStack {
    
}

impl SymbolTableStack {
    /// Creates a new symbol table stack
    pub fn new() -> Self {
        
    }

    /// Pushes a new table onto the stack
    pub fn push(&mut self, item: SymbolTable) {
        
    }

    /// Pops the topmost table off the stack
    pub fn pop(&mut self) -> Option<SymbolTable> {
        
    }


    /// Retreives all the tables off the stack
    pub fn get_elements(&self) -> &Vec<SymbolTable> {
        
    }

    /// Returns the element in the symbol table stack at a given index
    pub fn get_element(&self, index: usize) -> Result<SymbolTable, ErrorType> {
        
    }

    /// Converts the symbol table stack to a string representation
    pub fn to_string(&self) -> String {
        
    }

}

impl fmt::Display for SymbolTableStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for SymbolTableStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SymbolTableStack")
         .field("size", &self.elements.len())
         .field("tables", &self.elements.iter().map(|table| {
            format!("{:?}", table)
         }).collect::<Vec<String>>())
         .finish()
    }
}


impl PartialEq for SymbolTableStack {
    fn eq(&self, other: &Self) -> bool {
        if self.elements.len() != other.elements.len() {
            return false;
        }

        for (self_table, other_table) in self.elements.iter().zip(&other.elements) {

            let self_keys: HashSet<_> = self_table.values.keys().collect();
            let other_keys: HashSet<_> = other_table.values.keys().collect();

            if self_keys != other_keys {
                return false;
            }

            for key in self_keys {
                let current_symbol_info: &SymbolInfo = self_table.get(key).expect("No symbol info found in comparison of tables!");
                let current_other_symbol_info: &SymbolInfo = other_table.get(key).expect("No symbol info found in comparison of tables!");

                if current_symbol_info.get_value() != current_other_symbol_info.get_value() {
                    return false;
                }

                if current_symbol_info.get_data_type() != current_other_symbol_info.get_data_type() {
                    return false;
                }

            }
        }

        true
    }
}

impl Eq for SymbolTableStack {}