-- TEST: compile_and_run
-- EXPECTED: 8\n10\n3
module Main where
data Expr = Lit Int | Var Int | Add Expr Expr | Mul Expr Expr | IfPos Expr Expr Expr
lookupEnv :: [(Int, Int)] -> Int -> Int
lookupEnv [] _ = 0
lookupEnv ((k,v):rest) key = if k == key then v else lookupEnv rest key
eval :: [(Int, Int)] -> Expr -> Int
eval _ (Lit n) = n
eval env (Var i) = lookupEnv env i
eval env (Add a b) = eval env a + eval env b
eval env (Mul a b) = eval env a * eval env b
eval env (IfPos cond t f) = if eval env cond > 0 then eval env t else eval env f
main :: IO ()
main = do
  let env = [(0, 5), (1, 3)]
  print (eval env (Add (Var 0) (Var 1)))
  print (eval env (Mul (Var 0) (Lit 2)))
  print (eval env (IfPos (Var 0) (Var 1) (Lit 0)))
