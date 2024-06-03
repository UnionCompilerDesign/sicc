//! Testing for "edge cases", things which the IR generator may struggle with due to
//! relying on complex logic.

use std::sync::{Arc, Mutex};
use ir::core::IRGenerator;
use common::{
  ast::{ast_struct::{ASTNode, AST}, data_type::DataType, syntax_element::SyntaxElement},
  constants::DEFAULT_PRIORITY_MODELEMENT};
use symbol_table::symbol_table_struct::{SymbolInfo, SymbolTable, SymbolTableStack, SymbolValue};
use integration::module::{
    ModElement, Module, ast_stitch
};
use safe_llvm::{memory_management::resource_pools::ResourcePools, utils::utils_struct::Utils};

fn wrap_in_tle(ast_node: ASTNode) -> AST {
    let mut tle: ASTNode = ASTNode::new(SyntaxElement::TopLevelExpression);
    tle.add_child(ast_node);
    AST::new(tle)
}

#[test]
fn test_deeply_nested_loops1() {
    /*
    int testDeeplyNestedLoops() {
        for (int i = 5; 1;) {
            while (1) {
                do {
                    for (int j = 6; 1;) {
                        while (1) {
                            do {
                                return 0;
                            } while (1);
                        }
                    }
                } while (1);
            }
        }
    }

    ; ModuleID = 'dummy_module'
    source_filename = "dummy_module"

    define i64 @testDeeplyNestedLoops() {
    entryID0:
      %i = alloca i64, align 8
      store i64 5, ptr %i, align 4
      br label %for_condID1

    for_condID1:                                      ; preds = %for_incID1, %entryID0
      br i1 true, label %for_bodyID1, label %for_endID1

    for_bodyID1:                                      ; preds = %for_condID1
      br label %while_condID2

    while_condID2:                                    ; preds = %do_endID3, %for_bodyID1
      br i1 true, label %while_bodyID2, label %while_endID2

    while_bodyID2:                                    ; preds = %while_condID2
      br label %do_bodyID3

    do_bodyID3:                                       ; preds = %do_condID3, %while_bodyID2
      %j = alloca i64, align 8
      store i64 6, ptr %j, align 4
      br label %for_condID4

    for_condID4:                                      ; preds = %for_incID4, %do_bodyID3
      br i1 true, label %for_bodyID4, label %for_endID4

    for_bodyID4:                                      ; preds = %for_condID4
      br label %while_condID5

    while_condID5:                                    ; preds = %do_endID6, %for_bodyID4
      br i1 true, label %while_bodyID5, label %while_endID5

    while_bodyID5:                                    ; preds = %while_condID5
      br label %do_bodyID6

    do_bodyID6:                                       ; preds = %do_condID6, %while_bodyID5
      ret i64 0
      br label %do_condID6

    do_condID6:                                       ; preds = %do_bodyID6
      br i1 true, label %do_bodyID6, label %do_endID6

    do_endID6:                                        ; preds = %do_condID6
      br label %while_condID5

    while_endID5:                                     ; preds = %while_condID5
      br label %for_incID4

    for_incID4:                                       ; preds = %while_endID5
      br label %for_condID4

    for_endID4:                                       ; preds = %for_condID4
      br label %do_condID3

    do_condID3:                                       ; preds = %for_endID4
      br i1 true, label %do_bodyID3, label %do_endID3

    do_endID3:                                        ; preds = %do_condID3
      br label %while_condID2

    while_endID2:                                     ; preds = %while_condID2
      br label %for_incID1

    for_incID1:                                       ; preds = %while_endID2
      br label %for_condID1

    for_endID1:                                       ; preds = %for_condID1
    }
    */
    let mut function_ast = ASTNode::new(SyntaxElement::FunctionDeclaration);
    let fn_id = ASTNode::new(SyntaxElement::Identifier("testDeeplyNestedLoops".to_string()));
    let fn_type = ASTNode::new(SyntaxElement::Type(DataType::Integer));
    let mut fn_block = ASTNode::new(SyntaxElement::BlockExpression);

    let mut for_loop = ASTNode::new(SyntaxElement::ForLoop);
    let mut for_init = ASTNode::new(SyntaxElement::LoopInitializer);
    let mut assignment_node = ASTNode::new(SyntaxElement::Initialization);
    let mut for_cond = ASTNode::new(SyntaxElement::Condition);
    for_cond.add_child(ASTNode::new(SyntaxElement::Literal("true".to_string())));

    let mut for_init_var = ASTNode::new(SyntaxElement::Variable);
    for_init_var.add_child(ASTNode::new(SyntaxElement::Identifier("i".to_string())));
    for_init_var.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));
    assignment_node.add_child(for_init_var);

    let mut for_init_av = ASTNode::new( SyntaxElement::AssignedValue);
    for_init_av.add_child(ASTNode::new(SyntaxElement::Literal("5".to_string())));
    assignment_node.add_child(for_init_av);
    for_init.add_child(assignment_node);

    let mut for_body = ASTNode::new(SyntaxElement::BlockExpression);

    let mut while_loop = ASTNode::new(SyntaxElement::WhileLoop);
    let mut while_cond = ASTNode::new(SyntaxElement::Condition); 
    while_cond.add_child(ASTNode::new(SyntaxElement::Literal("true".to_string())));
    let mut while_body = ASTNode::new(SyntaxElement::BlockExpression);

    let mut do_while_loop = ASTNode::new(SyntaxElement::DoWhileLoop);
    let mut do_while_cond = ASTNode::new(SyntaxElement::Condition); 
    do_while_cond.add_child( ASTNode::new(SyntaxElement::Literal("true".to_string()))); 
    let mut do_while_body = ASTNode::new(SyntaxElement::BlockExpression);

    let mut nested_for_loop = ASTNode::new(SyntaxElement::ForLoop);
    let mut nested_for_cond = ASTNode::new(SyntaxElement::Condition);
    nested_for_cond.add_child(ASTNode::new(SyntaxElement::Literal("true".to_string())));
    let mut nested_for_init = ASTNode::new(SyntaxElement::LoopInitializer);
    let mut nested_for_variable = ASTNode::new(SyntaxElement::Initialization);
    let mut nested_for_init_var = ASTNode::new(SyntaxElement::Variable);
    nested_for_init_var.add_child(ASTNode::new(SyntaxElement::Identifier("j".to_string())));
    nested_for_init_var.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));

    let mut nested_for_init_val = ASTNode::new(SyntaxElement::AssignedValue);
    nested_for_init_val.add_child(ASTNode::new(SyntaxElement::Literal("6".to_string())));
    // nested_for_init.add_child(nested_for_init_var);
    nested_for_variable.add_child(nested_for_init_var);
    nested_for_variable.add_child(nested_for_init_val);
    nested_for_init.add_child(nested_for_variable);

    let mut nested_for_body = ASTNode::new(SyntaxElement::BlockExpression);

    let mut nested_while_loop = ASTNode::new(SyntaxElement::WhileLoop);
    let mut nested_while_cond = ASTNode::new(SyntaxElement::Condition);
    nested_while_cond.add_child(ASTNode::new(SyntaxElement::Literal("true".to_string())));
    let mut nested_while_body = ASTNode::new(SyntaxElement::BlockExpression);

    let mut nested_do_while_loop = ASTNode::new(SyntaxElement::DoWhileLoop);
    let mut nested_do_while_body = ASTNode::new(SyntaxElement::BlockExpression);
    let mut nested_do_while_blockexp = ASTNode::new(SyntaxElement::Return);
    let mut nested_do_while_av = ASTNode::new(SyntaxElement::AssignedValue);
    nested_do_while_av.add_child(ASTNode::new(SyntaxElement::Literal("0".to_string())));
    nested_do_while_blockexp.add_child(nested_do_while_av);
    nested_do_while_body.add_child(nested_do_while_blockexp);
    let mut nested_do_while_cond = ASTNode::new(SyntaxElement::Condition); 
    nested_do_while_cond.add_child(ASTNode::new(SyntaxElement::Literal("true".to_string())));

    nested_do_while_loop.add_child(nested_do_while_body);
    nested_do_while_loop.add_child(nested_do_while_cond);

    nested_while_body.add_child(nested_do_while_loop);
    nested_while_loop.add_child(nested_while_cond);
    nested_while_loop.add_child(nested_while_body);

    nested_for_body.add_child(nested_while_loop);
    nested_for_loop.add_child(nested_for_init);
    nested_for_loop.add_child(nested_for_cond);
    nested_for_loop.add_child(nested_for_body);

    do_while_body.add_child(nested_for_loop);
    do_while_loop.add_child(do_while_body);
    do_while_loop.add_child(do_while_cond);

    while_body.add_child(do_while_loop);
    while_loop.add_child(while_cond);
    while_loop.add_child(while_body);

    for_body.add_child(while_loop);
    for_loop.add_child(for_init);
    for_loop.add_child(for_cond);
    for_loop.add_child(for_body);

    fn_block.add_child(for_loop);
    function_ast.add_child(fn_id);
    function_ast.add_child(fn_type);
    function_ast.add_child(fn_block);

    let ast = wrap_in_tle(function_ast);
    let mut symbol_table_stack = SymbolTableStack::new();
    let mut symbol_table_global = SymbolTable::new();
    let fn_value = SymbolValue::FunctionValue { parameters: Vec::new() };
    let fn_info = SymbolInfo::new(DataType::Integer, fn_value);
    symbol_table_global.add("testDeeplyNestedLoops".to_string(), fn_info);
    symbol_table_stack.push(symbol_table_global);

    let mod_ast: Module = ast_stitch(vec![ModElement::new(ast, symbol_table_stack, DEFAULT_PRIORITY_MODELEMENT)]);
    let mut ir_generator = IRGenerator::new();
    let module_tag = ir_generator.generate_ir(mod_ast);
    let pools: Arc<Mutex<ResourcePools>> = ir_generator.get_resource_pools();
    let pools = pools.try_lock().expect("Failed to lock resource pool mutex in deeply nested loops IR!");

    let module = pools.get_module(module_tag).expect("Failed to get module");
    let write_result = Utils::write_to_file(module.clone(), "output_deeply_nested_loops1.ll");
    match write_result {
        Ok(_) => println!("Deeply nested loops test file written correctly!"),
        Err(e) => panic!("Deeply nested loops test file failed to write! Error: {}", e)
    }
    let get_str = Utils::write_to_string(module);
    let test_str = match get_str {
        Ok(str) => str,
        Err(e) => panic!("{}", e)
    };
    let expected_str = "; ModuleID = 'dummy_module'\nsource_filename = \"dummy_module\"\n\ndefine i64 @testDeeplyNestedLoops() {\nentryID0:\n  %i = alloca i64, align 8\n  store i64 5, ptr %i, align 4\n  br label %for_condID1\n\nfor_condID1:                                      ; preds = %for_incID1, %entryID0\n  br i1 true, label %for_bodyID1, label %for_endID1\n\nfor_bodyID1:                                      ; preds = %for_condID1\n  br label %while_condID2\n\nwhile_condID2:                                    ; preds = %do_endID3, %for_bodyID1\n  br i1 true, label %while_bodyID2, label %while_endID2\n\nwhile_bodyID2:                                    ; preds = %while_condID2\n  br label %do_bodyID3\n\ndo_bodyID3:                                       ; preds = %do_condID3, %while_bodyID2\n  %j = alloca i64, align 8\n  store i64 6, ptr %j, align 4\n  br label %for_condID4\n\nfor_condID4:                                      ; preds = %for_incID4, %do_bodyID3\n  br i1 true, label %for_bodyID4, label %for_endID4\n\nfor_bodyID4:                                      ; preds = %for_condID4\n  br label %while_condID5\n\nwhile_condID5:                                    ; preds = %do_endID6, %for_bodyID4\n  br i1 true, label %while_bodyID5, label %while_endID5\n\nwhile_bodyID5:                                    ; preds = %while_condID5\n  br label %do_bodyID6\n\ndo_bodyID6:                                       ; preds = %do_condID6, %while_bodyID5\n  ret i64 0\n  br label %do_condID6\n\ndo_condID6:                                       ; preds = %do_bodyID6\n  br i1 true, label %do_bodyID6, label %do_endID6\n\ndo_endID6:                                        ; preds = %do_condID6\n  br label %while_condID5\n\nwhile_endID5:                                     ; preds = %while_condID5\n  br label %for_incID4\n\nfor_incID4:                                       ; preds = %while_endID5\n  br label %for_condID4\n\nfor_endID4:                                       ; preds = %for_condID4\n  br label %do_condID3\n\ndo_condID3:                                       ; preds = %for_endID4\n  br i1 true, label %do_bodyID3, label %do_endID3\n\ndo_endID3:                                        ; preds = %do_condID3\n  br label %while_condID2\n\nwhile_endID2:                                     ; preds = %while_condID2\n  br label %for_incID1\n\nfor_incID1:                                       ; preds = %while_endID2\n  br label %for_condID1\n\nfor_endID1:                                       ; preds = %for_condID1\n}\n";

    assert_eq!(test_str, expected_str)
}

#[test]
fn test_deeply_nested_loops2() {
    /*
    int testDeeplyNestedLoops2() {
        while (1) {
            for (;1;) {
                do {
                    while (1) {
                        for (1) {
                            do {
                                return 42;
                            } while (1);
                        }
                    }
                } while (0);
            }
        }
    }
    ; ModuleID = 'dummy_module'
    source_filename = "dummy_module"

    define i64 @testSwappedWhileForLoops() {
    entryID0:
      br label %while_condID1

    while_condID1:                                    ; preds = %for_endID2, %entryID0
      br i1 true, label %while_bodyID1, label %while_endID1

    while_bodyID1:                                    ; preds = %while_condID1
      br label %for_condID2

    for_condID2:                                      ; preds = %for_incID2, %while_bodyID1
      br i1 true, label %for_bodyID2, label %for_endID2

    for_bodyID2:                                      ; preds = %for_condID2
      br label %do_bodyID3

    do_bodyID3:                                       ; preds = %do_condID3, %for_bodyID2
      br label %while_condID4

    while_condID4:                                    ; preds = %for_endID5, %do_bodyID3
      br i1 true, label %while_bodyID4, label %while_endID4

    while_bodyID4:                                    ; preds = %while_condID4
      br label %for_condID5

    for_condID5:                                      ; preds = %for_incID5, %while_bodyID4
      br i1 true, label %for_bodyID5, label %for_endID5

    for_bodyID5:                                      ; preds = %for_condID5
      br label %do_bodyID6

    do_bodyID6:                                       ; preds = %do_condID6, %for_bodyID5
      ret i64 42
      br label %do_condID6

    do_condID6:                                       ; preds = %do_bodyID6
      br i1 true, label %do_bodyID6, label %do_endID6

    do_endID6:                                        ; preds = %do_condID6
      br label %for_incID5

    for_incID5:                                       ; preds = %do_endID6
      br label %for_condID5

    for_endID5:                                       ; preds = %for_condID5
      br label %while_condID4

    while_endID4:                                     ; preds = %while_condID4
      br label %do_condID3

    do_condID3:                                       ; preds = %while_endID4
      br i1 true, label %do_bodyID3, label %do_endID3

    do_endID3:                                        ; preds = %do_condID3
      br label %for_incID2

    for_incID2:                                       ; preds = %do_endID3
      br label %for_condID2

    for_endID2:                                       ; preds = %for_condID2
      br label %while_condID1

    while_endID1:                                     ; preds = %while_condID1
    }
    */
    let mut function_ast = ASTNode::new(SyntaxElement::FunctionDeclaration);
    let fn_id = ASTNode::new(SyntaxElement::Identifier("testSwappedWhileForLoops".to_string()));
    let fn_type = ASTNode::new(SyntaxElement::Type(DataType::Integer));
    let mut fn_block = ASTNode::new(SyntaxElement::BlockExpression);

    let mut outer_while_loop = ASTNode::new(SyntaxElement::WhileLoop);
    let mut outer_while_cond = ASTNode::new(SyntaxElement::Condition);
    outer_while_cond.add_child(ASTNode::new(SyntaxElement::Literal("true".to_string()))); 
    let mut outer_while_body = ASTNode::new(SyntaxElement::BlockExpression);

    let mut for_loop = ASTNode::new(SyntaxElement::ForLoop);
    let mut for_loop_cond = ASTNode::new(SyntaxElement::Condition);
    let mut for_loop_body = ASTNode::new(SyntaxElement::BlockExpression);

    for_loop_cond.add_child(ASTNode::new(SyntaxElement::Literal("true".to_string()))); 
    for_loop.add_child(for_loop_cond);

    let mut do_while_loop = ASTNode::new(SyntaxElement::DoWhileLoop);
    let mut do_while_cond = ASTNode::new(SyntaxElement::Condition);
    do_while_cond.add_child(ASTNode::new(SyntaxElement::Literal("true".to_string())));
    let mut do_while_body = ASTNode::new(SyntaxElement::BlockExpression);

    let mut inner_while_loop = ASTNode::new(SyntaxElement::WhileLoop);
    let mut inner_while_cond = ASTNode::new(SyntaxElement::Condition);
    inner_while_cond.add_child(ASTNode::new(SyntaxElement::Literal("true".to_string())));
    let mut inner_while_body = ASTNode::new(SyntaxElement::BlockExpression);

    let mut inner_for_loop = ASTNode::new(SyntaxElement::ForLoop);
    let mut inner_for_loop_body = ASTNode::new(SyntaxElement::BlockExpression);
    let mut inner_for_loop_cond = ASTNode::new(SyntaxElement::Condition);
    inner_for_loop_cond.add_child(ASTNode::new(SyntaxElement::Literal("true".to_string()))); 
    inner_for_loop.add_child(inner_for_loop_cond);

    let mut inner_do_while_loop = ASTNode::new(SyntaxElement::DoWhileLoop);
    let mut inner_do_while_body = ASTNode::new(SyntaxElement::BlockExpression);
    let mut inner_do_while_statement = ASTNode::new(SyntaxElement::Return);
    let mut inner_do_while_av = ASTNode::new(SyntaxElement::AssignedValue);
    inner_do_while_av.add_child(ASTNode::new(SyntaxElement::Literal("42".to_string())));
    inner_do_while_statement.add_child(inner_do_while_av);
    inner_do_while_body.add_child(inner_do_while_statement);
    let mut inner_do_while_cond = ASTNode::new(SyntaxElement::Condition);
    inner_do_while_cond.add_child(ASTNode::new(SyntaxElement::Literal("true".to_string())));

    inner_do_while_loop.add_child(inner_do_while_body);
    inner_do_while_loop.add_child(inner_do_while_cond);

    inner_for_loop_body.add_child(inner_do_while_loop);
    inner_for_loop.add_child(inner_for_loop_body);
    inner_while_body.add_child(inner_for_loop);
    inner_while_loop.add_child(inner_while_cond);
    inner_while_loop.add_child(inner_while_body);

    do_while_body.add_child(inner_while_loop);
    do_while_loop.add_child(do_while_body);
    do_while_loop.add_child(do_while_cond);

    for_loop_body.add_child(do_while_loop);
    for_loop.add_child(for_loop_body);
    outer_while_body.add_child(for_loop);
    outer_while_loop.add_child(outer_while_cond);
    outer_while_loop.add_child(outer_while_body);

    fn_block.add_child(outer_while_loop);
    function_ast.add_child(fn_id);
    function_ast.add_child(fn_type);
    function_ast.add_child(fn_block);

    let ast = wrap_in_tle(function_ast);
    let mut symbol_table_stack = SymbolTableStack::new();
    let mut symbol_table_global = SymbolTable::new();
    let fn_value = SymbolValue::FunctionValue { parameters: Vec::new() };
    let fn_info = SymbolInfo::new(DataType::Integer, fn_value);
    symbol_table_global.add("testSwappedWhileForLoops".to_string(), fn_info);
    symbol_table_stack.push(symbol_table_global);

    let mod_ast: Module = ast_stitch(vec![ModElement::new(ast, symbol_table_stack, DEFAULT_PRIORITY_MODELEMENT)]);
    let mut ir_generator = IRGenerator::new();
    let module_tag = ir_generator.generate_ir(mod_ast);
    let pools: Arc<Mutex<ResourcePools>> = ir_generator.get_resource_pools();
    let pools = pools.try_lock().expect("Failed to lock resource pool mutex in deeply nested loops IR!");

    let module = pools.get_module(module_tag).expect("Failed to get module");
    let write_result = Utils::write_to_file(module.clone(), "output_deeply_nested_loops2.ll");
    match write_result {
        Ok(_) => println!("Swapped while-for loops test file written correctly!"),
        Err(e) => panic!("Swapped while-for loops test file failed to write! Error: {}", e)
    }
    let get_str = Utils::write_to_string(module);
    let test_str = match get_str {
        Ok(str) => str,
        Err(e) => panic!("{}", e)
    };
    let expected_str = "; ModuleID = 'dummy_module'\nsource_filename = \"dummy_module\"\n\ndefine i64 @testSwappedWhileForLoops() {\nentryID0:\n  br label %while_condID1\n\nwhile_condID1:                                    ; preds = %for_endID2, %entryID0\n  br i1 true, label %while_bodyID1, label %while_endID1\n\nwhile_bodyID1:                                    ; preds = %while_condID1\n  br label %for_condID2\n\nfor_condID2:                                      ; preds = %for_incID2, %while_bodyID1\n  br i1 true, label %for_bodyID2, label %for_endID2\n\nfor_bodyID2:                                      ; preds = %for_condID2\n  br label %do_bodyID3\n\ndo_bodyID3:                                       ; preds = %do_condID3, %for_bodyID2\n  br label %while_condID4\n\nwhile_condID4:                                    ; preds = %for_endID5, %do_bodyID3\n  br i1 true, label %while_bodyID4, label %while_endID4\n\nwhile_bodyID4:                                    ; preds = %while_condID4\n  br label %for_condID5\n\nfor_condID5:                                      ; preds = %for_incID5, %while_bodyID4\n  br i1 true, label %for_bodyID5, label %for_endID5\n\nfor_bodyID5:                                      ; preds = %for_condID5\n  br label %do_bodyID6\n\ndo_bodyID6:                                       ; preds = %do_condID6, %for_bodyID5\n  ret i64 42\n  br label %do_condID6\n\ndo_condID6:                                       ; preds = %do_bodyID6\n  br i1 true, label %do_bodyID6, label %do_endID6\n\ndo_endID6:                                        ; preds = %do_condID6\n  br label %for_incID5\n\nfor_incID5:                                       ; preds = %do_endID6\n  br label %for_condID5\n\nfor_endID5:                                       ; preds = %for_condID5\n  br label %while_condID4\n\nwhile_endID4:                                     ; preds = %while_condID4\n  br label %do_condID3\n\ndo_condID3:                                       ; preds = %while_endID4\n  br i1 true, label %do_bodyID3, label %do_endID3\n\ndo_endID3:                                        ; preds = %do_condID3\n  br label %for_incID2\n\nfor_incID2:                                       ; preds = %do_endID3\n  br label %for_condID2\n\nfor_endID2:                                       ; preds = %for_condID2\n  br label %while_condID1\n\nwhile_endID1:                                     ; preds = %while_condID1\n}\n";

    assert_eq!(test_str, expected_str)
}

#[test]
fn test_deply_nested_loops3() {
    /*
    int testMultipleDoWhileLoops3() {
        do {
            do {
                for (int i = 42; 1;) {
                    while (1) {
                        do {
                            return 42;
                        } while (1);
                    }
                }
            } while (2);
        } while (3);
    }
    ; ModuleID = 'dummy_module'
    source_filename = "dummy_module"

    define i64 @testMultipleDoWhileLoops3() {
    entryID0:
      br label %do_bodyID1

    do_bodyID1:                                       ; preds = %do_condID1, %entryID0
      br label %do_bodyID2

    do_bodyID2:                                       ; preds = %do_condID2, %do_bodyID1
      %i = alloca i64, align 8
      store i64 42, ptr %i, align 4
      br label %for_condID3

    for_condID3:                                      ; preds = %for_incID3, %do_bodyID2
      br i1 true, label %for_bodyID3, label %for_endID3

    for_bodyID3:                                      ; preds = %for_condID3
      br label %while_condID4

    while_condID4:                                    ; preds = %do_endID5, %for_bodyID3
      br i1 true, label %while_bodyID4, label %while_endID4

    while_bodyID4:                                    ; preds = %while_condID4
      br label %do_bodyID5

    do_bodyID5:                                       ; preds = %do_condID5, %while_bodyID4
      ret i64 42
      br label %do_condID5

    do_condID5:                                       ; preds = %do_bodyID5
      br i1 true, label %do_bodyID5, label %do_endID5

    do_endID5:                                        ; preds = %do_condID5
      br label %while_condID4

    while_endID4:                                     ; preds = %while_condID4
      br label %for_incID3

    for_incID3:                                       ; preds = %while_endID4
      br label %for_condID3

    for_endID3:                                       ; preds = %for_condID3
      br label %do_condID2

    do_condID2:                                       ; preds = %for_endID3
      br i1 true, label %do_bodyID2, label %do_endID2

    do_endID2:                                        ; preds = %do_condID2
      br label %do_condID1

    do_condID1:                                       ; preds = %do_endID2
      br i1 true, label %do_bodyID1, label %do_endID1

    do_endID1:                                        ; preds = %do_condID1
    }
    */
    let mut function_ast = ASTNode::new(SyntaxElement::FunctionDeclaration);
    let fn_id = ASTNode::new(SyntaxElement::Identifier("testMultipleDoWhileLoops3".to_string()));
    let fn_type = ASTNode::new(SyntaxElement::Type(DataType::Integer));
    let mut fn_block = ASTNode::new(SyntaxElement::BlockExpression);

    let mut outer_do_while_loop = ASTNode::new(SyntaxElement::DoWhileLoop);
    let mut outer_do_while_cond = ASTNode::new(SyntaxElement::Condition);
    outer_do_while_cond.add_child(ASTNode::new(SyntaxElement::Literal("true".to_string())));
    let mut outer_do_while_body = ASTNode::new(SyntaxElement::BlockExpression);

    let mut middle_do_while_loop = ASTNode::new(SyntaxElement::DoWhileLoop);
    let mut middle_do_while_cond = ASTNode::new(SyntaxElement::Condition);
    middle_do_while_cond.add_child(ASTNode::new(SyntaxElement::Literal("true".to_string())));
    let mut middle_do_while_body = ASTNode::new(SyntaxElement::BlockExpression);

    let mut for_loop = ASTNode::new(SyntaxElement::ForLoop);
    let mut for_loop_cond = ASTNode::new(SyntaxElement::Condition);
    for_loop_cond.add_child(ASTNode::new(SyntaxElement::Literal("true".to_string())));
    for_loop.add_child(for_loop_cond);
    let mut for_init = ASTNode::new(SyntaxElement::LoopInitializer);
    let mut for_init_var_declaration = ASTNode::new(SyntaxElement::Initialization);
    let mut for_init_var = ASTNode::new(SyntaxElement::Variable);
    for_init_var.add_child(ASTNode::new(SyntaxElement::Identifier("i".to_string())));
    for_init_var.add_child(ASTNode::new(SyntaxElement::Type(DataType::Integer)));
    let mut for_init_val = ASTNode::new(SyntaxElement::AssignedValue);
    for_init_val.add_child(ASTNode::new(SyntaxElement::Literal("42".to_string())));
    for_init_var_declaration.add_child(for_init_var);
    for_init_var_declaration.add_child(for_init_val);
    for_init.add_child(for_init_var_declaration);

    let mut for_body = ASTNode::new(SyntaxElement::BlockExpression);
    let mut while_loop = ASTNode::new(SyntaxElement::WhileLoop);
    let mut while_cond = ASTNode::new(SyntaxElement::Condition);
    while_cond.add_child(ASTNode::new(SyntaxElement::Literal("true".to_string())));
    let mut while_body = ASTNode::new(SyntaxElement::BlockExpression);

    let mut inner_do_while_loop = ASTNode::new(SyntaxElement::DoWhileLoop);
    let mut inner_do_while_body = ASTNode::new(SyntaxElement::BlockExpression);
    let mut inner_do_while_return = ASTNode::new(SyntaxElement::Return);
    let mut inner_do_while_av = ASTNode::new(SyntaxElement::AssignedValue);
    inner_do_while_av.add_child(ASTNode::new(SyntaxElement::Literal("42".to_string())));
    inner_do_while_return.add_child(inner_do_while_av);
    inner_do_while_body.add_child(inner_do_while_return);
    let mut inner_do_while_cond = ASTNode::new(SyntaxElement::Condition);
    inner_do_while_cond.add_child(ASTNode::new(SyntaxElement::Literal("true".to_string())));

    inner_do_while_loop.add_child(inner_do_while_body);
    inner_do_while_loop.add_child(inner_do_while_cond);

    while_body.add_child(inner_do_while_loop);
    while_loop.add_child(while_cond);
    while_loop.add_child(while_body);

    for_body.add_child(while_loop);
    for_loop.add_child(for_init);
    for_loop.add_child(for_body);

    middle_do_while_body.add_child(for_loop);
    middle_do_while_loop.add_child(middle_do_while_body);
    middle_do_while_loop.add_child(middle_do_while_cond);

    outer_do_while_body.add_child(middle_do_while_loop);
    outer_do_while_loop.add_child(outer_do_while_body);
    outer_do_while_loop.add_child(outer_do_while_cond);

    fn_block.add_child(outer_do_while_loop);
    function_ast.add_child(fn_id);
    function_ast.add_child(fn_type);
    function_ast.add_child(fn_block);

    let ast = wrap_in_tle(function_ast);
    let mut symbol_table_stack = SymbolTableStack::new();
    let mut symbol_table_global = SymbolTable::new();
    let fn_value = SymbolValue::FunctionValue { parameters: Vec::new() };
    let fn_info = SymbolInfo::new(DataType::Integer, fn_value);
    symbol_table_global.add("testMultipleDoWhileLoops3".to_string(), fn_info);
    symbol_table_stack.push(symbol_table_global);

    let mod_ast: Module = ast_stitch(vec![ModElement::new(ast, symbol_table_stack, DEFAULT_PRIORITY_MODELEMENT)]);
    let mut ir_generator = IRGenerator::new();
    let module_tag = ir_generator.generate_ir(mod_ast);
    let pools: Arc<Mutex<ResourcePools>> = ir_generator.get_resource_pools();
    let pools = pools.try_lock().expect("Failed to lock resource pool mutex in multiple do while loops IR!");

    let module = pools.get_module(module_tag).expect("Failed to get module");
    let write_result = Utils::write_to_file(module.clone(), "output_deeply_nested_loops3.ll");
    match write_result {
        Ok(_) => println!("Multiple do-while loops test file written correctly!"),
        Err(e) => panic!("Multiple do-while loops test file failed to write! Error: {}", e)
    }
    let get_str = Utils::write_to_string(module);
    let test_str = match get_str {
        Ok(str) => str,
        Err(e) => panic!("{}", e)
    };
    let expected_str = "; ModuleID = 'dummy_module'\nsource_filename = \"dummy_module\"\n\ndefine i64 @testMultipleDoWhileLoops3() {\nentryID0:\n  br label %do_bodyID1\n\ndo_bodyID1:                                       ; preds = %do_condID1, %entryID0\n  br label %do_bodyID2\n\ndo_bodyID2:                                       ; preds = %do_condID2, %do_bodyID1\n  %i = alloca i64, align 8\n  store i64 42, ptr %i, align 4\n  br label %for_condID3\n\nfor_condID3:                                      ; preds = %for_incID3, %do_bodyID2\n  br i1 true, label %for_bodyID3, label %for_endID3\n\nfor_bodyID3:                                      ; preds = %for_condID3\n  br label %while_condID4\n\nwhile_condID4:                                    ; preds = %do_endID5, %for_bodyID3\n  br i1 true, label %while_bodyID4, label %while_endID4\n\nwhile_bodyID4:                                    ; preds = %while_condID4\n  br label %do_bodyID5\n\ndo_bodyID5:                                       ; preds = %do_condID5, %while_bodyID4\n  ret i64 42\n  br label %do_condID5\n\ndo_condID5:                                       ; preds = %do_bodyID5\n  br i1 true, label %do_bodyID5, label %do_endID5\n\ndo_endID5:                                        ; preds = %do_condID5\n  br label %while_condID4\n\nwhile_endID4:                                     ; preds = %while_condID4\n  br label %for_incID3\n\nfor_incID3:                                       ; preds = %while_endID4\n  br label %for_condID3\n\nfor_endID3:                                       ; preds = %for_condID3\n  br label %do_condID2\n\ndo_condID2:                                       ; preds = %for_endID3\n  br i1 true, label %do_bodyID2, label %do_endID2\n\ndo_endID2:                                        ; preds = %do_condID2\n  br label %do_condID1\n\ndo_condID1:                                       ; preds = %do_endID2\n  br i1 true, label %do_bodyID1, label %do_endID1\n\ndo_endID1:                                        ; preds = %do_condID1\n}\n";

    assert_eq!(test_str, expected_str)
}

#[test]
fn test_function_with_while_if_else_break_continue() {
    /* 
    int testFunction() {
        while (1) {
            if (1) {
                break;
            } else {
                continue;
            }
        }
    }

    ; ModuleID = 'dummy_module'
    source_filename = "dummy_module"

    define i64 @testFunction() {
    entryID0:
      br label %while_condID1

    while_condID1:                                    ; preds = %mergeID2, %elseID2, %entryID0
      br i1 true, label %while_bodyID1, label %while_endID1

    while_bodyID1:                                    ; preds = %while_condID1
      br i1 true, label %thenID2, label %elseID2

    thenID2:                                          ; preds = %while_bodyID1
      br label %while_endID1
      br label %mergeID2

    elseID2:                                          ; preds = %while_bodyID1
      br label %while_condID1
      br label %mergeID2

    mergeID2:                                         ; preds = %elseID2, %thenID2
      br label %while_condID1

    while_endID1:                                     ; preds = %thenID2, %while_condID1
    }

    */ 

    let mut while_condition = ASTNode::new(SyntaxElement::Condition);
    let while_condition_value = ASTNode::new(SyntaxElement::Literal("true".to_string()));
    while_condition.add_child(while_condition_value);
    let mut while_body = ASTNode::new(SyntaxElement::BlockExpression);

    let mut while_statement = ASTNode::new(SyntaxElement::WhileLoop);
    while_statement.add_child(while_condition);
    

    let mut if_statement = ASTNode::new(SyntaxElement::IfStatement);

    let mut if_condition = ASTNode::new(SyntaxElement::Condition);

    let if_value = ASTNode::new(SyntaxElement::Literal("true".to_string()));
    if_condition.add_child(if_value);

    let mut then_branch = ASTNode::new(SyntaxElement::BlockExpression);

    then_branch.add_child(ASTNode::new(SyntaxElement::Break));

    if_statement.add_child(if_condition);
    if_statement.add_child(then_branch);

    let mut else_branch = ASTNode::new(SyntaxElement::ElseStatement);
    let mut else_block = ASTNode::new(SyntaxElement::BlockExpression);

    else_block.add_child(ASTNode::new(SyntaxElement::Continue));

    else_branch.add_child(else_block);

    if_statement.add_child(else_branch);
    while_body.add_child(if_statement);
    while_statement.add_child(while_body);


    let mut fn_block = ASTNode::new(SyntaxElement::BlockExpression);

    fn_block.add_child(while_statement);

    let fn_type = ASTNode::new(SyntaxElement::Type(DataType::Integer));
    let fn_id = ASTNode::new(SyntaxElement::Identifier("testFunction".to_string()));

    let mut fn_declaration_node = ASTNode::new(SyntaxElement::FunctionDeclaration);
    
    fn_declaration_node.add_child(fn_id);
    fn_declaration_node.add_child(fn_type);
    fn_declaration_node.add_child(fn_block);

    let ast: AST = wrap_in_tle(fn_declaration_node);

    let mut symbol_table_stack = SymbolTableStack::new();
    let mut symbol_table_global = SymbolTable::new();
    let fn_value = SymbolValue::FunctionValue{
        parameters: Vec::new(),
    };
    let fn_info = SymbolInfo::new(DataType::Integer, fn_value);
    symbol_table_global.add("testFunction".to_string(), fn_info);
    symbol_table_stack.push(symbol_table_global);
    symbol_table_stack.push(SymbolTable::new());


    let mod_ast: Module = ast_stitch(vec![ModElement::new(ast, symbol_table_stack, DEFAULT_PRIORITY_MODELEMENT)]);

    let mut ir_generator = IRGenerator::new();

    let module_tag = ir_generator.generate_ir(mod_ast);  

    let pools: Arc<Mutex<ResourcePools>> = ir_generator.get_resource_pools();
    let pools = pools.try_lock().expect("Failed to lock resource pool mutex in test IR!");

    let module = pools.get_module(module_tag).expect("Failed to get module");
    let write_result = Utils::write_to_file(module.clone(), "output_while_if_else_br_cont.ll");
    match write_result {
        Ok(_) => eprintln!("Break continue breaktest file written correctly!"),
        Err(_) => panic!("Break continue test file failed to write!")
    }
    let get_str = Utils::write_to_string(module);
    let test_str = match get_str {
        Ok(str) => str,
        Err(e) => panic!("{}", e)
    };
    let expected_str = "; ModuleID = 'dummy_module'\nsource_filename = \"dummy_module\"\n\ndefine i64 @testFunction() {\nentryID0:\n  br label %while_condID1\n\nwhile_condID1:                                    ; preds = %mergeID2, %elseID2, %entryID0\n  br i1 true, label %while_bodyID1, label %while_endID1\n\nwhile_bodyID1:                                    ; preds = %while_condID1\n  br i1 true, label %thenID2, label %elseID2\n\nthenID2:                                          ; preds = %while_bodyID1\n  br label %while_endID1\n  br label %mergeID2\n\nelseID2:                                          ; preds = %while_bodyID1\n  br label %while_condID1\n  br label %mergeID2\n\nmergeID2:                                         ; preds = %elseID2, %thenID2\n  br label %while_condID1\n\nwhile_endID1:                                     ; preds = %thenID2, %while_condID1\n}\n";

    assert_eq!(test_str, expected_str)
}