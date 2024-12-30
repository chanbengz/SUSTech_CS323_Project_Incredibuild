; ModuleID = 'test_3_r03.spl'
source_filename = "test_3_r03.spl"

@0 = internal global [4 x i8] c"%d\0A\00"

define i32 @main() {
entry:
  %N = alloca i32, align 4
  store i32 30, ptr %N, align 4
  %num = alloca i32, align 4
  store i32 0, ptr %num, align 4
  %i = alloca i32, align 4
  store i32 1, ptr %i, align 4
  %k = alloca i32, align 4
  store i32 1, ptr %k, align 4
  br label %cond

cond:                                             ; preds = %merge20, %entry
  %k1 = load i32, ptr %k, align 4
  %N2 = load i32, ptr %N, align 4
  %letmp = icmp sle i32 %k1, %N2
  br i1 %letmp, label %body, label %merge

body:                                             ; preds = %cond
  br label %cond3

merge:                                            ; preds = %cond
  ret i32 0

cond3:                                            ; preds = %merge13, %body
  %i6 = load i32, ptr %i, align 4
  %k7 = load i32, ptr %k, align 4
  %letmp8 = icmp sle i32 %i6, %k7
  br i1 %letmp8, label %body4, label %merge5

body4:                                            ; preds = %cond3
  %k9 = load i32, ptr %k, align 4
  %k10 = load i32, ptr %k, align 4
  %i11 = load i32, ptr %i, align 4
  %divtmp = udiv i32 %k10, %i11
  %i12 = load i32, ptr %i, align 4
  %multmp = mul i32 %divtmp, %i12
  %eqtmp = icmp eq i32 %k9, %multmp
  br i1 %eqtmp, label %then, label %merge13

merge5:                                           ; preds = %cond3
  %num17 = load i32, ptr %num, align 4
  %eqtmp18 = icmp eq i32 %num17, 2
  br i1 %eqtmp18, label %then19, label %merge20

then:                                             ; preds = %body4
  %num14 = load i32, ptr %num, align 4
  %addtmp = add i32 %num14, 1
  store i32 %addtmp, ptr %num, align 4
  br label %merge13

merge13:                                          ; preds = %then, %body4
  %i15 = load i32, ptr %i, align 4
  %addtmp16 = add i32 %i15, 1
  store i32 %addtmp16, ptr %i, align 4
  br label %cond3

then19:                                           ; preds = %merge5
  %k21 = load i32, ptr %k, align 4
  %0 = call i32 (ptr, ...) @printf(ptr @0, i32 %k21)
  br label %merge20

merge20:                                          ; preds = %then19, %merge5
  store i32 1, ptr %i, align 4
  store i32 0, ptr %num, align 4
  %k22 = load i32, ptr %k, align 4
  %addtmp23 = add i32 %k22, 1
  store i32 %addtmp23, ptr %k, align 4
  br label %cond
}

declare i32 @printf(ptr, ...)
