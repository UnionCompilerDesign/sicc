//! This file contains the core definitions and functionalities of the SymbolTableStack (STS).

use std::{collections::{HashMap, HashSet}, fmt};
use common::{
    ast::{
        core::{ASTNode, AST}, 
        data_type::DataType,
    }, 
    error::ErrorType
};

/// Initialized values in a scope.
#[derive(Clone)]
pub struct SymbolTable {
    values: HashMap<String, SymbolInfo>,
}

/// Types of symbol values in a symbol table.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum SymbolValue {
    /// No expression
    #[default]
    NoAssociatedValue,

    /// An enum's value (variants)
    EnumValue { 
        /// Variants of the enum
        variants: Vec<String>,
    },

    /// A struct's value (fields)
    StructValue { 
        /// Fields of the struct
        fields: Vec<(String, DataType)>,
    },
    
    /// A function's value (params, return type)
    FunctionValue { 
        /// Parameters of the function
        parameters: Vec<(String, DataType)>, 

    },
}

/// Information on a symbol in a symboltable.
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
        Self {
            data_type,
            value,
        }
    }

    /// Returns a clone of this `SymbolInfo`'s value
    ///
    /// # Returns
    ///
    /// - `SymbolValue` - The `SymbolInfo`'s value as a cloned value.
    pub fn get_value(&self) -> SymbolValue {
        self.value.clone()
    }

    /// Returns a clone of this `SymbolInfo`'s data type
    ///
    /// # Returns
    ///
    /// - `DataType` - The `SymbolInfo`'s data type as a cloned value.
    pub fn get_data_type(&self) -> DataType {
        self.data_type.clone()
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
        SymbolTable {
            values: HashMap::new(),
        }
    }

    /// Adds a new symbol and its `SymbolInfo` to the table.
    ///
    /// # Parameters
    ///
    /// - `name`: The `String` name of the symbol being added.
    /// - `info`: A `SymbolInfo` instance representing the info associated with this symbol.
    ///
    pub fn add(&mut self, name: String, info: SymbolInfo) {
        self.values.insert(name, info);
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
        self.values.get(name)
    }

    /// Checks if the symbol table is empty
    ///
    /// # Returns
    ///
    /// - `bool` - true if this table is empty, false otherwise.
    ///
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
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

/// A stack of symbol tables, used to represent different levels of scopes for an AST's symbols.
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
    pub fn gen_sym_table_stack(ast: AST) -> Result<(AST, SymbolTableStack), Vec<ErrorType>> {
        let _ = ast;
        unimplemented!()
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
    /// Returns a vector of errors if there was a problem during STS generation.
    pub fn sym_table_stack_router(&mut self, node: &ASTNode) -> Result<(), Vec<ErrorType>> {
        let _ = node;
        unimplemented!()
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
        SymbolTableStack {
            elements: Vec::new(),
        }
    }

    /// Pushes a new table onto the stack.
    /// 
    /// # Parameters
    ///
    /// - `item`: A `SymbolTable` to be pushed onto the stack.
    ///
    pub fn push(&mut self, item: SymbolTable) {
        self.elements.push(item);
    }

    /// Retrieves the size of the stack.
    /// 
    /// # Returns
    ///
    /// Returns a `usize` of how many `SymbolTable`s exist in the stack.
    ///
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    /// Retreives a reference to all tables in the stack.
    /// 
    /// # Returns
    ///
    /// Returns an immutable reference to the current stack.
    ///
    pub fn get_elements(&self) -> &Vec<SymbolTable> {
        &self.elements
    }

    /// Adds an element to the symbol table at the specified index (or global scope by default).
    /// 
    /// # Parameters
    ///
    /// - `name`: The `String` name of the symbol to be added.
    /// - `info`: The `SymbolInfo` to be attached to the name in the tables.
    /// 
    /// # Returns
    ///
    /// Returns a `Result<(), Vec<ErrorType>>` with Ok indicating the process was successful or
    /// Err containing an error encountered if any were encountered.
    pub fn add_element(&mut self, name: String, info: SymbolInfo) -> Result<(), ErrorType> {
        let index = self.size() - 1;
        if let Some(table) = self.elements.get_mut(index) {
            table.add(name, info);
            Ok(())
        } else {
            Err(ErrorType::DevError { message: "Scope index out of range".to_string() })
        }
    }

    /// Returns the `SymbolTable` in the symbol table stack at a given index.
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
        if index < self.elements.len() {
            match self.elements.get(index) {
                Some(sts) => {
                    return Ok(sts.clone());
                },
                None => {
                    return Err(ErrorType::DevError{ message: "No element found".to_string()});
                }
            }
        } 
        panic!("Invalid index: {:} for size {:}", index, self.elements.len())
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