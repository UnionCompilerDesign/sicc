use crate::ast::data_type::DataType;

use std::fmt;

#[derive(Debug, Clone, PartialEq, Default, Eq, Hash)]
/// Defines acceptable syntax elements, as part of an AST
pub enum SyntaxElement {
    /// No expression
    #[default]
    NoExpression,

    /// --- BASE EXPRESSION SECTION --- ///
    /// Literal
    Literal(String),

    /// Identifier
    Identifier(String),

    /// Operator
    Operator(String),

    /// Type
    Type(DataType),

    /// Constant
    Constant(String),

    /// --- CONTROL FLOW SECTION --- ///
    /// If statement
    IfStatement,

    /// Else statement
    ElseStatement,

    /// For loop
    ForLoop,

    /// While loop
    WhileLoop,

    /// Do while loop
    DoWhileLoop,

    /// Break statement
    Break,

    /// Continue statement
    Continue,

    /// Return statement
    Return,

    /// Switch statement
    SwitchStatement,

    /// Case of switch
    Case,

    /// Default branch of switch
    Default,

    /// --- DECLARATION SECTION --- ///
    /// Assignment of an existing variable
    Assignment,

    /// Initialization of a variable
    Initialization,

    /// Function declaration
    FunctionDeclaration,

    /// Struct declaration
    StructDeclaration,

    /// Enum declaration
    EnumDeclaration,

    /// --- MODULE & SCOPING SECTION --- ///
    /// Module expression
    ModuleExpression,

    /// Top level expression
    TopLevelExpression,

    /// Block
    BlockExpression,

    /// --- STATEMENT SECTION --- ///
    /// Condition,
    Condition,

    /// Action,
    Action,

    /// Variant,
    Variant,

    /// Assigned value (used in: initialization, assignment, return)
    AssignedValue,

    /// Field of a struct
    Field,

    /// Parameter
    Parameter,

    /// Variable
    Variable,

    /// Binary expression
    BinaryExpression,

    /// Unary expression
    UnaryExpression,

    /// Function call
    FunctionCall,

    /// Operand
    Operand,

    /// --- LOOP CONTROL SECTION --- ///
    /// Initializer to a loop
    LoopInitializer,

    /// Incrementer on a loop
    LoopIncrement,


}


impl fmt::Display for SyntaxElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxElement::NoExpression => write!(f, "NoExpression"),
            SyntaxElement::Literal(value) => write!(f, "Literal({})", value),
            SyntaxElement::Constant(value) => write!(f, "Constant({})", value),
            SyntaxElement::Variable => write!(f, "Variable)"),
            SyntaxElement::BinaryExpression => write!(f, "BinaryExpression"),
            SyntaxElement::UnaryExpression => write!(f, "UnaryExpression"),
            SyntaxElement::Identifier(id) => write!(f, "Identifier({})", id),
            SyntaxElement::FunctionCall => write!(f, "FunctionCall"),
            SyntaxElement::Operator(op) => write!(f, "Operator({})", op),
            SyntaxElement::Operand => write!(f, "Operand"),
            SyntaxElement::Type(data_type) => write!(f, "Type({})", data_type),
            SyntaxElement::IfStatement => write!(f, "IfStatement"),
            SyntaxElement::ElseStatement => write!(f, "ElseStatement"),
            SyntaxElement::ForLoop => write!(f, "ForLoop"),
            SyntaxElement::WhileLoop => write!(f, "WhileLoop"),
            SyntaxElement::DoWhileLoop => write!(f, "DoWhileLoop"),
            SyntaxElement::SwitchStatement => write!(f, "SwitchStatement"),
            SyntaxElement::Case => write!(f, "Case"),
            SyntaxElement::Break => write!(f, "Break"),
            SyntaxElement::Continue => write!(f, "Continue"),
            SyntaxElement::Return => write!(f, "Return"),
            SyntaxElement::Assignment => write!(f, "Assignment"),
            SyntaxElement::Initialization => write!(f, "Initialization"),
            SyntaxElement::FunctionDeclaration => write!(f, "FunctionDeclaration"),
            SyntaxElement::Parameter => write!(f, "Parameter"),
            SyntaxElement::StructDeclaration => write!(f, "StructDeclaration"),
            SyntaxElement::EnumDeclaration => write!(f, "EnumDeclaration"),
            SyntaxElement::ModuleExpression => write!(f, "ModuleExpression"),
            SyntaxElement::TopLevelExpression => write!(f, "TopLevelExpression"),
            SyntaxElement::BlockExpression => write!(f, "BlockExpression"),
            SyntaxElement::LoopInitializer => write!(f, "LoopInitializer"),
            SyntaxElement::LoopIncrement => write!(f, "LoopIncrement"),
            SyntaxElement::Condition => write!(f, "Condition"),
            SyntaxElement::Action => write!(f, "Action"),
            SyntaxElement::Variant => write!(f, "Variant"),
            SyntaxElement::AssignedValue => write!(f, "AssignedValue"),
            SyntaxElement::Field => write!(f, "Field"),
            SyntaxElement::Default => write!(f, "Default"),
        }
    }
}