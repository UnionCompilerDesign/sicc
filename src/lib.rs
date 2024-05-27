//! This file serves as the main entry point for the `starter_code` library, as part of the SLICC project.

extern crate llvm_sys as llvm;
extern crate threadpool;

pub extern crate lexer;
pub extern crate parser;
pub extern crate sem_analysis;
pub extern crate symbol_table;
pub extern crate common;
pub extern crate ir;
pub extern crate integration;

/// Contains the main driver function for the compiler.
///
/// The `compile` module defines the stages of compilation, including lexing, parsing,
/// semantic analysis, and IR code generation. It is responsible for taking source code files,
/// processing them through these stages, and generating an pre-compiled LLVM module.
pub mod compile;

/// Contains functionality to run pre-compiled LLVM modules.
///
/// The `run` module defines functions for executing pre-compiled modules,
/// including setting up the execution environment and handling runtime parameters.
pub mod run;
