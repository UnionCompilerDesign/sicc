use std::sync::{Arc, RwLock};

use safe_llvm::{
    jit::{execution_engine, target::TargetConfigurator}, 
    memory_management::pointer::CPointer
};

/// Executes a pre-compiled LLVM module. 
pub fn execute<T: TargetConfigurator>(
    module: Arc<RwLock<CPointer>>, 
    debug_info: bool,
    target_configurator: T,
    entry_function_name: String
) {
    let mut execution_engine = execution_engine::ExecutionEngine::new(module, debug_info);
    execution_engine.init_target(target_configurator).expect("Failed to initialize target");
    let args: Vec<String> = Vec::new();
    execution_engine.execute::<i64, Vec<std::string::String>>(&entry_function_name, args).expect("Failed to execute module");
}
