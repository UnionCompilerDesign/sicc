use std::sync::{Arc, Mutex};

use integration::module::{ModElement, Module};

use safe_llvm::memory_management::resource_pools::{BasicBlockTag, BuilderTag, ContextTag, ModuleTag, ResourcePools, Tag, ValueTag};

use common::{
    ast::{
        ast_struct::{ASTNode, AST},
        syntax_element::SyntaxElement, 
    },
    error::ErrorType
};

use symbol_table::symbol_table_struct::SymbolTableStack;


use crate::store::Store;

/// A struct for generating LLVM Intermediate Representation (IR) from a module of abstract syntax trees (AST) and symbol table stacks (STS).
pub struct IRGenerator {
    /// Resource pools for working with SafeLLVM's IR generation.
    resource_pools: Arc<Mutex<ResourcePools>>,
    /// Current context tag for IR generation, with the context itself stored in `resource_pools`. 
    context: Option<ContextTag>,
    /// Current module tag for IR generation, with the module itself stored in `resource_pools`. 
    module: Option<ModuleTag>,
    /// Current builder tag for IR generation, with the builder itself stored in `resource_pools`. 
    builder: Option<BuilderTag>,
    /// Current function tag for IR generation, with the function stored in `resource_pools`. 
    function: Option<ValueTag>,
    /// Current symbol table stack (sts) from the module.
    sts: Option<SymbolTableStack>,
    /// Index pointing to the current symbol table within the sts.
    sts_pointer: usize,
    /// Stack of targets to keep track of the current target blocks for branch statements.
    current_target_stack: Option<Vec<BranchTarget>>, 
    /// Integer to make sure labels are unique. 
    current_label_id: usize,
    /// Allocation store for managing variable allocations across scopes. 
    store: Store,
    /// Stores a pointer to the current block to insert after.
    current_insert_block: Option<BasicBlockTag>
}

#[derive(Clone)]
/// Stores branch targets to be used in IR generation
pub enum BranchTarget {
    /// Holds a tag to break to, and a tag to continue to
    BreakAndContinueTarget(Option<BasicBlockTag>, Option<BasicBlockTag>)
}


impl IRGenerator {
    /// Constructs a new IRGenerator, initializing the resource pools, context, module, and builder.
    pub fn new() -> Self {
        let resource_pools: Arc<Mutex<ResourcePools>> = Arc::new(Mutex::new(ResourcePools::new()));
        let mut resource_pools_guard = resource_pools.lock().expect("Failed to lock mutex while creating new IR generator!");

        let context = resource_pools_guard.create_context().expect("Failed to create context");
        let module = resource_pools_guard.create_module("dummy_module", context.clone()).expect("Failed to create module");
        let builder = resource_pools_guard.create_builder(context.clone()).expect("Failed to create builder");

        drop(resource_pools_guard);

        Self {
            resource_pools,
            context: Some(context),
            module: Some(module),
            builder: Some(builder),
            function: None,
            sts: None,
            sts_pointer: 0,
            current_target_stack: Some(Vec::new()),
            current_label_id: 0,
            store: Store::new(),
            current_insert_block: None
        }
    }

    /// Returns a protected reference to the resource pools.
    pub fn get_resource_pools(&mut self) -> Arc<Mutex<ResourcePools>> {
        self.resource_pools.clone()
    }

    /// Returns the current IR generation's context, so that a single module of IR generation occurs within the same scope.
    pub fn get_context(&self) -> ContextTag {
        self.context.clone().expect("Missing context")
    }

    /// Returns the current module being generated.
    pub fn get_module(&self) -> ModuleTag {
        self.module.clone().expect("Missing module")
    }

    /// Returns the builder used for constructing IR nodes.
    pub fn get_builder(&self) -> BuilderTag {
        self.builder.clone().expect("Missing builder")
    }

    /// Retrieves the current function being built.
    pub fn get_function(&self) -> Option<ValueTag> {
        self.function.clone()
    }

    /// Sets the current function being built.
    pub fn set_function(&mut self, function: ValueTag) {
        self.function = Some(function)
    }

    /// Retrieves the current block to insert after
    pub fn get_current_insert_block(&self) -> Option<BasicBlockTag> {
        self.current_insert_block.clone()
    }

    /// Sets the current block to insert after
    pub fn set_current_insert_block(&mut self, to_set: BasicBlockTag) {
        self.current_insert_block = Some(to_set);
    }

    /// Retrieves the current break continue target
    pub fn get_break_continue_target(&self) -> Option<Vec<BasicBlockTag>> {
        let target = if let Some(current_target_stack) = self.current_target_stack.as_ref() {
            current_target_stack.last().cloned().expect("Failed to get break cont target!")
        } else {
            panic!("Failed to get current target stack")
        };

        match target {
            BranchTarget::BreakAndContinueTarget(t1, t2) => {
                let mut out_vec: Vec<BasicBlockTag> = Vec::new();

                if t1.is_none() {
                    panic!("No break target found!");
                }

                out_vec.push(t1.unwrap());

                if !t2.is_none() {
                    out_vec.push(t2.unwrap());
                }

                Some(out_vec)
            },
        }
    }

    /// Pushes a new break and continue target onto the stack
    pub fn push_break_continue_target(&mut self, break_block_tag: BasicBlockTag, continue_block_tag: BasicBlockTag) {
        if let Some(current_target_stack) = self.current_target_stack.as_mut() {
            current_target_stack.push(BranchTarget::BreakAndContinueTarget(Some(break_block_tag), Some(continue_block_tag)));
        } else {
            panic!("Failed to get current target stack");
        }
    }


    /// Pops current target off the stack
    pub fn pop_target(&mut self) {
        if let Some(current_target_stack) = self.current_target_stack.as_mut() {
            current_target_stack.pop();
        } else {
            panic!("Failed to get current target stack");
        }
    }

    /// Gets and increments the next ID number as a string for differentiating labels
    pub fn get_next_label_id(&mut self) -> String {
        let next_id_str: String = self.current_label_id.to_string();
        self.current_label_id += 1;
        next_id_str
    }

    /// Makes a new current table in the store
    pub fn make_new_store_table(&mut self) {
        self.store.push_table();
    }

    /// Deletes the current table in the store
    pub fn delete_store_table(&mut self) {
        self.store.delete_table().expect("Failed to delete table");
    }

    /// Searches the store table for a variable with a name
    pub fn search_store_table(&self, name: String) -> ValueTag{
        let value = self.store.search_for_var(name);

        match value {
            Ok(tag) => tag,
            Err(e) => panic!("{:?}", e)
        }
    }

    /// adds an allocation tag to the current store table
    pub fn add_tag_to_store_table(&mut self, name: String, tag: ValueTag){
        self.store.add_tag_to_top_table(name, tag).expect("Failed to add tag to table");
    }

    /// Retrieves the current basic block the builder is pointing into.
    pub fn get_current_block(&mut self) -> Option<BasicBlockTag> {
        let resource_pools = self.get_resource_pools();
        let mut resource_pools_guard = resource_pools.lock().expect("Failed to lock mutex in getting current block!");
        resource_pools_guard.get_current_block(self.get_builder())
    }
    
    /// Retrieves the next basic block relative to the current one being pointed into by the builder.
    pub fn get_next_block(&mut self) -> Option<BasicBlockTag> {
        let resource_pools = self.get_resource_pools();
        let mut resource_pools_guard = resource_pools.lock().expect("Failed to lock mutex in getting next block!");
        resource_pools_guard.get_next_block(self.get_builder())
    }

    /// Retrieves the previous basic block relative to the current block being pointed into by the builder.
    pub fn get_previous_block(&mut self) -> Option<BasicBlockTag> {
        let resource_pools = self.get_resource_pools();
        let mut resource_pools_guard = resource_pools.lock().expect("Failed to lock mutex in getting previous block!");
        resource_pools_guard.get_previous_block(self.get_builder())
    }

    /// Increments the stack pointer.
    pub fn increment_stack_pointer(&mut self) {
        self.sts_pointer += 1;
    }

    /// Decrements the stack pointer.
    pub fn decrement_stack_pointer(&mut self) {
        self.sts_pointer -= 1;
    }

    /// Resets the stack pointer to zero.
    pub fn reset_stack_pointer(&mut self) {
        self.sts_pointer = 0;
    }

    /// Retrieves a clone of the current symbol table stack.
    pub fn get_stack(&self) -> Option<SymbolTableStack> {
        self.sts.clone()
    }

    /// Sets the current symbol table stack to a new sts.
    pub fn set_stack(&mut self, new_sts: SymbolTableStack) {
        self.sts = Some(new_sts);
    }

    /// Retrieves the current position of the stack pointer within the symbol table stack..
    pub fn get_stack_pointer(&self) -> usize {
        self.sts_pointer
    }

    /// Generates LLVM IR from a given module by processing its AST.
    /// This method iterates over the module's elements, generating IR for each and managing the state transitions required.
    pub fn generate_ir(&mut self, mut input: Module) -> ModuleTag {
        let module: &mut Vec<ModElement> = input.get_mut_children();

        while let Some(mod_element) = module.pop() {
            let symbol_table_stack: SymbolTableStack = mod_element.get_sym_table_stack();
            self.set_stack(symbol_table_stack);
            self.reset_stack_pointer();

            let ast: AST = mod_element.get_ast();
            let root = ast.get_root();
            self.ir_router(&root).expect("Failed on root");
        }

        self.get_module()
    }

    /// Routes the generation of LLVM IR based on the type of AST node encountered.
    pub fn ir_router(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        match &node.get_element() {
            // --- MODULE & SCOPING SECTION --- //
            SyntaxElement::ModuleExpression |
            SyntaxElement::NoExpression |
            SyntaxElement::TopLevelExpression |

            // --- DECLARATION SECTION --- //
            SyntaxElement::FunctionDeclaration |

            // --- BASE EXPRESSION SECTION --- //
            SyntaxElement::BlockExpression |
            SyntaxElement::DoWhileLoop => |
            SyntaxElement::WhileLoop => |
            SyntaxElement::ForLoop => |
            SyntaxElement::IfStatement |

            // --- CONTROL FLOW SECTION --- //
            // SyntaxElement::Assignment => self.generate_assignment_ir(node),
            SyntaxElement::Assignment => |
            SyntaxElement::Return => |
            SyntaxElement::Condition => |

            // --- LOOP CONTROL SECTION --- //
            SyntaxElement::LoopIncrement | 
            SyntaxElement::Action |
            SyntaxElement::Variant |
            SyntaxElement::LoopInitializer |
            SyntaxElement::AssignedValue => |
            SyntaxElement::Break => |
            SyntaxElement::Continue => |
            SyntaxElement::ElseStatement => |

            // --- PRIMITIVE SECTION --- //
            SyntaxElement::Literal(_) => |
            SyntaxElement::Type(t) => |
            SyntaxElement::Initialization => |

            // --- TODO SECTION --- //
            SyntaxElement::Variable |
            SyntaxElement::UnaryExpression |
            SyntaxElement::Field |
            SyntaxElement::Parameter |
            SyntaxElement::BinaryExpression |
            SyntaxElement::FunctionCall |
            SyntaxElement::EnumDeclaration |
            SyntaxElement::StructDeclaration |
            SyntaxElement::Identifier(_) |
            SyntaxElement::Operator(_) |
            SyntaxElement::Operand |
            SyntaxElement::Constant(_) => todo!(),
            SyntaxElement::SwitchStatement => todo!(),
            SyntaxElement::Case => todo!(),
            SyntaxElement::Default => todo!(),
        }
    }
}