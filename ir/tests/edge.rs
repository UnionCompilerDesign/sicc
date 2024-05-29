use std::sync::{Arc, Mutex};

use ir::core::IRGenerator;

use common::
        ast::{
            ast_struct::{
                ASTNode, AST
            }, 
            data_type::DataType, 
            syntax_element::SyntaxElement
        };
use symbol_table::symbol_table_struct::{SymbolInfo, SymbolTable, SymbolTableStack, SymbolValue};
use integration::module::{
    ModElement, Module, ast_stitch
};
 
use safe_llvm::{memory_management::resource_pools::ResourcePools, utils::utils_struct::Utils};

pub const DEFAULT_PRIORITY_MODELEMENT: i32 = -1;

fn wrap_in_tle(ast_node: ASTNode) -> AST {
    let mut tle: ASTNode = ASTNode::new(SyntaxElement::TopLevelExpression);
    tle.add_child(ast_node);
    AST::new(tle)
}

// #[test]
// fn test_deeply_nested_loops1() {
//     /*
//     int testDeeplyNestedLoops() {
//         for (int i = 0) {
//             while (i) {
//                 do {
//                     for (int j = 0) {
//                         while (1) {
//                             do {
//                                 return 0;
//                             } while (1);
//                         }
//                     }
//                 } while (2);
//             }
//         }
//     }
//     */
//     let mut function_ast = ASTNode::new(SyntaxElement::FunctionDeclaration);
//     let fn_id = ASTNode::new(SyntaxElement::Identifier("testDeeplyNestedLoops".to_string()));
//     let fn_type = ASTNode::new(SyntaxElement::Type(DataType::Integer));
//     let mut fn_block = ASTNode::new(SyntaxElement::BlockExpression);

//     let mut for_loop = ASTNode::new(SyntaxElement::ForLoop);
//     let mut for_init = ASTNode::new(SyntaxElement::Initialization);

//     let mut for_init_var = ASTNode::new(SyntaxElement::Variable);
//     for_init_var.add_child(ASTNode::new(SyntaxElement::Identifier("i".to_string())));
//     for_init_var.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));

//     let for_init_val = ASTNode::new(SyntaxElement::Literal("0".to_string())); 
//     for_init.add_child(for_init_var);
//     for_init.add_child(for_init_val);

//     let mut for_body = ASTNode::new(SyntaxElement::BlockExpression);

//     let mut while_loop = ASTNode::new(SyntaxElement::WhileLoop);
//     let while_cond = ASTNode::new(SyntaxElement::Literal("i".to_string())); 
//     let mut while_body = ASTNode::new(SyntaxElement::BlockExpression);

//     let mut do_while_loop = ASTNode::new(SyntaxElement::DoWhileLoop);
//     let do_while_cond = ASTNode::new(SyntaxElement::Literal("2".to_string())); 
//     let mut do_while_body = ASTNode::new(SyntaxElement::BlockExpression);

//     let mut nested_for_loop = ASTNode::new(SyntaxElement::ForLoop);
//     let mut nested_for_init = ASTNode::new(SyntaxElement::Initialization);
//     let mut nested_for_init_var = ASTNode::new(SyntaxElement::Variable);
//     nested_for_init_var.add_child(ASTNode::new(SyntaxElement::Identifier("j".to_string())));
//     nested_for_init_var.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));

//     let nested_for_init_val = ASTNode::new(SyntaxElement::Literal("0".to_string()));
//     nested_for_init.add_child(nested_for_init_var);
//     nested_for_init.add_child(nested_for_init_val);

//     let mut nested_for_body = ASTNode::new(SyntaxElement::BlockExpression);

//     let mut nested_while_loop = ASTNode::new(SyntaxElement::WhileLoop);
//     let nested_while_cond = ASTNode::new(SyntaxElement::Literal("1".to_string())); 
//     let mut nested_while_body = ASTNode::new(SyntaxElement::BlockExpression);

//     let mut nested_do_while_loop = ASTNode::new(SyntaxElement::DoWhileLoop);
//     let mut nested_do_while_body = ASTNode::new(SyntaxElement::Return);
//     nested_do_while_body.add_child(ASTNode::new(SyntaxElement::Literal("0".to_string())));
//     let nested_do_while_cond = ASTNode::new(SyntaxElement::Literal("1".to_string())); 

//     nested_do_while_loop.add_child(nested_do_while_body);
//     nested_do_while_loop.add_child(nested_do_while_cond);

//     nested_while_body.add_child(nested_do_while_loop);
//     nested_while_loop.add_child(nested_while_cond);
//     nested_while_loop.add_child(nested_while_body);

//     nested_for_body.add_child(nested_while_loop);
//     nested_for_loop.add_child(nested_for_init);
//     nested_for_loop.add_child(nested_for_body);

//     do_while_body.add_child(nested_for_loop);
//     do_while_loop.add_child(do_while_body);
//     do_while_loop.add_child(do_while_cond);

//     while_body.add_child(do_while_loop);
//     while_loop.add_child(while_cond);
//     while_loop.add_child(while_body);

//     for_body.add_child(while_loop);
//     for_loop.add_child(for_init);
//     for_loop.add_child(for_body);

//     fn_block.add_child(for_loop);
//     function_ast.add_child(fn_id);
//     function_ast.add_child(fn_type);
//     function_ast.add_child(fn_block);

//     let ast = wrap_in_tle(function_ast);
//     let mut symbol_table_stack = SymbolTableStack::new();
//     let mut symbol_table_global = SymbolTable::new();
//     let fn_value = SymbolValue::FunctionValue { parameters: Vec::new() };
//     let fn_info = SymbolInfo::new(DataType::Integer, fn_value);
//     symbol_table_global.add("testDeeplyNestedLoops1".to_string(), fn_info);
//     symbol_table_stack.push(symbol_table_global);

//     let mod_ast: Module = ast_stitch(vec![ModElement::new(ast, symbol_table_stack, DEFAULT_PRIORITY_MODELEMENT)]);
//     let mut ir_generator = IRGenerator::new();
//     let module_tag = ir_generator.generate_ir(mod_ast);
//     let pools: Arc<Mutex<ResourcePools>> = ir_generator.get_resource_pools();
//     let pools = pools.try_lock().expect("Failed to lock resource pool mutex in deeply nested loops IR!");

//     let module = pools.get_module(module_tag).expect("Failed to get module");
//     let write_result = Utils::write_to_file(module.clone(), "output_deeply_nested_loops1.ll");
//     match write_result {
//         Ok(_) => println!("Deeply nested loops test file written correctly!"),
//         Err(e) => panic!("Deeply nested loops test file failed to write! Error: {}", e)
//     }
// }

// #[test]
// fn test_deeply_nested_loops2() {
//     /*
//     int testDeeplyNestedLoops2() {
//         while (1) {
//             for (2) {
//                 do {
//                     while (3) {
//                         for (0) {
//                             do {
//                                 return 0;
//                             } while (1);
//                         }
//                     }
//                 } while (3);
//             }
//         }
//     }
//     */
//     let mut function_ast = ASTNode::new(SyntaxElement::FunctionDeclaration);
//     let fn_id = ASTNode::new(SyntaxElement::Identifier("testSwappedWhileForLoops".to_string()));
//     let fn_type = ASTNode::new(SyntaxElement::Type(DataType::Integer));
//     let mut fn_block = ASTNode::new(SyntaxElement::BlockExpression);

//     let mut outer_while_loop = ASTNode::new(SyntaxElement::WhileLoop);
//     let outer_while_cond = ASTNode::new(SyntaxElement::Literal("1".to_string())); 
//     let mut outer_while_body = ASTNode::new(SyntaxElement::BlockExpression);

//     let mut for_loop = ASTNode::new(SyntaxElement::ForLoop);
//     for_loop.add_child(ASTNode::new(SyntaxElement::Literal("2".to_string()))); // Simulating the `for (2)` placeholder

//     let mut do_while_loop = ASTNode::new(SyntaxElement::DoWhileLoop);
//     let do_while_cond = ASTNode::new(SyntaxElement::Literal("3".to_string()));
//     let mut do_while_body = ASTNode::new(SyntaxElement::BlockExpression);

//     let mut inner_while_loop = ASTNode::new(SyntaxElement::WhileLoop);
//     let inner_while_cond = ASTNode::new(SyntaxElement::Literal("3".to_string()));
//     let mut inner_while_body = ASTNode::new(SyntaxElement::BlockExpression);

//     let mut inner_for_loop = ASTNode::new(SyntaxElement::ForLoop);
//     inner_for_loop.add_child(ASTNode::new(SyntaxElement::Literal("0".to_string()))); // Simulating the `for (0)` placeholder

//     let mut inner_do_while_loop = ASTNode::new(SyntaxElement::DoWhileLoop);
//     let mut inner_do_while_body = ASTNode::new(SyntaxElement::Return);
//     inner_do_while_body.add_child(ASTNode::new(SyntaxElement::Literal("0".to_string())));
//     let inner_do_while_cond = ASTNode::new(SyntaxElement::Literal("1".to_string()));

//     inner_do_while_loop.add_child(inner_do_while_body);
//     inner_do_while_loop.add_child(inner_do_while_cond);

//     inner_for_loop.add_child(inner_do_while_loop);
//     inner_while_body.add_child(inner_for_loop);
//     inner_while_loop.add_child(inner_while_cond);
//     inner_while_loop.add_child(inner_while_body);

//     do_while_body.add_child(inner_while_loop);
//     do_while_loop.add_child(do_while_body);
//     do_while_loop.add_child(do_while_cond);

//     for_loop.add_child(do_while_loop);
//     outer_while_body.add_child(for_loop);
//     outer_while_loop.add_child(outer_while_cond);
//     outer_while_loop.add_child(outer_while_body);

//     fn_block.add_child(outer_while_loop);
//     function_ast.add_child(fn_id);
//     function_ast.add_child(fn_type);
//     function_ast.add_child(fn_block);

//     let ast = wrap_in_tle(function_ast);
//     let mut symbol_table_stack = SymbolTableStack::new();
//     let mut symbol_table_global = SymbolTable::new();
//     let fn_value = SymbolValue::FunctionValue { parameters: Vec::new() };
//     let fn_info = SymbolInfo::new(DataType::Integer, fn_value);
//     symbol_table_global.add("testSwappedWhileForLoops".to_string(), fn_info);
//     symbol_table_stack.push(symbol_table_global);

//     let mod_ast: Module = ast_stitch(vec![ModElement::new(ast, symbol_table_stack, DEFAULT_PRIORITY_MODELEMENT)]);
//     let mut ir_generator = IRGenerator::new();
//     let module_tag = ir_generator.generate_ir(mod_ast);
//     let pools: Arc<Mutex<ResourcePools>> = ir_generator.get_resource_pools();
//     let pools = pools.try_lock().expect("Failed to lock resource pool mutex in deeply nested loops IR!");

//     let module = pools.get_module(module_tag).expect("Failed to get module");
//     let write_result = Utils::write_to_file(module.clone(), "output_deeply_nested_loops2.ll");
//     match write_result {
//         Ok(_) => println!("Swapped while-for loops test file written correctly!"),
//         Err(e) => panic!("Swapped while-for loops test file failed to write! Error: {}", e)
//     }
// }

// #[test]
// fn test_deply_nested_loops3() {
//     /*
//     int testMultipleDoWhileLoops3() {
//         do {
//             do {
//                 for (int i = 0) {
//                     while (1) {
//                         do {
//                             return 0;
//                         } while (1);
//                     }
//                 }
//             } while (2);
//         } while (3);
//     }
//     */
//     let mut function_ast = ASTNode::new(SyntaxElement::FunctionDeclaration);
//     let fn_id = ASTNode::new(SyntaxElement::Identifier("testMultipleDoWhileLoops3".to_string()));
//     let fn_type = ASTNode::new(SyntaxElement::Type(DataType::Integer));
//     let mut fn_block = ASTNode::new(SyntaxElement::BlockExpression);

//     let mut outer_do_while_loop = ASTNode::new(SyntaxElement::DoWhileLoop);
//     let outer_do_while_cond = ASTNode::new(SyntaxElement::Literal("3".to_string()));
//     let mut outer_do_while_body = ASTNode::new(SyntaxElement::BlockExpression);

//     let mut middle_do_while_loop = ASTNode::new(SyntaxElement::DoWhileLoop);
//     let middle_do_while_cond = ASTNode::new(SyntaxElement::Literal("2".to_string()));
//     let mut middle_do_while_body = ASTNode::new(SyntaxElement::BlockExpression);

//     let mut for_loop = ASTNode::new(SyntaxElement::ForLoop);
//     let mut for_init = ASTNode::new(SyntaxElement::Initialization);
//     let mut for_init_var = ASTNode::new(SyntaxElement::Variable);
//     for_init_var.add_child(ASTNode::new(SyntaxElement::Identifier("i".to_string())));
//     for_init_var.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));
//     let for_init_val = ASTNode::new(SyntaxElement::Literal("0".to_string()));
//     for_init.add_child(for_init_var);
//     for_init.add_child(for_init_val);

//     let mut for_body = ASTNode::new(SyntaxElement::BlockExpression);
//     let mut while_loop = ASTNode::new(SyntaxElement::WhileLoop);
//     let while_cond = ASTNode::new(SyntaxElement::Literal("1".to_string()));
//     let mut while_body = ASTNode::new(SyntaxElement::BlockExpression);

//     let mut inner_do_while_loop = ASTNode::new(SyntaxElement::DoWhileLoop);
//     let mut inner_do_while_body = ASTNode::new(SyntaxElement::Return);
//     inner_do_while_body.add_child(ASTNode::new(SyntaxElement::Literal("0".to_string())));
//     let inner_do_while_cond = ASTNode::new(SyntaxElement::Literal("1".to_string()));

//     inner_do_while_loop.add_child(inner_do_while_body);
//     inner_do_while_loop.add_child(inner_do_while_cond);

//     while_body.add_child(inner_do_while_loop);
//     while_loop.add_child(while_cond);
//     while_loop.add_child(while_body);

//     for_body.add_child(while_loop);
//     for_loop.add_child(for_init);
//     for_loop.add_child(for_body);

//     middle_do_while_body.add_child(for_loop);
//     middle_do_while_loop.add_child(middle_do_while_body);
//     middle_do_while_loop.add_child(middle_do_while_cond);

//     outer_do_while_body.add_child(middle_do_while_loop);
//     outer_do_while_loop.add_child(outer_do_while_body);
//     outer_do_while_loop.add_child(outer_do_while_cond);

//     fn_block.add_child(outer_do_while_loop);
//     function_ast.add_child(fn_id);
//     function_ast.add_child(fn_type);
//     function_ast.add_child(fn_block);

//     let ast = wrap_in_tle(function_ast);
//     let mut symbol_table_stack = SymbolTableStack::new();
//     let mut symbol_table_global = SymbolTable::new();
//     let fn_value = SymbolValue::FunctionValue { parameters: Vec::new() };
//     let fn_info = SymbolInfo::new(DataType::Integer, fn_value);
//     symbol_table_global.add("testMultipleDoWhileLoops".to_string(), fn_info);
//     symbol_table_stack.push(symbol_table_global);

//     let mod_ast: Module = ast_stitch(vec![ModElement::new(ast, symbol_table_stack, DEFAULT_PRIORITY_MODELEMENT)]);
//     let mut ir_generator = IRGenerator::new();
//     let module_tag = ir_generator.generate_ir(mod_ast);
//     let pools: Arc<Mutex<ResourcePools>> = ir_generator.get_resource_pools();
//     let pools = pools.try_lock().expect("Failed to lock resource pool mutex in multiple do while loops IR!");

//     let module = pools.get_module(module_tag).expect("Failed to get module");
//     let write_result = Utils::write_to_file(module.clone(), "output_deeply_nested_loops3.ll");
//     match write_result {
//         Ok(_) => println!("Multiple do-while loops test file written correctly!"),
//         Err(e) => panic!("Multiple do-while loops test file failed to write! Error: {}", e)
//     }
// }

// #[test]
// fn test_variables() {
//     /*
//     int testVariableEdgeCases() {
//         int x = 0;
//         if (x) {
//             int x = 1; 
//             while (x) {
//                 int x = x + 2; 
//                 if (x) {
//                     break; 
//                 }
//             }
//         }
//         return x; 
//     }
//     */
// }

// #[test]
// fn test_break_continue() {
//     /*
//     int testBreakContinueUsage() {
//         for (int i = 0;) {
//             if (0) continue; 
//             if (0) break; 
//             for (int j = 0;) {
//                 if (j) break; 
//                 while (j) {
//                     if (j) continue; 
//                     break; 
//                 }
//             }
//         }
//         return 0; 
//     }
//     */
// }
