-- For God so loved the world that he gave his only begotten Son, that whoever
-- believes in him should not perish but have eternal life. — John 3:16
-- TEST: compile_and_run
-- EXPECTED: 15\n15\n120\nhello world
module Main where
myFoldr :: (a -> b -> b) -> b -> [a] -> b
myFoldr _ z [] = z
myFoldr f z (x:xs) = f x (myFoldr f z xs)
myFoldl :: (b -> a -> b) -> b -> [a] -> b
myFoldl _ acc [] = acc
myFoldl f acc (x:xs) = myFoldl f (f acc x) xs
main = do
  print (myFoldr (+) 0 [1,2,3,4,5])
  print (myFoldl (+) 0 [1,2,3,4,5])
  print (myFoldl (*) 1 [1,2,3,4,5])
  putStrLn (myFoldr (\x acc -> x ++ acc) "" ["hello", " ", "world"])
