-- TEST: compile_and_run
-- EXPECTED: 3
module Main where
data Tree = Leaf | Node Int Tree Tree
max' :: Int -> Int -> Int
max' a b = if a >= b then a else b
depth :: Tree -> Int
depth Leaf = 0
depth (Node _ l r) = 1 + max' (depth l) (depth r)
main :: IO ()
main = print (depth (Node 1 (Node 2 Leaf Leaf) (Node 3 (Node 4 Leaf Leaf) Leaf)))
