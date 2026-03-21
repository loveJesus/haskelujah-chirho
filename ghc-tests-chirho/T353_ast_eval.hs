-- TEST: compile_and_run
-- EXPECTED: (3+4) = 7\n((2+3)*4) = 20\n((2*3)+(4*5)) = 26
module Main where
data Expr = Num Int | Add Expr Expr | Mul Expr Expr
eval :: Expr -> Int
eval (Num n) = n; eval (Add a b) = eval a + eval b; eval (Mul a b) = eval a * eval b
showExpr :: Expr -> String
showExpr (Num n) = show n
showExpr (Add a b) = "(" ++ showExpr a ++ "+" ++ showExpr b ++ ")"
showExpr (Mul a b) = "(" ++ showExpr a ++ "*" ++ showExpr b ++ ")"
main :: IO ()
main = do
  putStrLn (showExpr (Add (Num 3) (Num 4)) ++ " = " ++ show (eval (Add (Num 3) (Num 4))))
  putStrLn (showExpr (Mul (Add (Num 2) (Num 3)) (Num 4)) ++ " = " ++ show (eval (Mul (Add (Num 2) (Num 3)) (Num 4))))
  putStrLn (showExpr (Add (Mul (Num 2) (Num 3)) (Mul (Num 4) (Num 5))) ++ " = " ++ show (eval (Add (Mul (Num 2) (Num 3)) (Mul (Num 4) (Num 5)))))
