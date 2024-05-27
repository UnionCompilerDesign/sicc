//! # Intermediate Representation (IR) Module
//!
//! This module is responsible for generating LLVM intermediate representation (IR). It is the final step 
//! of the compiler before the backend.  
//!
//! ## Structure
//!
//! The `ir` module is organized into submodules that handle different aspects of LLVM IR generation:
//!
//! - `core`: Drives the generation process.
//! - `block`: Handles generation for block-containing elements like loops and functions.
//! - `statement`: Handles generation for statements and operations within blocks.
//! - `primitive`: Handles generation for primitive data types and operations.


/// Core of the LLVM IR generation process.
pub mod core;

/// LLVM IR generation for expressions containing blocks like loops, functions, enums, etc.
mod block;

/// LLVM IR for statements and operations within blocks.
mod statement;

/// LLVM IR generation for primitive data types and operations.
mod primitive;

/// Allocation store for managing variable allocations across scopes. 
mod store;