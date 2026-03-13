module Layout where

f x = let a = 1
          b = 2
      in a + b + x

g y = case y of
  Nothing -> 0
  Just x  -> x

h = do
  putStrLn "hello"
  putStrLn "world"
  return ()

i x
  | x > 0     = "positive"
  | x < 0     = "negative"
  | otherwise  = "zero"
