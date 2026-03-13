module Operators where

infixl 6 +!
infixr 5 .:

(+!) :: Int -> Int -> Int
x +! y = x + y + 1

(.:) :: (b -> c) -> (a1 -> a2 -> b) -> a1 -> a2 -> c
(.:) = (.) . (.)

result = 1 +! 2
composed = show .: (+)

qualified = Data.List.sort [3, 1, 2]
