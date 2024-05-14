use frontend::{
    lexer::token::*,
    lexer::lexer_core::*,
    utils::error::*,
};

/// cargo test --test lexer_tests

#[test]
fn test_eol() {
    let input = "* / /* */\n * /* * / \n\n/*\n";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::ASTERISK, Token::FSLASH,
         Token::ASTERISK, Token::EOF,
    ];
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
    let input = "+ = ; ( ) { } , :";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::PLUS, Token::EQUAL, Token::SEMICOLON,
        Token::LPAREN, Token::RPAREN, Token::LBRACKET, Token::RBRACKET,
        Token::COMMA, Token::COLON, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

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
    let input = "     x   = 5  ;  ";
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
fn test_invalid_char() {
    let input = "let $invalid = 5;";
    let result = Lexer::lex(input);
    let expected_error = ErrorType::UnrecognizedToken{token: "$".to_string()};
    let expected = Err(vec![expected_error]);
    assert_eq!(result, expected);
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
    let input = "+ - * / %";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::PLUS, Token::DASH, Token::ASTERISK,
        Token::FSLASH, Token::PERCENT,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));

    let input2 = "5 - 5";
    let result2 = Lexer::lex(input2);
    let expected2=vec![
        Token::NUMBER(vec!['5']), 
        Token::DASH,
        Token::NUMBER(vec!['5']),
        Token::EOF,
    ];
    assert_eq!(result2, Ok(expected2));
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
    let input = "if else for while do break continue switch";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::IF, Token::ELSE,
        Token::FOR, Token::WHILE, Token::DO,
        Token::BREAK, Token::CONTINUE, Token::SWITCH,
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
    let input = "int float char void unsigned long double";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TINTEGER, Token::TFLOAT, Token::TCHAR, Token::TVOID,
        Token::TUSIGN, Token::TLONG, Token::TDOUBLE, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_dot_operators() {
    let input = ".";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::DOT, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_block_comments() {
    let input = "* / /* */ * /* * / /*";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::ASTERISK, Token::FSLASH, Token::ASTERISK, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_block_comments_with_surrounding_code() {
    let input = "int x = 6; /* hello \n there \n my freind \n */ int y = 10;";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TINTEGER, Token::IDENTIFIER(vec!['x']), Token::EQUAL,
        Token::NUMBER(vec!['6']), Token::SEMICOLON, Token::TINTEGER, Token::IDENTIFIER(vec!['y']), Token::EQUAL,
        Token::NUMBER(vec!['1', '0']), Token::SEMICOLON, Token::EOF
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_single_line_comment_stripping() {
    let input = "// this is a comment! \n int x = 5;";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TINTEGER, Token::IDENTIFIER(vec!['x']), Token::EQUAL, Token::NUMBER(vec!['5']), Token::SEMICOLON, Token::EOF
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_single_line_comment() {
    let input = "//";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::EOF
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_misc_tokens() {
    let input = "break -> case const continue switch void ?";
    let result = Lexer::lex(input);
    let expected = vec![
	Token::BREAK, Token::POINTER, Token::CASE, Token::CONST,
	Token::CONTINUE, Token::SWITCH, Token::TVOID, Token::CTRUE, Token::EOF
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_reading_char() {
    let input = "char x = 'a';";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TCHAR, Token::IDENTIFIER(vec!('x')), Token::EQUAL,
        Token::CHAR('a'), Token::SEMICOLON, Token::EOF
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_bitwise_ops() {
    let input = "^ ~ | &";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::CARET, Token::TILDE, Token::BAR, Token::AMPERSAND, Token::EOF
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_string_literal() {
    let input = "\"hello\"";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::STRINGLITERAL(vec!('h', 'e', 'l', 'l', 'o')), Token::EOF
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_initialization_string_literal() {
    let input = "char str[] = \"hello\";";
    let result = Lexer::lex(input);
    let expected = vec![Token::TCHAR, Token::IDENTIFIER(vec!['s', 't', 'r']), Token::LBRACE, 
        Token::RBRACE, Token::EQUAL, Token::STRINGLITERAL(vec!['h', 'e', 'l', 'l', 'o']), 
        Token::SEMICOLON, Token::EOF
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_char_literal() {
    let input = "'a'";
    let result = Lexer::lex(input);
    let expected = vec![Token::CHAR('a'), Token::EOF
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_char_literal_initialization() {
    let input = "char x = 'a';";
    let result = Lexer::lex(input);
    let expected = vec![Token::TCHAR, Token::IDENTIFIER(vec!('x')), Token::EQUAL, Token::CHAR('a'), Token::SEMICOLON, Token::EOF
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_assignment_ops() {
    let input = "++ --";
    let result = Lexer::lex(input);
    let expected = vec![Token::PLUSPLUS, Token::MINUSMINUS, Token::EOF
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_assignment_ops_in_context() {
    let input = "x ++; y --;";
    let result = Lexer::lex(input);
    let expected = vec![Token::IDENTIFIER(vec!('x')), Token::PLUSPLUS, Token::SEMICOLON,
                        Token::IDENTIFIER(vec!('y')), Token::MINUSMINUS, Token::SEMICOLON, Token::EOF
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_switch_statement() {
    let input = "switch (y) {
        case 1:
            int x = 6;
            break;
        case 2:
            int x = 7;
            break;
        default:
            int x = 8;
            break;
    }";

    let result = Lexer::lex(input);

    let expected = vec![
        Token::SWITCH,
        Token::LPAREN,
        Token::IDENTIFIER(vec!['y']),
        Token::RPAREN,
        Token::LBRACKET,
        Token::CASE,
        Token::NUMBER(vec!['1']),
        Token::COLON,
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['6']),
        Token::SEMICOLON,
        Token::BREAK,
        Token::SEMICOLON,
        Token::CASE,
        Token::NUMBER(vec!['2']),
        Token::COLON,
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['7']),
        Token::SEMICOLON,
        Token::BREAK,
        Token::SEMICOLON,
        Token::DEFAULT,
        Token::COLON,
        Token::TINTEGER,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::NUMBER(vec!['8']),
        Token::SEMICOLON,
        Token::BREAK,
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::EOF,
    ];

    assert_eq!(result, Ok(expected));
}

#[test]
fn test_signed_unsigned() {
    let input = "signed unsigned";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TSIGNINT, Token::TUSIGN, Token::EOF
    ];
    assert_eq!(result, Ok(expected));
}