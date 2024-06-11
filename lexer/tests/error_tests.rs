use common::error::ErrorType;
use lexer::{
    lexer_core::Lexer,
    token::Token,
};

/// cargo test --test error_tests
/// Tests combinations of tokens that should flag an error. 

#[test]
fn test_dollar_sign() {
    let input = "let $invalid = 5;";
    let result = Lexer::lex(input);
    let expected_error = ErrorType::UnrecognizedToken{token: "$".to_string()};
    let expected = Err(vec![expected_error]);
    assert_eq!(result, expected);
}

#[test]
fn test_hashtag_in_statement() {
    let input = "if #invalid == 2;";
    let result = Lexer::lex(input);
    let expected_error = ErrorType::UnrecognizedToken{token: "#".to_string()};
    let expected = Err(vec![expected_error]);
    assert_eq!(result, expected);
}

#[test]
fn test_hashtag_allone() {
    let input = "#";
    let result = Lexer::lex(input);
    let expected_error = ErrorType::UnrecognizedToken{token: "#".to_string()};
    let expected = Err(vec![expected_error]);
    assert_eq!(result, expected);
}

#[test]
fn test_at_alone() {
    let input = "@";
    let result = Lexer::lex(input);
    let expected_error = ErrorType::UnrecognizedToken{token: "@".to_string()};
    let expected = Err(vec![expected_error]);
    assert_eq!(result, expected);
}

#[test]
fn test_at_in_statement() {
    let input = "let @x = a + b;";
    let result = Lexer::lex(input);
    let expected_error = ErrorType::UnrecognizedToken{token: "@".to_string()};
    let expected = Err(vec![expected_error]);
    assert_eq!(result, expected);
}
