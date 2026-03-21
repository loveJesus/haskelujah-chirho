-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: Sum 15\nProduct 120\nAll True\nAny True
module Main where
-- Newtypes with show — common Hackage pattern (Data.Monoid style)
data Sum = Sum Int
data Product = Product Int
data All = All Bool
data Any = Any Bool
showSum :: Sum -> String
showSum (Sum n) = "Sum " ++ show n
showProduct :: Product -> String
showProduct (Product n) = "Product " ++ show n
showAll :: All -> String
showAll (All b) = "All " ++ show b
showAny :: Any -> String
showAny (Any b) = "Any " ++ show b
myFoldr :: (a -> b -> b) -> b -> [a] -> b
myFoldr _ z [] = z
myFoldr f z (x:xs) = f x (myFoldr f z xs)
sumConcat :: [Int] -> Sum
sumConcat xs = Sum (myFoldr (+) 0 xs)
productConcat :: [Int] -> Product
productConcat xs = Product (myFoldr (*) 1 xs)
allConcat :: [Bool] -> All
allConcat xs = All (myFoldr (\a b -> a && b) True xs)
anyConcat :: [Bool] -> Any
anyConcat xs = Any (myFoldr (\a b -> a || b) False xs)
main :: IO ()
main = do
  putStrLn (showSum (sumConcat [1,2,3,4,5]))
  putStrLn (showProduct (productConcat [1,2,3,4,5]))
  putStrLn (showAll (allConcat [True, True, True]))
  putStrLn (showAny (anyConcat [False, True, False]))
