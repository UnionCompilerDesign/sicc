//! This file hosts all of the functions necessary for generating LLVM IR
//! for primitives such as basic data types and literal values.

use common::{ast::{core::ASTNode, data_type::DataType}, error::ErrorType};
use safe_llvm::ir::core::Tag;
use crate::core::IRGenerator;

impl IRGenerator {
    /// Generates an LLVM type tag for a data type.
    ///
    /// # Parameters
    ///
    /// - `data_type`: A reference to a `DataType` representing the type that should be created in the 
    /// resource pools.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Tag>, ErrorType>` containing the constructed tag of the generated type
    ///  if successful, or an `ErrorType` if there was an error generating this type tag.
    ///
    /// # Errors
    ///
    /// - Returns an ErrorType if generation of this type tag failed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// //let d_type: DataType = /* Some DataType we want a data type tag for */
    /// //let type_result = self.generate_data_type_ir(&d_type);
    /// /* check if type_result was Ok or Err, if Ok extract the TypeTag from
    /// the Tag and use this for other functions. */
    /// ```
    pub fn generate_data_type_ir(&mut self, data_type: &DataType) -> Result<Option<Tag>, ErrorType> {
        let _ = data_type;
        unimplemented!();
    }

    /// Generates LLVM IR for a literal.
    /// 
    /// # Parameters
    ///
    /// - `node`: A reference to an `ASTNode` to generate IR for a literal value from.
    ///
    /// # Returns
    ///
    /// Returns a `Result<Option<Tag>, ErrorType>` containing the tag of the generated 
    /// literal value or an Error if there was a problem generating the literal.
    ///
    /// # Errors
    ///
    /// - Returns an ErrorType if generation of this literal failed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// //let a_node: ASTNode = /* Some ASTNode we want to generate a literal value from */
    /// //let result = self.generate_literal_ir(&a_node);
    /// /* check if type_result was Ok or Err, if Ok extract the ValueTag from
    /// the Tag and use this for other functions. */
    /// ```
    pub fn generate_literal_ir(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        let _ = node;
        unimplemented!();
    }
}