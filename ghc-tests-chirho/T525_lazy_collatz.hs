-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: [27,82,41,124,62,31,94,47,142,71]\n111
module Main where
myTake :: Int -> [a] -> [a]
myTake 0 _ = []
myTake n (x:xs) = x : myTake (n-1) xs
myLength :: [a] -> Int
myLength [] = 0
myLength (_:xs) = 1 + myLength xs
myTakeWhile :: (a -> Bool) -> [a] -> [a]
myTakeWhile _ [] = []
myTakeWhile p (x:xs) = if p x then x : myTakeWhile p xs else []
-- Infinite Collatz sequence via lazy list
collatz :: Int -> [Int]
collatz n = n : collatz (if mod n 2 == 0 then div n 2 else 3 * n + 1)
main = do
  print (myTake 10 (collatz 27))
  print (myLength (myTakeWhile (\x -> x /= 1) (collatz 27)))
