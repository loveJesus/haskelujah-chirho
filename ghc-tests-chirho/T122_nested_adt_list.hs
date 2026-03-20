-- TEST: compile
-- Nested ADT with list of ADTs
module Main where
data Value = VInt Int | VBool Bool | VList [Value]
showValue :: Value -> String
showValue (VInt n) = show n
showValue (VBool True) = "true"
showValue (VBool False) = "false"
showValue (VList vs) = "[" ++ showItems vs ++ "]"
showItems :: [Value] -> String
showItems [] = ""
showItems (v:[]) = showValue v
showItems (v:vs) = showValue v ++ "," ++ showItems vs
main :: IO ()
main = putStrLn (showValue (VList [VInt 1, VInt 2, VInt 3]))
