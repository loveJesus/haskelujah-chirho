-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: [0,1,3,6,10,15,21,28,36,45]\n[1,1,2,6,24,120,720,5040]
module Main where
myTake :: Int -> [a] -> [a]
myTake 0 _ = []
myTake n (x:xs) = x : myTake (n-1) xs
myScanl :: (b -> a -> b) -> b -> [a] -> [b]
myScanl _ acc [] = [acc]
myScanl f acc (x:xs) = acc : myScanl f (f acc x) xs
from :: Int -> [Int]
from n = n : from (n + 1)
-- Infinite running sums via scanl on natural numbers
runningSums :: [Int]
runningSums = myScanl (+) 0 (from 1)
-- Factorials via scanl
factorials :: [Int]
factorials = myScanl (*) 1 (from 1)
main = do
  print (myTake 10 runningSums)
  print (myTake 8 factorials)
