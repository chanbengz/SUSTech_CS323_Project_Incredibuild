; ModuleID = 'test_3_r06.spl'
source_filename = "test_3_r06.spl"

@0 = internal global [4 x i8] c"%d\0A\00"
@1 = internal global [4 x i8] c"%d\0A\00"

define i32 @hanoi(i32 %n, i32 %p1, i32 %p2, i32 %p3) {
entry:
  %n1 = alloca i32, align 4
  store i32 %n, ptr %n1, align 4
  %p12 = alloca i32, align 4
  store i32 %p1, ptr %p12, align 4
  %p23 = alloca i32, align 4
  store i32 %p2, ptr %p23, align 4
  %p34 = alloca i32, align 4
  store i32 %p3, ptr %p34, align 4
  %n5 = load i32, ptr %n1, align 4
  %eqtmp = icmp eq i32 %n5, 1
  br i1 %eqtmp, label %then, label %else

then:                                             ; preds = %entry
  %p16 = load i32, ptr %p12, align 4
  %multmp = mul i32 %p16, 1000000
  %p37 = load i32, ptr %p34, align 4
  %addtmp = add i32 %multmp, %p37
  %0 = call i32 (ptr, ...) @printf(ptr @0, i32 %addtmp)
  br label %merge

else:                                             ; preds = %entry
  %n8 = load i32, ptr %n1, align 4
  %subtmp = sub i32 %n8, 1
  %p19 = load i32, ptr %p12, align 4
  %p310 = load i32, ptr %p34, align 4
  %p211 = load i32, ptr %p23, align 4
  %n12 = load i32, ptr %n1, align 4
  %subtmp13 = sub i32 %n12, 1
  %p114 = load i32, ptr %p12, align 4
  %p315 = load i32, ptr %p34, align 4
  %p216 = load i32, ptr %p23, align 4
  %hanoi = call i32 @hanoi(i32 %subtmp13, i32 %p114, i32 %p315, i32 %p216)
  %p117 = load i32, ptr %p12, align 4
  %multmp18 = mul i32 %p117, 1000000
  %p319 = load i32, ptr %p34, align 4
  %addtmp20 = add i32 %multmp18, %p319
  %1 = call i32 (ptr, ...) @printf(ptr @1, i32 %addtmp20)
  %n21 = load i32, ptr %n1, align 4
  %subtmp22 = sub i32 %n21, 1
  %p223 = load i32, ptr %p23, align 4
  %p124 = load i32, ptr %p12, align 4
  %p325 = load i32, ptr %p34, align 4
  %n26 = load i32, ptr %n1, align 4
  %subtmp27 = sub i32 %n26, 1
  %p228 = load i32, ptr %p23, align 4
  %p129 = load i32, ptr %p12, align 4
  %p330 = load i32, ptr %p34, align 4
  %hanoi31 = call i32 @hanoi(i32 %subtmp27, i32 %p228, i32 %p129, i32 %p330)
  br label %merge

merge:                                            ; preds = %else, %then
  ret i32 0
}

declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %sum = alloca i32, align 4
  store i32 3, ptr %sum, align 4
  %sum1 = load i32, ptr %sum, align 4
  %sum2 = load i32, ptr %sum, align 4
  %hanoi = call i32 @hanoi(i32 %sum2, i32 1, i32 2, i32 3)
  ret i32 0
}
