use common::{
    ast::{
        ast_struct::{ASTNode, AST}, data_type::DataType, syntax_element::SyntaxElement
    }, 
    error::ErrorType
};
use symbol_table::symbol_table_struct::{SymbolInfo, SymbolTable, SymbolTableStack, SymbolValue};

/*
Make sure to complete missing tests and expand upon this testing file to cover more 
than just the creation of structures and top level expressions. The more tests the better. 
*/

#[test]
fn test_function_declaration_sym_table() {
    let mut expected_sts: SymbolTableStack = SymbolTableStack::new();
    let mut global_scope: SymbolTable = SymbolTable::new();
    let fn_value: SymbolValue = SymbolValue::FunctionValue { parameters: vec![]};

    let fn_info: SymbolInfo = SymbolInfo::new(DataType::Function, fn_value);
    global_scope.add("test_function".to_string(), fn_info);
    expected_sts.push(global_scope);

    expected_sts.push(SymbolTable::new());  // fn scope

    let fn_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("test_function".to_string()));
    let block_exp: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);

    let mut root: ASTNode = ASTNode::new(SyntaxElement::FunctionDeclaration);
    root.add_child(fn_id);
    root.add_child(block_exp);

    let ast: AST = AST::new(root);
    let result_sts_gen: Result<(AST, SymbolTableStack), Vec<ErrorType>> = SymbolTableStack::gen_sym_table_stack(ast);
    match result_sts_gen {
        Ok((_ast, sts)) => {
            assert_eq!(sts, expected_sts);
        }
        Err(e) => panic!("Couldn't parse AST, error: {:?}", e)
    }
}

#[test]
fn test_struct_declaration_sym_table() {
    let mut expected_sts: SymbolTableStack = SymbolTableStack::new();
    let mut global_scope: SymbolTable = SymbolTable::new();

    let struct_value: SymbolValue = SymbolValue::StructValue { 
        fields: vec![("field1".to_string(), DataType::Integer)],
    };

    let struct_info: SymbolInfo = SymbolInfo::new(DataType::Struct, struct_value);
    global_scope.add("test_struct".to_string(), struct_info);
    expected_sts.push(global_scope);

    let struct_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("test_struct".to_string()));

    let mut field_node: ASTNode = ASTNode::new(SyntaxElement::Field);
    let field_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("field1".to_string()));
    let field_type: ASTNode = ASTNode::new(SyntaxElement::Type(DataType::Integer));
    field_node.add_child(field_id);
    field_node.add_child(field_type);

    let mut root: ASTNode = ASTNode::new(SyntaxElement::StructDeclaration);
    root.add_child(struct_id);
    root.add_child(field_node);

    let ast: AST = AST::new(root);
    let result_sts_gen: Result<(AST, SymbolTableStack), Vec<ErrorType>> = SymbolTableStack::gen_sym_table_stack(ast);

    match result_sts_gen {
        Ok((_ast, sts)) => {
            assert_eq!(sts, expected_sts);
        },
        Err(e) => panic!("Couldn't parse AST, error: {:?}", e),
    }
}


#[test]
fn test_enum_declaration_sym_table() {
    let mut expected_sts: SymbolTableStack = SymbolTableStack::new();
    let mut global_scope: SymbolTable = SymbolTable::new();
    let enum_value: SymbolValue = SymbolValue::EnumValue { 
        variants: vec!["Variant1".to_string(), "Variant2".to_string()],
    };

    let enum_info: SymbolInfo = SymbolInfo::new(DataType::Enum, enum_value);
    global_scope.add("test_enum".to_string(), enum_info);
    expected_sts.push(global_scope);

    let enum_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("test_enum".to_string()));

    let mut variant1: ASTNode = ASTNode::new(SyntaxElement::Variant);
    let mut variant2: ASTNode = ASTNode::new(SyntaxElement::Variant);
    let variant1_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("Variant1".to_string()));
    let variant2_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("Variant2".to_string()));
    variant1.add_child(variant1_id);
    variant2.add_child(variant2_id);

    let mut root: ASTNode = ASTNode::new(SyntaxElement::EnumDeclaration);
    root.add_child(enum_id);
    root.add_child(variant1);
    root.add_child(variant2);
    
    let ast: AST = AST::new(root);
    let result_sts_gen: Result<(AST, SymbolTableStack), Vec<ErrorType>> = SymbolTableStack::gen_sym_table_stack(ast);

    match result_sts_gen {
        Ok((_ast, sts)) => {
            assert_eq!(sts, expected_sts);
        },
        Err(e) => panic!("Couldn't parse AST, error: {:?}", e),
    }
}


#[test]
fn test_variable_initialization_sym_table() {
    let mut expected_sts: SymbolTableStack = SymbolTableStack::new();
    let mut global_scope: SymbolTable = SymbolTable::new();
    let var_value: SymbolValue = SymbolValue::StrValue("true".to_string().into()); 

    let var_info: SymbolInfo = SymbolInfo::new(DataType::Boolean, var_value);
    global_scope.add("test_var".to_string(), var_info);
    expected_sts.push(global_scope);

    let var_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("test_var".to_string()));
    let var_type: ASTNode = ASTNode::new(SyntaxElement::Type(DataType::Boolean));
    let mut var_value: ASTNode = ASTNode::new(SyntaxElement::Variable);
    var_value.add_child(var_id);
    var_value.add_child(var_type);

    let mut assigned_value_node: ASTNode = ASTNode::new(SyntaxElement::AssignedValue);
    let assigned_value_literal: ASTNode = ASTNode::new(SyntaxElement::Literal("true".to_string()));
    assigned_value_node.add_child(assigned_value_literal);

    let mut root: ASTNode = ASTNode::new(SyntaxElement::Initialization);
    root.add_child(var_value);
    root.add_child(assigned_value_node);

    let ast: AST = AST::new(root);
    let result_sts_gen: Result<(AST, SymbolTableStack), Vec<ErrorType>> = SymbolTableStack::gen_sym_table_stack(ast);

    match result_sts_gen {
        Ok((_ast, sts)) => {
            assert_eq!(sts, expected_sts);
        },
        Err(e) => panic!("Couldn't parse AST, error: {:?}", e),
    }
}

#[test]
fn test_sts_equality() {
    let mut expected_sts: SymbolTableStack = SymbolTableStack::new();
    let mut global_scope: SymbolTable = SymbolTable::new();
    let enum_value: SymbolValue = SymbolValue::EnumValue { 
        variants: vec!["Variant1".to_string(), "Variant2".to_string()],
    };

    let enum_info: SymbolInfo = SymbolInfo::new(DataType::Enum, enum_value);
    global_scope.add("test_enum".to_string(), enum_info);
    expected_sts.push(global_scope);

    let mut expected_sts_2: SymbolTableStack = SymbolTableStack::new();
    let mut global_scope_2: SymbolTable = SymbolTable::new();
    let enum_value_2: SymbolValue = SymbolValue::EnumValue { 
        variants: vec!["Variant1".to_string(), "VariantX".to_string()], // edited value here
    };

    let enum_info_2: SymbolInfo = SymbolInfo::new(DataType::Enum, enum_value_2);
    global_scope_2.add("test_enum".to_string(), enum_info_2);
    expected_sts_2.push(global_scope_2);

    let enum_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("test_enum".to_string()));

    let mut variant1: ASTNode = ASTNode::new(SyntaxElement::Variant);
    let mut variant2: ASTNode = ASTNode::new(SyntaxElement::Variant);
    let variant1_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("Variant1".to_string()));
    let variant2_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("Variant2".to_string()));
    variant1.add_child(variant1_id);
    variant2.add_child(variant2_id);

    let mut root: ASTNode = ASTNode::new(SyntaxElement::EnumDeclaration);
    root.add_child(enum_id);
    root.add_child(variant1);
    root.add_child(variant2);
    
    let ast: AST = AST::new(root);
    let result_sts_gen: Result<(AST, SymbolTableStack), Vec<ErrorType>> = SymbolTableStack::gen_sym_table_stack(ast);

    match result_sts_gen {
        Ok((_ast, sts)) => {

            assert_eq!(sts, expected_sts);
            assert_eq!(sts, sts.clone());
            assert_ne!(sts, expected_sts_2)
        },
        Err(e) => panic!("Couldn't parse AST, error: {:?}", e),
    }
}

#[test]
fn test_switch_sym_table() {
    // Create the expected symbol table stack
    let mut expected_sts: SymbolTableStack = SymbolTableStack::new();
    let mut global_scope: SymbolTable = SymbolTable::new();

    // Add the switch statement to the expected global scope
    let switch_value: SymbolValue = SymbolValue::SwitchValue {
        cases: vec![
            ASTNode::new(SyntaxElement::Case),
            ASTNode::new(SyntaxElement::Case)
        ],
        default: Some(ASTNode::new(SyntaxElement::Default)),
    };
    let switch_info: SymbolInfo = SymbolInfo::new(DataType::Switch, switch_value);
    global_scope.add("test_switch".to_string(), switch_info);
    expected_sts.push(global_scope);

    // Create AST for switch statement
    let switch_id: ASTNode = ASTNode::new(SyntaxElement::Identifier("test_switch".to_string()));
    let case1: ASTNode = ASTNode::new(SyntaxElement::Case);
    let case2: ASTNode = ASTNode::new(SyntaxElement::Case);
    let default_case: ASTNode = ASTNode::new(SyntaxElement::Default);

    let mut root: ASTNode = ASTNode::new(SyntaxElement::SwitchStatement);
    root.add_child(switch_id);
    root.add_child(case1);
    root.add_child(case2);
    root.add_child(default_case);

    let ast: AST = AST::new(root);
    let result_sts_gen: Result<(AST, SymbolTableStack), Vec<ErrorType>> = SymbolTableStack::gen_sym_table_stack(ast);

    match result_sts_gen {
        Ok((_ast, sts)) => {
            println!("Generated STS:\n{}", sts.to_string());  // Print the generated symbol table stack
            println!("Expected STS:\n{}", expected_sts.to_string());  // Print the expected symbol table stack
            assert_eq!(sts.size(), expected_sts.size());
            assert_eq!(sts.to_string(), expected_sts.to_string());
        }
        Err(e) => panic!("Couldn't parse AST, error: {:?}", e)
    }
}



#[test]
fn test_if_else_sym_table() {
    // Create the expected symbol table stack
    let mut expected_sts: SymbolTableStack = SymbolTableStack::new();
    let mut global_scope: SymbolTable = SymbolTable::new();

    // Add the if-else statement to the expected global scope
    let if_else_value: SymbolValue = SymbolValue::IfElseValue {
        condition: ASTNode::new(SyntaxElement::Condition),
        if_block: ASTNode::new(SyntaxElement::BlockExpression),
        else_block: Some(ASTNode::new(SyntaxElement::BlockExpression)),
    };
    let if_else_info: SymbolInfo = SymbolInfo::new(DataType::IfElse, if_else_value);
    global_scope.add("if_else".to_string(), if_else_info);
    expected_sts.push(global_scope);

    // Create AST for if-else statement
    let condition: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let if_block: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);
    let else_block: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);

    let mut root: ASTNode = ASTNode::new(SyntaxElement::IfStatement);
    root.add_child(condition.clone());
    root.add_child(if_block.clone());
    root.add_child(else_block.clone());

    let ast: AST = AST::new(root);
    let result_sts_gen: Result<(AST, SymbolTableStack), Vec<ErrorType>> = SymbolTableStack::gen_sym_table_stack(ast);

    match result_sts_gen {
        Ok((_ast, sts)) => {
            println!("Generated STS:\n{}", sts.to_string());  // Print the generated symbol table stack
            println!("Expected STS:\n{}", expected_sts.to_string());  // Print the expected symbol table stack
            assert_eq!(sts.size(), expected_sts.size());
            assert_eq!(sts.to_string(), expected_sts.to_string());
        }
        Err(e) => panic!("Couldn't parse AST, error: {:?}", e)
    }
}

#[test]
fn test_for_loop_sym_table() {
    // Create the expected symbol table stack
    let mut expected_sts: SymbolTableStack = SymbolTableStack::new();
    let mut global_scope: SymbolTable = SymbolTable::new();

    // Add the for loop to the expected global scope
    let for_loop_value: SymbolValue = SymbolValue::ForLoopValue {
        initializer: ASTNode::new(SyntaxElement::LoopInitializer),
        condition: ASTNode::new(SyntaxElement::Condition),
        increment: ASTNode::new(SyntaxElement::LoopIncrement),
        body: ASTNode::new(SyntaxElement::BlockExpression),
    };
    let for_loop_info: SymbolInfo = SymbolInfo::new(DataType::ForLoop, for_loop_value);
    global_scope.add("for_loop".to_string(), for_loop_info);
    expected_sts.push(global_scope);

    // Create AST for for loop
    let initializer: ASTNode = ASTNode::new(SyntaxElement::LoopInitializer);
    let condition: ASTNode = ASTNode::new(SyntaxElement::Condition);
    let increment: ASTNode = ASTNode::new(SyntaxElement::LoopIncrement);
    let body: ASTNode = ASTNode::new(SyntaxElement::BlockExpression);

    let mut root: ASTNode = ASTNode::new(SyntaxElement::ForLoop);
    root.add_child(initializer.clone());
    root.add_child(condition.clone());
    root.add_child(increment.clone());
    root.add_child(body.clone());

    let ast: AST = AST::new(root);
    let result_sts_gen: Result<(AST, SymbolTableStack), Vec<ErrorType>> = SymbolTableStack::gen_sym_table_stack(ast);

    match result_sts_gen {
        Ok((_ast, sts)) => {
            println!("Generated STS:\n{}", sts.to_string());  // Print the generated symbol table stack
            println!("Expected STS:\n{}", expected_sts.to_string());  // Print the expected symbol table stack
            assert_eq!(sts.size(), expected_sts.size());
            assert_eq!(sts.to_string(), expected_sts.to_string());
        }
        Err(e) => panic!("Couldn't parse AST, error: {:?}", e)
    }
}

#[test]
fn test_while_loop_sym_table() {
    
}

/// Do-While test
#[test]
fn test_do_while_loop_sym_table() {
    
}

