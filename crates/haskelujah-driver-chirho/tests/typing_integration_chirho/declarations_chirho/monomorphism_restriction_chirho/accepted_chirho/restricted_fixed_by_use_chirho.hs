-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

-- Restricted: the later use fixes both the monad and the number, so this is Maybe Int.
retChirho = return 5

main :: IO ()
main = print (retChirho :: Maybe Int)
