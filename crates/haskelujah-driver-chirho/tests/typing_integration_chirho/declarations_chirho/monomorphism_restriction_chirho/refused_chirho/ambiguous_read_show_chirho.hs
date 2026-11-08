-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

-- Restricted, with a String result: the `Read a, Show a` it leaves on `a` has no default.
textChirho = show (read "1")

main :: IO ()
main = putStrLn textChirho
