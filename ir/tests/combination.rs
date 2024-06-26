//! Testing for "combination cases", things which the IR generator should be able to combine
//! together to make more complex programs.

use std::sync::{Arc, Mutex};
use ir::core::IRGenerator;
use common::{
    ast::{core::{ASTNode, AST}, data_type::DataType, node_type::NodeType},
    constants::DEFAULT_PRIORITY_MODELEMENT};
use safe_llvm::{common::io, ir::core::IRManager};
use sts::core::{SymbolInfo, SymbolTable, SymbolTableStack, SymbolValue};
use integration::module::{
    ModElement, Module, ast_stitch
};

fn wrap_in_tle(ast_node: ASTNode) -> AST {
    let mut tle: ASTNode = ASTNode::new(NodeType::TopLevelExpression);
    tle.add_child(ast_node);
    AST::new(tle)
}

#[test]
fn test_triple_function_declaration() {
    /* `
    int testFunction() {};
    int testFunction2() {};
    int testFunction3() {};

    ; ModuleID = 'dummy_module'
    source_filename = "dummy_module"

    define i64 @testFunction() {
    entryID0:
    }

    define i64 @testFunction2() {
    entryID1:
    }

    define i64 @testFunction3() {
    entryID2:
    }
    */ 

    let mut top_level = ASTNode::new(NodeType::TopLevelExpression);
    
    let mut function_ast = ASTNode::new(NodeType::FunctionDeclaration);

    let fn_id = ASTNode::new(NodeType::Identifier("testFunction".to_string()));
    let fn_type = ASTNode::new(NodeType::Type(DataType::Integer));
    let fn_block_exp = ASTNode::new(NodeType::BlockExpression);
    function_ast.add_child(fn_id);
    function_ast.add_child(fn_type);
    function_ast.add_child(fn_block_exp);

    let mut function_ast_2 = ASTNode::new(NodeType::FunctionDeclaration);
    let fn_id_2 = ASTNode::new(NodeType::Identifier("testFunction2".to_string()));
    let fn_type_2 = ASTNode::new(NodeType::Type(DataType::Integer));
    let fn_block_exp_2 = ASTNode::new(NodeType::BlockExpression);
    function_ast_2.add_child(fn_id_2);
    function_ast_2.add_child(fn_type_2);
    function_ast_2.add_child(fn_block_exp_2);

    let mut function_ast_3 = ASTNode::new(NodeType::FunctionDeclaration);
    let fn_id_3 = ASTNode::new(NodeType::Identifier("testFunction3".to_string()));
    let fn_type_3 = ASTNode::new(NodeType::Type(DataType::Integer));
    let fn_block_exp_3 = ASTNode::new(NodeType::BlockExpression);
    function_ast_3.add_child(fn_id_3);
    function_ast_3.add_child(fn_type_3);
    function_ast_3.add_child(fn_block_exp_3);

    top_level.add_child(function_ast);
    top_level.add_child(function_ast_2);
    top_level.add_child(function_ast_3);

    let ast = AST::new(top_level);

    let mut sts_stack = SymbolTableStack::new();
    let mut sts_global = SymbolTable::new();
    let mut sts_global_2 = SymbolTable::new();
    let mut sts_global_3 = SymbolTable::new();

    let fn_value = SymbolValue::FunctionValue{
        parameters: Vec::new(),
    };
    let fn_info = SymbolInfo::new(DataType::Integer, fn_value);
    let fn_value_2 = SymbolValue::FunctionValue{
        parameters: Vec::new(),
    };
    let fn_info_2 = SymbolInfo::new(DataType::Integer, fn_value_2);
    let fn_value_3 = SymbolValue::FunctionValue{
        parameters: Vec::new(),
    };
    let fn_info_3 = SymbolInfo::new(DataType::Integer, fn_value_3);
    sts_global.add("testFunction".to_string(), fn_info);
    sts_global_2.add("testFunction2".to_string(), fn_info_2);
    sts_global_3.add("testFunction3".to_string(), fn_info_3);
    sts_stack.push(sts_global);
    sts_stack.push(sts_global_2);
    sts_stack.push(sts_global_3);

    let mod_ast: Module = ast_stitch(vec![ModElement::new(ast, sts_stack, DEFAULT_PRIORITY_MODELEMENT)]);

    let mut ir_generator = IRGenerator::new();
    let module_tag = ir_generator.generate_ir(mod_ast);  

    let pools: Arc<Mutex<IRManager>> = ir_generator.get_resource_pools();
    let pools = pools.try_lock().expect("Failed to lock resource pool mutex in IR!");

    let module = pools.get_module(module_tag).expect("Failed to get module");
    let write_result = io::write_ir_to_file(module.clone(), "output_fn_declaration_2.ll");
    match write_result {
        Ok(_) => eprintln!("FN2 test file written correctly!"),
        Err(_) => panic!("FN2 test file failed to write!")
    }
    let get_str = io::write_to_string(module);
    let test_str = match get_str {
        Ok(str) => str,
        Err(e) => panic!("{}", e)
    };
    let expected_str = "; ModuleID = 'dummy_module'\nsource_filename = \"dummy_module\"\n\ndefine i64 @testFunction() {\nentryID0:\n}\n\ndefine i64 @testFunction2() {\nentryID1:\n}\n\ndefine i64 @testFunction3() {\nentryID2:\n}\n";

    assert_eq!(test_str, expected_str)
}

#[test]
fn test_function_with_while_if_else() {
    /* 
    int testFunction() {
        while (1) {
            if (1) {
                return 1;
            } else {
                return 1;
            }
        }
    }

    ; ModuleID = 'dummy_module'
    source_filename = "dummy_module"

    define i64 @testFunction() {
    entryID0:
      br label %while_condID1

    while_condID1:                                    ; preds = %mergeID2, %entryID0
      br i1 true, label %while_bodyID1, label %while_endID1

    while_bodyID1:                                    ; preds = %while_condID1
      br i1 true, label %thenID2, label %elseID2

    thenID2:                                          ; preds = %while_bodyID1
      ret i64 2
      br label %mergeID2

    elseID2:                                          ; preds = %while_bodyID1
      ret i64 1
      br label %mergeID2

    mergeID2:                                         ; preds = %elseID2, %thenID2
      br label %while_condID1

    while_endID1:                                     ; preds = %while_condID1
    }

    */ 

    let mut while_condition = ASTNode::new(NodeType::Condition);
    let while_condition_value = ASTNode::new(NodeType::Literal("true".to_string()));
    while_condition.add_child(while_condition_value);
    let mut while_body = ASTNode::new(NodeType::BlockExpression);

    let mut while_statement = ASTNode::new(NodeType::WhileLoop);
    while_statement.add_child(while_condition);
    

    let mut if_statement = ASTNode::new(NodeType::IfStatement);

    let mut if_condition = ASTNode::new(NodeType::Condition);

    let if_value = ASTNode::new(NodeType::Literal("true".to_string()));
    if_condition.add_child(if_value);

    let mut then_branch = ASTNode::new(NodeType::BlockExpression);
    let mut return_statement = ASTNode::new(NodeType::Return);
    let mut assigned_value = ASTNode::new(NodeType::AssignedValue);
    let then_ret_value = ASTNode::new(NodeType::Literal("2".to_string()));

    assigned_value.add_child(then_ret_value);

    return_statement.add_child(assigned_value);

    then_branch.add_child(return_statement);

    if_statement.add_child(if_condition);
    if_statement.add_child(then_branch);

    let mut else_branch = ASTNode::new(NodeType::ElseStatement);
    let mut else_block = ASTNode::new(NodeType::BlockExpression);

    let mut return_statement_else = ASTNode::new(NodeType::Return);
    let mut assigned_value = ASTNode::new(NodeType::AssignedValue);
    let return_value = ASTNode::new(NodeType::Literal("1".to_string()));
    assigned_value.add_child(return_value);

    return_statement_else.add_child(assigned_value);

    else_block.add_child(return_statement_else);

    else_branch.add_child(else_block);

    if_statement.add_child(else_branch);
    while_body.add_child(if_statement);
    while_statement.add_child(while_body);


    let mut fn_block = ASTNode::new(NodeType::BlockExpression);

    fn_block.add_child(while_statement);

    let fn_type = ASTNode::new(NodeType::Type(DataType::Integer));
    let fn_id = ASTNode::new(NodeType::Identifier("testFunction".to_string()));

    let mut fn_declaration_node = ASTNode::new(NodeType::FunctionDeclaration);
    
    fn_declaration_node.add_child(fn_id);
    fn_declaration_node.add_child(fn_type);
    fn_declaration_node.add_child(fn_block);

    let ast: AST = wrap_in_tle(fn_declaration_node);

    let mut sts_stack = SymbolTableStack::new();
    let mut sts_global = SymbolTable::new();
    let fn_value = SymbolValue::FunctionValue{
        parameters: Vec::new(),
    };
    let fn_info = SymbolInfo::new(DataType::Integer, fn_value);
    sts_global.add("testFunction".to_string(), fn_info);
    sts_stack.push(sts_global);
    sts_stack.push(SymbolTable::new());


    let mod_ast: Module = ast_stitch(vec![ModElement::new(ast, sts_stack, DEFAULT_PRIORITY_MODELEMENT)]);

    let mut ir_generator = IRGenerator::new();

    let module_tag = ir_generator.generate_ir(mod_ast);  

    let pools: Arc<Mutex<IRManager>> = ir_generator.get_resource_pools();
    let pools = pools.try_lock().expect("Failed to lock resource pool mutex in do while IR!");

    let module = pools.get_module(module_tag).expect("Failed to get module");
    let write_result = io::write_ir_to_file(module.clone(), "output_while_if_else.ll");
    match write_result {
        Ok(_) => eprintln!("If else test file written correctly!"),
        Err(_) => panic!("If else test file failed to write!")
    }
    let get_str = io::write_to_string(module);
    let test_str = match get_str {
        Ok(str) => str,
        Err(e) => panic!("{}", e)
    };
    let expected_str = "; ModuleID = 'dummy_module'\nsource_filename = \"dummy_module\"\n\ndefine i64 @testFunction() {\nentryID0:\n  br label %while_condID1\n\nwhile_condID1:                                    ; preds = %mergeID2, %entryID0\n  br i1 true, label %while_bodyID1, label %while_endID1\n\nwhile_bodyID1:                                    ; preds = %while_condID1\n  br i1 true, label %thenID2, label %elseID2\n\nthenID2:                                          ; preds = %while_bodyID1\n  ret i64 2\n  br label %mergeID2\n\nelseID2:                                          ; preds = %while_bodyID1\n  ret i64 1\n  br label %mergeID2\n\nmergeID2:                                         ; preds = %elseID2, %thenID2\n  br label %while_condID1\n\nwhile_endID1:                                     ; preds = %while_condID1\n}\n";

    assert_eq!(test_str, expected_str)
}

#[test]
fn test_nested_for_loop() {
    /* 
    int testFunction() {
        for(int i = 0; 1; i = 42){
            for(int j = 0; 1; j = 24){
                
            }
        }
    }
    
    ; ModuleID = 'dummy_module'
    source_filename = "dummy_module"

    define i64 @testForLoopNested() {
    entryID0:
      %test_var_outer = alloca i64, align 8
      store i64 0, ptr %test_var_outer, align 4
      br label %for_condID1

    for_condID1:                                      ; preds = %for_incID1
      br i1 true, label %for_bodyID1, label %for_endID1

    for_bodyID1:                                      ; preds = %for_condID1
      %test_var = alloca i64, align 8
      store i64 0, ptr %test_var, align 4
      br label %for_condID2

    for_condID2:                                      ; preds = %for_incID2, %for_bodyID2
      br i1 true, label %for_bodyID2, label %for_endID2

    for_bodyID2:                                      ; preds = %for_condID2
      br label %for_incID2
      br label %for_incID2

    for_incID2:                                       ; preds = %for_bodyID2
      store i64 42, ptr %test_var, align 4
      br label %for_condID2

    for_endID2:                                       ; preds = %for_condID2
      br label %for_incID1

    for_incID1:                                       ; preds = %for_endID2
      store i64 42, ptr %test_var_outer, align 4
      br label %for_condID1

    for_endID1:                                       ; preds = %for_condID1
    }

    */

    let mut assignment_node = ASTNode::new(NodeType::Initialization);

    let id_node = ASTNode::new(NodeType::Identifier("test_var".to_string()));
    let type_node = ASTNode::new(NodeType::Type(DataType::Integer));

    let mut assignment_node_2 = ASTNode::new(NodeType::Initialization);
    let id_node_2 = ASTNode::new(NodeType::Identifier("test_var_2".to_string()));
    let type_node_2 = ASTNode::new(NodeType::Type(DataType::Integer));
    let mut var_node_2 = ASTNode::new(NodeType::Variable);
    var_node_2.add_child(id_node_2);
    var_node_2.add_child(type_node_2);
    assignment_node_2.add_child(var_node_2);

    let mut var_node = ASTNode::new(NodeType::Variable);
    var_node.add_child(id_node);
    var_node.add_child(type_node);

    let mut value_node = ASTNode::new(NodeType::AssignedValue);

    let num_node = ASTNode::new(NodeType::Literal("0".to_string()));
    value_node.add_child(num_node);

    assignment_node.add_child(var_node.clone());
    assignment_node.add_child(value_node);

    let mut loop_node = ASTNode::new(NodeType::ForLoop);

    let mut init_node = ASTNode::new(NodeType::LoopInitializer);
    init_node.add_child(assignment_node);

    loop_node.add_child(init_node);

    let mut for_condition = ASTNode::new(NodeType::Condition);
    let for_condition_value = ASTNode::new(NodeType::Literal("true".to_string()));
    for_condition.add_child(for_condition_value);

    loop_node.add_child(for_condition);

    let mut for_inc = ASTNode::new(NodeType::LoopIncrement);

    let mut reassignment_node = ASTNode::new(NodeType::Assignment);
    reassignment_node.add_child(var_node);
    let mut value_node = ASTNode::new(NodeType::AssignedValue);
    value_node.add_child(ASTNode::new(NodeType::Literal("42".to_string())));
    reassignment_node.add_child(value_node);

    for_inc.add_child(reassignment_node);
    loop_node.add_child(for_inc);

    let mut for_body = ASTNode::new(NodeType::BlockExpression);
    for_body.add_child(ASTNode::new(NodeType::Continue));

    loop_node.add_child(for_body);

    // outer loop
    let mut assignment_node = ASTNode::new(NodeType::Initialization);

    let id_node = ASTNode::new(NodeType::Identifier("test_var_outer".to_string()));
    let type_node = ASTNode::new(NodeType::Type(DataType::Integer));

    let mut assignment_node_2 = ASTNode::new(NodeType::Initialization);
    let id_node_2 = ASTNode::new(NodeType::Identifier("test_var_outer_2".to_string()));
    let type_node_2 = ASTNode::new(NodeType::Type(DataType::Integer));
    let mut var_node_2 = ASTNode::new(NodeType::Variable);
    var_node_2.add_child(id_node_2);
    var_node_2.add_child(type_node_2);
    assignment_node_2.add_child(var_node_2);

    let mut var_node = ASTNode::new(NodeType::Variable);
    var_node.add_child(id_node);
    var_node.add_child(type_node);

    let mut value_node = ASTNode::new(NodeType::AssignedValue);

    let num_node = ASTNode::new(NodeType::Literal("0".to_string()));
    value_node.add_child(num_node);

    assignment_node.add_child(var_node.clone());
    assignment_node.add_child(value_node);

    let mut loop_node_outer = ASTNode::new(NodeType::ForLoop);

    let mut init_node = ASTNode::new(NodeType::LoopInitializer);
    init_node.add_child(assignment_node);

    loop_node_outer.add_child(init_node);

    let mut for_condition = ASTNode::new(NodeType::Condition);
    let for_condition_value = ASTNode::new(NodeType::Literal("true".to_string()));
    for_condition.add_child(for_condition_value);

    loop_node_outer.add_child(for_condition);

    let mut for_inc = ASTNode::new(NodeType::LoopIncrement);

    let mut reassignment_node = ASTNode::new(NodeType::Assignment);
    reassignment_node.add_child(var_node);
    let mut value_node = ASTNode::new(NodeType::AssignedValue);
    value_node.add_child(ASTNode::new(NodeType::Literal("42".to_string())));
    reassignment_node.add_child(value_node);

    for_inc.add_child(reassignment_node);
    loop_node_outer.add_child(for_inc);

    let mut for_body = ASTNode::new(NodeType::BlockExpression);
    for_body.add_child(loop_node);

    loop_node_outer.add_child(for_body);

    let mut fn_block = ASTNode::new(NodeType::BlockExpression);
    fn_block.add_child(loop_node_outer);
    //fn_block.add_child(assignment_node_2);

    let fn_type = ASTNode::new(NodeType::Type(DataType::Integer));
    let fn_id = ASTNode::new(NodeType::Identifier("testForLoopNested".to_string()));

    let mut fn_declaration_node = ASTNode::new(NodeType::FunctionDeclaration);
    fn_declaration_node.add_child(fn_id);
    fn_declaration_node.add_child(fn_type);
    fn_declaration_node.add_child(fn_block);

    let ast = wrap_in_tle(fn_declaration_node);

    let mut sts_stack = SymbolTableStack::new();
    let mut sts_global = SymbolTable::new();
    let fn_value = SymbolValue::FunctionValue{
        parameters: Vec::new(),
    };
    let fn_info = SymbolInfo::new(DataType::Integer, fn_value);
    sts_global.add("testForLoopNested".to_string(), fn_info);
    sts_stack.push(sts_global);
    sts_stack.push(SymbolTable::new());

    let mod_ast: Module = ast_stitch(vec![ModElement::new(ast, sts_stack, DEFAULT_PRIORITY_MODELEMENT)]);

    let mut ir_generator = IRGenerator::new();
    let module_tag = ir_generator.generate_ir(mod_ast);  

    let pools = ir_generator.get_resource_pools();

    let module = pools.lock().expect("coouldn't unlock pools mutex").get_module(module_tag).expect("No module found!");
    let write_result = io::write_ir_to_file(module.clone(), "output_nested_for_loop.ll");
    match write_result {
        Ok(_) => eprintln!("For loop test file written correctly!"),
        Err(_) => panic!("For loop test file failed to write!")
    }
    let get_str = io::write_to_string(module);
    let test_str = match get_str {
        Ok(str) => str,
        Err(e) => panic!("{}", e)
    };
    let expected_str = "; ModuleID = 'dummy_module'\nsource_filename = \"dummy_module\"\n\ndefine i64 @testForLoopNested() {\nentryID0:\n  %test_var_outer = alloca i64, align 8\n  store i64 0, ptr %test_var_outer, align 4\n  br label %for_condID1\n\nfor_condID1:                                      ; preds = %for_incID1, %entryID0\n  br i1 true, label %for_bodyID1, label %for_endID1\n\nfor_bodyID1:                                      ; preds = %for_condID1\n  %test_var = alloca i64, align 8\n  store i64 0, ptr %test_var, align 4\n  br label %for_condID2\n\nfor_condID2:                                      ; preds = %for_incID2, %for_bodyID1\n  br i1 true, label %for_bodyID2, label %for_endID2\n\nfor_bodyID2:                                      ; preds = %for_condID2\n  br label %for_incID2\n  br label %for_incID2\n\nfor_incID2:                                       ; preds = %for_bodyID2, %for_bodyID2\n  store i64 42, ptr %test_var, align 4\n  br label %for_condID2\n\nfor_endID2:                                       ; preds = %for_condID2\n  br label %for_incID1\n\nfor_incID1:                                       ; preds = %for_endID2\n  store i64 42, ptr %test_var_outer, align 4\n  br label %for_condID1\n\nfor_endID1:                                       ; preds = %for_condID1\n}\n";

    assert_eq!(test_str, expected_str)
}

#[test]
fn test_return_var() {
    /* 
    int testFunctionWithRetrieveReturn() {
        int i = 42;
        return i;
    }
    
    ; ModuleID = 'dummy_module'
    source_filename = "dummy_module"

    define i64 @testFunctionWithRetrieveReturn() {
    entryID0:
      %i = alloca i64, align 8
      store i64 42, ptr %i, align 4
      %vrecallID1 = load i64, ptr %i, align 4
      ret i64 %vrecallID1
}

    */
    let mut assignment_node = ASTNode::new(NodeType::Initialization);

    let id_node = ASTNode::new(NodeType::Identifier("i".to_string()));
    let type_node = ASTNode::new(NodeType::Type(DataType::Integer));

    let mut var_node = ASTNode::new(NodeType::Variable);
    var_node.add_child(id_node);
    var_node.add_child(type_node);

    let mut value_node = ASTNode::new(NodeType::AssignedValue);

    let num_node = ASTNode::new(NodeType::Literal("42".to_string()));
    value_node.add_child(num_node);

    assignment_node.add_child(var_node.clone());
    assignment_node.add_child(value_node);

    let mut ret_node = ASTNode::new(NodeType::Return);
    let mut val_node_return = ASTNode::new(NodeType::AssignedValue);

    val_node_return.add_child(var_node);
    ret_node.add_child(val_node_return);


    let mut fn_block = ASTNode::new(NodeType::BlockExpression);
    fn_block.add_child(assignment_node);
    fn_block.add_child(ret_node);

    let fn_type = ASTNode::new(NodeType::Type(DataType::Integer));
    let fn_id = ASTNode::new(NodeType::Identifier("testFunctionWithRetrieveReturn".to_string()));

    let mut fn_declaration_node = ASTNode::new(NodeType::FunctionDeclaration);
    fn_declaration_node.add_child(fn_id);
    fn_declaration_node.add_child(fn_type);
    fn_declaration_node.add_child(fn_block);

    let ast = wrap_in_tle(fn_declaration_node);

    let mut sts_stack = SymbolTableStack::new();
    let mut sts_global = SymbolTable::new();
    let fn_value = SymbolValue::FunctionValue{
        parameters: Vec::new(),
    };
    let fn_info = SymbolInfo::new(DataType::Integer, fn_value);
    sts_global.add("testFunctionWithRetrieveReturn".to_string(), fn_info);
    sts_stack.push(sts_global);
    sts_stack.push(SymbolTable::new());

    let mod_ast: Module = ast_stitch(vec![ModElement::new(ast, sts_stack, DEFAULT_PRIORITY_MODELEMENT)]);

    let mut ir_generator = IRGenerator::new();
    let module_tag = ir_generator.generate_ir(mod_ast);  

    let pools = ir_generator.get_resource_pools();

    let module = pools.lock().expect("coouldn't unlock pools mutex").get_module(module_tag).expect("No module found!");
    let write_result = io::write_ir_to_file(module.clone(), "output_retrieve_return.ll");
    match write_result {
        Ok(_) => eprintln!("Retrieve return test file written correctly!"),
        Err(_) => panic!("Retrieve return test file failed to write!")
    }
    let get_str = io::write_to_string(module);
    let test_str = match get_str {
        Ok(str) => str,
        Err(e) => panic!("{}", e)
    };
    let expected_str = "; ModuleID = 'dummy_module'\nsource_filename = \"dummy_module\"\n\ndefine i64 @testFunctionWithRetrieveReturn() {\nentryID0:\n  %i = alloca i64, align 8\n  store i64 42, ptr %i, align 4\n  %vrecallID1 = load i64, ptr %i, align 4\n  ret i64 %vrecallID1\n}\n";

    assert_eq!(test_str, expected_str)
}