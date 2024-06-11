//! This file defines the errors that can occur during the compilation process.

/// Defines types of errors that can occur during compilation.
///
/// Each error represents a different kind of issue that can be encountered during the lexing, parsing, analysis, 
/// or ir phases, such as type mismatches, undefined variables, and others. 
#[derive(Debug, PartialEq, Clone)]
pub enum ErrorType {    
    /// Occurs due to a type mismatch in a binary operation.
    TypeMismatch {
        /// The type of the left operand.
        left_type: String,
        /// The type of the right operand.
        right_type: String,
    },

    /// Occurs due to a variable being used in the code but not been declared.
    UndefinedVariable {
        /// The name of the variable that was not defined.
        variable_name: String,
    },

    /// Occurs due to an operator being use with a type that does not support it.
    UnsupportedOperator {
        /// The operator used.
        operator: String,
        /// The type of the operand with which the operator is incompatible.
        operand_type: String,
    },

    /// Syntax Error, most commonly found in the parsing stage.
    SyntaxError {
        /// Describes the parsing error found.
        message: String,
    },

    /// Occurs due to an operation attempting to divide by zero.
    DivisionByZero {
        /// Describes the operation that attempted the division by zero.
        operation: String,
    },

    /// Occurs due to an attempted assignment of a value to a non-assignable target.
    InvalidAssignment {
        /// The target of the assignment which was invalid.
        target: String,
    },

    /// Occurs due to a token being encountered in the source code that the lexer/parser does not recognize.
    UnrecognizedToken {
        /// The token that was not recognized.
        token: String,
    },

    /// A placeholder error for development use.
    DevError {
        /// A message describing what needs to be addressed.
        message: String,
    },
}

impl From<ErrorType> for Vec<ErrorType> {
    /// Converts an `ErrorType` into a vector containing that error.
    fn from(err: ErrorType) -> Self {
        vec![err]
    }
}
