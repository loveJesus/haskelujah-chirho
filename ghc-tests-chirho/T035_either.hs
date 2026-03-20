-- TEST: compile_and_run
-- EXPECTED: 42\n-1\n1\n0
module Main where
data Either a b = Left a | Right b
fromRight :: Int -> Either Int Int -> Int
fromRight def (Left _) = def
fromRight _ (Right x) = x
isRight :: Either Int Int -> Int
isRight (Right _) = 1
isRight (Left _) = 0
main :: IO ()
main = do
  print (fromRight 0 (Right 42))
  print (fromRight (-1) (Left 99))
  print (isRight (Right 7))
  print (isRight (Left 3))
