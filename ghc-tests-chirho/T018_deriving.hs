-- TEST: compile
-- Deriving Eq, Ord, Show
module T018 where

data Color = Red | Green | Blue
  deriving (Eq, Ord, Show)

data Pair a b = MkPair a b
  deriving (Eq, Show)
