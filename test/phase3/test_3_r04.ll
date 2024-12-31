; ModuleID = 'test_3_r04.spl'
source_filename = "test_3_r04.spl"

@0 = internal global [4 x i8] c"%d\0A\00"

define i32 @mod(i32 %x, i32 %n) {
entry:
  %x1 = alloca i32, align 4
  store i32 %x, ptr %x1, align 4
  %n2 = alloca i32, align 4
  store i32 %n, ptr %n2, align 4
  %x3 = load i32, ptr %x1, align 4
  %x4 = load i32, ptr %x1, align 4
  %n5 = load i32, ptr %n2, align 4
  %divtmp = udiv i32 %x4, %n5
  %n6 = load i32, ptr %n2, align 4
  %multmp = mul i32 %divtmp, %n6
  %subtmp = sub i32 %x3, %multmp
  ret i32 %subtmp
}

define i32 @isPerfectNumber(i32 %number) {
entry:
  %number1 = alloca i32, align 4
  store i32 %number, ptr %number1, align 4
  %sum = alloca i32, align 4
  store i32 0, ptr %sum, align 4
  %j = alloca i32, align 4
  store i32 1, ptr %j, align 4
  br label %cond

cond:                                             ; preds = %merge6, %entry
  %j2 = load i32, ptr %j, align 4
  %number3 = load i32, ptr %number1, align 4
  %divtmp = udiv i32 %number3, 2
  %letmp = icmp sle i32 %j2, %divtmp
  br i1 %letmp, label %body, label %merge

body:                                             ; preds = %cond
  %number4 = load i32, ptr %number1, align 4
  %j5 = load i32, ptr %j, align 4
  %mod = call i32 @mod(i32 %number4, i32 %j5)
  %eqtmp = icmp eq i32 %mod, 0
  br i1 %eqtmp, label %then, label %merge6

merge:                                            ; preds = %cond
  %sum11 = load i32, ptr %sum, align 4
  %number12 = load i32, ptr %number1, align 4
  %eqtmp13 = icmp eq i32 %sum11, %number12
  br i1 %eqtmp13, label %then14, label %else

then:                                             ; preds = %body
  %sum7 = load i32, ptr %sum, align 4
  %j8 = load i32, ptr %j, align 4
  %addtmp = add i32 %sum7, %j8
  store i32 %addtmp, ptr %sum, align 4
  br label %merge6

merge6:                                           ; preds = %then, %body
  %j9 = load i32, ptr %j, align 4
  %addtmp10 = add i32 %j9, 1
  store i32 %addtmp10, ptr %j, align 4
  br label %cond

then14:                                           ; preds = %merge
  ret i32 1

else:                                             ; preds = %merge
  ret i32 0
}

define i32 @main() {
entry:
  %count = alloca i32, align 4
  store i32 0, ptr %count, align 4
  %i = alloca i32, align 4
  store i32 1, ptr %i, align 4
  br label %cond

cond:                                             ; preds = %merge3, %entry
  %i1 = load i32, ptr %i, align 4
  %letmp = icmp sle i32 %i1, 100
  br i1 %letmp, label %body, label %merge

body:                                             ; preds = %cond
  %i2 = load i32, ptr %i, align 4
  %isPerfectNumber = call i32 @isPerfectNumber(i32 %i2)
  %eqtmp = icmp eq i32 %isPerfectNumber, 1
  br i1 %eqtmp, label %then, label %merge3

merge:                                            ; preds = %cond
  ret i32 0

then:                                             ; preds = %body
  %i4 = load i32, ptr %i, align 4
  %0 = call i32 (ptr, ...) @printf(ptr @0, i32 %i4)
  br label %merge3

merge3:                                           ; preds = %then, %body
  %i5 = load i32, ptr %i, align 4
  %addtmp = add i32 %i5, 1
  store i32 %addtmp, ptr %i, align 4
  br label %cond
}

declare i32 @printf(ptr, ...)
