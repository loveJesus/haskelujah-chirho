-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
module Main where

-- Statistics calculator
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs

myLength :: [Int] -> Int
myLength [] = 0
myLength (_:xs) = 1 + myLength xs

myMax :: Int -> [Int] -> Int
myMax best [] = best
myMax best (x:xs) = if x > best then myMax x xs else myMax best xs

myMin :: Int -> [Int] -> Int
myMin best [] = best
myMin best (x:xs) = if x < best then myMin x xs else myMin best xs

mean :: [Int] -> Int
mean xs = mySum xs `div` myLength xs

variance :: [Int] -> Int
variance xs = mySum (myMap (\x -> (x - m) * (x - m)) xs) `div` myLength xs
  where m = mean xs

myMap :: (Int -> Int) -> [Int] -> [Int]
myMap _ [] = []
myMap f (x:rest) = f x : myMap f rest

myFilter :: (Int -> Bool) -> [Int] -> [Int]
myFilter _ [] = []
myFilter p (x:rest) = if p x then x : myFilter p rest else myFilter p rest

-- Report
report :: String -> [Int] -> IO ()
report name xs = do
  putStrLn ("=== " ++ name ++ " ===")
  putStrLn ("  Count:    " ++ show (myLength xs))
  putStrLn ("  Sum:      " ++ show (mySum xs))
  putStrLn ("  Mean:     " ++ show (mean xs))
  putStrLn ("  Min:      " ++ show (myMin 999999 xs))
  putStrLn ("  Max:      " ++ show (myMax 0 xs))
  putStrLn ("  Variance: " ++ show (variance xs))
  putStrLn ("  Evens:    " ++ show (myLength (myFilter (\x -> x `mod` 2 == 0) xs)))

main :: IO ()
main = do
  report "Test Scores" [85, 92, 78, 95, 88, 76, 91, 83, 97, 79]
  putStrLn ""
  putStrLn "Compiled with Haskelujah Chirho"
  putStrLn "Glory to God!"
