-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
module Main where
collatzLen :: Int -> Int
collatzLen 1 = 0
collatzLen n = if n `mod` 2 == 0
               then 1 + collatzLen (n `div` 2)
               else 1 + collatzLen (3 * n + 1)
sumCollatz :: Int -> Int -> Int
sumCollatz lo hi = go lo 0
  where go i acc = if i > hi then acc else go (i + 1) (acc + collatzLen i)
main :: IO ()
main = print (sumCollatz 1 10000)
