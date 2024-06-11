//! This file hosts all of the necessary definitions for an Abstract Syntax Tree (AST)
//! and the nodes that make up the ASTs.

use core::fmt;
use std::cmp::{Eq, PartialEq};
use crate::ast::node_type::NodeType;

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
/// Each `ASTNode` contains a `NodeType` and zero or more children, which are also `ASTNode`s.
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
    node_type: NodeType,
    children: Vec<ASTNode>, 
}

impl AST {
    /// Creates a new `AST` instance.
    /// 
    /// # Parameters
    ///
    /// - `root`: An `ASTNode` which will be the root node of this tree.
    /// 
    /// # Returns
    ///
    /// Returns a new `AST` instance with the given ASTNode as its root.
    ///
    pub fn new(root: ASTNode) -> Self {
        AST { 
            root 
        }
    }

    /// Retrieves the root of the `AST` instance.
    /// 
    /// # Returns
    ///
    /// Returns a clone of the given `AST`'s root node.
    ///
    pub fn get_root(&self) -> ASTNode {
        self.root.clone()
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
    let node_repr_with_depth: String = std::iter::repeat("\t").take(depth).collect::<String>() + &root.get_node_type().to_string() + "\n";
    output_string_ref.push_str(&node_repr_with_depth);

    for child in root.get_children().iter() {
        ast_format(child, output_string_ref, depth + 1)
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

impl ASTNode {
    /// Creates a new `ASTNode` instance.
    /// 
    /// # Parameters
    ///
    /// - `node_type`: A `NodeType` which represents what type of ndoe this is and its data (if applicable).
    /// 
    /// # Returns
    ///
    /// Returns a new `ASTNode` instance with the given `NodeType` as its node type and an empty vector of children.
    ///
    pub fn new(node_type: NodeType) -> Self {
        ASTNode {
            node_type,
            children: Vec::new(),
        }
    }

    /// Retrieves the node type of the given `ASTNode` instance.
    /// 
    /// # Returns
    ///
    /// Returns a clone of the given `ASTNode`'s node type as a `NodeType`.
    ///
    pub fn get_node_type(&self) -> NodeType {
        self.node_type.clone()
    }

    /// Retrieves the children of the given `ASTNode` instance.
    /// 
    /// # Returns
    ///
    /// Returns a clone of the given `ASTNode`'s vector of children as a `Vec<ASTNode>`.
    ///
    pub fn get_children(&self) -> Vec<ASTNode> {
        self.children.clone()
    }

    /// Sets the children of this `ASTNode` to a specified vector.
    /// 
    /// # Parameters
    ///
    /// - `to_set`: A `Vec<ASTNode>` which is the vector that will become this node's vector of children.
    /// 
    pub fn set_children(&mut self, to_set: Vec<ASTNode>) {
        self.children = to_set;
    }

    /// Adds a child node to this `ASTNode` instance.
    /// 
    /// # Parameters
    ///
    /// - `to_add`: An `ASTNode` which is the node that will be added to this node's vector of children.
    /// 
    pub fn add_child(&mut self, to_add: ASTNode) {
        self.children.push(to_add);
    }

    /// Adds several children nodes to this `ASTNode` at a time.
    /// 
    /// # Parameters
    ///
    /// - `to_add`: A `Vec<ASTNode>` which is the vector that will be appended to this node's vector of children.
    ///
    pub fn add_children(&mut self, to_add: Vec<ASTNode>) {
        self.children.extend(to_add);
    }

    /// Checks if this `ASTNode` is a return statement.
    /// 
    /// # Returns
    ///
    /// Returns boolean `true` if this node represents a return statement, `false` otherwise.
    ///
    pub fn is_return(&self) -> bool {
        match &self.node_type {
            NodeType::Return => {
                return true;
            }
            _ => return false,
        }
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
        write!(f, "ASTNode: {}", self.node_type)
    }
}