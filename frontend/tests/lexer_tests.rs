use frontend::{
    lexer::token::*,
    lexer::lexer_core::*,
    utils::error::*,
};

/// cargo test --test lexer_tests

#[test]
fn test_keywords() {
    let inputs = "let true false if else return fn";
    let expected = vec![
        Token::LET, Token::TRUE, Token::FALSE,
        Token::IF, Token::ELSE, Token::RETURN, Token::FUNCTION, Token::EOF
    ];
    
    let result = Lexer::lex(inputs);
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_eol() {
    let input = "* / /* */\n * /* * / \n\n/*\n";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::MULTIPLY, Token::DIVIDE, Token::BCOMMENTBEGIN,
        Token::BCOMMENTEND,
        Token::EOL, Token::MULTIPLY, Token::BCOMMENTBEGIN,
        Token::MULTIPLY, Token::DIVIDE, Token::EOL, Token::EOL, 
        Token::BCOMMENTBEGIN, Token::EOL, Token::EOF,
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
        Token::INT(vec!['1', '2', '3']),
        Token::INT(vec!['4', '5', '6']),
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
    let input: &str = "let x = 5 + 10 / 5 % 3;";
    let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
    let expected: Vec<Token> = vec![
        Token::LET,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::INT(vec!['5']),
        Token::PLUS,
        Token::INT(vec!['1', '0']),
        Token::DIVIDE,
        Token::INT(vec!['5']),
        Token::MOD,
        Token::INT(vec!['3']),
        Token::SEMICOLON,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_whitespace_handling() {
    let input = "   let    x   = 5  ;  ";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::LET,
        Token::IDENTIFIER(vec!['x']),
        Token::EQUAL,
        Token::INT(vec!['5']),
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
    let input = "let x: Integer = 5;";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::LET,
        Token::IDENTIFIER(vec!['x']),
        Token::COLON,
        Token::TINTEGER, 
        Token::EQUAL,
        Token::INT(vec!['5']),
        Token::SEMICOLON,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_function_declarations() {
    let input: &str = "fn add(a: Integer, b: Integer) -> Integer { return a + b; }";
    let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
    let expected: Vec<Token> = vec![
        Token::FUNCTION,
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
        Token::RETARROW,
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
    let input: &str = "fn check(x: Integer) { if x > 0 { return true; } else { return false; } }";
    let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
    let expected: Vec<Token> = vec![
        Token::FUNCTION,
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
        Token::INT(vec!['0']),
        Token::LBRACKET,
        Token::RETURN,
        Token::TRUE,
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::ELSE,
        Token::LBRACKET,
        Token::RETURN,
        Token::FALSE,
        Token::SEMICOLON,
        Token::RBRACKET,
        Token::RBRACKET,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_async() {
    let input: &str = "async";
    let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
    let expected: Vec<Token> = vec![
	Token::ASYNC,
	Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_await() {
    let input: &str = "await";
    let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
    let expected: Vec<Token> = vec![
        Token::AWAIT,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_try() {
    let input: &str = "try";
    let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
    let expected: Vec<Token> = vec![
        Token::TRY,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_catch() {
    let input: &str = "catch";
    let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
    let expected: Vec<Token> = vec![
        Token::CATCH,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_finally() {
    let input: &str = "finally";
    let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
    let expected: Vec<Token> = vec![
        Token::FINALLY,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_logical_operators_and_parentheses() {
    let input: &str = "let result = (5 > 3) && (2 < 4);";
    let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
    let expected: Vec<Token> = vec![
        Token::LET,
        Token::IDENTIFIER(vec!['r', 'e', 's', 'u', 'l', 't']),
        Token::EQUAL,
        Token::LPAREN,
        Token::INT(vec!['5']),
        Token::GREATERTHAN, 
        Token::INT(vec!['3']),
        Token::RPAREN,
        Token::LOGICALAND,
        Token::LPAREN,
        Token::INT(vec!['2']),
        Token::LESSTHAN, 
        Token::INT(vec!['4']),
        Token::RPAREN,
        Token::SEMICOLON,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_nested_function_calls() {
    let input: &str = "let val = add(multiply(2, 3), 4);";
    let result: Result<Vec<Token>, Vec<ErrorType>> = Lexer::lex(input);
    let expected: Vec<Token> = vec![
        Token::LET,
        Token::IDENTIFIER(vec!['v', 'a', 'l']),
        Token::EQUAL,
        Token::IDENTIFIER(vec!['a', 'd', 'd']),
        Token::LPAREN,
        Token::IDENTIFIER(vec!['m', 'u', 'l', 't', 'i', 'p', 'l', 'y']),
        Token::LPAREN,
        Token::INT(vec!['2']),
        Token::COMMA,
        Token::INT(vec!['3']),
        Token::RPAREN,
        Token::COMMA,
        Token::INT(vec!['4']),
        Token::RPAREN,
        Token::SEMICOLON,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_boolean_literals() {
    let input = "true false";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TRUE, Token::FALSE, Token::EOF,
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
    let input = "+ * / % ^";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::PLUS, Token::MULTIPLY,
        Token::DIVIDE, Token::MOD, Token::EXPONENT,
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));

    let input2 = "5 - 5";
    let result2 = Lexer::lex(input2);
    let expected2=vec![
        Token::INT(vec!['5']), 
        Token::DASH,
        Token::INT(vec!['5']),
        Token::EOF,
    ];
    assert_eq!(result2, Ok(expected2));
}

#[test]
fn test_logical_operators() {
    let input = "&& || !";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::LOGICALAND, Token::LOGICALOR, Token::LOGICALNOT,
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
    let input = "if elif else for while do break continue";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::IF, Token::ELIF, Token::ELSE,
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
    let input = "Integer Float Boolean String Char Void Unsigned Signed Object Array Double Long Char";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::TINTEGER, Token::TFLOAT, Token::TBOOLEAN,
        Token::TSTRING, Token::TCHAR, Token::TVOID,
        Token::TUSIGN, Token::TSIGN, Token::TOBJECT, Token::TARRAY,
        Token::TDOUBLE, Token::TLONG, Token::TCHAR, 
        Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_dot_and_coloncolon_operators() {
    let input = ". ::";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::DOT, Token::COLONCOLON, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_block_comments() {
    let input = "* / /* */ * /* * / /*";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::MULTIPLY, Token::DIVIDE, Token::BCOMMENTBEGIN,
        Token::BCOMMENTEND, Token::MULTIPLY, Token::BCOMMENTBEGIN,
        Token::MULTIPLY, Token::DIVIDE, Token::BCOMMENTBEGIN, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_use_as_pri(){
    let input: &str = "use greetings::hello as greeting";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::USE, Token::IDENTIFIER(vec!['g','r','e','e','t','i','n','g','s']),
        Token::COLONCOLON, Token::IDENTIFIER(vec!['h','e','l','l','o']), Token::AS,
        Token::IDENTIFIER(vec!['g','r','e','e','t','i','n','g']), Token::EOF,

    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_module_creation_and_pub(){
    let input: &str = "mod greetings {pub fn hello(){}}";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::MODULE, Token::IDENTIFIER(vec!['g','r','e','e','t','i','n','g','s']),
        Token::LBRACKET, Token::PUBLIC, Token::FUNCTION, Token::IDENTIFIER(vec!['h','e','l','l','o']),
        Token::LPAREN, Token::RPAREN, Token::LBRACKET, Token::RBRACKET, Token::RBRACKET, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_single_line_comment() {
    let input = "//";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::SCOMMENT, Token::EOF,
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_class() {
    let input = "class foo {}";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::CLASS, Token::IDENTIFIER(vec!['f', 'o', 'o']), 
        Token::LBRACKET, Token::RBRACKET, Token::EOF
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_init_and_self() {
    let input = "class foo { 
                    Integer x; 
                    fn init () {
                        self.x = 10;
                    }}";
    let result = Lexer::lex(input);
    let expected = vec![
        Token::CLASS, 
        Token::IDENTIFIER(vec!['f', 'o', 'o']), 
        Token::LBRACKET, 
        Token::EOL,
        Token::TINTEGER, 
        Token::IDENTIFIER(vec!['x']), 
        Token::SEMICOLON, 
        Token::EOL,
        Token::FUNCTION, 
        Token::INIT, 
        Token::LPAREN, 
        Token::RPAREN, 
        Token::LBRACKET, 
        Token::EOL,
        Token::SELF, 
        Token::DOT, 
        Token::IDENTIFIER(vec!['x']), 
        Token::EQUAL, 
        Token::INT(vec!['1', '0']), 
        Token::SEMICOLON, 
        Token::EOL,
        Token::RBRACKET, 
        Token::RBRACKET, 
        Token::EOF
    ];
    assert_eq!(result, Ok(expected));
}
