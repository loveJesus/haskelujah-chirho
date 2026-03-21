-- TEST: compile_and_run
-- EXPECTED: 1\n0
module Main where
type ChurchBool = Int -> Int -> Int
ctrue :: ChurchBool
ctrue x _ = x
cfalse :: ChurchBool
cfalse _ y = y
cand :: ChurchBool -> ChurchBool -> ChurchBool
cand a b x y = a (b x y) y
main = do { print (cand ctrue ctrue 1 0); print (cand ctrue cfalse 1 0) }
