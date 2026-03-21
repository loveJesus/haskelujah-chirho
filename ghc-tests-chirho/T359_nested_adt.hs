-- TEST: compile_and_run
-- EXPECTED: red circle\nblue rect
module Main where
data Color = Red | Blue
data Shape = Circle Color | Rect Color
colorName Red = "red"; colorName Blue = "blue"
shapeName (Circle c) = colorName c ++ " circle"; shapeName (Rect c) = colorName c ++ " rect"
main = do { putStrLn (shapeName (Circle Red)); putStrLn (shapeName (Rect Blue)) }
