@.str.3 = private unnamed_addr constant [9 x i8] c"%d %d %d\00", align 1
@.str.2 = private unnamed_addr constant [22 x i8] c"Digite tres nemeros: \00", align 1
@.str.1 = private unnamed_addr constant [39 x i8] c"Os numeros que voce digitou: %d %d %d\0A\00", align 1
@x = dso_local global i32 0, align 4
@y = dso_local global i32 0, align 4
@z = dso_local global i32 0, align 4

declare i32 @printf(i8* noundef, ...) #1
declare i32 @__isoc99_scanf(i8* noundef, ...) #1

define dso_local i32 @main() #0 {
  store i32 -1, i32* @x, align 4
  br label %1

1:
  %2 = load i32, i32* @x, align 4
  %3 = icmp ne i32 %2, 0
  br i1 %3, label %4, label %5

4:
  call void @bloc_FAZER_SCAN()
  br label %1

5:
  %6 = load i32, i32* @x, align 4
  %7 = xor i32 %6, -1
  store i32 %7, i32* @x, align 4
  %8 = load i32, i32* @y, align 4
  %9 = mul nsw i32 %8, -1
  store i32 %9, i32* @y, align 4
  %10 = load i32, i32* @z, align 4
  %11 = load i32, i32* @y, align 4
  %12 = icmp sle i32 %10, %11
  %13 = zext i1 %12 to i32
  store i32 %13, i32* @z, align 4
  %14 = load i32, i32* @x, align 4
  %15 = load i32, i32* @y, align 4
  %16 = load i32, i32* @z, align 4
  %17 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([39 x i8], [39 x i8]* @.str.1, i64 0, i64 0), i32 noundef %14, i32 noundef %15, i32 noundef %16)
  ret i32 0
}

define dso_local void @bloc_FAZER_SCAN() #0 {
  %1 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([22 x i8], [22 x i8]* @.str.2, i64 0, i64 0))
  %2 = call i32 (i8*, ...) @__isoc99_scanf(i8* noundef getelementptr inbounds ([9 x i8], [9 x i8]* @.str.3, i64 0, i64 0), i32* noundef @x, i32* noundef @y, i32* noundef @z)
  ret void
}
