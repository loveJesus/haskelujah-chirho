-- TEST: compile_and_run
-- EXPECTED: 3\n5\n8
module Main where
type Church = (Int -> Int) -> Int -> Int
zero :: Church
zero _ x = x
succ_ :: Church -> Church
succ_ n f x = f (n f x)
add_ :: Church -> Church -> Church
add_ m n f x = m f (n f x)
toInt :: Church -> Int
toInt n = n (+1) 0
main :: IO ()
main = do
  let three = succ_ (succ_ (succ_ zero))
  let five = succ_ (succ_ three)
  print (toInt three)
  print (toInt five)
  print (toInt (add_ three five))
