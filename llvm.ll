@.str.3 = private unnamed_addr constant [9 x i8] c"%d %d %d\00", align 1
@.str.2 = private unnamed_addr constant [22 x i8] c"Digite tres nemeros: \00", align 1
@.str.1 = private unnamed_addr constant [39 x i8] c"Os numeros que voce digitou: %d %d %d\0A\00", align 1
@x = dso_local global i32 0, align 4
@y = dso_local global i32 0, align 4
@z = dso_local global i32 0, align 4

declare i32 @printf(i8* noundef, ...) #1
declare i32 @__isoc99_scanf(i8* noundef, ...) #1

define dso_local i32 @main() #0 {
  call void @bloc_FAZER_SCAN()
  %1 = load i32, i32* @x, align 4
  %2 = load i32, i32* @y, align 4
  %3 = load i32, i32* @z, align 4
  %4 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([39 x i8], [39 x i8]* @.str.1, i64 0, i64 0), i32 noundef %1, i32 noundef %2, i32 noundef %3)
  ret i32 0
}

define dso_local void @bloc_FAZER_SCAN() #0 {
  %1 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([22 x i8], [22 x i8]* @.str.2, i64 0, i64 0))
  %2 = call i32 (i8*, ...) @__isoc99_scanf(i8* noundef getelementptr inbounds ([9 x i8], [9 x i8]* @.str.3, i64 0, i64 0), i32* noundef @x, i32* noundef @y, i32* noundef @z)
  ret void
}
