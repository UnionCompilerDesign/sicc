//! This file defines the data types that can be used in our Abstract Syntax Tree (`AST`).
//! These data types represent the various types of values that can be part of the syntax in a program.

use std::fmt;

/// Represents the different data types associated with syntax elements in an `AST`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    /// An integer.
    Integer,
    /// A floating-point number.
    Float,
    /// A double-precision floating-point number.
    Double,
    /// A long integer.
    Long,
    /// A boolean.
    Boolean,
    /// A string.
    String,
    /// A character.
    Char,
    /// A function type, including return and parameter types.
    Function,
    /// A struct type, a composite data type.
    Struct,
    /// An enumeration.
    Enum,
    /// Represents no type, used in certain contexts where a type is not applicable.
    None,
    /// A void type for functions that do not return a value.
    Void,
    /// An unsigned integer.
    Unsign,
    /// A signed integer.
    Sign,
}

/// Provides a display implementation for `DataType`.
///
/// # Parameters
/// * `f` - The formatter.
///
/// # Returns
/// * `fmt::Result` - The result of the formatting operation.
impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Integer => write!(f, "Integer"),
            DataType::Float => write!(f, "Float"),
            DataType::Double => write!(f, "Double"),
            DataType::Long => write!(f, "Long"),
            DataType::Boolean => write!(f, "Boolean"),
            DataType::String => write!(f, "String"),
            DataType::Char => write!(f, "Char"),
            DataType::Function => write!(f, "Function"),
            DataType::Struct => write!(f, "Struct"),
            DataType::Enum => write!(f, "Enum"),
            DataType::None => write!(f, "None"),
            DataType::Void => write!(f, "Void"),
            DataType::Unsign => write!(f, "Unsigned"),
            DataType::Sign => write!(f, "Signed"),
        }
    }
}
