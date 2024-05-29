//! The `common` crate provides utilities and definitions used across SLICC.
//! It contains a collection of modules that support various aspects of the compilation process, including
//! the definition of `AST`, constants, preprocessing, error handling, and concurrent execution. 

/// Contains the abstract syntax tree (`AST`) structure and utilities.
pub mod ast;

/// Defines constants.
pub mod constants;

/// Provides functionality to locate entry points within a source file.
pub mod entry_points;

/// Defines error types.
pub mod error;

/// Provides concurrent execution using a thread pool.
pub mod thread_pool_executor;

