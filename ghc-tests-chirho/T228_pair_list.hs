-- TEST: compile_and_run
-- EXPECTED: 10\n20\n30
module Main where
data Pair = MkPair Int Int
fst' (MkPair x _) = x
snd' (MkPair _ y) = y
printPairs [] = return ()
printPairs (p:ps) = do { print (fst' p + snd' p); printPairs ps }
main = printPairs [MkPair 3 7, MkPair 8 12, MkPair 15 15]
