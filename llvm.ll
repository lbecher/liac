@.str.2 = private unnamed_addr constant [4 x i8] c"Hi\0A\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"%d %d %ld %ld\0A\00", align 1
@g_ = dso_local global i8 0, align 1
@h_ = dso_local global i8 0, align 1
@e_ = dso_local global i16 0, align 2
@f_ = dso_local global i16 0, align 2
@c_ = dso_local global i64 0, align 8
@d_ = dso_local global i64 0, align 8
@a_ = dso_local global i32 0, align 4
@b_ = dso_local global i32 0, align 4
@g = dso_local global i8 0, align 1
@h = dso_local global i8 0, align 1
@e = dso_local global i16 0, align 2
@f = dso_local global i16 0, align 2
@c = dso_local global i64 0, align 8
@d = dso_local global i64 0, align 8
@a = dso_local global i32 0, align 4
@b = dso_local global i32 0, align 4

declare i32 @printf(i8* noundef, ...) #1
declare i32 @__isoc99_scanf(i8* noundef, ...) #1

define dso_local i32 @main() #0 {
  %1 = load i32, i32* @b, align 4
  %2 = xor i32 %1, -1
  %3 = load i32, i32* @a, align 4
  %4 = add nsw i32 %3, %2
  store i32 %4, i32* @a, align 4
  %5 = load i64, i64* @d, align 8
  %6 = xor i64 %5, -1
  %7 = load i64, i64* @c, align 8
  %8 = add nsw i64 %7, %6
  store i64 %8, i64* @c, align 8
  %9 = load i16, i16* @f, align 2
  store i16 %9, i16* @e, align 2
  %10 = load i8, i8* @h, align 1
  store i8 %10, i8* @g, align 1
  %11 = load i32, i32* @b, align 4
  %12 = xor i32 %11, -1
  %13 = load i32, i32* @a, align 4
  %14 = add nsw i32 %13, %12
  %15 = load i16, i16* @f_, align 2
  store i16 %15, i16* @e_, align 2
  %16 = load i32, i32* @a, align 4
  %17 = load i32, i32* @b, align 4
  %18 = load i64, i64* @c, align 8
  %19 = load i64, i64* @d, align 8
  %20 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef %16, i32 noundef %17, i64 noundef %18, i64 noundef %19)
  ret i32 0
}

define dso_local void @bloc_HELLO() #0 {
  %1 = call i32 (i8*, ...) @printf(i8* noundef getelementptr inbounds ([4 x i8], [4 x i8]* @.str.2, i64 0, i64 0))
  ret void
}
