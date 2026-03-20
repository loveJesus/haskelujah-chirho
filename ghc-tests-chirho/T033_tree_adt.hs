-- TEST: compile_and_run
-- EXPECTED: 4\n42
module Main where
data Tree = Leaf | Node Int Tree Tree
count :: Tree -> Int
count Leaf = 0
count (Node _ l r) = 1 + count l + count r
sumTree :: Tree -> Int
sumTree Leaf = 0
sumTree (Node v l r) = v + sumTree l + sumTree r
main :: IO ()
main = do
  let t = Node 10 (Node 5 Leaf Leaf) (Node 15 (Node 12 Leaf Leaf) Leaf)
  print (count t)
  print (sumTree t)
