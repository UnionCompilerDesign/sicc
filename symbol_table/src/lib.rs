//! # Symbol Table Stack Module
//!
//! This module is responsible for providing the representation and generation of a symbol table stack 
//! for later stages of the compiler.
//! 
//! ## Structure
//!
//! The `symbol_table` module is organized into submodules that handle different aspects of the symbol table stack:
//!
//! * `core`: Defines the symbol table stack structure and drives the generation process.
//! * `statement`: Handles stack generation for statements and operations within blocks.
//! * `block`: Handles stack generation for code blocks.

/// Definitions of the symbol table stack structure and core of the symbol table stack generation process
pub mod core;

/// Stack generation for nodes with code blocks
mod block;

/// Stack generation for nodes within code blocks
mod statement;