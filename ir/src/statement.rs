//! This file hosts all of the functions necessary for generating LLVM IR
//! for statements that do not evaluate to values such as break, 
//! continue, and assignment.

use common::{ast::core::ASTNode, error::ErrorType};
use safe_llvm::ir::core::Tag;
use crate::core::IRGenerator;

impl IRGenerator {
    /// Generates LLVM IR for an assignment
    /// 
    /// # Parameters
    ///
    /// - `node`: A reference to an `ASTNode` to generate IR for an assignment.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Tag>, ErrorType>` containing None if generation went smoothly or
    /// an Error if there was a problem generating the initialization.
    ///
    /// # Errors
    ///
    /// - Returns an ErrorType if generation of this assignment failed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// //let a_node: ASTNode = /* Some ASTNode we want to generate an assignment from */
    /// //let result = self.generate_assignment_ir(&a_node);
    /// /* check if type_result was Ok or Err, if Ok, it will contain None. */
    /// ```
    pub fn generate_assignment_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        let _ = node;
        unimplemented!();
    }

    /// Generates LLVM IR for an initialization
    /// 
    /// # Parameters
    ///
    /// - `node`: A reference to an `ASTNode` to generate IR for an initialization.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Tag>, ErrorType>` containing None if generation went smoothly or
    /// an Error if there was a problem generating the initialization.
    ///
    /// # Errors
    ///
    /// - Returns an ErrorType if generation of this initialization failed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// //let a_node: ASTNode = /* Some ASTNode we want to generate an initialization from */
    /// //let result = self.generate_initialization_ir(&a_node);
    /// /* check if type_result was Ok or Err, if Ok, it will contain None. */
    /// ```
    pub fn generate_initialization_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        let _ = node;
        unimplemented!();
    }
    

    /// Generates LLVM IR for a break statement.
    /// 
    /// # Parameters
    ///
    /// - `node`: A reference to an `ASTNode` to generate IR for a break statement.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Tag>, ErrorType>` containing None if generation went smoothly or
    /// an Error if there was a problem generating the break.
    ///
    /// # Errors
    ///
    /// - Returns an ErrorType if generation of this break failed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// //let a_node: ASTNode = /* Some ASTNode we want to generate a break statement from */
    /// //let result = self.generate_break_ir(&a_node);
    /// /* check if type_result was Ok or Err, if Ok, it will contain None. */
    /// ```
    pub fn generate_break_ir(&mut self, _node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }

    /// Generates LLVM IR for a continue statement.
    /// 
    /// # Parameters
    ///
    /// - `node`: A reference to an `ASTNode` to generate IR for a continue statement.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Tag>, ErrorType>` containing None if generation went smoothly or
    /// an Error if there was a problem generating the continue.
    ///
    /// # Errors
    ///
    /// - Returns an ErrorType if generation of this continue failed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// //let a_node: ASTNode = /* Some ASTNode we want to generate a continue statement from */
    /// //let result = self.generate_continue_ir(&a_node);
    /// /* check if type_result was Ok or Err, if Ok, it will contain None. */
    /// ```
    pub fn generate_continue_ir(&mut self, _node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }

    /// Generates LLVM IR for a return statement.
    /// 
    /// # Parameters
    ///
    /// - `node`: A reference to an `ASTNode` to generate IR for a return statement.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Tag>, ErrorType>` containing None if generation went smoothly or
    /// an Error if there was a problem generating the return.
    ///
    /// # Errors
    ///
    /// - Returns an ErrorType if generation of this return failed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// //let a_node: ASTNode = /* Some ASTNode we want to generate a return statement from */
    /// //let result = self.generate_return_ir(&a_node);
    /// /* check if type_result was Ok or Err, if Ok, it will contain None. */
    /// ```
    pub fn generate_return_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        let _ = node;
        unimplemented!();
    }

    /// Generates LLVM IR for recalling a variable
    /// 
    /// # Parameters
    ///
    /// - `node`: A reference to an `ASTNode` to generate IR for a variable recall statement.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Tag>, ErrorType>` containing the Tag of the recalled variable container
    /// if generation went smoothly or an Error if there was a problem in generation.
    ///
    /// # Errors
    ///
    /// - Returns an ErrorType if generation failed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// //let a_node: ASTNode = /* Some ASTNode we want to generate a recall statement from */
    /// //let result = self.generate_variable_ir(&a_node);
    /// /* check if type_result was Ok or Err, if Ok, it will contain a Tag that houses
    /// the allocation tag of the recalled variable container. */
    /// ```
    pub fn generate_variable_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        let _ = node;
        unimplemented!();
    }
}