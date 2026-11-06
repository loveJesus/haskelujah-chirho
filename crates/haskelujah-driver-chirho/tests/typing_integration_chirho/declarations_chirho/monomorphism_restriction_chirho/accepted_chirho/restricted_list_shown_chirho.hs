-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

-- Restricted: `Show [t]` reduces through the list instance to `Show t`, then Integer.
numbersChirho = [1, 2, 3]

main :: IO ()
main = print numbersChirho
