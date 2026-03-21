-- TEST: compile_and_run
-- EXPECTED: lex: 3 tokens\nparse: (3+4)\neval: 7
-- Mini compiler pipeline: lex → parse → eval
module Main where
data Token = TNum Int | TPlus | TEnd
data Expr = Num Int | Add Expr Expr
lex_ :: [Int] -> [Token]
lex_ [] = [TEnd]
lex_ (c:rest) | c == 43 = TPlus : lex_ rest | c >= 48 = TNum (c - 48) : lex_ rest | otherwise = lex_ rest
myLen :: [Token] -> Int
myLen [] = 0; myLen (_:xs) = 1 + myLen xs
parseSimple :: [Token] -> Expr
parseSimple [TNum a, TPlus, TNum b] = Add (Num a) (Num b)
parseSimple [TNum a] = Num a
parseSimple _ = Num 0
eval_ :: Expr -> Int
eval_ (Num n) = n; eval_ (Add a b) = eval_ a + eval_ b
showExpr :: Expr -> String
showExpr (Num n) = show n; showExpr (Add a b) = "(" ++ showExpr a ++ "+" ++ showExpr b ++ ")"
main :: IO ()
main = do
  let tokens = lex_ [51, 43, 52]
  let ast = parseSimple tokens
  putStrLn ("lex: " ++ show (myLen tokens) ++ " tokens")
  putStrLn ("parse: " ++ showExpr ast)
  putStrLn ("eval: " ++ show (eval_ ast))
