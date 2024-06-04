use common::error::ErrorType;

use lexer::{
    lexer_core::Lexer,
    token::Token,
};

/// cargo test --test combination_tests

#[test]
fn test_complex_expressions() {
    let input: &str = "x = 5 + 10 / 5 % 3;";
    let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
    let expected: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['5']),
        Token::PLUS,
        Token::NUMBER(vec!['1', '0']),
        Token::FSLASH,
        Token::NUMBER(vec!['5']),
        Token::PERCENT,
        Token::NUMBER(vec!['3']),
        Token::SEMICOLON,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_whitespace_handling() {
    let input = "  x   = 5  ;  ";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['5']),
        Token::SEMICOLON,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_assignment() {
    let input = "x: int = 5;";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::IDENTIFIER(vec!['x']),
        Token::COLON,
        Token::TINTEGER, 
        Token::EQUAL,
        Token::NUMBER(vec!['5']),
        Token::SEMICOLON,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_function_declarations() {
    let input: &str = "add(a: int, b: int) -> int { return a + b; }";
    let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
    let expected: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['a', 'd', 'd']),
        Token::LPAREN,
        Token::IDENTIFIER(vec!['a']),
        Token::COLON,
        Token::TINTEGER,
        Token::COMMA,
        Token::IDENTIFIER(vec!['b']),
        Token::COLON,
        Token::TINTEGER,
        Token::RPAREN,
        Token::POINTER,
        Token::TINTEGER,
        Token::LBRACKET,
        Token::RETURN,
        Token::IDENTIFIER(vec!['a']),
        Token::PLUS,
        Token::IDENTIFIER(vec!['b']),
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_if_else_in_function() {
    let input: &str = "check(x: int) { if x > 0 { return 1; } else { return 0; } }";
    let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
    let expected: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['c', 'h', 'e', 'c', 'k']),
        Token::LPAREN,
        Token::IDENTIFIER(vec!['x']),
        Token::COLON,
        Token::TINTEGER,
        Token::RPAREN,
        Token::LBRACKET,
        Token::IF,
        Token::IDENTIFIER(vec!['x']),
        Token::GREATERTHAN, 
        Token::NUMBER(vec!['0']),
        Token::LBRACKET,
        Token::RETURN,
        Token::NUMBER(vec!['1']),
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::ELSE,
        Token::LBRACKET,
        Token::RETURN,
        Token::NUMBER(vec!['0']),
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_logical_operators_and_parentheses() {
    let input: &str = "result = (5 > 3) && (2 < 4);";
    let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
    let expected: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['r', 'e', 's', 'u', 'l', 't']),
        Token::EQUAL,
        Token::LPAREN,
        Token::NUMBER(vec!['5']),
        Token::GREATERTHAN, 
        Token::NUMBER(vec!['3']),
        Token::RPAREN,
        Token::ANDAND,
        Token::LPAREN,
        Token::NUMBER(vec!['2']),
        Token::LESSTHAN, 
        Token::NUMBER(vec!['4']),
        Token::RPAREN,
        Token::SEMICOLON,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_nested_function_calls() {
    let input: &str = "val = add(multiply(2, 3), 4);";
    let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
    let expected: Vec<Token> = vec![
        Token::IDENTIFIER(vec!['v', 'a', 'l']),
        Token::EQUAL,
        Token::IDENTIFIER(vec!['a', 'd', 'd']),
        Token::LPAREN,
        Token::IDENTIFIER(vec!['m', 'u', 'l', 't', 'i', 'p', 'l', 'y']),
        Token::LPAREN,
        Token::NUMBER(vec!['2']),
        Token::COMMA,
        Token::NUMBER(vec!['3']),
        Token::RPAREN,
        Token::COMMA,
        Token::NUMBER(vec!['4']),
        Token::RPAREN,
        Token::SEMICOLON,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_class() {
    let input = "foo {}";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::IDENTIFIER(vec!['f', 'o', 'o']),
        Token::LBRACKET, Token::RBRACKET, Token::EOF
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_keywords() {
    let inputs = "if else return";
    let expected = vec![
        Token::IF, Token::ELSE, Token::RETURN, Token::EOF
    ];

    let result = Lexer::lex(inputs);
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_identifiers() {
    let input = "variable another_var";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::IDENTIFIER(vec!['v', 'a', 'r', 'i', 'a', 'b', 'l', 'e']),
        Token::IDENTIFIER(vec!['a', 'n', 'o', 't', 'h', 'e', 'r', '_', 'v', 'a', 'r']),
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_int_literals() {
    let input = "123 456";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::NUMBER(vec!['1', '2', '3']),
        Token::NUMBER(vec!['4', '5', '6']),
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_operators_and_special_chars() {
    let input = "+ - = ; ( ) { } , :";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::PLUS, Token::DASH, Token::EQUAL, Token::SEMICOLON,
        Token::LPAREN, Token::RPAREN, Token::LBRACKET, Token::RBRACKET,
        Token::COMMA, Token::COLON, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_invalid_char() {
    let input = "let $invalid = 5;";
    let result = Lexer::lex(input);
    let expected_error = ErrorType::UnrecognizedToken{token: "$".to_string()};
    let expected = Err(vec![expected_error]);
    assert_eq!(result, expected);
}

#[test]
fn test_comparison_operators() {
    let input = "< > <= >= == !=";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::LESSTHAN, Token::GREATERTHAN, Token::LESSTHANEQUAL,
        Token::GREATERTHANEQUAL, Token::EQUALEQUAL, Token::NOTEQUAL,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_arithmetic_operators() {
    let input = "+ - * / % ^";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::PLUS, Token::DASH, Token::ASTERISK,
        Token::FSLASH, Token::PERCENT, Token::CARET,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_logical_operators() {
    let input = "&& || !";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::ANDAND, Token::BARBAR, Token::EXCLAMATIONPOINT,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_struct_enum_declarations() {
    let input = "struct enum";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::STRUCT, Token::ENUM, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_control_flow_tokens() {
    let input = "if else for while do break continue";
    let result = Lexer::lex(input);
   let expected = vec![
        Token::IF, Token::ELSE,
        Token::FOR, Token::WHILE, Token::DO,
        Token::BREAK, Token::CONTINUE,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_braces_and_parentheses() {
    let input = "{ } [ ] ( )";
   let result = Lexer::lex(input);
    let expected = vec![
        Token::LBRACKET, Token::RBRACKET, Token::LBRACE,
        Token::RBRACE, Token::LPAREN, Token::RPAREN,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_type_annotations() {
    let input = "int float char void";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TINTEGER, Token::TFLOAT,
        Token::TCHAR, Token::TVOID,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_dot_and_coloncolon_operators() {
    let input = ". ::";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::DOT, Token::COLON, Token::COLON, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_block_comments() {
    let input = "* / /* */ * /* * / /*";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::ASTERISK, Token::FSLASH,
        Token::ASTERISK, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_single_line_comment() {
    let input = "//";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}
