/// Errors in the compilation process
#[derive(Debug, PartialEq, Clone)]
pub enum ErrorType {    
    /// Binary operation has incompatible types
    TypeMismatch {
        /// Left type in comparison
        left_type: String,

        /// Right type in comparison
        right_type: String,
    },

    /// Variable used but not declared
    UndefinedVariable {
        /// The requested variable's name
        variable_name: String,
    },

    /// Unsupported operator on types given
    UnsupportedOperator {
        /// Operator type
        operator: String,
        /// Type of operand in operation
        operand_type: String,
    },

    /// Divisor is zero
    DivisionByZero {
        /// Attempted operation
        operation: String,
    },

    /// Invalid assignment to a target 
    InvalidAssignment {
        /// The name of the target variable
        target: String,
    },

    /// Unrecognized token
    UnrecognizedToken {
        /// The unrecognized token
        token: String,
    },

    /// Stand-in errors that need to be updated for better error handling
    DevError {},
}

impl From<ErrorType> for Vec<ErrorType> {
    fn from(err: ErrorType) -> Self {
        vec![err]
    }
}
