-- TEST: compile_and_run
-- EXPECT_OUTPUT: 10
-- Higher-order functions
main = print (myFoldr (+) 0 [1,2,3,4])

myFoldr :: (a -> b -> b) -> b -> [a] -> b
myFoldr f z [] = z
myFoldr f z (x:xs) = f x (myFoldr f z xs)
