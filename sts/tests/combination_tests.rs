use common::ast::{
    core::{ASTNode, AST}, 
    node_type::NodeType, 
};
use sts::core::{SymbolTable, SymbolTableStack};

// Identical to previous test without function --> scope management for nested loops 
#[test]
fn test_nested_loops() {
    // Setup AST for nested loops
    let mut do_while_node = ASTNode::new(NodeType::DoWhileLoop);
    let mut do_while_block = ASTNode::new(NodeType::BlockExpression);
    let mut while_node = ASTNode::new(NodeType::WhileLoop);
    let mut while_block = ASTNode::new(NodeType::BlockExpression);
    let mut for_node = ASTNode::new(NodeType::ForLoop);
    let for_condition = ASTNode::new(NodeType::Condition);
    let mut for_block = ASTNode::new(NodeType::BlockExpression);
    let mut switch_node = ASTNode::new(NodeType::SwitchStatement);
    let mut case_node = ASTNode::new(NodeType::Case);
    let case_block = ASTNode::new(NodeType::BlockExpression);

    // Constructing the nested loop structure
    case_node.add_child(case_block);
    switch_node.add_child(case_node);
    for_block.add_child(switch_node);
    for_node.add_child(for_condition);
    for_node.add_child(for_block);
    while_block.add_child(for_node);
    while_node.add_child(while_block);
    do_while_block.add_child(while_node);
    do_while_node.add_child(do_while_block);

    // Generate symbol table stack from AST.
    let ast: AST = AST::new(do_while_node);
    let generated_stack = SymbolTableStack::gen_sym_table_stack(ast.clone());

    // Initialize expected symbol tables.
    let global_table = SymbolTable::new();
    let do_while_table = SymbolTable::new();
    let while_table = SymbolTable::new();
    let for_condition_table = SymbolTable::new();
    let for_table = SymbolTable::new();
    let switch_table = SymbolTable::new();

    // Construct the expected symbol table stack.
    let mut expected_stack = SymbolTableStack::new();
    expected_stack.push(global_table);
    expected_stack.push(do_while_table);
    expected_stack.push(while_table);
    expected_stack.push(for_condition_table);
    expected_stack.push(for_table);
    expected_stack.push(switch_table);

    // Compare the generated stack with the expected stack.
    assert_eq!(generated_stack, Ok((ast, expected_stack)), "Generated symbol table stack should match the expected stack");
}


/// 4 times nested for loop scope test
#[test]
fn test_nested_for_loops() {
    // Setup AST for nested for loops
    let mut outer_for_loop = ASTNode::new(NodeType::ForLoop);
    let outer_condition = ASTNode::new(NodeType::Condition);
    let mut outer_block = ASTNode::new(NodeType::BlockExpression);

    let mut mid_for_loop = ASTNode::new(NodeType::ForLoop);
    let mid_condition = ASTNode::new(NodeType::Condition);
    let mut mid_block = ASTNode::new(NodeType::BlockExpression);

    let mut inner_for_loop = ASTNode::new(NodeType::ForLoop);
    let inner_condition = ASTNode::new(NodeType::Condition);
    let mut inner_block = ASTNode::new(NodeType::BlockExpression);

    let mut most_inner_for_loop = ASTNode::new(NodeType::ForLoop);
    let most_inner_condition = ASTNode::new(NodeType::Condition);
    let most_inner_block = ASTNode::new(NodeType::BlockExpression);

    most_inner_for_loop.add_child(most_inner_condition);
    most_inner_for_loop.add_child(most_inner_block);
    inner_block.add_child(most_inner_for_loop);
    inner_for_loop.add_child(inner_condition);
    inner_for_loop.add_child(inner_block);
    mid_block.add_child(inner_for_loop);
    mid_for_loop.add_child(mid_condition);
    mid_for_loop.add_child(mid_block);
    outer_block.add_child(mid_for_loop);
    outer_for_loop.add_child(outer_condition);
    outer_for_loop.add_child(outer_block);

    // Generate symbol table stack from AST.
    let ast: AST = AST::new(outer_for_loop);
    let generated_stack = SymbolTableStack::gen_sym_table_stack(ast.clone());

    // Initialize expected symbol tables.
    let global_table = SymbolTable::new();
    let outer_for_condition_table = SymbolTable::new();
    let outer_for_table = SymbolTable::new();
    let mid_for_condition_table = SymbolTable::new();
    let mid_for_table = SymbolTable::new();
    let inner_for_condition_table = SymbolTable::new();
    let inner_for_table = SymbolTable::new();
    let most_inner_for_condition_table: SymbolTable = SymbolTable::new();
    let most_inner_for_table: SymbolTable = SymbolTable::new();


    // Create expected symbol table stack.
    let mut expected_stack = SymbolTableStack::new();
    expected_stack.push(global_table);
    expected_stack.push(outer_for_condition_table);
    expected_stack.push(outer_for_table);
    expected_stack.push(mid_for_condition_table);
    expected_stack.push(mid_for_table);
    expected_stack.push(inner_for_condition_table);
    expected_stack.push(inner_for_table);
    expected_stack.push(most_inner_for_condition_table);
    expected_stack.push(most_inner_for_table);

    // Compare the generated stack with the expected stack.
    assert_eq!(generated_stack, Ok((ast, expected_stack)), "Generated symbol table stack should match the expected stack");
}

#[test]
fn test_nested_while_loops() {
    // Setup AST for nested while loops
    let mut outer_while_loop = ASTNode::new(NodeType::WhileLoop);
    let mut outer_while_block = ASTNode::new(NodeType::BlockExpression);

    let mut mid_while_loop = ASTNode::new(NodeType::WhileLoop);
    let mut mid_while_block = ASTNode::new(NodeType::BlockExpression);

    let mut inner_while_loop = ASTNode::new(NodeType::WhileLoop);
    let mut inner_while_block = ASTNode::new(NodeType::BlockExpression);

    let mut most_inner_while_loop = ASTNode::new(NodeType::WhileLoop);
    let most_inner_while_block = ASTNode::new(NodeType::BlockExpression);

    most_inner_while_loop.add_child(most_inner_while_block);
    inner_while_block.add_child(most_inner_while_loop);
    inner_while_loop.add_child(inner_while_block);
    mid_while_block.add_child(inner_while_loop);
    mid_while_loop.add_child(mid_while_block);
    outer_while_block.add_child(mid_while_loop);
    outer_while_loop.add_child(outer_while_block);

    // Generate symbol table stack from AST.
    let ast: AST = AST::new(outer_while_loop);
    let generated_stack = SymbolTableStack::gen_sym_table_stack(ast.clone());

    // Initialize expected symbol tables.
    let global_table = SymbolTable::new();
    let outer_while_table = SymbolTable::new();
    let mid_while_table = SymbolTable::new();
    let inner_while_table = SymbolTable::new();
    let most_inner_while_table = SymbolTable::new();

    // Create expected symbol table stack.
    let mut expected_stack = SymbolTableStack::new();
    expected_stack.push(global_table);
    expected_stack.push(outer_while_table);
    expected_stack.push(mid_while_table);
    expected_stack.push(inner_while_table);
    expected_stack.push(most_inner_while_table);

    // Compare the generated stack with the expected stack.
    assert_eq!(generated_stack, Ok((ast, expected_stack)), "Generated symbol table stack should match the expected stack");
}


#[test]
fn test_nested_do_while_loops() {
    // Setup AST for nested do-while loops
    let mut outer_do_while_loop = ASTNode::new(NodeType::DoWhileLoop);
    let mut outer_do_while_block = ASTNode::new(NodeType::BlockExpression);

    let mut mid_do_while_loop = ASTNode::new(NodeType::DoWhileLoop);
    let mut mid_do_while_block = ASTNode::new(NodeType::BlockExpression);

    let mut inner_do_while_loop = ASTNode::new(NodeType::DoWhileLoop);
    let mut inner_do_while_block = ASTNode::new(NodeType::BlockExpression);

    let mut most_inner_do_while_loop = ASTNode::new(NodeType::DoWhileLoop);
    let most_inner_do_while_block = ASTNode::new(NodeType::BlockExpression);

    most_inner_do_while_loop.add_child(most_inner_do_while_block);
    inner_do_while_block.add_child(most_inner_do_while_loop);
    inner_do_while_loop.add_child(inner_do_while_block);
    mid_do_while_block.add_child(inner_do_while_loop);
    mid_do_while_loop.add_child(mid_do_while_block);
    outer_do_while_block.add_child(mid_do_while_loop);
    outer_do_while_loop.add_child(outer_do_while_block);

    // Generate symbol table stack from AST.
    let ast: AST = AST::new(outer_do_while_loop);
    let generated_stack = SymbolTableStack::gen_sym_table_stack(ast.clone());

    // Initialize expected symbol tables.
    let global_table = SymbolTable::new();
    let outer_do_while_table = SymbolTable::new();
    let mid_do_while_table = SymbolTable::new();
    let inner_do_while_table = SymbolTable::new();
    let most_inner_do_while_table = SymbolTable::new();

    // Create expected symbol table stack.
    let mut expected_stack = SymbolTableStack::new();
    expected_stack.push(global_table);
    expected_stack.push(outer_do_while_table);
    expected_stack.push(mid_do_while_table);
    expected_stack.push(inner_do_while_table);
    expected_stack.push(most_inner_do_while_table);

    // Compare the generated stack with the expected stack.
    assert_eq!(generated_stack, Ok((ast, expected_stack)), "Generated symbol table stack should match the expected stack");
}


#[test]
fn test_nested_switch_statements() {
    // Setup AST for nested switch statements
    let mut outer_switch = ASTNode::new(NodeType::SwitchStatement);
    let mut outer_case = ASTNode::new(NodeType::Case);
    let mut outer_case_block = ASTNode::new(NodeType::BlockExpression);

    let mut mid_switch = ASTNode::new(NodeType::SwitchStatement);
    let mut mid_case = ASTNode::new(NodeType::Case);
    let mut mid_case_block = ASTNode::new(NodeType::BlockExpression);

    let mut inner_switch = ASTNode::new(NodeType::SwitchStatement);
    let mut inner_case = ASTNode::new(NodeType::Case);
    let mut inner_case_block = ASTNode::new(NodeType::BlockExpression);

    let mut most_inner_switch = ASTNode::new(NodeType::SwitchStatement);
    let mut most_inner_case = ASTNode::new(NodeType::Case);
    let most_inner_case_block = ASTNode::new(NodeType::BlockExpression);

    most_inner_case.add_child(most_inner_case_block);
    most_inner_switch.add_child(most_inner_case);
    
    inner_case_block.add_child(most_inner_switch);
    inner_case.add_child(inner_case_block);
    inner_switch.add_child(inner_case);

    mid_case_block.add_child(inner_switch);
    mid_case.add_child(mid_case_block);
    mid_switch.add_child(mid_case);

    outer_case_block.add_child(mid_switch);
    outer_case.add_child(outer_case_block);
    outer_switch.add_child(outer_case);

    // Generate symbol table stack from AST.
    let ast: AST = AST::new(outer_switch);
    let generated_stack = SymbolTableStack::gen_sym_table_stack(ast.clone());

    // Initialize expected symbol tables.
    let  global_table = SymbolTable::new();
    let outer_switch_table = SymbolTable::new();
    let mid_switch_table = SymbolTable::new();
    let inner_switch_table = SymbolTable::new();
    let most_inner_switch_table = SymbolTable::new();

    // Create expected symbol table stack.
    let mut expected_stack = SymbolTableStack::new();
    expected_stack.push(global_table);
    expected_stack.push(outer_switch_table);
    expected_stack.push(mid_switch_table);
    expected_stack.push(inner_switch_table);
    expected_stack.push(most_inner_switch_table);

    // Compare the generated stack with the expected stack.
    assert_eq!(generated_stack, Ok((ast, expected_stack)), "Generated symbol table stack should match the expected stack");
}

