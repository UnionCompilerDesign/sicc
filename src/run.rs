//! This file provides functionality for executing pre-compiled LLVM modules.

use std::sync::{Arc, RwLock};
use safe_llvm::{
    jit::{execution_engine, target::TargetConfigurator}, 
    memory_management::pointer::CPointer
};

/// Executes a pre-compiled LLVM module.
/// 
/// This function initializes an execution engine with the given module and target configurator, and then executes
/// a specified entry function within the module. It supports passing arguments to the entry function and toggling
/// debug information.
///
/// # Type Parameters
/// * `TargetType` - A type that implements the `TargetConfigurator` trait, used to configure the target for execution.
///
/// # Parameters
/// * `module` - An `Arc<RwLock<CPointer>>` pointing to the pre-compiled LLVM module to be executed.
/// * `debug_info` - A boolean indicating whether to enable debug information during execution.
/// * `target_configurator` - An instance of TargetConfigurator used to configure the execution target.
/// * `entry_function_name` - A string specifying the name of the entry function to execute within the module.
///
/// ```
pub fn execute<TargetType: TargetConfigurator>(
    module: Arc<RwLock<CPointer>>, 
    debug_info: bool,
    target_configurator: TargetType,
    entry_function_name: String
) {
    let mut execution_engine = execution_engine::ExecutionEngine::new(module, debug_info);
    execution_engine.init_target(target_configurator).expect("Failed to initialize target");
    let args: Vec<String> = Vec::new();
    execution_engine.execute::<i64, Vec<std::string::String>>(&entry_function_name, args).expect("Failed to execute module");
}
