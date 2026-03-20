-- TEST: compile_and_run
-- EXPECTED: 24
module Main where
data Op = Plus | Minus | Times
data Step = MkStep Op Int
apply :: Op -> Int -> Int -> Int
apply Plus a b = a + b
apply Minus a b = a - b
apply Times a b = a * b
run :: Int -> [Step] -> Int
run acc [] = acc
run acc (MkStep op v : rest) = run (apply op acc v) rest
main = print (run 10 [MkStep Plus 5, MkStep Minus 3, MkStep Times 2])
