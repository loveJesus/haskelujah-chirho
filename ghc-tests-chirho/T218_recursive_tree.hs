-- TEST: compile_and_run
-- EXPECTED: 42\n3
module Main where
data Tree = Leaf | Node Int Tree Tree
sumTree Leaf = 0; sumTree (Node v l r) = v + sumTree l + sumTree r
depth Leaf = 0; depth (Node _ l r) = 1 + (if depth l > depth r then depth l else depth r)
main = do
  let t = Node 10 (Node 5 Leaf Leaf) (Node 15 (Node 12 Leaf Leaf) Leaf)
  print (sumTree t)
  print (depth t)
