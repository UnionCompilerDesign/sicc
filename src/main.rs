//! Charge Compiler and Runner

use clap::Parser;
use compiler_core::compile;

#[derive(Parser, Debug)]
#[clap(author="Union College", 
        version="0.1.0", 
        about="Charge Language Compiler and Runner" )]
        
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    Compile {
        file: String,
        
        #[clap(long)]
        jit: bool,
    
        #[clap(long)]
        emit_ir: bool,
    },
    Run {
        file: String,
    },
}

fn main() {
    let builder: Cli = Cli::parse();

    match &builder.command {
        Commands::Compile { file, jit, emit_ir} => {
            let _ = compile::compile(file, *jit, *emit_ir);
        },
        Commands::Run { file: _ } => {
            unimplemented!("Running unimplemented")
        }
    }
}
