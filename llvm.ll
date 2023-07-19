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
  %2 = icmp sle i32 %1, 0
  %3 = zext i1 %2 to i32
  store i32 %3, i32* @menor_igual, align 4
  %4 = load i32, i32* @menor_igual, align 4
  %5 = icmp ne i32 %4, 0
  br i1 %5, label %6, label %8

6:
  br i1 0, label %7, label %14

7:
  br label %6

8:
  %9 = load i32, i32* @num, align 4
  %10 = load i32, i32* @res, align 4
  %11 = mul nsw i32 %9, %10
  store i32 %11, i32* @res, align 4
  %12 = load i32, i32* @n, align 4
  %13 = sub nsw i32 %12, 1
  store i32 %13, i32* @n, align 4
  call void @bloc_POTENCIA()
  br label %14

14:
  ret void
}
