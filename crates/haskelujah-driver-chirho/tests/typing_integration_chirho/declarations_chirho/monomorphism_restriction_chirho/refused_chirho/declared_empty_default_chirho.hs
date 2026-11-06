-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

default ()

-- An explicit empty default list disables defaulting, so this literal stays ambiguous.
fiveChirho = 5

main :: IO ()
main = print fiveChirho
