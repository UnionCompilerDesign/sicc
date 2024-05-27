use std::sync::{Arc, Mutex};

use safe_llvm::memory_management::resource_pools::{ ResourcePools, Tag, ValueTag};

use common::{ast::{ast_struct::ASTNode, syntax_element::SyntaxElement}, error::ErrorType};

use crate::core::IRGenerator;

impl IRGenerator {
    /// Generates LLVM IR for an assignment
    pub fn generate_assignment_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
        
    }

    /// Generates LLVM IR for an initialization
    pub fn generate_initialization_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }
    

    /// Generates LLVM IR for a break statement.
    pub fn generate_break_ir(&mut self, _node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        let resource_pools: Arc<Mutex<ResourcePools>> = self.get_resource_pools();
        let block_tag_targets = self.get_break_continue_target().expect("No current break target!");
        let block_tag = block_tag_targets.get(0).expect("break tag not found!").clone();
        let mut resource_pools = resource_pools.try_lock().expect("Failed to lock resource pool mutex in break IR!");
        resource_pools.create_br(self.get_builder(), block_tag);
        Ok(None)
    }

    /// Generates LLVM IR for a continue statement.
    pub fn generate_continue_ir(&mut self, _node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        let resource_pools: Arc<Mutex<ResourcePools>> = self.get_resource_pools();
        let block_tag_targets = self.get_break_continue_target().expect("No current continue target!");
        let block_tag = block_tag_targets.last().expect("Continue tag not found!").clone();
        let mut resource_pools = resource_pools.try_lock().expect("Failed to lock resource pool mutex in continue IR!");
        resource_pools.create_br(self.get_builder(), block_tag);
        Ok(None)
    }

    /// Generates LLVM IR for a return statement.
    pub fn generate_return_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }
    
}