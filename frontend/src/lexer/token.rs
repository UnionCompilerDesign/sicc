use std::fmt;

/// Defines acceptable tokens in the program
#[derive(PartialEq, Debug, Clone, Default)]
pub enum Token {
    /// Default token type
    #[default]
    DEFAULT,
    
    /// End of file
    EOF,
    /// End of line
    EOL,

    /// --- ASSIGNMENT SECTION --- ///
    /// --- MULTI-CHARACTER SECTION --- ///
    /// Integer
    INT(Vec<char>),

    /// Double
    Double(Vec<char>),

    /// Identifier
    IDENTIFIER(Vec<char>),

    /// --- ASSIGNMENT OPERATORS --- ///
    /// Increment (++)
    PLUSPLUS,

    /// Decrement (--)
    MINUSMINUS,

    /// --- BINARY OP SECTION --- ///
    /// Division
    DIVIDE,

    /// Floor division
    FLOORDIVISION,

    /// Subtraction and Negative (Unary and Binary "-")
    DASH,

    /// Addition
    PLUS,

    /// Assignment
    EQUAL,

    /// Modulo (%)
    MOD,

    /// Multiply
    MULTIPLY,

    /// --- SCOPE CHANGING SECTION --- ///
    /// Struct
    STRUCT,

    /// Enum
    ENUM,

    /// If
    IF,

    /// Else
    ELSE,

    /// Return
    RETURN,

    /// For
    FOR,

    /// While
    WHILE,

    /// Do
    DO,

    /// Break
    BREAK,

    /// Continue
    CONTINUE,

    ///  --- SPECIAL CHARACTER SECTION --- ///
    /// Right bracket }
    RBRACKET,

    /// Left bracket {
    LBRACKET,

    /// Left parenthesis (
    LPAREN,

    /// Right parenthesis
    RPAREN,

    /// Semicolon
    SEMICOLON,

    /// Comma
    COMMA,

    /// Colon
    COLON,

    /// Left bracket [
    LBRACE,

    ///  Right bracket ]
    RBRACE,

    /// Dot
    DOT,
    
    /// Block Comment Begin
    BCOMMENTBEGIN,

    /// Block Comment End
    BCOMMENTEND,

    /// Single Comment
    SCOMMENT,


    /// --- BOOLEAN SECTION --- ///
    /// Logical and (&&)
    LOGICALAND,

    /// Logical or (||)
    LOGICALOR,

    /// Logical not (!)
    LOGICALNOT,

    /// Less than (<)
    LESSTHAN,

    /// Greater than (>)
    GREATERTHAN,

    /// Not equals (!=)
    NOTEQUAL,

    /// Equality check (==)
    EQUALEQUAL, 

    /// Less than or equal to (<=)
    LESSTHANEQUAL,

    /// Greater than or equal to (>=)
    GREATERTHANEQUAL,

    
    /// --- TYPE ANNOTATION SECTION --- ///
    /// Integer type
    TINTEGER,

    /// Double Type
    TDOUBLE,

    /// Float type
    TFLOAT,

    /// Character type
    TCHAR,

    /// Void type
    TVOID,

    /// Signed Integer type
    TSIGN, 

    /// Unsigned Integer type
    TUSIGN,

    /// Signed Int type
    TSIGNINT,

    /// Long type
    TLONG,

    /// --- BITWISE OPERATORS --- ///
    /// Bitwise and, address of
    BITAND,

    /// Bitwise or
    BITOR,

    /// Bitwise xor
    XOR,

    /// --- MISC SECTION --- ///
    /// Pointer to member (->)
    POINTER,

    /// Switch
    SWITCH,

    /// Case
    CASE,

    /// Constant
    CONST,

    /// Conditional True (?)
    CTRUE,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
