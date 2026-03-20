-- TEST: compile_and_run
-- EXPECTED: xs\ns\nm\nl\nxl
module Main where
size :: Int -> String
size n | n > 100 = "xl" | n > 50 = "l" | n > 25 = "m" | n > 10 = "s" | otherwise = "xs"
main = do { putStrLn (size 5); putStrLn (size 15); putStrLn (size 30); putStrLn (size 60); putStrLn (size 200) }
