-- TEST: compile_and_run
-- EXPECTED: 2\n42
module Main where
data Op = Plus | Minus | Times | Divide | Modulo
applyOp :: Op -> Int -> Int -> Int
applyOp Plus a b = a + b
applyOp Minus a b = a - b
applyOp Times a b = a * b
applyOp Divide a b = a `div` b
applyOp Modulo a b = a `mod` b
calculate :: Int -> [(Op, Int)] -> Int
calculate acc [] = acc
calculate acc ((op, val):rest) = calculate (applyOp op acc val) rest
main :: IO ()
main = do
  print (calculate 17 [(Modulo, 5)])
  print (calculate 100 [(Divide, 7), (Times, 3)])
