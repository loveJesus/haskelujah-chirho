-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: [1,2,3,4,5,6,7]\n7\n28
module Main where
data Tree a = Leaf | Node (Tree a) a (Tree a)

insert :: Int -> Tree Int -> Tree Int
insert x Leaf = Node Leaf x Leaf
insert x (Node l v r)
  | x < v = Node (insert x l) v r
  | x > v = Node l v (insert x r)
  | otherwise = Node l v r

inorder :: Tree a -> [a]
inorder Leaf = []
inorder (Node l v r) = inorder l ++ [v] ++ inorder r

treeSize :: Tree a -> Int
treeSize Leaf = 0
treeSize (Node l _ r) = 1 + treeSize l + treeSize r

treeSum :: Tree Int -> Int
treeSum Leaf = 0
treeSum (Node l v r) = v + treeSum l + treeSum r

fromList :: [Int] -> Tree Int
fromList [] = Leaf
fromList (x:xs) = insert x (fromList xs)

main :: IO ()
main = do
  let t = fromList [4,2,6,1,3,5,7]
  print (inorder t)
  print (treeSize t)
  print (treeSum t)
