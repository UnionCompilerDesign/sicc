/// Lexer Module
///
/// This module converts our source code into tokens to be sent to the parser.
/// Though this module is simple, it is important for efficiency and allowing the other modules to be simpler. 
///
/// ## Structure
///
/// The `lexer` module is organized into submodules:
///
/// - `core`: Core takes source code as input and outputs tokens and ignores comments and whitespace. 
/// - 'token': Tokens are the base unit of our compiler. This module contains accepted tokens.

/// Core of the Lexer
pub mod core;

/// Lists accepted tokens
pub mod token;
