-- TEST: compile_and_run
-- EXPECTED: 10\n3
module Main where
data Pair = MkPair Int Int
fst' :: Pair -> Int
fst' (MkPair x _) = x
snd' :: Pair -> Int
snd' (MkPair _ y) = y
swap :: Pair -> Pair
swap (MkPair x y) = MkPair y x
main :: IO ()
main = do
  let p = swap (MkPair 3 10)
  print (fst' p)
  print (snd' p)
