-- TEST: compile_and_run
-- EXPECTED: 1\n0
module Main where
bti True = 1; bti False = 0
data Set = EmptySet | Node Int Set Set
member_ :: Int -> Set -> Bool
member_ _ EmptySet = False
member_ x (Node v left right)
  | x == v = True
  | x < v = member_ x left
  | otherwise = member_ x right
insert_ :: Int -> Set -> Set
insert_ x EmptySet = Node x EmptySet EmptySet
insert_ x (Node v l r)
  | x == v = Node v l r
  | x < v = Node v (insert_ x l) r
  | otherwise = Node v l (insert_ x r)
main :: IO ()
main = do
  let s = insert_ 5 (insert_ 3 (insert_ 7 EmptySet))
  print (bti (member_ 3 s))
  print (bti (member_ 4 s))
