//! The driver for the IR generation process, uses recursive descent much like the parser
//! which takes the abstract syntax tree and symbol table stack generated by parsing
//! and semantic analysis, and generates LLVM IR from it, completing the final step for
//! turning code into a (one step above a) directly runnable format.

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
    /// Creates a new `IRGenerator` instance.
    ///
    /// This initializer sets up an `IRGenerator` instance, setting up its own context, module, and builder
    /// which are necessary components of writing IR. The most important to understand is the builder,
    /// which performs the function of writing IR into the module you are creating. 
    /// 
    /// The current function is not initialized yet, and is set to None. This will change when you
    /// generate a function. Once you do so, you must set this so that you can know where to write IR into
    /// for each function. Remember, main() is a function!
    /// 
    /// sts is not set yet, and is None. Additionally, the sts stack pointer is set to 0. Set the STS 
    /// when you have one and make sure to increment and decrement the stack pointer as you generate IR
    /// to stay in step with the STS.
    /// 
    /// current_target_stack is initialized as an empty stack. In C, in a while loop for example, break;
    /// will end the loop entirely upon reaching that statement, and continue; will jump back to checking
    /// the loop conditions. The targets for break and continue are stored here, so you know where to point to
    /// when you hit one of these.
    /// 
    /// current_label_id starts at 0 and is the suffix for labels in IR. Jumping to labels is how we do complex
    /// logic like loops, and all labels have to be unique (otherwise how would we know which label was which?)
    /// so this along with its supporting functions allow unique IDs to be created for every label.
    /// 
    /// store is initialized to a new store. The store is a space for managing variables, allowing you
    /// to assign and reassign variables in IR while keeping track of their tags.
    /// 
    /// current_insert_block is set to None, this is a space to hold the "basic block" (labeled area of code)
    /// that new blocks should be inserted after. ResourcePools contains a function that can create basic blocks
    /// after a given basic block, and this helps support that. This will be necessary for inserting code blocks
    /// in the body of a while loop for example.
    ///
    /// # Returns
    ///
    /// Returns a new `IRGenerator` instance ready to generate a module from an AST and STS generated in
    /// previous steps.
    ///
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

    /// Gets the current resource pools of this IRGenerator.
    ///
    /// # Returns
    ///
    /// Returns a mutex protected reference to the resource pools.
    ///
    pub fn get_resource_pools(&mut self) -> Arc<Mutex<ResourcePools>> {
        self.resource_pools.clone()
    }

    /// Gets the current IR generation's context, so that a single 
    /// module of IR generation occurs within the same scope.
    /// 
    /// # Returns
    ///
    /// Returns the current context's tag for use in other IR generation functions
    /// that make use of it.
    ///
    pub fn get_context(&self) -> ContextTag {
        self.context.clone().expect("Missing context")
    }

    /// Returns the current module being generated.
    /// 
    /// # Returns
    ///
    /// Returns the current module's tag for use in getting the completed module
    /// after IR has been generated.
    ///
    pub fn get_module(&self) -> ModuleTag {
        self.module.clone().expect("Missing module")
    }

    /// Returns the builder used for constructing IR statements.
    /// 
    /// # Returns
    ///
    /// Returns this IRGenerator's builder's tag for use with functions that require the builder.
    ///
    pub fn get_builder(&self) -> BuilderTag {
        self.builder.clone().expect("Missing builder")
    }

    /// Retrieves the current function being built.
    /// 
    /// # Returns
    ///
    /// Returns the tag of the function currently being built for use
    /// with functions that need this information, could be None if called
    /// before initialized.
    ///
    pub fn get_function(&self) -> Option<ValueTag> {
        self.function.clone()
    }

    /// Sets the current function being built.
    ///
    /// # Parameters
    ///
    /// - `function`: The ValueTag of the new function to be set as the current function
    ///
    pub fn set_function(&mut self, function: ValueTag) {
        self.function = Some(function)
    }

    /// Retrieves the current block to insert after
    /// 
    /// # Returns
    ///
    /// Returns the tag of the basic block which is set as the block to insert
    /// after currently. Could be None if this is called when no basic block
    /// is currently set.
    ///
    pub fn get_current_insert_block(&self) -> Option<BasicBlockTag> {
        self.current_insert_block.clone()
    }

    /// Sets the current block to insert after
    /// 
    /// # Parameters
    ///
    /// - `to_set`: The BasicBlockTag of the new baic block to be set as the insert block
    ///
    pub fn set_current_insert_block(&mut self, to_set: BasicBlockTag) {
        self.current_insert_block = Some(to_set);
    }

    /// Retrieves the current break and potentially the current continue target
    /// 
    /// # Returns
    ///
    /// Returns a vector of basic block tags where index 0 is the break target and index 1
    /// is the continue target (if one exists, otherwise the vector will only have index 0).
    /// This may return None if an error occurred getting the targets from the target stack.
    ///
    pub fn get_break_continue_target(&self) -> Option<Vec<BasicBlockTag>> {
        let target = if let Some(current_target_stack) = self.current_target_stack.as_ref() {
            current_target_stack.last().cloned().expect("Failed to get break cont target!")
        } else {
            return None
        };

        match target {
            BranchTarget::BreakAndContinueTarget(t1, t2) => {
                let mut out_vec: Vec<BasicBlockTag> = Vec::new();

                if t1.is_none() {
                    return None
                }

                out_vec.push(t1.unwrap());

                if !t2.is_none() {
                    out_vec.push(t2.unwrap());
                }

                Some(out_vec)
            },
        }
    }

    /// Pushes a tag for break; and a tag for continue; on the target stack.
    /// 
    /// # Parameters
    ///
    /// - `break_block_tag`: The BasicBlockTag of the tag to branch to when break; is encountered.
    /// - `continue_block_tag`: The BasicBlockTag of the tag to branch to when continue; is encountered.
    ///
    pub fn push_break_continue_target(&mut self, break_block_tag: BasicBlockTag, continue_block_tag: BasicBlockTag) {
        if let Some(current_target_stack) = self.current_target_stack.as_mut() {
            current_target_stack.push(BranchTarget::BreakAndContinueTarget(Some(break_block_tag), Some(continue_block_tag)));
        } else {
            panic!("Failed to get current target stack");
        }
    }

    /// Pushes only a tag for break; on the target stack.
    /// 
    /// # Parameters
    ///
    /// - `break_block_tag`: The BasicBlockTag of the tag to branch to when break; is encountered.
    ///
    pub fn push_break_target(&mut self, break_block_tag: BasicBlockTag) {
        if let Some(current_target_stack) = self.current_target_stack.as_mut() {
            current_target_stack.push(BranchTarget::BreakAndContinueTarget(Some(break_block_tag), None));
        } else {
            panic!("Failed to get current target stack");
        }
    }


    /// Pops current break continue target off the stack
    /// 
    pub fn pop_target(&mut self) {
        if let Some(current_target_stack) = self.current_target_stack.as_mut() {
            current_target_stack.pop();
        } else {
            panic!("Failed to get current target stack");
        }
    }

    /// Gets and increments the next ID number as a string for differentiating labels
    /// 
    /// # Returns
    ///
    /// Returns a string representing a unique ID number, advancing the current number
    /// that produced it by 1 so the next time this function is called it also returns a 
    /// unique ID number.
    ///
    /// # Examples
    ///
    /// ```
    /// /* IR is generated in different sections using labels, and labels must be unique so the code goes to
    /// the right places. This allows you to use a system like */
    /// //let label: String = "some_label_nameID".to_string();
    /// //let next_id: String = self.get_next_label_id();
    /// //let new_label_name = label + &next_id;
    /// /* to create a new unique label name stored in new_label_name. For example if the next ID 
    /// was 42 then this label would be some_label_nameID42, and if a new label needed to be generated
    /// in this part of IR then it would do the same thing and be named some_label_nameID43, ensuring
    /// all label names are unique. */
    /// ```
    pub fn get_next_label_id(&mut self) -> String {
        let next_id_str: String = self.current_label_id.to_string();
        self.current_label_id += 1;
        next_id_str
    }

    /// Makes a new current table in the store
    /// 
    /// # Examples
    ///
    /// ```
    /// /* When we enter a new scope, we need to create a new table in the store
    /// to correctly generate IR such that variables only exist in their
    /// current scope and inner scopes. When we do get into a new scope, use*/
    /// //self.make_new_store_table();
    /// /* to create a new table for this scope. */
    /// ```
    pub fn make_new_store_table(&mut self) {
        self.store.push_table();
    }

    /// Deletes the current table in the store
    /// 
    /// # Examples
    ///
    /// ```
    /// /* When we leave a scope, we need to delete the current tab;e in the store
    /// to correctly generate IR such that variables only exist in their
    /// current scope and inner scopes, and when we return to an outer scope the 
    /// variables in the current scope no longer exist. When we do get out of a
    /// current scope, use*/
    /// //self.delete_store_table();
    /// /* to delete the current table for this scope. */
    /// ```
    pub fn delete_store_table(&mut self) {
        self.store.delete_table().expect("Failed to delete table");
    }

    /// Searches the store table for a variable with a name
    /// 
    /// # Parameters
    ///
    /// - `name`: A `String` name of a variable to get an allocation tag for.
    ///
    /// # Returns
    ///
    /// Returns the allocation tag of the variable associated with the given string variable name.
    ///
    /// # Examples
    ///
    /// ```
    /// /* To retrieve a variable's value first you must get its allocation tag, use */
    /// //let var_name: String = /* some variable name that exists in the current scope or an outer scope */
    /// //let alloca_tag = self.search_store_table(var_name);
    /// /* to do this. */
    /// ```
    pub fn search_store_table(&self, name: String) -> ValueTag{
        let value = self.store.search_for_var(name);

        match value {
            Ok(tag) => tag,
            Err(e) => panic!("{:?}", e)
        }
    }

    /// adds an allocation tag to the current store table
    /// 
    /// # Parameters
    ///
    /// - `name`: A `String` name of a variable to set an allocation tag for.
    /// - `tag`: A `ValueTag` allocation tag of the variable to store in the table
    ///
    /// # Examples
    ///
    /// ```
    /// /* When we create a new variable, it creates an allocation tag pointing that variable
    /// that we must keep track of if we want to use that variable or reassign it in the future.
    /// To do this, we associate variable names with tags in the store so we can retrieve
    /// these allocation tags later. Use */
    /// //let var_name: String = /* some variable name that we want to create*/
    /// //let alloca_tag = /* allocation tag of the variable we've created with IR generation */
    /// //self.add_tag_to_store_table(var_name, alloca_tag);
    /// /* to do this. */
    /// ```
    pub fn add_tag_to_store_table(&mut self, name: String, tag: ValueTag){
        self.store.add_tag_to_top_table(name, tag).expect("Failed to add tag to table");
    }

    /// Retrieves the current basic block the builder is pointing into.
    /// # Returns
    ///
    /// Returns the BasicBlockTag of the current basic block that the builder is working in.
    /// May be None if called before builder is positioned.
    ///
    /// # Examples
    ///
    /// ```
    /// /* Inside the IR generation loop it may be necessary to get the tag of the current block for use in
    /// other functions. Use*/
    /// //let current_bb_tag = self.get_current_block();
    /// /* to do this. */
    /// ```
    pub fn get_current_block(&mut self) -> Option<BasicBlockTag> {
        let resource_pools = self.get_resource_pools();
        let mut resource_pools_guard = resource_pools.lock().expect("Failed to lock mutex in getting current block!");
        resource_pools_guard.get_current_block(self.get_builder())
    }

    /// Increments the stack pointer.
    ///
    /// # Examples
    ///
    /// ```
    /// /* It will be necessary to increment the stack pointer when we need to move
    ///  to a new scope. Use*/
    /// //self.increment_stack_pointer();
    /// /* to do this. */
    /// ```
    pub fn increment_stack_pointer(&mut self) {
        self.sts_pointer += 1;
    }

    /// Decrements the stack pointer.
    /// 
    /// # Examples
    ///
    /// ```
    /// /* It will be necessary to decrement the stack pointer when we need to move back
    ///  to a previous scope. Use*/
    /// //self.decrement_stack_pointer();
    /// /* to do this. */
    /// ```
    pub fn decrement_stack_pointer(&mut self) {
        self.sts_pointer -= 1;
    }

    /// Resets the stack pointer to zero.
    /// 
    /// # Examples
    ///
    /// ```
    /// /* Sometimes it could be necessary to reset the stack pointer to 0 if we need to move
    ///  to a new STS. Use*/
    /// //self.reset_stack_pointer();
    /// /* to do this. */
    /// ```
    pub fn reset_stack_pointer(&mut self) {
        self.sts_pointer = 0;
    }

    /// Retrieves a clone of the current symbol table stack.
    /// 
    /// # Returns
    ///
    /// Returns a clone of the current symbol table stack (may be None if called before
    /// the STS is initialized).
    ///
    /// # Examples
    ///
    /// ```
    /// /* Inside the IR generation loop it may sometimes be necessary to retrieve
    /// the current STS. Use*/
    /// //let sts = self.get_stack();
    /// /* to do this. */
    /// ```
    pub fn get_stack(&self) -> Option<SymbolTableStack> {
        self.sts.clone()
    }

    /// Sets the current symbol table stack to a new STS.
    /// 
    /// # Parameters
    ///
    /// - `new_sts`: `The new SymbolTableStack` to be set as the STS for this IRGenerator.
    ///
    /// # Examples
    ///
    /// ```
    /// /* with new_stack as some STS that is desired to be the current STS */
    /// //self.set_stack(new_stack);
    /// ```
    pub fn set_stack(&mut self, new_sts: SymbolTableStack) {
        self.sts = Some(new_sts);
    }

    /// Retrieves the current position of the stack pointer within the symbol table stack.
    ///
    /// # Returns
    ///
    /// Returns a `usize` representing the current stack pointer.
    ///
    /// # Examples
    ///
    /// ```
    /// /* when we need to access the STS, the way to find which stack to access is by 
    /// using */
    /// //let stack_pointer = self.get_stack_pointer();
    /// /* to find the current value of the stack pointer, and access the STS with stack_pointer. */
    /// ```
    pub fn get_stack_pointer(&self) -> usize {
        self.sts_pointer
    }

    /// Generates LLVM IR from a given module by processing its AST.
    /// This method iterates over the module's elements, generating IR for each and managing the state transitions required.
    ///
    /// # Parameters
    ///
    /// - `input`: A `Module` Containing an AST and an STS.
    ///
    /// # Returns
    ///
    /// Returns a `ModuleTag` containing the constructed module when IR generation is complete.
    ///
    pub fn generate_ir(&mut self, mut input: Module) -> ModuleTag {
        let module: &mut Vec<ModElement> = input.get_mut_children();

        unimplemented!();

        self.get_module()
    }

    /// Routes the generation of LLVM IR based on the type of AST node encountered.
    /// 
    /// # Parameters
    ///
    /// - `node`: A reference to an `ASTNode` to generate IR for.
    /// 
    /// # Returns
    ///
    /// Returns a `Result<Option<Tag>, ErrorType>` containing the tag of the data created by IR generation
    /// (may be None) or an error if there was a problem during IR generation.
    ///
    /// # Errors
    ///
    /// - Returns an error if there was a problem during IR generation such as a malformed node.
    ///
    pub fn ir_router(&mut self, node: &ASTNode) -> Result<Option<Tag>, ErrorType> {
        unimplemented!();
    }
}