-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16

module Main where

-- Binary Search Tree
data Tree = Leaf Int | Node Tree Int Tree

insert :: Int -> Tree -> Tree
insert x (Leaf _) = Node (Leaf 0) x (Leaf 0)
insert x (Node left val right) =
  if x < val
    then Node (insert x left) val right
    else Node left val (insert x right)

treeSum :: Tree -> Int
treeSum (Leaf _) = 0
treeSum (Node left val right) = treeSum left + val + treeSum right

treeSize :: Tree -> Int
treeSize (Leaf _) = 0
treeSize (Node left _ right) = 1 + treeSize left + treeSize right

-- Mutual Recursion
isEven :: Int -> Int
isEven 0 = 1
isEven n = isOdd (n - 1)

isOdd :: Int -> Int
isOdd 0 = 0
isOdd n = isEven (n - 1)

main :: IO ()
main = do
  putStrLn "=== Binary Search Tree ==="
  let empty = Leaf 0
  let t = insert 9 (insert 1 (insert 7 (insert 3 (insert 5 empty))))
  print (treeSum t)
  print (treeSize t)

  putStrLn "=== Mutual Recursion ==="
  print (isEven 42)
  print (isOdd 42)

  putStrLn "Done! Glory to God."
