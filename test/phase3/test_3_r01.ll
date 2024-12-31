; ModuleID = 'test_3_r01.spl'
source_filename = "test_3_r01.spl"

@0 = internal global [3 x i8] c"%d\00"
@1 = internal global [5 x i8] c"Yes\0A\00"
@2 = internal global [4 x i8] c"No\0A\00"

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
  %modtmp = srem i32 %num4, 10
  store i32 %modtmp, ptr %digit, align 4
  %reversed5 = load i32, ptr %reversed, align 4
  %multmp = mul i32 %reversed5, 10
  %digit6 = load i32, ptr %digit, align 4
  %addtmp = add i32 %multmp, %digit6
  store i32 %addtmp, ptr %reversed, align 4
  %num7 = load i32, ptr %num1, align 4
  %divtmp = udiv i32 %num7, 10
  store i32 %divtmp, ptr %num1, align 4
  br label %cond

merge:                                            ; preds = %cond
  %reversed8 = load i32, ptr %reversed, align 4
  %original9 = load i32, ptr %original, align 4
  %eqtmp = icmp eq i32 %reversed8, %original9
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
  %isPalindrome = call i32 @isPalindrome(i32 %number1)
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
