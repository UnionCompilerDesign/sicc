//! This file hosts all of the functions necessary for generating LLVM IR
//! for "block" nodes, nodes that generate and manipulate basic blocks.

use common::{ast::ast_struct::ASTNode, error::ErrorType};
use safe_llvm::ir::core::Tag;
use crate::core::IRGenerator;

impl IRGenerator {
    /// Generates LLVM IR for a function declaration.
    /// 
    /// # Parameters
    ///
    /// - `node`: A reference to an `ASTNode` to generate IR for a function declaration.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Tag>, ErrorType>` containing the Tag of this function
    /// if generation went smoothly or an Error if there was a problem generating the function.
    ///
    /// # Errors
    ///
    /// - Returns an ErrorType if generation failed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// //let a_node: ASTNode = /* Some ASTNode we want to generate a function declaration from */
    /// //let result = self.generate_fn_declaration_ir(&a_node);
    /// /* check if type_result was Ok or Err, if Ok, it will contain the Tag that houses the 
    /// function's ValueTag. */
    /// ```
    pub fn generate_fn_declaration_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }
    
    
    /// Generates LLVM IR for a block expression.
    /// 
    /// # Parameters
    ///
    /// - `node`: A reference to an `ASTNode` to generate IR for a block expression.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Tag>, ErrorType>` containing None
    /// if generation went smoothly or an Error if there was a problem generating the block expression.
    ///
    /// # Errors
    ///
    /// - Returns an ErrorType if generation failed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// //let a_node: ASTNode = /* Some ASTNode we want to generate a block expression from */
    /// //let result = self.generate_block_exp(&a_node);
    /// /* check if type_result was Ok or Err, if Ok, it will contain None. */
    /// ```
    pub fn generate_block_exp(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }

    /// Generates LLVM IR for a do while loop.
    /// 
    /// # Parameters
    ///
    /// - `node`: A reference to an `ASTNode` to generate IR for a do while loop.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Tag>, ErrorType>` containing None
    /// if generation went smoothly or an Error if there was a problem generating the do while.
    ///
    /// # Errors
    ///
    /// - Returns an ErrorType if generation failed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// //let a_node: ASTNode = /* Some ASTNode we want to generate a do while loop from */
    /// //let result = self.generate_do_while_ir(&a_node);
    /// /* check if type_result was Ok or Err, if Ok, it will contain None. */
    /// ```
    pub fn generate_do_while_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }

    /// Generates LLVM IR for a while loop.
    /// 
    /// # Parameters
    ///
    /// - `node`: A reference to an `ASTNode` to generate IR for a while loop.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Tag>, ErrorType>` containing None
    /// if generation went smoothly or an Error if there was a problem generating the while.
    ///
    /// # Errors
    ///
    /// - Returns an ErrorType if generation failed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// //let a_node: ASTNode = /* Some ASTNode we want to generate a while loop from */
    /// //let result = self.generate_while_ir(&a_node);
    /// /* check if type_result was Ok or Err, if Ok, it will contain None. */
    /// ```
    pub fn generate_while_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }
    
    /// Generates LLVM IR for a for loop.
    /// 
    /// # Parameters
    ///
    /// - `node`: A reference to an `ASTNode` to generate IR for a for loop.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Tag>, ErrorType>` containing None
    /// if generation went smoothly or an Error if there was a problem generating the for loop.
    ///
    /// # Errors
    ///
    /// - Returns an ErrorType if generation failed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// //let a_node: ASTNode = /* Some ASTNode we want to generate a for loop from */
    /// //let result = self.generate_for_ir(&a_node);
    /// /* check if type_result was Ok or Err, if Ok, it will contain None. */
    /// ```
    pub fn generate_for_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }

    /// Generates LLVM IR for an if statement.
    /// 
    /// # Parameters
    ///
    /// - `node`: A reference to an `ASTNode` to generate IR for an if statement.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Tag>, ErrorType>` containing None
    /// if generation went smoothly or an Error if there was a problem generating the if statement.
    ///
    /// # Errors
    ///
    /// - Returns an ErrorType if generation failed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// //let a_node: ASTNode = /* Some ASTNode we want to generate an if statement from */
    /// //let result = self.generate_for_ir(&a_node);
    /// /* check if type_result was Ok or Err, if Ok, it will contain None. */
    /// ```
    pub fn generate_if_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }
} 