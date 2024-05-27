//! This file defines constants used throughout the system.

/// Default entry label for functions.
pub const DEFAULT_ENTRY_LABEL: &str = "entryID";

/// Default label for the body of a do-while loop.
pub const DEFAULT_DO_BODY_LABEL: &str = "do_bodyID";

/// Default label for the condition of a do-while loop.
pub const DEFAULT_DO_CONDITION_LABEL: &str = "do_condID";

/// Default label for the end of a do-while loop.
pub const DEFAULT_DO_WHILE_END_LABEL: &str = "do_endID";

/// Default label for the body of a while loop.
pub const DEFAULT_WHILE_BODY_LABEL: &str = "while_bodyID";

/// Default label for the condition of a while loop.
pub const DEFAULT_WHILE_COND_LABEL: &str = "while_condID";

/// Default label for the end of a while loop.
pub const DEFAULT_WHILE_END_LABEL: &str = "while_endID";

/// Default label for the body of a for loop.
pub const DEFAULT_FOR_BODY_LABEL: &str = "for_bodyID";

/// Default label for the condition of a for loop.
pub const DEFAULT_FOR_COND_LABEL: &str = "for_condID";

/// Default label for the end of a for loop.
pub const DEFAULT_FOR_END_LABEL: &str = "for_endID";

/// Default label for the increment section of a for loop.
pub const DEFAULT_FOR_INCREMENT_LABEL: &str = "for_incID";

/// Default label for the 'then' branch of an if condition.
pub const DEFAULT_THEN_LABEL: &str = "thenID";

/// Default label for the 'else' branch of an if condition.
pub const DEFAULT_ELSE_LABEL: &str = "elseID";

/// Default label for the merging point after branches of an if condition.
pub const DEFAULT_MERGE_LABEL: &str = "mergeID";

/// Default priority setting for a `ModElement`.
pub const DEFAULT_PRIORITY_MODELEMENT: i32 = -1;

/// Default mutability setting for literals.
pub const DEFAULT_MUTABLITY_LITERALS: bool = false;

/// Default mutability setting for variables.
pub const DEFAULT_MUTABILITY_VARIABLES: bool = false;
