-- TEST: compile
-- Stack calculator with ADT ops, list-as-stack, where recursion
module Main where
data Op = Push Int | Add | Mul | Sub
evalOp :: Op -> [Int] -> [Int]
evalOp (Push n) stack = n : stack
evalOp Add (a:b:rest) = (b + a) : rest
evalOp Mul (a:b:rest) = (b * a) : rest
evalOp Sub (a:b:rest) = (b - a) : rest
evalOp _ stack = stack
runProgram :: [Op] -> Int
runProgram ops = go ops []
  where
    go [] (result:_) = result
    go [] _ = 0
    go (op:rest) stack = go rest (evalOp op stack)
main :: IO ()
main = print (runProgram [Push 3, Push 4, Add, Push 2, Mul])
