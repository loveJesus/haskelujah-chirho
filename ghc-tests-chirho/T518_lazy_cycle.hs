-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: [1,2,3,1,2,3,1,2,3,1]\n19
module Main where
myTake :: Int -> [a] -> [a]
myTake 0 _ = []
myTake n (x:xs) = x : myTake (n-1) xs
myCycle :: [a] -> [a]
myCycle xs = go xs
  where go [] = go xs
        go (y:ys) = y : go ys
sumList :: [Int] -> Int
sumList [] = 0
sumList (x:xs) = x + sumList xs
main = do
  print (myTake 10 (myCycle [1,2,3]))
  print (sumList (myTake 10 (myCycle [1,2,3])))
