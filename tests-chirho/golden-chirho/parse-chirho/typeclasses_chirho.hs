module Typeclasses where

class Describable a where
  describe :: a -> String

class (Eq a) => Orderable a where
  compare' :: a -> a -> Ordering

instance Describable Int where
  describe n = "An integer: " ++ show n

instance Describable Bool where
  describe True  = "Yes"
  describe False = "No"

data Shape = Circle Double | Rectangle Double Double

instance Describable Shape where
  describe (Circle r) = "Circle with radius " ++ show r
  describe (Rectangle w h) = "Rectangle " ++ show w ++ "x" ++ show h
