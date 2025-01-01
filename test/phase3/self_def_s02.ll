; ModuleID = 'self_def_s02.spl'
source_filename = "self_def_s02.spl"

@arr = global [10 x i32] [i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 7, i32 8, i32 9, i32 10]
@0 = internal global [16 x i8] c"%d %d %d %d %d\0A\00"
@1 = internal global [16 x i8] c"%d %d %d %d %d\0A\00"

define i32 @main() {
entry:
  %arr2 = alloca [5 x i32], align 4
  %index = getelementptr [5 x i32], ptr %arr2, i32 0, i32 0
  store i32 1, ptr %index, align 4
  %index1 = getelementptr [5 x i32], ptr %arr2, i32 0, i32 1
  store i32 2, ptr %index1, align 4
  %index2 = getelementptr [5 x i32], ptr %arr2, i32 0, i32 2
  store i32 3, ptr %index2, align 4
  %index3 = getelementptr [5 x i32], ptr %arr2, i32 0, i32 3
  store i32 4, ptr %index3, align 4
  %index4 = getelementptr [5 x i32], ptr %arr2, i32 0, i32 4
  store i32 5, ptr %index4, align 4
  store i32 8, ptr getelementptr inbounds ([10 x i32], ptr @arr, i32 0, i32 5), align 4
  %index5 = getelementptr inbounds [5 x i32], ptr %arr2, i32 0, i32 3
  %index6 = getelementptr [2 x i32], ptr %index5, i32 0, i32 0
  store i32 7, ptr %index6, align 4
  %index7 = getelementptr [2 x i32], ptr %index5, i32 0, i32 1
  store i32 8, ptr %index7, align 4
  %arr = load [10 x i32], ptr getelementptr inbounds ([10 x i32], ptr @arr, i32 0, i32 3), align 4
  %arr8 = load [10 x i32], ptr getelementptr inbounds ([10 x i32], ptr @arr, i32 0, i32 4), align 4
  %arr9 = load [10 x i32], ptr getelementptr inbounds ([10 x i32], ptr @arr, i32 0, i32 5), align 4
  %arr10 = load [10 x i32], ptr getelementptr inbounds ([10 x i32], ptr @arr, i32 0, i32 6), align 4
  %arr11 = load [10 x i32], ptr getelementptr inbounds ([10 x i32], ptr @arr, i32 0, i32 7), align 4
  %0 = call i32 (ptr, ...) @printf(ptr @0, [10 x i32] %arr, [10 x i32] %arr8, [10 x i32] %arr9, [10 x i32] %arr10, [10 x i32] %arr11)
  %index12 = getelementptr inbounds [5 x i32], ptr %arr2, i32 0, i32 0
  %arr213 = load [5 x i32], ptr %index12, align 4
  %index14 = getelementptr inbounds [5 x i32], ptr %arr2, i32 0, i32 1
  %arr215 = load [5 x i32], ptr %index14, align 4
  %index16 = getelementptr inbounds [5 x i32], ptr %arr2, i32 0, i32 2
  %arr217 = load [5 x i32], ptr %index16, align 4
  %index18 = getelementptr inbounds [5 x i32], ptr %arr2, i32 0, i32 3
  %arr219 = load [5 x i32], ptr %index18, align 4
  %index20 = getelementptr inbounds [5 x i32], ptr %arr2, i32 0, i32 4
  %arr221 = load [5 x i32], ptr %index20, align 4
  %1 = call i32 (ptr, ...) @printf(ptr @1, [5 x i32] %arr213, [5 x i32] %arr215, [5 x i32] %arr217, [5 x i32] %arr219, [5 x i32] %arr221)
  ret i32 0
}

declare i32 @printf(ptr, ...)
