-- TEST: compile_and_run
-- EXPECTED: 1\n1\n0
module Main where
bti True = 1; bti False = 0
match :: String -> String -> Bool
match [] [] = True; match [] _ = False; match _ [] = False
match (p:ps) (c:cs) | p == 63 = match ps cs | p == c = match ps cs | otherwise = False
main = do
  print (bti (match [104,105] [104,105]))
  print (bti (match [104,63] [104,105]))
  print (bti (match [104,105] [104,111]))
