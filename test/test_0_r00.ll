; ModuleID = 'test_0_r00.spl'
source_filename = "test_0_r00.spl"

define i32 @main() {
entry:
  %a = alloca i32, align 4
  store i32 3, ptr %a, align 4
  br label %cond

cond:                                             ; preds = %merge3, %entry
  br i1 true, label %body, label %merge

body:                                             ; preds = %cond
  %a1 = load i32, ptr %a, align 4
  %addtmp = add i32 %a1, 1
  store i32 %addtmp, ptr %a, align 4
  %a2 = load i32, ptr %a, align 4
  %eqtmp = icmp eq i32 %a2, 5
  br i1 %eqtmp, label %then, label %merge3

merge:                                            ; preds = %then, %cond
  %a4 = load i32, ptr %a, align 4
  ret i32 %a4

then:                                             ; preds = %body
  br label %merge

merge3:                                           ; preds = %body
  br label %cond
}
