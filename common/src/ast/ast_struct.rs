/*
Represents an abstract syntax tree
 */

use core::fmt;

use std::cmp::{Eq, PartialEq};

use crate::ast::syntax_element::SyntaxElement;

/// An Abstract Syntax Tree
#[derive(Debug, PartialEq, Clone)]
pub struct AST {
    root: ASTNode,
}

/// A node of an abstract syntax tree
#[derive(Debug, Clone, PartialEq, Default, Eq, Hash)]
pub struct ASTNode {
    element: SyntaxElement,
    children: Vec<ASTNode>, 
}

impl AST {
    /// Create a new abstract syntax tree
    pub fn new(root: ASTNode) -> Self {
        AST { 
            root 
        }
    }

    /// Retrieves the root of the abstract syntax tree
    pub fn get_root(&self) -> ASTNode {
        self.root.clone()
    }

}

/// Formats an AST
fn ast_format(root: &ASTNode, output_string_ref: &mut String, depth: usize) {
    let node_repr_with_depth: String = std::iter::repeat("\t").take(depth).collect::<String>() + &root.get_element().to_string() + "\n";
    output_string_ref.push_str(&node_repr_with_depth);

    for child in root.get_children().iter() {
        ast_format(child, output_string_ref, depth + 1)
    }

}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output_str = String::from("AST: \n");

        ast_format(&self.get_root(), &mut output_str, 0);

        write!(f, "{}", output_str)
    }
}

impl ASTNode {
    /// Creates a new ast node
    pub fn new(element: SyntaxElement) -> Self {
        ASTNode {
            element,
            children: Vec::new(),
        }
    }

    /// Gets the syntax element of the node
    pub fn get_element(&self) -> SyntaxElement {
        self.element.clone()
    }

    /// Gets the children of the node
    pub fn get_children(&self) -> Vec<ASTNode> {
        self.children.clone()
    }

    /// Sets the children of this node to a specified vector
    pub fn set_children(&mut self, to_set: Vec<ASTNode>) {
        self.children = to_set;
    }

    /// Adds a child node
    pub fn add_child(&mut self, to_add: ASTNode) {
        self.children.push(to_add);
    }

    /// Adds children nodes
    pub fn add_children(&mut self, to_add: Vec<ASTNode>) {
        self.children.extend(to_add);
    }

    /// Checks if the node is a return statement
    pub fn is_return(&self) -> bool {
        match &self.element {
            SyntaxElement::Return => {
                return true;
            }
            _ => return false,
        }
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ASTNode: {}", self.element)
    }
}