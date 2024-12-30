; ModuleID = 'test_3_r05.spl'
source_filename = "test_3_r05.spl"

@0 = internal global [3 x i8] c"%d\00"
@1 = internal global [3 x i8] c"0\0A\00"
@2 = internal global [3 x i8] c"1\0A\00"
@3 = internal global [4 x i8] c"%d\0A\00"

define i32 @main() {
entry:
  %i = alloca i32, align 4
  %tu1 = alloca i32, align 4
  %tu2 = alloca i32, align 4
  %tu3 = alloca i32, align 4
  %m = alloca i32, align 4
  %a = alloca i32, align 4
  %b = alloca i32, align 4
  %c = alloca i32, align 4
  store i32 1, ptr %a, align 4
  store i32 2, ptr %b, align 4
  store i32 3, ptr %c, align 4
  store i32 1, ptr %tu1, align 4
  store i32 1, ptr %tu2, align 4
  %0 = call i32 (ptr, ...) @scanf(ptr @0, ptr %m)
  %m1 = load i32, ptr %m, align 4
  %lttmp = icmp slt i32 %m1, 1
  br i1 %lttmp, label %then, label %else

then:                                             ; preds = %entry
  %1 = call i32 (ptr, ...) @printf(ptr @1)
  br label %merge

else:                                             ; preds = %entry
  %m2 = load i32, ptr %m, align 4
  %eqtmp = icmp eq i32 %m2, 1
  %m3 = load i32, ptr %m, align 4
  %eqtmp4 = icmp eq i32 %m3, 2
  %ortmp = or i1 %eqtmp, %eqtmp4
  br i1 %ortmp, label %then5, label %else6

merge:                                            ; preds = %merge7, %then
  ret i32 0

then5:                                            ; preds = %else
  %2 = call i32 (ptr, ...) @printf(ptr @2)
  br label %merge7

else6:                                            ; preds = %else
  %m8 = load i32, ptr %m, align 4
  %gttmp = icmp sgt i32 %m8, 2
  br i1 %gttmp, label %then9, label %merge10

merge7:                                           ; preds = %merge10, %then5
  br label %merge

then9:                                            ; preds = %else6
  store i32 3, ptr %i, align 4
  br label %cond

merge10:                                          ; preds = %merge11, %else6
  br label %merge7

cond:                                             ; preds = %body, %then9
  %i12 = load i32, ptr %i, align 4
  %m13 = load i32, ptr %m, align 4
  %letmp = icmp sle i32 %i12, %m13
  br i1 %letmp, label %body, label %merge11

body:                                             ; preds = %cond
  %tu114 = load i32, ptr %tu1, align 4
  %tu215 = load i32, ptr %tu2, align 4
  %addtmp = add i32 %tu114, %tu215
  store i32 %addtmp, ptr %tu3, align 4
  %tu216 = load i32, ptr %tu2, align 4
  store i32 %tu216, ptr %tu1, align 4
  %tu317 = load i32, ptr %tu3, align 4
  store i32 %tu317, ptr %tu2, align 4
  %i18 = load i32, ptr %i, align 4
  %addtmp19 = add i32 %i18, 1
  store i32 %addtmp19, ptr %i, align 4
  br label %cond

merge11:                                          ; preds = %cond
  %tu320 = load i32, ptr %tu3, align 4
  %3 = call i32 (ptr, ...) @printf(ptr @3, i32 %tu320)
  br label %merge10
}

declare i32 @scanf(ptr, ...)

declare i32 @printf(ptr, ...)
