-- TEST: compile
-- Type aliases
module T020 where

type Name = String
type Pair a = (a, a)
type IntPair = Pair Int
type Predicate a = a -> Bool

isPositive :: Predicate Int
isPositive n = n > 0
