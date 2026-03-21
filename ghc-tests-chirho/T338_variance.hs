-- TEST: compile_and_run
-- EXPECTED: 49
module Main where
myMap _ [] = []; myMap f (x:xs) = f x : myMap f xs
mySum [] = 0; mySum (x:xs) = x + mySum xs
myLen [] = 0; myLen (_:xs) = 1 + myLen xs
mean xs = mySum xs `div` myLen xs
variance xs = mySum (myMap (\x -> (x - m) * (x - m)) xs) `div` myLen xs where m = mean xs
main = print (variance [85, 92, 78, 95, 88, 76, 91, 83, 97, 79])
