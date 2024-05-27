use std::fmt;

/// Acceptable data types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    /// Integer type
    Integer,
    /// Float type
    Float,
    /// Double type
    Double,
    /// Long type
    Long,
    /// Boolean type
    Boolean,
    /// String type
    String,
    /// Char type
    Char,
    /// Function type
    Function,
    /// No type
    None,
    /// Struct type
    Struct,
    /// Enum type
    Enum,
    /// Void type
    Void,
    /// Unsigned type
    Unsign,
    /// Signed type
    Sign,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Integer => {
                write!(f, "Integer")
            },
            DataType::Float => {
                write!(f, "Float")
            },
            DataType::Double => {
                write!(f, "Double")
            },
            DataType::Long => {
                write!(f, "Long")
            },
            DataType::Boolean => {
                write!(f, "Boolean")
            },
            DataType::String => {
                write!(f, "String")
            },
            DataType::Char => {
                write!(f, "Char")
            },
            DataType::Function => {
                write!(f, "Function")
            },
            DataType::Struct => {
                write!(f, "Struct")
            },
            DataType::Enum => {
                write!(f, "Enum")
            },
            DataType::None => {
                write!(f, "None")
            },
            DataType::Void => {
                write!(f, "Void")
            }
            DataType::Unsign => {
                write!(f, "Unsigned")
            },
            DataType::Sign => {
                write!(f, "Signed")
            },
        }
    }
}

