-- TEST: compile_and_run
-- EXPECTED: 10\n3\n13
module Main where
data Pair = MkPair Int Int
fst' :: Pair -> Int
fst' (MkPair x _) = x
snd' :: Pair -> Int
snd' (MkPair _ y) = y
addPair :: Pair -> Int
addPair (MkPair x y) = x + y
main :: IO ()
main = do
  let p = MkPair 10 3
  print (fst' p)
  print (snd' p)
  print (addPair p)
