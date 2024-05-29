//! This file defines the core structures and functionalities associated with our Abstract Syntax Tree (AST).
//! It's used to represent the syntactic structure of source code in a tree.

use core::fmt;
use std::cmp::{Eq, PartialEq};
use crate::ast::syntax_element::SyntaxElement;

/// The `AST` struct is an Abstract Syntax Tree.
/// 
/// This structure holds the top-level node of the tree and provides protected access to the root node.
///
/// # Fields
/// * `root` - The root node of the AST, who's children contain the entire structure of the parsed source code.
#[derive(Debug, PartialEq, Clone)]
pub struct AST {
    root: ASTNode,
}

/// Represents a single node within an AST.
///
/// Each `ASTNode` contains a `SyntaxElement` and zero or more children, which are also `ASTNode`s.
/// Nodes represent syntactic constructs (like expressions, statements, etc.) and are linked hierarchically.
/// A node's children describe the node itself. For example, for the source file:
/// 
/// // main.c
/// int main() {
///     if (0) {
///         return 0;
///     }
///     return 1;
/// }
/// The corresponding tree will look like:
/// ModuleExpression
///      TopLevelExpression
///           FunctionExpression
///                Type(int)
///                Identifier("main")
///                BlockExpression
///                     IfStatement
///                          Condition
///                               Constant(0)
///                          BlockExpression
///                               Return
///                                    AssignedValue
///                                         Constant(0)
///                     Return
///                         AssignedValue
///                              Constant(1)
///
/// # Fields
/// * `element` - The syntactic element this node represents.
/// * `children` - A vector of child nodes, which further define the structure of the syntax tree.
#[derive(Debug, Clone, PartialEq, Default, Eq, Hash)]
pub struct ASTNode {
    element: SyntaxElement,
    children: Vec<ASTNode>, 
}

impl AST {
    /// Creates a new `AST` with a root node.
    /// 
    /// # Parameters
    /// * `root` - The `ASTNode` that is the root of the tree.
    pub fn new(root: ASTNode) -> Self {
        AST { 
            root 
        }
    }

    /// Retrieves the root node of the AST.
    ///
    /// # Returns
    /// * `ASTNode` - A clone of the root node.
    pub fn get_root(&self) -> ASTNode {
        self.root.clone()
    }

}


/// Provides a display implementation for `AST`.
///
/// # Parameters
/// * `f` - The formatter.
///
/// # Returns
/// * `fmt::Result` - The result of the formatting operation.
impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output_str = String::from("AST: \n");

        ast_format(&self.get_root(), &mut output_str, 0);

        write!(f, "{}", output_str)
    }
}

/// Formats an `AST` starting from a specific node and appends the formatted string to the provided output string.
///
/// This function recurses through all children of the node, increasing the indentation level for each level of depth
/// to represent the hierarchy of the `AST`.
///
/// # Parameters
/// * `root` - The current node of the AST being formatted.
/// * `output_string_ref` - A mutable reference to a string where the formatted output is appended.
/// * `depth` - The current depth in the AST, used to determine the indentation level.
fn ast_format(root: &ASTNode, output_string_ref: &mut String, depth: usize) {
    let node_repr_with_depth: String = std::iter::repeat("\t").take(depth).collect::<String>() + &root.get_element().to_string() + "\n";
    output_string_ref.push_str(&node_repr_with_depth);

    for child in root.get_children().iter() {
        ast_format(child, output_string_ref, depth + 1);
    }
}


impl ASTNode {
    /// Constructs a new `ASTNode` with the given syntax element and an empty list of children.
    /// 
    /// # Parameters
    /// * `element` - The `SyntaxElement` associated with this node.
    pub fn new(element: SyntaxElement) -> Self {
        ASTNode {
            element,
            children: Vec::new(),
        }
    }

    /// Gets the `SyntaxElement` of this `ASTNode`.
    ///
    /// # Returns
    /// * `SyntaxElement` - A clone of the element this node represents.
    pub fn get_element(&self) -> SyntaxElement {
        self.element.clone()
    }

    /// Retrieves the child nodes of this `ASTNode`.
    ///
    /// # Returns
    /// * `Vec<ASTNode>` - A vector containing clones of the child nodes.
    pub fn get_children(&self) -> Vec<ASTNode> {
        self.children.clone()
    }

    /// Sets the children of this node to a vector of `ASTNode`.
    ///
    /// # Parameters
    /// * `to_set` - A vector of `ASTNode` to set as children of this node.
    pub fn set_children(&mut self, to_set: Vec<ASTNode>) {
        self.children = to_set;
    }

    /// Adds a single child node to this `ASTNode`.
    ///
    /// # Parameters
    /// * `to_add` - The `ASTNode` to add as a child.
    pub fn add_child(&mut self, to_add: ASTNode) {
        self.children.push(to_add);
    }

    /// Adds multiple child nodes to this `ASTNode`.
    ///
    /// # Parameters
    /// * `to_add` - A vector of `ASTNode` to add as children.
    pub fn add_children(&mut self, to_add: Vec<ASTNode>) {
        self.children.extend(to_add);
    }

    /// Checks if the node represents a return statement.
    ///
    /// # Returns
    /// * `bool` - `true` if this node is a return statement, otherwise `false`.
    pub fn is_return(&self) -> bool {
        matches!(self.element, SyntaxElement::Return)
    }
}

/// Provides a display implementation for `ASTNode`.
///
/// # Parameters
/// * `f` - The formatter.
///
/// # Returns
/// * `fmt::Result` - The result of the formatting operation.
impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ASTNode: {}", self.element)
    }
}
