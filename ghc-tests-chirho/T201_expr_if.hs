-- TEST: compile_and_run
-- EXPECTED: 7\n42\n-1
module Main where
data Expr = Num Int | Add Expr Expr | If Expr Expr Expr
eval :: Expr -> Int
eval (Num n) = n
eval (Add a b) = eval a + eval b
eval (If cond t f) = if eval cond > 0 then eval t else eval f
main :: IO ()
main = do
  print (eval (Add (Num 3) (Num 4)))
  print (eval (If (Num 1) (Num 42) (Num 0)))
  print (eval (If (Num 0) (Num 42) (Num (-1))))
