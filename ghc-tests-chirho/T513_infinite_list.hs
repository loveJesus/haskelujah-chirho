-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: [42,42,42,42,42]\n[1,1,1,1,1,1,1,1,1,1]
module Main where
myRepeat :: a -> [a]
myRepeat x = x : myRepeat x
myTake :: Int -> [a] -> [a]
myTake 0 _ = []
myTake n (x:xs) = x : myTake (n-1) xs
main = do
  print (myTake 5 (myRepeat 42))
  print (myTake 10 (myRepeat 1))
