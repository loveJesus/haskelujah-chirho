-- TEST: compile_and_run
-- EXPECTED: 15
module Main where
myScanl :: (Int -> Int -> Int) -> Int -> [Int] -> [Int]
myScanl _ acc [] = [acc]
myScanl f acc (x:xs) = acc : myScanl f (f acc x) xs
myLast :: [Int] -> Int
myLast [x] = x
myLast (_:xs) = myLast xs
myLast [] = 0
add :: Int -> Int -> Int
add a b = a + b
main :: IO ()
main = print (myLast (myScanl add 0 [1,2,3,4,5]))
