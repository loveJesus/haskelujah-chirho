-- TEST: compile
-- Algebraic data types
module T004 where

data Color = Red | Green | Blue

data Maybe' a = Nothing' | Just' a

data List a = Nil | Cons a (List a)

data Tree a = Leaf | Node (Tree a) a (Tree a)
