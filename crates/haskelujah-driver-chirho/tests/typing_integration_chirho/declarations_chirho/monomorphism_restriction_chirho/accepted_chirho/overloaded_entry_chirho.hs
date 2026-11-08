-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

-- An explicitly overloaded entry: the entry's role instantiates its Num result and runs it.
main :: Num a => IO a
main = print 1 >> return 5
