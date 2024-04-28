use std::fmt;

/// Defines acceptable tokens in the program
#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    /// End of file
    EOF,
    /// End of line
    EOL,

    /// --- ASSIGNMENT SECTION --- ///
    /// Let
    LET,

    /// Increment (+=)
    PLUSASSIGN, 

    /// Decrement (-=)
    MINUSASSIGN,

    /// Multiply increment (*=)
    MULTIPLYASSIGN,

    /// Divide increment (/=) 
    DIVIDEASSIGN,

    /// Modulo decrement (%=) 
    MODASSIGN,


    /// --- MULTI-CHARACTER SECTION --- ///
    /// Integer
    INT(Vec<char>),

    /// Double
    Double(Vec<char>),

    /// Identifier
    IDENTIFIER(Vec<char>),

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

    /// Exponent
    EXPONENT,


    /// --- SCOPE CHANGING SECTION --- ///
    /// Function 
    FUNCTION,
    
    /// Struct
    STRUCT,

    /// Enum
    ENUM,

    /// If
    IF,

    /// Else if
    ELIF,

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

    /// Match expression
    MATCH,

    /// Arrow
    ARROW,

    /// Class declaration
    CLASS,

    /// Constructor
    INIT,

    /// Module
    MODULE,

    /// Use
    USE,

    /// As
    AS, 

    /// Public
    PUBLIC, 

    /// Private
    PRIVATE,

    /// Switch
    SWITCH,

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
    
    /// Double colon ::
    COLONCOLON,

    /// Block Comment Begin
    BCOMMENTBEGIN,

    /// Block Comment End
    BCOMMENTEND,

    /// Single Comment
    SCOMMENT,


    /// --- BOOLEAN SECTION --- ///
    /// Logical and (&&)
    LOGICALAND,

    /// Logical or (|)
    LOGICALOR,

    /// Logical not (!)
    LOGICALNOT,

    /// 1
    TRUE,

    /// 0
    FALSE,

    /// Less than (<)
    LESSTHAN,

    /// Greater than (>)
    GREATERTHAN,

    /// Not equal (!=)
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

    /// Long type
    TLONG,

    /// Boolean type
    TBOOLEAN,

    /// String type
    TSTRING,

    /// Character type
    TCHAR,

    /// Void type
    TVOID,

    /// Signed Integer type
    TSIGN, 

    /// Unsigned Integer type
    TUSIGN,

    /// Base Object
    TOBJECT,

    /// Array Type
    TARRAY,

    /// Function Return Arrow (->)
    RETARROW,

    ///  --- MISC SECTION --- ///
    /// Self
    SELF,

    /// Async
    ASYNC,

    /// Await
    AWAIT,

    /// Try
    TRY,

    /// Catch
    CATCH,

    /// Finally
    FINALLY,

    /// Case
    CASE,

    /// Default
    DEFAULT,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
