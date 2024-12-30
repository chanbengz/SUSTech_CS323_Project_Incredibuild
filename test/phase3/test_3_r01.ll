; ModuleID = 'test_3_r01.spl'
source_filename = "test_3_r01.spl"

@0 = internal global [3 x i8] c"%d\00"
@1 = internal global [5 x i8] c"Yes\0A\00"
@2 = internal global [4 x i8] c"No\0A\00"

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

define i32 @isPalindrome(i32 %num) {
entry:
  %num1 = alloca i32, align 4
  store i32 %num, ptr %num1, align 4
  %reversed = alloca i32, align 4
  store i32 0, ptr %reversed, align 4
  %original = alloca i32, align 4
  %num2 = load i32, ptr %num1, align 4
  store i32 %num2, ptr %original, align 4
  %digit = alloca i32, align 4
  store i32 0, ptr %digit, align 4
  br label %cond

cond:                                             ; preds = %body, %entry
  %num3 = load i32, ptr %num1, align 4
  %netmp = icmp ne i32 %num3, 0
  br i1 %netmp, label %body, label %merge

body:                                             ; preds = %cond
  %num4 = load i32, ptr %num1, align 4
  %num5 = load i32, ptr %num1, align 4
  %mod = call i32 @mod(i32 %num5, i32 10)
  store i32 %mod, ptr %digit, align 4
  %reversed6 = load i32, ptr %reversed, align 4
  %multmp = mul i32 %reversed6, 10
  %digit7 = load i32, ptr %digit, align 4
  %addtmp = add i32 %multmp, %digit7
  store i32 %addtmp, ptr %reversed, align 4
  %num8 = load i32, ptr %num1, align 4
  %divtmp = udiv i32 %num8, 10
  store i32 %divtmp, ptr %num1, align 4
  br label %cond

merge:                                            ; preds = %cond
  %reversed9 = load i32, ptr %reversed, align 4
  %original10 = load i32, ptr %original, align 4
  %eqtmp = icmp eq i32 %reversed9, %original10
  br i1 %eqtmp, label %then, label %else

then:                                             ; preds = %merge
  ret i32 1

else:                                             ; preds = %merge
  ret i32 0
}

define i32 @main() {
entry:
  %number = alloca i32, align 4
  %0 = call i32 (ptr, ...) @scanf(ptr @0, ptr %number)
  %number1 = load i32, ptr %number, align 4
  %number2 = load i32, ptr %number, align 4
  %isPalindrome = call i32 @isPalindrome(i32 %number2)
  %eqtmp = icmp eq i32 %isPalindrome, 1
  br i1 %eqtmp, label %then, label %else

then:                                             ; preds = %entry
  %1 = call i32 (ptr, ...) @printf(ptr @1)
  br label %merge

else:                                             ; preds = %entry
  %2 = call i32 (ptr, ...) @printf(ptr @2)
  br label %merge

merge:                                            ; preds = %else, %then
  ret i32 0
}

declare i32 @scanf(ptr, ...)

declare i32 @printf(ptr, ...)
