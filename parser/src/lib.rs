//! # Parser Module
//!
//! This module is responsible for generating the abstract syntax tree (AST) from a vector of tokens. It is the step before
//! semantic analysis and after lexing.
//!
//! ## Structure
//!
//! The `parser` module is organized into submodules that handle different aspects parsing generation:
//!
//! - `core`: Drives the generation process.
//! - `statement`: Handles generation for statements and operations within blocks.
//! - `block`: Handles generation for code blocks.
//! - `primitive`: Handles generation for primitive data types and operations.

/// Core of the parsing process.
pub mod core;

/// Parses statements and operations within blocks.
mod statement;

/// Parses expressions containing blocks like loops, functions, enums, etc.
mod block;

/// Parses primitive data types and operations.
mod primitive;

