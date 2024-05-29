use std::sync::{Arc, Mutex};

use common::{
    ast::{
        ast_struct::ASTNode, data_type::DataType, syntax_element::SyntaxElement
    }, constants::{DEFAULT_DO_BODY_LABEL, DEFAULT_DO_CONDITION_LABEL, DEFAULT_DO_WHILE_END_LABEL, DEFAULT_ELSE_LABEL, DEFAULT_ENTRY_LABEL, DEFAULT_FOR_BODY_LABEL, DEFAULT_FOR_COND_LABEL, DEFAULT_FOR_END_LABEL, DEFAULT_FOR_INCREMENT_LABEL, DEFAULT_MERGE_LABEL, DEFAULT_THEN_LABEL, DEFAULT_WHILE_BODY_LABEL, DEFAULT_WHILE_COND_LABEL, DEFAULT_WHILE_END_LABEL}, error::ErrorType
};

use safe_llvm::memory_management::resource_pools::{BasicBlockTag, ResourcePools, Tag, TypeTag, ValueTag};

use symbol_table::symbol_table_struct::{SymbolTable, SymbolValue};

use crate::core::IRGenerator;

impl IRGenerator {
    /// Generates LLVM IR for a function declaration.
    pub fn generate_fn_declaration_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        if let SyntaxElement::FunctionDeclaration = node.get_element() {
            unimplemented!();
        } else {
            panic!("Unexpected node: {:?}", node);
        }
    }
    
    
    /// Generates LLVM IR for a block expression.
    pub fn generate_block_exp(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }

    /// Generates LLVM IR for a do while loop.
    pub fn generate_do_while_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
            unimplemented!();
    }
    /// Generates LLVM IR for a while loop.
    pub fn generate_while_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }
    
    
    /// Generates LLVM IR for a for loop.
    pub fn generate_for_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }

    /// Generates LLVM IR for an if statement
    pub fn generate_if_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }
} 