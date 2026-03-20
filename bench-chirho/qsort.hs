-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
module Main where
filter' :: (Int -> Bool) -> [Int] -> [Int]
filter' f [] = []
filter' f (x:xs) = if f x then x : filter' f xs else filter' f xs
append :: [Int] -> [Int] -> [Int]
append [] ys = ys
append (x:xs) ys = x : append xs ys
qsort :: [Int] -> [Int]
qsort [] = []
qsort (p:xs) = append (qsort (filter' (\x -> x < p) xs))
                       (p : qsort (filter' (\x -> x >= p) xs))
mySum :: [Int] -> Int
mySum [] = 0
mySum (x:xs) = x + mySum xs
range :: Int -> Int -> [Int]
range lo hi = if lo > hi then [] else lo : range (lo + 1) hi
main :: IO ()
main = print (mySum (qsort (range 1 1000)))
