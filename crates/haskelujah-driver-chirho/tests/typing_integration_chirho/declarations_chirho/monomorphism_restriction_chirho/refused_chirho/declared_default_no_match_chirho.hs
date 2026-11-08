-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

default (Int)

-- Int is not Fractional, so the declared list has no match; nothing else may stand in.
halfChirho = 5 / 2

main :: IO ()
main = print halfChirho
