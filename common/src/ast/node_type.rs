//! This file contains a definition of an enum representing the acceptable node types of ASTs

use crate::ast::data_type::DataType;
use std::fmt;

/// Defines acceptable syntax elements as part of an `AST`.
///
/// Each element A different kind of syntactic construct that can appear in source code, such as constants, identifiers, 
/// operators, and control structures. These elements are used to build a tree representation of the code's syntactic structure.
#[derive(Debug, Clone, PartialEq, Default, Eq, Hash)]
pub enum NodeType {
    /// A lack of expression, often used as a placeholder.
    #[default]
    NoExpression,

    /// A literal value, stored as a string.
    Literal(String),

    /// An identifier, such as a variable name, stored as a string.
    Identifier(String),

    /// An operator, such as '+', '-', etc., stored as a string.
    Operator(String),

    /// A data type, linking directly to the `DataType` enum.
    Type(DataType),

    /// A constant value, stored as a string.
    Constant(String),

    /// An `if` statement.
    IfStatement,

    /// An `else` statement.
    ElseStatement,

    /// A `for` loop.
    ForLoop,

    /// A `while` loop.
    WhileLoop,

    /// A `do-while` loop.
    DoWhileLoop,

    /// A `break` statement.
    Break,

    /// A `continue` statement.
    Continue,

    /// A `return` statement.
    Return,

    /// A `switch` statement.
    SwitchStatement,

    /// A case in a `switch` statement.
    Case,

    /// The default branch in a `switch` statement.
    Default,

    /// An assignment of a value to an existing variable.
    Assignment,

    /// The initialization of a variable.
    Initialization,

    /// A function declaration.
    FunctionDeclaration,

    /// A struct declaration.
    StructDeclaration,

    /// An enum declaration.
    EnumDeclaration,

    /// A module-level expression, often used for scoping.
    ModuleExpression,

    /// A top-level expression in a module or file.
    TopLevelExpression,

    /// A block of expressions or statements.
    BlockExpression,

    /// A condition in control flows.
    Condition,

    /// An action to be taken, typically in control structures.
    Action,

    /// A variant in enums or other similar structures.
    Variant,

    /// A value assigned in statements like initialization and assignment.
    AssignedValue,

    /// A field in a struct or similar data structure.
    Field,

    /// A parameter in function declarations.
    Parameter,

    /// A variable, used in various expressions and statements.
    Variable,

    /// A binary expression, involving two operands and an operator.
    BinaryExpression,

    /// A unary expression, involving one operand and an operator.
    UnaryExpression,

    /// A function call.
    FunctionCall,

    /// An operand in an expression.
    Operand,

    /// The initializer section of a loop.
    LoopInitializer,

    /// The increment section of a loop.
    LoopIncrement,
}

/// Provides a display implementation for `DataType`.
///
/// # Parameters
/// * `f` - The formatter.
///
/// # Returns
/// * `fmt::Result` - The result of the formatting operation.
impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeType::Literal(value) => write!(f, "Literal({})", value),
            NodeType::Identifier(id) => write!(f, "Identifier({})", id),
            NodeType::Operator(op) => write!(f, "Operator({})", op),
            NodeType::Type(data_type) => write!(f, "Type({})", data_type),
            NodeType::Constant(value) => write!(f, "Constant({})", value),
            _ => write!(f, "{:?}", self) 
        }
    }
}
