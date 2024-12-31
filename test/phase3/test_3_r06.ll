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
  %hanoi = call i32 @hanoi(i32 %subtmp, i32 %p19, i32 %p310, i32 %p211)
  %p112 = load i32, ptr %p12, align 4
  %multmp13 = mul i32 %p112, 1000000
  %p314 = load i32, ptr %p34, align 4
  %addtmp15 = add i32 %multmp13, %p314
  %1 = call i32 (ptr, ...) @printf(ptr @1, i32 %addtmp15)
  %n16 = load i32, ptr %n1, align 4
  %subtmp17 = sub i32 %n16, 1
  %p218 = load i32, ptr %p23, align 4
  %p119 = load i32, ptr %p12, align 4
  %p320 = load i32, ptr %p34, align 4
  %hanoi21 = call i32 @hanoi(i32 %subtmp17, i32 %p218, i32 %p119, i32 %p320)
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
  %hanoi = call i32 @hanoi(i32 %sum1, i32 1, i32 2, i32 3)
  ret i32 0
}
