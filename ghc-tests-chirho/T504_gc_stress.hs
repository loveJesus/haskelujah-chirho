-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: 55\n5050
module Main where
sumList :: [Int] -> Int
sumList [] = 0
sumList (x:xs) = x + sumList xs
range :: Int -> Int -> [Int]
range lo hi = if lo > hi then [] else lo : range (lo + 1) hi
main = do
  print (sumList (range 1 10))
  print (sumList (range 1 100))
