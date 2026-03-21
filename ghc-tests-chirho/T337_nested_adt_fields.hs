-- TEST: compile_and_run
-- EXPECTED: red circle r=5\nblue rect 3x4\n147
module Main where
data Color = Red | Green | Blue | Yellow
data Shape = Circle Color Int | Rectangle Color Int Int
colorName :: Color -> String
colorName Red = "red"; colorName Green = "green"; colorName Blue = "blue"; colorName Yellow = "yellow"
area :: Shape -> Int
area (Circle _ r) = r * r * 3
area (Rectangle _ w h) = w * h
describe :: Shape -> String
describe (Circle c r) = colorName c ++ " circle r=" ++ show r
describe (Rectangle c w h) = colorName c ++ " rect " ++ show w ++ "x" ++ show h
main :: IO ()
main = do
  putStrLn (describe (Circle Red 5))
  putStrLn (describe (Rectangle Blue 3 4))
  print (area (Circle Yellow 7))
