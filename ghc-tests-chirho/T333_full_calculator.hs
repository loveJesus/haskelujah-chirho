-- TEST: compile_and_run
-- EXPECTED: 24\n50\n120
module Main where
data Op = Plus | Minus | Times | Divide
applyOp :: Op -> Int -> Int -> Int
applyOp Plus a b = a + b
applyOp Minus a b = a - b
applyOp Times a b = a * b
applyOp Divide a b = a `div` b
calculate :: Int -> [(Op, Int)] -> Int
calculate acc [] = acc
calculate acc ((op, val):rest) = calculate (applyOp op acc val) rest
main :: IO ()
main = do
  print (calculate 10 [(Plus, 5), (Minus, 3), (Times, 2)])
  print (calculate 100 [(Divide, 4), (Plus, 25)])
  print (calculate 1 [(Times, 2), (Times, 3), (Times, 4), (Times, 5)])
