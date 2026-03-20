-- TEST: compile_and_run
-- EXPECTED: 0\n1\n1\n2\n3\n5\n8\n13\n21\n34
module Main where
fibList :: Int -> [Int]
fibList n = go n 0 1
  where go 0 _ _ = []
        go k a b = a : go (k-1) b (a+b)
printList :: [Int] -> IO ()
printList [] = return ()
printList (x:xs) = do { print x; printList xs }
main :: IO ()
main = printList (fibList 10)
