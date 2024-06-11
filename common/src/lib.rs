//! The `common` crate provides utilities and definitions used across SLICC.
//! It contains a collection of modules that support various aspects of the compilation process, including
//! the definition of `AST`, constants, preprocessing, error handling, and concurrent execution. 

/// Contains the abstract syntax tree (`AST`) structure and utilities.
pub mod ast;

/// Defines constants.
pub mod constants;

/// Defines error types.
pub mod error;

