//! SLICC (Simple Lightweight Instructional C Compiler)
//!
//! This file defines the command-line interface (CLI) for the SLICC project.

use clap::Parser;
use compiler_core::compile;

/// Defines the command-line interface for SLICC.
///
/// This struct uses the `clap` crate to parse command-line arguments and provide metadata about the program.
#[derive(Parser, Debug)]
#[clap(author = "Union College", version = "0.1.0", about = "SLICC (Simple Lightweight Instructional C Compiler)")]
struct Cli {
    /// The subcommand to execute, either `compile` or `run`.
    #[clap(subcommand)]
    command: Commands,
}

/// Defines the subcommands for the CLI.
///
/// This enum lists the supported subcommands: `compile` for compiling SLICC programs and `run` for running them.
#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Compiles a SLICC program.
    ///
    /// # Parameters
    /// * `file` - The path to the SLICC source file to be compiled.
    /// * `jit` - A boolean flag to indicate if Just-In-Time compilation should be performed.
    /// * `emit_ir` - A boolean flag to indicate if the intermediate representation (IR) should be emitted.
    Compile {
        /// The path to the source file.
        file: String,
        
        /// Enables Just-In-Time (JIT) compilation.
        #[clap(long)]
        jit: bool,
    
        /// Emits intermediate representation (IR).
        #[clap(long)]
        emit_ir: bool,
    },
    /// Runs a pre-compiled SLICC program.
    ///
    /// # Parameters
    /// * `file` - The path to the pre-compiled SLICC binary to be executed.
    Run {
        /// The path to the binary file.
        file: String,
    },
}

/// Entry point for the SLICC Compiler and Runner.
///
/// This function parses the command-line arguments and executes the corresponding subcommand.
fn main() {
    let builder: Cli = Cli::parse();

    match &builder.command {
        Commands::Compile { file, jit, emit_ir } => {
            let _ = compile::compile(file, *jit, *emit_ir);
        },
        Commands::Run { file: _ } => {
            unimplemented!("Running unimplemented")
        }
    }
}
