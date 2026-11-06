-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

-- Restricted, so it has ONE type: Fractional from `/` and Int from `length` cannot both hold.
halfChirho = 5

main :: IO ()
main = print (halfChirho / 2, length "ab" + halfChirho)
