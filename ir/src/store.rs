//! This defines the store, which is used to keep track of the allocation tags
//! of variables in the current and outer scopes.
//! Look around if you like, but don't do anything here. All of these
//! functions have wrappers in `IRGenerator` that you should use to access the store,
//! please do not attempt to access it directly.

use std::collections::HashMap;
use common::error::ErrorType;
use safe_llvm::memory_management::resource_pools::ValueTag;

pub struct Store {
    current_table_id: Option<usize>,
    table_stack: Vec<HashMap<String, ValueTag>>
}

impl Store {
    /// creates a new store
    pub fn new() -> Self {
        Self {
            current_table_id: None,
            table_stack: Vec::new()
        }
    }

    /// creates a new table in the store
    pub fn push_table(&mut self){
        self.table_stack.push(HashMap::new());
        if self.current_table_id.is_none() {
            self.current_table_id = Some(0);
        }
        else {
            self.current_table_id = usize::checked_add(self.current_table_id.unwrap(), 1);
        }
    }

    fn get_current_id(&self) -> Option<usize> {
        self.current_table_id.clone()
    }

    /// Deletes the current table from the store 
    pub fn delete_table(&mut self) -> Result<(), ErrorType> {
        let cur_id = self.get_current_id();

        if cur_id.is_none() {
            return Err(ErrorType::DevError { message: "No tables to delete!".to_string() })
        }
        else {
            self.table_stack.pop();
            self.current_table_id = usize::checked_sub(self.current_table_id.unwrap(), 1);
            return Ok(())
        }
    }

    fn search_tables_for_var(&self, name: String) -> Option<usize> {
        let mut current_id: Option<usize> = self.get_current_id();

        while current_id.is_some() {
            let keys = self.table_stack.get(current_id.unwrap()).unwrap();

            if keys.contains_key(&name) {
                return current_id
            }

            current_id = usize::checked_sub(current_id.unwrap(), 1);
        }

        None
    }

    /// Initializes a tag in the current store table, Error if name is already defined
    pub fn add_tag_to_top_table(&mut self, var_name: String, alloca_tag: ValueTag) -> Result<(), ErrorType> {
        let id_of_found: Option<usize> = self.search_tables_for_var(var_name.clone());
        let table_id = self.get_current_id();
        if table_id.is_none() {
            return Err(ErrorType::DevError { message: "No tables to set tag in!".to_string() })
        }

        if id_of_found.is_some() {
            return Err(ErrorType::DevError { message: "Allocation tag already exists in table!".to_string() })
        }

        if !self.table_stack.is_empty() {
            let cur_table: &mut HashMap<String, ValueTag> = self.table_stack.get_mut(table_id.unwrap()).unwrap();
            cur_table.insert(var_name ,alloca_tag);
            
        }
        else{
            return Err(ErrorType::DevError { message: "No tables in the store to modify!".to_string() })
        }

        Ok(())
    }

    /// Searches for a variable in the table and outer tables, Error if it's not found
    pub fn search_for_var(&self, var_name: String) -> Result<ValueTag, ErrorType> {
        let id_of_found: Option<usize> = self.search_tables_for_var(var_name.clone());

        if id_of_found.is_none() {
            return Err(ErrorType::DevError { message: "Variable not found in table!".to_string() })
        }

        if !self.table_stack.is_empty() {
            let cur_table: &HashMap<String, ValueTag> = self.table_stack.get(id_of_found.unwrap()).unwrap();
            return Ok(cur_table.get(&var_name).unwrap().clone())
        }
        else{
            return Err(ErrorType::DevError { message: "No tables in the store to modify!".to_string() })
        }
    }
}