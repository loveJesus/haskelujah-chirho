-- TEST: compile_and_run
-- EXPECTED: 24
module Main where
data Op = Plus | Minus | Times
applyOp :: Op -> Int -> Int -> Int
applyOp Plus a b = a + b
applyOp Minus a b = a - b
applyOp Times a b = a * b
calculate :: Int -> [(Op, Int)] -> Int
calculate acc [] = acc
calculate acc ((op, val):rest) = calculate (applyOp op acc val) rest
main :: IO ()
main = print (calculate 10 [(Plus, 5), (Minus, 3), (Times, 2)])
