-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import Prelude hiding ((>>))

(>>) :: Int -> Int -> Int
leftChirho >> rightChirho = leftChirho * 10 + rightChirho

digitsChirho :: Monad mChirho => mChirho Int -> mChirho Int
digitsChirho actionChirho = actionChirho >>= \valueChirho -> return (valueChirho >> 3)

main :: IO ()
main = print (digitsChirho (Just 4)) >>= \_ -> digitsChirho (return 5) >>= print
