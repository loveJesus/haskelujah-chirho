-- TEST: compile_and_run
-- EXPECTED: 42\n0
module Main where
data Dict = Empty | Entry Int Int Dict
lookup_ :: Int -> Dict -> Int
lookup_ _ Empty = 0
lookup_ key (Entry k v rest) = if key == k then v else lookup_ key rest
insert :: Int -> Int -> Dict -> Dict
insert k v m = Entry k v m
main :: IO ()
main = do
  let m = insert 1 42 (insert 2 99 Empty)
  print (lookup_ 1 m)
  print (lookup_ 3 m)
