-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: [1,2,3,4,5]\n[0,1,2,3,4,5,6,7,8,9]\n21
module Main where
myIterate :: (a -> a) -> a -> [a]
myIterate f x = x : myIterate f (f x)
myTake :: Int -> [a] -> [a]
myTake 0 _ = []
myTake n (x:xs) = x : myTake (n-1) xs
sumList :: [Int] -> Int
sumList [] = 0
sumList (x:xs) = x + sumList xs
nats :: [Int]
nats = myIterate (+ 1) 0
main = do
  print (myTake 5 (myIterate (+ 1) 1))
  print (myTake 10 nats)
  print (sumList (myTake 6 (myIterate (+ 1) 1)))
