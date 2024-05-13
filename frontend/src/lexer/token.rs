use std::fmt;

/// Represents all possible tokens that can be recognized by the lexer.
#[derive(PartialEq, Debug, Clone, Default)]
pub enum Token {
    /// The default token state.
    #[default]
    DEFAULT,
    
    /// Token indicating the end of the file.
    EOF,

    // ----- Dynamically Sized Tokens -----
    /// Represents a number.
    NUMBER(Vec<char>),

    /// Represents identifiers like variable names.
    IDENTIFIER(Vec<char>),

    /// Represents a string literal like "hello world"
    STRINGLITERAL(Vec<char>),

    /// Character literal like 'a'
    CHAR(char),

    // ----- Assignment Operators -----
    /// Increment operator `++`.
    PLUSPLUS,
    /// Decrement operator `--`.
    MINUSMINUS,

    // ----- Binary Operators -----
    /// Division operator `/`.
    FSLASH,

    /// Noth subtraction and unary negative `-`.
    DASH,

    /// Addition operator `+`.
    PLUS,

    /// Assignment operator `=`.
    EQUAL,

    /// Modulo operator `%`.
    PERCENT,

    /// Multiplication operator `*`.
    ASTERISK,

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


    // ----- Boolean and Comparison Operators -----
    /// Logical and (&&)
    ANDAND,

    /// Logical or (||)
    BARBAR,

    /// Logical not (!)
    EXCLAMATIONPOINT,

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

    // ----- Bitwise Operators -----
    /// Bitwise and, address of (&)
    AMPERSAND,

    /// Bitwise or (|)
    BAR,

    /// Bitwise xor (^)
    CARET,

    /// Bitwise not (~)
    TILDE,


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
