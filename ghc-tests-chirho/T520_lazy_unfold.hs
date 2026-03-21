-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: [1,1,2,3,5,8,13,21,34,55]\n[1,2,4,8,16,32,64,128,256,512]
module Main where
myTake :: Int -> [a] -> [a]
myTake 0 _ = []
myTake n (x:xs) = x : myTake (n-1) xs
-- Unfold: generate an infinite list from a seed
myUnfoldr :: (b -> (a, b)) -> b -> [a]
myUnfoldr f seed = let (val, nextSeed) = f seed in val : myUnfoldr f nextSeed
-- Fibonacci via unfold
fibUnfold :: [Int]
fibUnfold = myUnfoldr (\(a, b) -> (a, (b, a + b))) (1, 1)
-- Powers of 2 via unfold
powersOf2 :: [Int]
powersOf2 = myUnfoldr (\n -> (n, n * 2)) 1
main = do
  print (myTake 10 fibUnfold)
  print (myTake 10 powersOf2)
