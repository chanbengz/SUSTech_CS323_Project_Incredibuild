source_filename = "test_0_r00.spl"

define i32 @main() #0 {
  %1 = alloca i32, align 4
  %2 = alloca i32, align 4
  store i32 0, ptr %1, align 4
  store i32 3, ptr %2, align 4
  br label %3

3:                                                ; preds = %0, %9
  %4 = load i32, ptr %2, align 4
  %5 = add nsw i32 %4, 1
  store i32 %5, ptr %2, align 4
  %6 = load i32, ptr %2, align 4
  %7 = icmp eq i32 %6, 5
  br i1 %7, label %8, label %9

8:                                                ; preds = %3
  br label %10

9:                                                ; preds = %3
  br label %3

10:                                               ; preds = %8
  %11 = load i32, ptr %2, align 4
  ret i32 %11
}