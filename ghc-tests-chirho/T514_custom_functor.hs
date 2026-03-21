-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: [2,4,6]\n[10,20,30,40]
module Main where
class MyFunctor f where
  fmap2 :: (a -> b) -> f a -> f b
data MyList a = MyNil | MyCons a (MyList a)
instance MyFunctor MyList where
  fmap2 _ MyNil = MyNil
  fmap2 f (MyCons x xs) = MyCons (f x) (fmap2 f xs)
toList :: MyList a -> [a]
toList MyNil = []
toList (MyCons x xs) = x : toList xs
main = do
  let xs = MyCons 1 (MyCons 2 (MyCons 3 MyNil))
  print (toList (fmap2 (* 2) xs))
  let ys = MyCons 10 (MyCons 20 (MyCons 30 (MyCons 40 MyNil)))
  print (toList (fmap2 id ys))
