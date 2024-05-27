use common::ast::{ast_struct::{ASTNode, AST}, syntax_element::SyntaxElement};
use symbol_table::symbol_table_struct::SymbolTableStack;

/// A Module
pub struct Module {
    children: Vec<ModElement>,
}

impl Module {
    /// A new module
    pub fn new() -> Self {
        Module {
            children: Vec::new(),
        }
    }

    /// Adds a child to the module
    pub fn add_child(&mut self, child: ModElement) {
        self.children.push(child);
    }

    /// Retrieves the child with index
    pub fn get_child(&self, index: usize) -> Option<&ModElement> {
        self.children.get(index)
    }

    /// Retrieves all children
    pub fn get_children(&self) -> &Vec<ModElement> {
        &self.children
    }

    pub fn get_mut_children(&mut self) -> &mut Vec<ModElement>{
        &mut self.children
    }
}

impl Clone for Module {
    fn clone(&self) -> Self {
        Module {
            children: self.children.clone(),
        }
    }
}

/// An element of a module
#[derive(Clone)]
pub struct ModElement {
    ast: AST,
    sym_table_stack: SymbolTableStack,
    priority: i32,
}

impl ModElement {
    /// Creates a new module element
    pub fn new(ast: AST, sym_table_stack: SymbolTableStack, priority: i32) -> Self {
        Self {
            ast,
            sym_table_stack,
            priority,
        }
    }
    
    /// Retrieves the ast of the mod element
    pub fn get_ast(&self) -> AST {
        self.ast.clone()
    }

    /// Retrieves the symbol table stack of the mod element
    pub fn get_sym_table_stack(&self) -> SymbolTableStack {
        self.sym_table_stack.clone()
    }

    /// Retrieves the priority of the mod element
    pub fn set_priority(&mut self, new: i32) {
        self.priority = new;
    }
}

impl Ord for ModElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for ModElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ModElement {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for ModElement {} 

/// Pieces together mod elements into a cohesive module
pub fn ast_stitch(input: Vec<ModElement>) -> Module {
    let mut mod_ast: Module = Module::new(); 
    for mut mod_element in input {
        let root = mod_element.get_ast().get_root();
        let priority = get_ast_priority(root);
        mod_element.set_priority(priority);
        mod_ast.add_child(mod_element);
    }
    mod_ast
}

fn get_ast_priority(ast_root: ASTNode) -> i32 {
    match ast_root.get_element() {
        SyntaxElement::ModuleExpression => {
            return 20;
        },
        SyntaxElement::TopLevelExpression => {
            return 10;
        },
        _ => panic!("Not a valid root expression: {:?}", ast_root.get_element())
    }
}