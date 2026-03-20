-- TEST: compile_and_run
-- EXPECTED: [1,2,3]
module Main where
data Value = VInt Int | VList [Value]
showValue :: Value -> String
showValue (VInt n) = show n
showValue (VList vs) = "[" ++ showItems vs ++ "]"
showItems :: [Value] -> String
showItems [] = ""
showItems (v:[]) = showValue v
showItems (v:vs) = showValue v ++ "," ++ showItems vs
main :: IO ()
main = putStrLn (showValue (VList [VInt 1, VInt 2, VInt 3]))
