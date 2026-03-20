-- TEST: compile_and_run
-- EXPECTED: 42\n30
module Main where
makeAdder :: Int -> Int -> Int
makeAdder n = \x -> x + n
applyN :: Int -> (Int -> Int) -> Int -> Int
applyN 0 _ x = x
applyN n f x = applyN (n - 1) f (f x)
main :: IO ()
main = do
  print (makeAdder 5 37)
  print (applyN 3 (makeAdder 10) 0)
