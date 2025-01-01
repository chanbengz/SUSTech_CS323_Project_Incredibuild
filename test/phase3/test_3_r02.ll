; ModuleID = 'test_3_r02.spl'
source_filename = "test_3_r02.spl"

@0 = internal global [5 x i8] c"%d%d\00"
@1 = internal global [4 x i8] c"%d\0A\00"

define i32 @calculateBinomialCoefficient(i32 %row, i32 %col) {
entry:
  %row1 = alloca i32, align 4
  store i32 %row, ptr %row1, align 4
  %col2 = alloca i32, align 4
  store i32 %col, ptr %col2, align 4
  %coefficient = alloca i32, align 4
  store i32 1, ptr %coefficient, align 4
  %i = alloca i32, align 4
  store i32 0, ptr %i, align 4
  %row3 = load i32, ptr %row1, align 4
  %col4 = load i32, ptr %col2, align 4
  %lttmp = icmp slt i32 %row3, %col4
  br i1 %lttmp, label %then, label %merge

then:                                             ; preds = %entry
  ret i32 0

merge:                                            ; preds = %entry
  %col5 = load i32, ptr %col2, align 4
  %row6 = load i32, ptr %row1, align 4
  %col7 = load i32, ptr %col2, align 4
  %subtmp = sub i32 %row6, %col7
  %gttmp = icmp sgt i32 %col5, %subtmp
  br i1 %gttmp, label %then8, label %merge9

then8:                                            ; preds = %merge
  %row10 = load i32, ptr %row1, align 4
  %col11 = load i32, ptr %col2, align 4
  %subtmp12 = sub i32 %row10, %col11
  store i32 %subtmp12, ptr %col2, align 4
  br label %merge9

merge9:                                           ; preds = %then8, %merge
  br label %cond

cond:                                             ; preds = %body, %merge9
  %i14 = load i32, ptr %i, align 4
  %col15 = load i32, ptr %col2, align 4
  %lttmp16 = icmp slt i32 %i14, %col15
  br i1 %lttmp16, label %body, label %merge13

body:                                             ; preds = %cond
  %coefficient17 = load i32, ptr %coefficient, align 4
  %row18 = load i32, ptr %row1, align 4
  %i19 = load i32, ptr %i, align 4
  %subtmp20 = sub i32 %row18, %i19
  %multmp = mul i32 %coefficient17, %subtmp20
  store i32 %multmp, ptr %coefficient, align 4
  %coefficient21 = load i32, ptr %coefficient, align 4
  %i22 = load i32, ptr %i, align 4
  %addtmp = add i32 %i22, 1
  %divtmp = udiv i32 %coefficient21, %addtmp
  store i32 %divtmp, ptr %coefficient, align 4
  %i23 = load i32, ptr %i, align 4
  %addtmp24 = add i32 %i23, 1
  store i32 %addtmp24, ptr %i, align 4
  br label %cond

merge13:                                          ; preds = %cond
  %coefficient25 = load i32, ptr %coefficient, align 4
  ret i32 %coefficient25
}

define i32 @main() {
entry:
  %r = alloca i32, align 4
  %c = alloca i32, align 4
  %coe = alloca i32, align 4
  %0 = call i32 (ptr, ...) @scanf(ptr @0, ptr %r, ptr %c)
  %r1 = load i32, ptr %r, align 4
  %c2 = load i32, ptr %c, align 4
  %calculateBinomialCoefficient = call i32 @calculateBinomialCoefficient(i32 %r1, i32 %c2)
  store i32 %calculateBinomialCoefficient, ptr %coe, align 4
  %coe3 = load i32, ptr %coe, align 4
  %1 = call i32 (ptr, ...) @printf(ptr @1, i32 %coe3)
  ret i32 0
}

declare i32 @scanf(ptr, ...)

declare i32 @printf(ptr, ...)
