//! This file contains all of the definitions of a symbol table stack and symbol tables, 
//! and provides the implementations of these to drive the symbol table stack
//! generation process.

use std::{collections::{HashMap, HashSet}, fmt};
use common::{
    ast::{
        ast_struct::{ASTNode, AST}, 
        syntax_element::SyntaxElement, 
        data_type::DataType,
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
    /// Simple values that can be represented as a string
    StrValue(Box<str>),

    /// A calculated symbol value
    Node(ASTNode),

    /// Add additional SymbolValues below
    unimplemented!(); 
}

impl SymbolValue{

    pub fn to_string(&self) -> String {
        match self {
            SymbolValue::StrValue(val) => format!("StrValue(\"{}\")", val),
            SymbolValue::Node(node) => format!("Node({:?})", node),
            
            /// Add a similar to_string as the provided examples for each SymbolValue 
        }
    }
}

/// Information on a symbol in a symboltable
#[derive(Clone, Debug)]
pub struct SymbolInfo {
    data_type: DataType,
    value: SymbolValue,
}

impl SymbolInfo {
    /// Creates a new `SymbolInfo` instance with the given input paraneters.
    ///
    /// This initializer sets up a `SymbolInfo` to keep track of information about
    /// a particular symbol in a `SymbolTable` using the input data type and 
    /// symbol value. 
    ///
    /// # Parameters
    ///
    /// - `data_type`: A `DataType` representing the data type of the symbol.
    /// - `value`: A `SymbolValue` representing the symbol's value.
    ///
    /// # Returns
    ///
    /// Returns a new `SymbolInfo` instance incorporating the input parameters.
    pub fn new(data_type: DataType, value: SymbolValue) -> Self {
        unimplemented!();
    }

    /// Returns a clone of this `SymbolInfo`'s value
    ///
    /// # Returns
    ///
    /// - `SymbolValue` - The `SymbolInfo`'s value as a cloned value.
    ///
    pub fn get_value(&self) -> SymbolValue {
        unimplemented!();
    }

    /// Returns a clone of this `SymbolInfo`'s data type
    ///
    /// # Returns
    ///
    /// - `DataType` - The `SymbolInfo`'s data type as a cloned value.
    ///
    pub fn get_data_type(&self) -> DataType {
        unimplemented!();
    }

    pub fn to_string(&self) -> String {
        format!("{{ data_type: {:?}, value: {} }}", self.data_type, self.value.to_string())
    }
}


impl SymbolTable {
    /// Creates a new `SymbolTable` instance.
    ///
    /// This initializer sets up a `SymbolTable` to keep track of individual tables of 
    /// symbols inside a symbol table stack. 
    ///
    /// # Returns
    ///
    /// Returns a new `SymbolTable` instance with an empty table.
    pub fn new() -> Self {
        unimplemented!();
    }

    /// Adds a new symbol and its `SymbolInfo` to the table.
    ///
    /// # Parameters
    ///
    /// - `name`: The `String` name of the symbol being added.
    /// - `info`: A `SymbolInfo` instance representing the info associated with this symbol.
    ///
    pub fn add(&mut self, name: String, info: SymbolInfo) {
        unimplemented!();
    }

    /// Retrieves a value from the symbol table.
    /// 
    /// # Parameters
    ///
    /// - `name`: The `String` name of the symbol being retrieved.
    ///
    /// # Returns
    ///
    /// - `Option<&SymbolInfo>` - The `SymbolInfo` of a symbol if it exists, else None.
    ///
    pub fn get(&self, name: &str) -> Option<&SymbolInfo> {
        unimplemented!();
    }

    /// Checks if the symbol table is empty
    ///
    /// # Returns
    ///
    /// - `bool` - true if this table is empty, false otherwise.
    ///
    pub fn is_empty(&self) -> bool {
        unimplemented!();
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
    elements: Vec<SymbolTable>,
}

impl SymbolTableStack {
    /// Drives the symbol table stack generation process.
    ///
    /// # Parameters
    ///
    /// - `ast`: An `AST` instance to generate the symbol table stack for.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing an `AST` and `SymbolTableStack` pair if Ok and a vector
    /// of errors if any errors were encountered.
    ///
    pub fn gen_sym_table_stack(ast: AST) -> Result<(AST, SymbolTableStack), Vec<ErrorType>> {
        unimplemented!();
    }

    /// Updates a symbol in the stack with a new value
    /// 
    /// # Parameters
    ///
    /// - `identifier`: The `&str` name of the symbol being modified.
    /// - `new_value`: the `SymbolValue` value to set the symbol's value to
    ///
    /// # Returns
    ///
    /// Returns a `Result` with Ok indicating the process was successful and Err if there was
    /// no symbol with that name in the table.
    pub fn update_symbol(&mut self, identifier: &str, new_value: SymbolValue) -> Result<(), ErrorType> {
        unimplemented!();
    }

    /// Routes the generation of the SymbolTableStack based on the type of node encountered.
    /// 
    /// # Parameters
    ///
    /// - `node`: A reference to an `ASTNode` to use in generating the next iteration of the symbol table stack.
    /// 
    /// # Returns
    ///
    /// Returns a `Result<(), Vec<ErrorType>>` with Ok indicating the process was successful or
    /// Err containing a vector of errors encountered if any were encountered.
    ///
    /// # Errors
    ///
    /// - Returns a vector of errors if there was a problem during STS generation.
    ///
    pub fn sym_table_stack_router(&mut self, node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        unimplemented!();
    }


    /// Creates a new `SymbolTableStack` instance.
    ///
    /// This initializer sets up a `SymbolTableStack` with no tables in it
    /// yet. 
    ///
    /// # Returns
    ///
    /// Returns a new empty `SymbolTableStack` instance.
    pub fn new() -> Self {
        unimplemented!();
    }

    /// Pushes a new table onto the stack
    /// 
    /// # Parameters
    ///
    /// - `item`: A `SymbolTable` to be pushed onto the stack.
    ///
    pub fn push(&mut self, item: SymbolTable) {
        unimplemented!();
    }

    /// Pops the topmost table off the stack and returns it
    /// 
    /// # Returns
    ///
    /// Returns the topmost `SymbolTable` popped off the stack, or None if there was no
    /// topmost table.
    ///
    pub fn pop(&mut self) -> Option<SymbolTable> {
        unimplemented!();
    }

    /// Checks if there are no tables on the stack
    ///
    /// # Returns
    ///
    /// - `bool` - true if this STS is empty, false otherwise.
    ///
    pub fn is_empty(&self) -> bool {
        unimplemented!();
    }

    /// Retrieves the size of the stack
    /// 
    /// # Returns
    ///
    /// Returns a `usize` of how many `SymbolTable`s exist in the stack.
    ///
    pub fn size(&self) -> usize {
        unimplemented!();
    }

    /// Retreives a reference to all tables in the stack
    /// 
    /// # Returns
    ///
    /// Returns an immutable reference to the current stack.
    ///
    pub fn get_elements(&self) -> &Vec<SymbolTable> {
        unimplemented!();
    }

    /// Adds an element to the symbol table at the specified index (or global scope by default)
    /// 
    /// # Parameters
    ///
    /// - `name`: The `String` name of the symbol to be added.
    /// - `info`: The `SymbolInfo` to be attached to the name in the tables.
    /// - `scope_index`: Optional, either a `usize` index into the stack, or None to modify the global scope.
    /// 
    /// # Returns
    ///
    /// Returns a `Result<(), Vec<ErrorType>>` with Ok indicating the process was successful or
    /// Err containing an error encountered if any were encountered.
    pub fn add_element(&mut self, name: String, info: SymbolInfo, scope_index: Option<usize>) -> Result<(), ErrorType> {
        unimplemented!();
    }

    /// Returns the `SymbolTable` in the symbol table stack at a given index
    /// 
    /// # Parameters
    ///
    /// - `index`: The `usize` index of the table in the stack to retrieve.
    /// 
    /// # Returns
    ///
    /// Returns a `Result<SymbolTable, Vec<ErrorType>>` with Ok containing a clone of the desired 
    /// `SymbolTable` if the process was successful or Err containing an error encountered if 
    /// any were encountered.
    pub fn get_element(&self, index: usize) -> Result<SymbolTable, ErrorType> {
        unimplemented!();
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        
        for (i, table) in self.elements.iter().enumerate() {
            result.push_str(&format!("Table {}: {{\n", i + 1));
            let mut entries: Vec<_> = table.values.iter().collect();
            entries.sort_by_key(|(k, _)| *k); // Sort entries by key for consistent ordering
            
            for (key, value) in entries {
                result.push_str(&format!("  \"{}\": {}\n", key, value.to_string()));
            }
            result.push_str("}\n");
        }
        
        result
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