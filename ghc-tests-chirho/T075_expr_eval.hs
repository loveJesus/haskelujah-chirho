-- TEST: compile_and_run
-- EXPECTED: 14\n-8
module Main where
data Expr = Lit Int | Add Expr Expr | Mul Expr Expr | Neg Expr
eval :: Expr -> Int
eval (Lit n) = n
eval (Add a b) = eval a + eval b
eval (Mul a b) = eval a * eval b
eval (Neg e) = 0 - eval e
main :: IO ()
main = do
  print (eval (Mul (Add (Lit 3) (Lit 4)) (Lit 2)))
  print (eval (Neg (Add (Lit 5) (Lit 3))))
