-- TEST: compile_and_run
-- EXPECTED: red shape with area 75\nblue shape with area 12
module Main where
data Color = Red | Green | Blue
data Shape = Circle Int | Rectangle Int Int
colorName :: Color -> String
colorName Red = "red"
colorName Green = "green"
colorName Blue = "blue"
area :: Shape -> Int
area (Circle r) = r * r * 3
area (Rectangle w h) = w * h
describe :: Color -> Shape -> String
describe c s = colorName c ++ " shape with area " ++ show (area s)
main :: IO ()
main = do
  putStrLn (describe Red (Circle 5))
  putStrLn (describe Blue (Rectangle 3 4))
