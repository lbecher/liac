@x = dso_local global i32 0, align 4
@y = dso_local global i32 0, align 4
@z = dso_local global i32 0, align 4

define dso_local i32 @main() #0 {
  store i32 1, i32* @x, align 4
  %1 = load i32, i32* @x, align 4
  store i32 %1, i32* @y, align 4
  %2 = load i32, i32* @y, align 4
  %3 = load i32, i32* @x, align 4
  %4 = sub nsw i32 %2, %3
  %5 = load i32, i32* @x, align 4
  %6 = add nsw i32 %5, %4
  store i32 %6, i32* @z, align 4
  ret i32 0
}
