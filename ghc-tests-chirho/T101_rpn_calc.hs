-- TEST: compile
-- RPN calculator with 4-constructor Token ADT and list-as-stack
module Main where
data Token = Num Int | Plus | Times | Minus
eval :: [Token] -> [Int] -> Int
eval [] (r:_) = r
eval [] _ = 0
eval (Num n : rest) stack = eval rest (n : stack)
eval (Plus : rest) (a:b:stack) = eval rest ((b + a) : stack)
eval (Times : rest) (a:b:stack) = eval rest ((b * a) : stack)
eval (Minus : rest) (a:b:stack) = eval rest ((b - a) : stack)
eval (_:rest) stack = eval rest stack
rpn :: [Token] -> Int
rpn tokens = eval tokens []
main :: IO ()
main = print (rpn [Num 3, Num 4, Plus, Num 2, Times])
