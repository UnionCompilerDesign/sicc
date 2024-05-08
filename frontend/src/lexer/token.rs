use std::fmt;

/// Represents all possible tokens that can be recognized by the lexer.
#[derive(PartialEq, Debug, Clone, Default)]
pub enum Token {
    /// The default token state.
    #[default]
    DEFAULT,
    
    /// Token indicating the end of the file.
    EOF,
    /// Token indicating the end of a line.
    EOL,

    // ----- Dynamically Sized Tokens -----
    /// Represents a whole integer.
    INT(Vec<char>),
    /// Represents a floating-point double.
    Double(Vec<char>),
    /// Represents identifiers like variable names.
    IDENTIFIER(Vec<char>),

    // ----- Assignment Operators -----
    /// Increment operator `++`.
    PLUSPLUS,
    /// Decrement operator `--`.
    MINUSMINUS,

    // ----- Binary Operators -----
    /// Division operator `/`.
    DIVIDE,
    /// Noth subtraction and unary negative `-`.
    DASH,
    /// Addition operator `+`.
    PLUS,
    /// Assignment operator `=`.
    EQUAL,
    /// Modulo operator `%`.
    MOD,
    /// Multiplication operator `*`.
    MULTIPLY,

    // ----- Scope Definition Tokens -----
    /// A struct definition.
    STRUCT,
    /// An enum definition.
    ENUM,
    /// If conditional.
    IF,
    /// Else branch.
    ELSE,
    /// Return statement.
    RETURN,
    /// For loop.
    FOR,
    /// While loop.
    WHILE,
    /// Do-while loop.
    DO,
    /// Break keyword to exit loops.
    BREAK,
    /// Continue keyword to skip to the next loop iteration.
    CONTINUE,
    /// Switch statement.
    SWITCH,
    /// Case keyword for switch cases.
    CASE,

    // ----- Special Character Tokens -----
    /// Right curly bracket `}`.
    RBRACKET,
    /// Left curly bracket `{`.
    LBRACKET,
    /// Left parenthesis `(`.
    LPAREN,
    /// Right parenthesis `)`.
    RPAREN,
    /// Semicolon `;`.
    SEMICOLON,
    /// Comma `,`.
    COMMA,
    /// Colon `:`.
    COLON,
    /// Left square bracket `[`.
    LBRACE,
    /// Right square bracket `]`.
    RBRACE,
    /// Period `.`.
    DOT,
    /// Start of a block comment `/*`.
    BCOMMENTBEGIN,
    /// End of a block comment `*/`.
    BCOMMENTEND,
    /// Single line comment `//`.
    SCOMMENT,

    // ----- Boolean and Comparison Operators -----
    /// Logical AND `&&`.
    LOGICALAND,
    /// Logical OR `||`.
    LOGICALOR,
    /// Logical NOT `!`.
    LOGICALNOT,
    /// Less than `<`.
    LESSTHAN,
    /// Greater than `>`.
    GREATERTHAN,
    /// Not equal `!=`.
    NOTEQUAL,
    /// Equality `==`.
    EQUALEQUAL,
    /// Less than or equal to `<=`.
    LESSTHANEQUAL,
    /// Greater than or equal to `>=`.
    GREATERTHANEQUAL,

    // ----- Type Annotation Tokens -----
    /// Integer type annotation.
    TINTEGER,
    /// Double type annotation.
    TDOUBLE,
    /// Float type annotation.
    TFLOAT,
    /// Char type annotation.
    TCHAR,
    /// Void type annotation.
    TVOID,
    /// Signed type annotation.
    TSIGN, 
    /// Unsigned type annotation.
    TUSIGN,
    /// Signed integer type annotation.
    TSIGNINT,
    /// Long type annotation.
    TLONG,

    // ----- Bitwise Operators -----
    /// Bitwise AND and address-of `&`.
    BITAND,
    /// Bitwise OR `|`.
    BITOR,
    /// Bitwise XOR `^`.
    XOR,

    // ----- Miscellaneous -----
    /// Pointer to member operator `->`.
    POINTER,
    /// Constant declaration.
    CONST,
    /// Conditional true `?`.
    CTRUE,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
