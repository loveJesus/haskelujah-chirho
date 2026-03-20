-- TEST: compile_and_run
-- EXPECTED: 3
module Main where
f :: Maybe (Maybe Int) -> Int
f (Just (Just x)) = x
f (Just Nothing) = -1
f Nothing = -2
main = print (f (Just (Just 3)))
