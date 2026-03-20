-- TEST: compile_and_run
-- EXPECTED: 1\n4\n9
module Main where
myMap _ [] = []; myMap f (x:xs) = f x : myMap f xs
printList [] = return (); printList (x:xs) = do { print x; printList xs }
main = printList (myMap (\x -> x * x) [1,2,3])
