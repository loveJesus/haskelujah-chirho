module DataTypes where

data Color = Red | Green | Blue
  deriving (Show, Eq)

data Maybe a = Nothing | Just a

data Tree a
  = Leaf a
  | Branch (Tree a) (Tree a)
  deriving (Show)

newtype Wrapper a = Wrapper { unwrap :: a }

type Name = String
type Pair a b = (a, b)
