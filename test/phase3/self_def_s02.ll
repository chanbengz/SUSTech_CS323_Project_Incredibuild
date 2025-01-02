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
  %arr = load i32, ptr getelementptr inbounds ([10 x i32], ptr @arr, i32 0, i32 3), align 4
  %addtmp = add i32 8, %arr
  store i32 %addtmp, ptr getelementptr inbounds ([10 x i32], ptr @arr, i32 0, i32 5), align 4
  %arr5 = load i32, ptr @arr, align 4
  %addtmp6 = add i32 7, %arr5
  %index7 = getelementptr inbounds [5 x i32], ptr %arr2, i32 0, i32 3
  store i32 %addtmp6, ptr %index7, align 4
  %arr8 = load i32, ptr getelementptr inbounds ([10 x i32], ptr @arr, i32 0, i32 3), align 4
  %arr9 = load i32, ptr getelementptr inbounds ([10 x i32], ptr @arr, i32 0, i32 4), align 4
  %arr10 = load i32, ptr getelementptr inbounds ([10 x i32], ptr @arr, i32 0, i32 5), align 4
  %arr11 = load i32, ptr getelementptr inbounds ([10 x i32], ptr @arr, i32 0, i32 6), align 4
  %arr12 = load i32, ptr getelementptr inbounds ([10 x i32], ptr @arr, i32 0, i32 7), align 4
  %0 = call i32 (ptr, ...) @printf(ptr @0, i32 %arr8, i32 %arr9, i32 %arr10, i32 %arr11, i32 %arr12)
  %index13 = getelementptr inbounds [5 x i32], ptr %arr2, i32 0, i32 0
  %arr214 = load i32, ptr %index13, align 4
  %index15 = getelementptr inbounds [5 x i32], ptr %arr2, i32 0, i32 1
  %arr216 = load i32, ptr %index15, align 4
  %index17 = getelementptr inbounds [5 x i32], ptr %arr2, i32 0, i32 2
  %arr218 = load i32, ptr %index17, align 4
  %index19 = getelementptr inbounds [5 x i32], ptr %arr2, i32 0, i32 3
  %arr220 = load i32, ptr %index19, align 4
  %index21 = getelementptr inbounds [5 x i32], ptr %arr2, i32 0, i32 4
  %arr222 = load i32, ptr %index21, align 4
  %1 = call i32 (ptr, ...) @printf(ptr @1, i32 %arr214, i32 %arr216, i32 %arr218, i32 %arr220, i32 %arr222)
  ret i32 0
}

declare i32 @printf(ptr, ...)
