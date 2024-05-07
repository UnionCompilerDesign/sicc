/*
Converts raw text into lexemes
*/

use crate::{
    lexer::token::Token,
    utils::error::ErrorType,
};

/// Structure of the lexing process
pub struct Lexer {
    input: Vec<char>, // Source code
    position: usize, // Current position in source code
    current: char, // Current character being read
}

impl Lexer {
    /// Creates a new lexer
    fn new(input: Vec<char>) -> Self {
        Self {
            input,
            position: 0,
            current: '~', // EOF token, set initially but not necessarily the first token
        }
    }

    /// Returns a vector of tokens
    pub fn lex(input: &str) -> Result<Vec<Token>, Vec<ErrorType>> {
        let mut lexer: Lexer = Lexer::new(input.chars().collect());
        let mut errors: Vec<ErrorType> = Vec::new();
        let mut tokens: Vec<Token> = Vec::new();
        lexer.current = lexer.input[0];

        loop {
            let token: Result<Token, ErrorType> = lexer.next_token();
            match token {
                Ok(token) => {
                    if token == Token::EOF {
                        tokens.push(token);
                        break;
                    }
                    tokens.push(token);
                }
                Err(error) => {
                    errors.push(error);
                }
            }

        }
        if errors.is_empty() {
            return Ok(tokens);
        }
        Err(errors)
    }
    
    /// Advances the currently read character
    fn read_char(&mut self) {
        self.position += 1;
        if self.position >= self.input.len() {
            self.current = '~';
        } else {
            self.current = self.input[self.position];
        }
    }

    /// Ignores whitespace
    fn skip_whitespace(&mut self) {
        loop {
            if matches!(self.current, ' ' | '\t' | '\r') {
                self.read_char();
            }
            else {
                break;
            }
        }
    }

    /// Returns the current token type and advances to the next token
    fn next_token(&mut self) -> Result<Token, ErrorType> {
        self.skip_whitespace();

        let tok: Result<Token, ErrorType> = match self.current {
            '~' => Ok(Token::EOF),
            '\n' => Ok(Token::EOL),

            '/' => {
                if self.peek_char() == '*' {
                    self.read_char(); 
                    Ok(Token::BCOMMENTBEGIN)
                } else if self.peek_char() == '/' {
                    self.read_char();
                    Ok(Token::SCOMMENT)
                } else {
                    Ok(Token::DIVIDE)
                }
            },
            '-' => {
                if self.peek_char() == '>' {
                    self.read_char();
                    Ok(Token::POINTER)
                } else if self.peek_char() == '-' {
                    self.read_char();
                    Ok(Token::MINUSMINUS)
                }
                else{
                    Ok(Token::DASH)
                }
            },
            '+' => {
                if self.peek_char() == '+' {
                    self.read_char();
                    Ok(Token::PLUSPLUS)
                } else {
                    Ok(Token::PLUS)
                }
	        },
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Ok(Token::EQUALEQUAL)
                }
                else {
                    Ok(Token::EQUAL)
                }
            },

            '}' => Ok(Token::RBRACKET),
            '{' => Ok(Token::LBRACKET), // depending on your text editor, this character may cause problems, but
            '(' => Ok(Token::LPAREN),   //      the rustc compiler is fine with this
            ')' => Ok(Token::RPAREN),
            ';' => Ok(Token::SEMICOLON),
            ':' => Ok(Token::COLON),
            ',' => Ok(Token::COMMA),
            '%' => Ok(Token::MOD),
            '[' => Ok(Token::LBRACE),
            ']' => Ok(Token::RBRACE),
            '.' => Ok(Token::DOT),
	    '?' => Ok(Token::CTRUE),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Ok(Token::NOTEQUAL)
                }
                else {
                    Ok(Token::LOGICALNOT)
                }
            },
            '*' => {
                if self.peek_char() == '/' {
                    self.read_char(); 
                    Ok(Token::BCOMMENTEND)
                } else {
                    Ok(Token::MULTIPLY)
                }
            },
            '^' => Ok(Token::XOR),
            
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char(); 
                    Ok(Token::LESSTHANEQUAL)
                } else {
                    Ok(Token::LESSTHAN)
                }
            },
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char(); 
                    Ok(Token::GREATERTHANEQUAL)
                } else {
                    Ok(Token::GREATERTHAN)
                }
            },

            '&' => {
                if self.peek_char() == '&' {
                    self.read_char(); 
                    self.read_char(); 
                    Ok(Token::LOGICALAND)
                } else {
                    Ok(Token::BITAND)
                }
            },
            '|' => {
                if self.peek_char() == '|' {
                    self.read_char(); 
                    self.read_char(); 
                    Ok(Token::LOGICALOR)
                } else {
                    Ok(Token::BITOR)
                }
            },
            _ if is_letter(self.current) => {
                let identifier: Vec<char> = self.read_identifier();
                Ok(get_token(&identifier).unwrap_or_else(|_| Token::IDENTIFIER(identifier))) // i don't love this solution
            },
            _ if is_digit(self.current) => Ok(Token::INT(self.read_number())),

            _ => { 
                let mut err_token = String::new();
                err_token.push(self.current);
                Err(ErrorType::UnrecognizedToken { token: err_token })
            },
        };

        self.read_char();
        tok
    }

    /// Reads an identifier from the input.
    fn read_identifier(&mut self) -> Vec<char> {
        self.read_while(is_letter)
    }

    /// Reads a number from the input.
    fn read_number(&mut self) -> Vec<char> {
        self.read_while(is_digit)
    }

    /// Gives the next character without changing the position
    fn peek_char(&self) -> char {
        if self.position + 1 >= self.input.len() {
            '~' // EOF token
        } else {
            self.input[self.position + 1]
        }
    }

    /// Reads characters from the input while the given predicate is true.
    fn read_while<F>(&mut self, predicate: F) -> Vec<char>
        where
            F: Fn(char) -> bool,
        {
            let start_pos = self.position;
            while self.position < self.input.len() && predicate(self.current) {
                self.read_char();
            }
            self.position = self.position - 1; // hacky solution, fix later
            self.input[start_pos..=self.position].to_vec() 
        }
}

fn is_letter(current: char) -> bool {
    'a' <= current && current <= 'z' || 
        'A' <= current && current <= 'Z' || current == '_'
}

fn is_digit(current: char) -> bool {
    '0' <= current && current <= '9'
}

/// retrieves a token if text matches, else error
fn get_token(raw_text: &Vec<char>) -> Result<Token, ErrorType> {
    let identifier: String = raw_text.into_iter().collect();
    match &identifier[..] {
        "if" => Ok(Token::IF),
        "else" => Ok(Token::ELSE),
        "int" => Ok(Token::TINTEGER),
        "float" => Ok(Token::TFLOAT),
        "long" => Ok(Token::TLONG),
        "return" => Ok(Token::RETURN),
        "unsigned" => Ok(Token::TUSIGN),
        "signed" => Ok(Token::TSIGNINT),
        "switch" => Ok(Token::SWITCH),
        "struct" => Ok(Token::STRUCT),
        "enum" => Ok(Token::ENUM),
        "void" => Ok(Token::TVOID),
        "char" => Ok(Token::TCHAR),
        "for" => Ok(Token::FOR),
        "break" => Ok(Token::BREAK),
        "do" => Ok(Token::DO),
        "while" => Ok(Token::WHILE),
        "continue" => Ok(Token::CONTINUE),
        "case" => Ok(Token::CASE),
        "const" => Ok(Token::CONST),
        "double" => Ok(Token::TDOUBLE),
        _ => Err(ErrorType::UnrecognizedToken { token: String::from("Unrecognized token") }),
    }
}
