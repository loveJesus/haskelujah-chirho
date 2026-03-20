-- TEST: compile
-- foldr with 2-arg HOF should compile
module T028 where
myFoldr :: (a -> b -> b) -> b -> [a] -> b
myFoldr _ z [] = z
myFoldr f z (x:xs) = f x (myFoldr f z xs)
result :: Int
result = myFoldr (+) 0 [1, 2, 3, 4, 5]
