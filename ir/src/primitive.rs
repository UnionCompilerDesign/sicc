use std::sync::{Arc, Mutex};

use safe_llvm::memory_management::resource_pools::{ResourcePools, Tag, TypeTag};

use common::{
    ast::{
        ast_struct::ASTNode, data_type::DataType, syntax_element::SyntaxElement
    },
    error::ErrorType
};

use crate::core::IRGenerator;

impl IRGenerator {
    /// Generates LLVM IR for a data type.
    pub fn generate_data_type_ir(&mut self, data_type: &DataType) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }

    /// TODO Temporary function that needs to be removed
    pub fn map_data_type(&mut self, data_type: &DataType) -> Result<Option<TypeTag>, ErrorType> {
        unimplemented!();
    }

    /// Generates LLVM IR for a literal.
    pub fn generate_literal_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }
}