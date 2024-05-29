//! This file defines the the main driver function for the compiler, which drives the stages of
//! compilation, including lexing, parsing, semantic analysis, and code generation.

use std::{fs, path::Path};
use common::{ast::ast_struct::AST, constants::DEFAULT_PRIORITY_MODELEMENT, entry_points::entry_points, error::ErrorType};
use integration::module::{ast_stitch, ModElement, Module};
use lexer::{lexer_core::Lexer, token::Token};
use parser::parser_core::Parser;
use symbol_table::symbol_table_struct::SymbolTableStack;
use sem_analysis::sem_analysis_core::SemAnalysis;

/// Main driver of the compiler.
///
/// This function drives the compilation process, starting from reading the source file,
/// identifying entry points, generating module elements, and finally producing a compiled LLVM module.
///
/// # Parameters
/// * `file_path` - The path to the source file to be compiled.
/// * `jit` - A boolean indicating whether to perform Just-In-Time compilation.
/// * `emit_ir` - A boolean indicating whether to emit intermediate representation (IR) code.
///
/// # Returns
/// A result containing a vector of bytes representing the compiled object code on success,
/// or a vector of `ErrorType` on failure.
pub fn compile(file_path: &str, jit: bool, emit_ir: bool) -> Result<Vec<u8>, Vec<ErrorType>> {
    let path: &Path = Path::new(file_path);
    validate_file_path(path, file_path)?;

    let entry_points: Vec<usize> = entry_points(path);
    let content: String = fs::read_to_string(path).expect("no file");

    let mut mod_elements: Vec<ModElement> = Vec::new();

    if entry_points.is_empty() {
        panic!("empty file");
    }

    for window in entry_points.windows(2) {
        let start: usize = window[0];
        let end: usize = window[1];
        let slice: &str = &content[start..end];

        match generate_mod_element(slice.to_string()) {
            Ok(ast_with_sym_table) => mod_elements.push(ast_with_sym_table),
            Err(errors) => return Err(errors),
        }
    }

    let last_start: usize = *entry_points.last().unwrap();
    match generate_mod_element(content[last_start..].to_string()) {
        Ok(mod_element) => mod_elements.push(mod_element),
        Err(errors) => return Err(errors),
    }

    let mod_ast: Module = ast_stitch(mod_elements);

    ast_to_obj(mod_ast, jit, emit_ir)
}

/// Ensures the passed in file exists.
///
/// This function checks whether the specified file path exists and is a file.
///
/// # Parameters
/// * `path` - The path to the file.
/// * `file_path` - The string representation of the file path for error messages.
///
/// # Returns
/// An `Ok` result if the file exists, or an error of type `ErrorType`.
fn validate_file_path(path: &Path, file_path: &str) -> Result<(), Vec<ErrorType>> {
    if !path.exists() || !path.is_file() {
        eprintln!("Error: File not found - {}", file_path);
        panic!("file not found"); 
    }
    Ok(())
}

/// Generates a module element from an input program.
///
/// This function performs lexing, parsing, and symbol table generation for a slice of the source code,
/// producing a `ModElement` if successful.
///
/// # Parameters
/// * `content` - A string slice of the source code to be processed.
///
/// # Returns
/// A result containing a `ModElement` on success, or a vector of `ErrorType` on failure.
fn generate_mod_element(content: String) -> Result<ModElement, Vec<ErrorType>> {
    let tokens: Vec<Token> = Lexer::lex(&content)?;
    let ast: AST = Parser::parse(tokens)?;
    match SymbolTableStack::gen_sym_table_stack(ast) {
        Ok((ast, symbol_table_stack)) => {
            Ok(ModElement::new(ast, symbol_table_stack, DEFAULT_PRIORITY_MODELEMENT))
        }
        Err(e) => {
            Err(e)
        }
    }
}

/// Generates object code from a module.
///
/// This function performs semantic analysis on the module and generates the final object code.
/// The object code can be either Just-In-Time compiled or statically compiled based on the parameters.
///
/// # Parameters
/// * `content` - The `Module` to be compiled.
/// * `jit` - A boolean indicating whether to perform Just-In-Time compilation.
/// * `emit_ir` - A boolean indicating whether to emit intermediate representation (IR) code.
///
/// # Returns
/// A result containing a vector of bytes representing the compiled object code on success,
/// or a vector of `ErrorType` on failure.
fn ast_to_obj(content: Module, jit: bool, emit_ir: bool) -> Result<Vec<u8>, Vec<ErrorType>> {
    // let sem_analysis_result: Result<Module, Vec<ErrorType>> = SemAnalysis::sem_analysis(content);

    // match sem_analysis_result {
    //     Ok(_processed_content) => {
    //         // let _generated_ir: ModuleTag = IRGenerator::generate_ir(processed_content); 
    //         eprintln!("Successfully Compiled.");
    //         if emit_ir {
    //             // match write_ir::write_to_file(unsafe { &*generated_ir.get_ref() }, "output_builder.ll") { 
    //             //     Ok(_) => eprintln!("IR written to file."),
    //             //     Err(e) => {
    //             //         eprintln!("File write error: {}", e);
    //             //         panic!()
    //             //     }
    //             // }
    //         }

    //         if jit {
    //             // match ExecutionEngine::execute_ir(generated_ir, &[]) { 
    //             //     Ok(_) => println!("Executed using JIT."),
    //             //     Err(e) => {
    //             //         eprintln!("Execution error: {}", e);
    //             //         panic!()
    //             //     }
    //             // }
    //         }

    //         Ok(Vec::new()) 
    //     },
    //     Err(sem_analysis_errors) => {
    //         for error in &sem_analysis_errors {
    //             eprintln!("Syntax Error: {:?}", error);
    //         }
    //         Err(sem_analysis_errors)
    //     }
    // }
    todo!("Needs to be revamped")
}
