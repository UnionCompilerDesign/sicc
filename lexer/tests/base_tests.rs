use lexer::{
    core::Lexer,
    token::Token,
};

/// cargo test --test base_tests
/// Tests tokens individually.

#[test]
fn test_eof() {
    let input = "";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_num() {
    let input = "2";
    let result = Lexer::lex(input);
    let expected = vec![
    	Token::NUMBER(vec!['2']), Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_increment() {
    let input = "++";
    let result = Lexer::lex(input);
    let expected = vec![
    	Token::PLUSPLUS, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_decrement() {
    let input = "--";
    let result = Lexer::lex(input);
    let expected = vec![
    	Token::MINUSMINUS, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_divide() {
    let input = "/";
    let result = Lexer::lex(input);
    let expected = vec![
    	Token::FSLASH, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_minus() {
    let input = "-";
    let result = Lexer::lex(input);
    let expected = vec![
    	Token::DASH, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_plus() {
    let input = "+";
    let result = Lexer::lex(input);
    let expected = vec![
    	Token::PLUS, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_equals() {
    let input = "=";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::EQUAL, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_mod() {
    let input = "%";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::PERCENT, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_multiply() {
    let input = "*";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::ASTERISK, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_for() {
    let input = "for";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::FOR, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_while() {
    let input = "while";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::WHILE, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_do() {
    let input = "do";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::DO, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_break() {
    let input = "break";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::BREAK, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_continue() {
    let input = "continue";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::CONTINUE, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_switch() {
    let input = "switch";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::SWITCH, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_case() {
    let input = "case";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::CASE, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_struct() {
    let input = "struct";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::STRUCT, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_enum() {
    let input = "enum";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::ENUM, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_if() {
    let input = "if";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::IF, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_else() {
    let input = "else";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::ELSE, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_return() {
    let input = "return";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::RETURN, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_rightcurly() {
    let input = "}";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::RBRACKET, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_leftcurly() {
    let input = "{";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::LBRACKET, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_leftparen() {
    let input = "(";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::LPAREN, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_rightparen() {
    let input = ")";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::RPAREN, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_semicolon() {
    let input = ";";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::SEMICOLON, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_comma() {
    let input = ",";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::COMMA, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_colon() {
    let input = ":";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::COLON, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_leftsquare() {
    let input = "[";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::LBRACE, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_rightsquare() {
    let input = "]";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::RBRACE, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_period() {
    let input = ".";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::DOT, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_lessthan() {
    let input = "<";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::LESSTHAN, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_logicand() {
    let input = "&&";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::ANDAND, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_logicor() {
    let input = "||";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::BARBAR, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_logicnot() {
    let input = "!";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::EXCLAMATIONPOINT, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_greaterthan() {
    let input = ">";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::GREATERTHAN, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_neq() {
    let input = "!=";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::NOTEQUAL, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_equalequal() {
    let input = "==";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::EQUALEQUAL, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_leq() {
    let input = "<=";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::LESSTHANEQUAL, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_geq() {
    let input = ">=";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::GREATERTHANEQUAL, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_int() {
    let input = "int";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TINTEGER, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_double() {
    let input = "double";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TDOUBLE, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_float() {
    let input = "float";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TFLOAT, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_chartype() {
    let input = "char";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TCHAR, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_void() {
    let input = "void";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TVOID, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_signedint() {
    let input = "signed";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TSIGNINT, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_unsignedint() {
    let input = "unsigned";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TUSIGN, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_long() {
    let input = "long";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TLONG, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_and() {
    let input = "&";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::AMPERSAND, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_or() {
    let input = "|";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::BAR, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_caret() {
    let input = "^";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::CARET, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_not() {
    let input = "~";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TILDE, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_pointer() {
    let input = "->";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::POINTER, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_const() {
    let input = "const";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::CONST, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_true() {
    let input = "?";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::CTRUE, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}
