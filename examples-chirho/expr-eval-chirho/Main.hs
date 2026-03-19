-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

module Main where

-- A simple expression language
data Expr = Lit Int | Add Expr Expr | Mul Expr Expr | Neg Expr

-- Recursive evaluator
eval :: Expr -> Int
eval (Lit n) = n
eval (Add a b) = eval a + eval b
eval (Mul a b) = eval a * eval b
eval (Neg e) = 0 - eval e

-- Calculator operations
data Op = OpAdd | OpSub | OpMul | OpDiv
calc :: Op -> Int -> Int -> Int
calc OpAdd a b = a + b
calc OpSub a b = a - b
calc OpMul a b = a * b
calc OpDiv a b = if b == 0 then 0 else a `div` b

main :: IO ()
main = do
  putStrLn "=== Expression Evaluator ==="
  print (eval (Add (Lit 10) (Lit 32)))
  print (eval (Mul (Add (Lit 3) (Lit 4)) (Lit 6)))
  print (eval (Neg (Add (Lit 20) (Lit 22))))
  putStrLn ""
  putStrLn "=== Calculator ==="
  print (calc OpAdd 10 32)
  print (calc OpMul 6 7)
  print (calc OpDiv 100 4)
  print (calc OpDiv 42 0)
  putStrLn ""
  putStrLn "Done! Glory to God."
