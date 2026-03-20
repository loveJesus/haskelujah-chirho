-- TEST: compile_and_run
-- EXPECTED: 24
module Main where
data Op = Plus | Minus | Times
data Step = MkStep Op Int
applyOp :: Op -> Int -> Int -> Int
applyOp Plus a b = a + b
applyOp Minus a b = a - b
applyOp Times a b = a * b
calculate :: Int -> [Step] -> Int
calculate acc [] = acc
calculate acc (MkStep op val : rest) = calculate (applyOp op acc val) rest
main :: IO ()
main = print (calculate 10 [MkStep Plus 5, MkStep Minus 3, MkStep Times 2])
