; ModuleID = 'self_def_s01.spl'
source_filename = "self_def_s01.spl"

%Point = type { i32, i32 }

@0 = internal global [14 x i8] c"a = (%d, %d)\0A\00"
@1 = internal global [14 x i8] c"b = (%d, %d)\0A\00"

define i32 @main() {
entry:
  %a = alloca %Point, align 8
  %x = getelementptr inbounds %Point, ptr %a, i32 0, i32 0
  store i32 1, ptr %x, align 4
  %y = getelementptr inbounds %Point, ptr %a, i32 0, i32 1
  store i32 2, ptr %y, align 4
  %b = alloca %Point, align 8
  %x1 = getelementptr inbounds %Point, ptr %b, i32 0, i32 0
  store i32 3, ptr %x1, align 4
  %y2 = getelementptr inbounds %Point, ptr %b, i32 0, i32 1
  store i32 4, ptr %y2, align 4
  %x3 = getelementptr inbounds %Point, ptr %a, i32 0, i32 0
  %a.x = load i32, ptr %x3, align 4
  %y4 = getelementptr inbounds %Point, ptr %a, i32 0, i32 1
  %a.y = load i32, ptr %y4, align 4
  %0 = call i32 (ptr, ...) @printf(ptr @0, i32 %a.x, i32 %a.y)
  %x5 = getelementptr inbounds %Point, ptr %b, i32 0, i32 0
  %b.x = load i32, ptr %x5, align 4
  %y6 = getelementptr inbounds %Point, ptr %b, i32 0, i32 1
  %b.y = load i32, ptr %y6, align 4
  %1 = call i32 (ptr, ...) @printf(ptr @1, i32 %b.x, i32 %b.y)
  ret i32 0
}

declare i32 @printf(ptr, ...)
