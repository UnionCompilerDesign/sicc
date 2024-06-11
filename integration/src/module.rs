//! This file defines the core structures and functionalities associated with our `Module`.
//! `Module` types are composite structures that aggregate multiple module elements, each containing an AST and a symbol table stack.
use common::ast::{core::{ASTNode, AST}, node_type::NodeType};
use sts::core::SymbolTableStack;

/// Represents a module, which is a collection of `ModElement` instances.
pub struct Module {
    /// A vector of top level expressions that make up this module.
    children: Vec<ModElement>,
}

impl Module {
    /// Constructs a new empty `Module`.
    pub fn new() -> Self {
        Module {
            children: Vec::new(),
        }
    }

    /// Adds a `ModElement` to the module.
    ///
    /// # Parameters
    /// * `child` - The `ModElement` to be added to the module.
    pub fn add_child(&mut self, child: ModElement) {
        self.children.push(child);
    }

    /// Retrieves a reference to a child `ModElement` at a specific index, if it exists.
    ///
    /// # Parameters
    /// * `index` - The index of the child to retrieve.
    ///
    /// # Returns
    /// An `Option` containing a reference to the `ModElement` if found, or `None` if the index is out of bounds.
    pub fn get_child(&self, index: usize) -> Option<&ModElement> {
        self.children.get(index)
    }

    /// Retrieves a reference to all children `ModElement` within the module.
    ///
    /// # Returns
    /// A reference to a vector of `ModElement`.
    pub fn get_children(&self) -> &Vec<ModElement> {
        &self.children
    }

    /// Retrieves a mutable reference to all children `ModElement` within the module.
    /// This allows for modifying each element directly within the module.
    ///
    /// # Returns
    /// A mutable reference to a vector of `ModElement`.
    pub fn get_mut_children(&mut self) -> &mut Vec<ModElement> {
        &mut self.children
    }
}

/// Provides a clone implementation for `Module`.
///
/// # Returns
/// * `Module` - A new instance of `Module` with cloned children.
impl Clone for Module {
    fn clone(&self) -> Self {
        Module {
            children: self.children.clone(),
        }
    }
}

/// Represents an individual element of a module, encapsulating an AST and a symbol table stack.
///
/// `ModElement` is designed to hold the syntactic and semantic information necessary for a top level expression.
#[derive(Clone)]
pub struct ModElement {
    /// The Abstract Syntax Tree (AST) associated with this module element.
    ast: AST,
    /// The symbol table stack providing the context for the AST.
    sym_table_stack: SymbolTableStack,
    /// The priority of this element.
    priority: i32,
}

impl ModElement {
    /// Constructs a new `ModElement` with an AST, a symbol table stack, and a priority.
    ///
    /// # Parameters
    /// * `ast` - The `AST` of the module element.
    /// * `sym_table_stack` - The symbol table stack associated with the module element.
    /// * `priority` - The processing priority of the module element.
    pub fn new(ast: AST, sym_table_stack: SymbolTableStack, priority: i32) -> Self {
        Self {
            ast,
            sym_table_stack,
            priority,
        }
    }
    
    /// Retrieves the AST of this module element.
    ///
    /// # Returns
    /// A clone of the `AST` associated with this module element.
    pub fn get_ast(&self) -> AST {
        self.ast.clone()
    }

    /// Retrieves the symbol table stack of this module element.
    ///
    /// # Returns
    /// A clone of the `SymbolTableStack` associated with this module element.
    pub fn get_sym_table_stack(&self) -> SymbolTableStack {
        self.sym_table_stack.clone()
    }

    /// Sets a new priority for this module element.
    ///
    /// # Parameters
    /// * `new` - The new priority to be set for this module element.
    pub fn set_priority(&mut self, new: i32) {
        self.priority = new;
    }
}

/// Provides an ordering implementation for `ModElement`.
///
/// # Parameters
/// * `other` - The other `ModElement` to compare against.
///
/// # Returns
/// * `std::cmp::Ordering` - The result of the comparison.
impl Ord for ModElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

/// Provides a partial ordering implementation for `ModElement`.
///
/// # Parameters
/// * `other` - The other `ModElement` to compare against.
///
/// # Returns
/// * `Option<std::cmp::Ordering>` - The result of the partial comparison.
impl PartialOrd for ModElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Provides an equality comparison implementation for `ModElement`.
///
/// # Parameters
/// * `other` - The other `ModElement` to compare against.
///
/// # Returns
/// * `bool` - `true` if the priorities are equal, otherwise `false`.
impl PartialEq for ModElement {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

/// Provides a full equality comparison for `ModElement`.
impl Eq for ModElement {} 


/// Constructs a `Module` by stitching together a vector of `ModElement` based on their priorities.
/// This function takes a vector of `ModElement`, assigns a priority based on the AST root, and aggregates them into a `Module`.
///
/// # Parameters
/// * `input` - A vector of `ModElement` to be stitched into a module.
///
/// # Returns
/// A `Module` composed of the provided elements, ordered by their priority.
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

/// Determines the priority of a `ModElement` based on the type of its AST root.
///
/// # Parameters
/// * `ast_root` - The root node of the AST of a module element.
///
/// # Returns
/// An integer representing the priority of the module element.
fn get_ast_priority(ast_root: ASTNode) -> i32 {
    match ast_root.get_node_type() {
        NodeType::ModuleExpression => 20,
        NodeType::TopLevelExpression => 10,
        _ => panic!("Not a valid root expression: {:?}", ast_root.get_node_type())
    }
}
