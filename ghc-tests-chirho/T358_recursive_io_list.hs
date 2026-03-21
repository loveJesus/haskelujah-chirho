-- TEST: compile_and_run
-- EXPECTED: 1\n2\n3\n4\n5
module Main where
printAll :: [Int] -> IO ()
printAll [] = return ()
printAll (x:xs) = do { print x; printAll xs }
main = printAll [1,2,3,4,5]
