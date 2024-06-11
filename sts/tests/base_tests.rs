//! Base Case Testing for STS.

use common::ast::{
    core::{ASTNode, AST}, 
    data_type::DataType, 
    node_type::NodeType
};
use sts::core::{SymbolInfo, SymbolTable, SymbolTableStack, SymbolValue};

#[test]
fn test_empty_function() {
    let mut fn_node = ASTNode::new(NodeType::FunctionDeclaration);
    fn_node.set_children(vec![
        ASTNode::new(NodeType::Identifier("empty_function".to_string())),
        ASTNode::new(NodeType::Type(DataType::Void)),
    ]);

    let stack_pair_result = SymbolTableStack::gen_sym_table_stack(AST::new(fn_node.clone()));

    assert_eq!(stack_pair_result, Ok((AST::new(fn_node), {
        let mut stack = SymbolTableStack::new();
        let mut table = SymbolTable::new();
        table.add("empty_function".to_string(), SymbolInfo::new(DataType::Void, SymbolValue::FunctionValue{parameters: vec![]}));
        stack.push(table);
        stack
    })));
}

#[test]
fn test_empty_function_with_parameter() {
    let mut param_node = ASTNode::new(NodeType::Parameter);
    param_node.set_children(vec![
        ASTNode::new(NodeType::Identifier("param1".to_string())),
        ASTNode::new(NodeType::Type(DataType::Integer)),
    ]);

    let mut fn_node = ASTNode::new(NodeType::FunctionDeclaration);
    fn_node.set_children(vec![
        ASTNode::new(NodeType::Identifier("empty_function_with_param".to_string())),
        ASTNode::new(NodeType::Type(DataType::Void)),
        param_node,
    ]);

    let stack_pair_result = SymbolTableStack::gen_sym_table_stack(AST::new(fn_node.clone()));

    assert_eq!(stack_pair_result, Ok((AST::new(fn_node), {
        let mut stack = SymbolTableStack::new();
        let mut table = SymbolTable::new();
        table.add("empty_function_with_param".to_string(), SymbolInfo::new(DataType::Void, SymbolValue::FunctionValue{
            parameters: vec![("param1".to_string(), DataType::Integer)]
        }));
        stack.push(table);
        stack
    })));
}


#[test]
fn test_for_loop() {
    let condition_node = ASTNode::new(NodeType::Condition);
    let block_node = ASTNode::new(NodeType::BlockExpression);
    let mut for_node = ASTNode::new(NodeType::ForLoop);
    for_node.set_children(vec![condition_node, block_node]);

    let stack_pair_result = SymbolTableStack::gen_sym_table_stack(AST::new(for_node.clone()));

    assert_eq!(stack_pair_result, Ok((AST::new(for_node), {
        let mut stack = SymbolTableStack::new();
        let global_table = SymbolTable::new();
        let loop_condition_table = SymbolTable::new();
        let loop_block_table = SymbolTable::new();
        stack.push(global_table);
        stack.push(loop_condition_table);
        stack.push(loop_block_table);
        stack
    })));
}

#[test]
fn test_while_loop() {
    let block_node = ASTNode::new(NodeType::BlockExpression);
    let mut while_node = ASTNode::new(NodeType::WhileLoop);
    while_node.set_children(vec![block_node]);

    let stack_pair_result = SymbolTableStack::gen_sym_table_stack(AST::new(while_node.clone()));

    assert_eq!(stack_pair_result, Ok((AST::new(while_node), {
        let mut stack = SymbolTableStack::new();
        let global_table = SymbolTable::new();
        let loop_table = SymbolTable::new();
        stack.push(global_table);
        stack.push(loop_table);
        stack
    })));
}

#[test]
fn test_do_while_loop() {
    let block_node = ASTNode::new(NodeType::BlockExpression);
    let mut do_while_node = ASTNode::new(NodeType::DoWhileLoop);
    do_while_node.set_children(vec![block_node]);

    let stack_pair_result = SymbolTableStack::gen_sym_table_stack(AST::new(do_while_node.clone()));

    assert_eq!(stack_pair_result, Ok((AST::new(do_while_node), {
        let mut stack = SymbolTableStack::new();
        let global_table = SymbolTable::new();
        let loop_table = SymbolTable::new();
        stack.push(global_table);
        stack.push(loop_table);
        stack
    })));
}

#[test]
fn test_if_else_statement() {
    let if_block_node = ASTNode::new(NodeType::BlockExpression);
    let else_block_node = ASTNode::new(NodeType::BlockExpression);
    let mut if_node = ASTNode::new(NodeType::IfStatement);
    if_node.set_children(vec![if_block_node]);
    let mut else_node = ASTNode::new(NodeType::ElseStatement);
    else_node.set_children(vec![else_block_node]);
    let mut if_else_node = ASTNode::new(NodeType::IfStatement);
    if_else_node.set_children(vec![if_node, else_node]);

    let stack_pair_result = SymbolTableStack::gen_sym_table_stack(AST::new(if_else_node.clone()));

    assert_eq!(stack_pair_result, Ok((AST::new(if_else_node), {
        let mut stack = SymbolTableStack::new();
        let global_table = SymbolTable::new();
        let if_table = SymbolTable::new();
        let else_table = SymbolTable::new();
        stack.push(global_table);
        stack.push(if_table);
        stack.push(else_table);
        stack
    })));
}

#[test]
fn test_switch_statement() {
    let mut case_node1 = ASTNode::new(NodeType::Case);
    case_node1.set_children(vec![ASTNode::new(NodeType::BlockExpression)]);
    let mut case_node2 = ASTNode::new(NodeType::Case);
    case_node2.set_children(vec![ASTNode::new(NodeType::BlockExpression)]);
    let mut case_node3 = ASTNode::new(NodeType::Case);
    case_node3.set_children(vec![ASTNode::new(NodeType::BlockExpression)]);
    let mut switch_node = ASTNode::new(NodeType::SwitchStatement);
    switch_node.set_children(vec![case_node1, case_node2, case_node3]);

    let stack_pair_result = SymbolTableStack::gen_sym_table_stack(AST::new(switch_node.clone()));

    assert_eq!(stack_pair_result, Ok((AST::new(switch_node), {
        let mut stack = SymbolTableStack::new();
        let global_table = SymbolTable::new();
        let case1_table = SymbolTable::new();
        let case2_table = SymbolTable::new();
        let case3_table = SymbolTable::new();
        stack.push(global_table);
        stack.push(case1_table);
        stack.push(case2_table);
        stack.push(case3_table);
        stack
    })));
}

#[test]
fn test_initialization() {
    let name = ASTNode::new(NodeType::Identifier("foo".to_string()));
    let kind = ASTNode::new(NodeType::Type(DataType::Integer));
    let mut init = ASTNode::new(NodeType::Initialization);

    init.add_child(name);
    init.add_child(kind);

    let ast = AST::new(init.clone());

    let generated_stack = SymbolTableStack::gen_sym_table_stack(ast);

    assert_eq!(generated_stack, Ok((AST::new(init), {
        let mut stack = SymbolTableStack::new();
        let mut global_table = SymbolTable::new();
        let init_info = SymbolInfo::new(
            DataType::Integer,
            SymbolValue::NoAssociatedValue,
        );
        global_table.add("foo".to_string(), init_info);
        stack.push(global_table);
        stack
    })));
}