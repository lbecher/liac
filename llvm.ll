@.str.2 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@.str.1 = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
@n = dso_local global i32 0, align 4
@res = dso_local global i32 0, align 4
@num = dso_local global i32 0, align 4
@menor_igual = dso_local global i32 0, align 4

declare i32 @printf(i8* noundef, ...) #1
declare i32 @__isoc99_scanf(i8* noundef, ...) #1

define dso_local i32 @main() #0 {
  store i32 1, i32* @res, align 4
  store i32 2, i32* @num, align 4
  store i32 4, i32* @n, align 4
  call void @bloc_POTENCIA()
  %1 = load i32, i32* @res, align 4
  %2 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([4 x i8], [4 x i8]* @.str.1, i64 0, i64 0), i32 noundef %1)
  ret i32 0
}

define dso_local void @bloc_POTENCIA() #0 {
  %1 = load i32, i32* @n, align 4
  %2 = sub nsw i32 %1, 1
  store i32 %2, i32* @n, align 4
  %3 = load i32, i32* @n, align 4
  %4 = icmp sle i32 %3, 0
  %5 = zext i1 %4 to i32
  store i32 %5, i32* @menor_igual, align 4
  %6 = load i32, i32* @menor_igual, align 4
  %7 = icmp ne i32 %6, 0
  br i1 %7, label %8, label %12

8:
  %9 = load i32, i32* @res, align 4
  %10 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([4 x i8], [4 x i8]* @.str.2, i64 0, i64 0), i32 noundef %9)
  br i1 0, label %11, label %16

11:
  br label %8

12:
  %13 = load i32, i32* @num, align 4
  %14 = load i32, i32* @res, align 4
  %15 = mul nsw i32 %13, %14
  store i32 %15, i32* @res, align 4
  call void @bloc_POTENCIA()
  br label %16

16:
  ret void
}
