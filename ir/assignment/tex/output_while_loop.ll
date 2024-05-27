; ModuleID = 'dummy_module'
source_filename = "dummy_module"

define i64 @testFunctionWithWhileLoop() {
entry:
  br label %while_cond

while_cond:                                     
  br i1 true, label %while_body, label %while_end

while_body:                                       
  ret i64 42
  br label %while_cond

while_end:                                       
}
