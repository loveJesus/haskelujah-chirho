-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

-- Function syntax is never restricted, so one definition serves Int and Double.
plusOneChirho valueChirho = valueChirho + 1

main :: IO ()
main = print (plusOneChirho (1 :: Int), plusOneChirho (2.5 :: Double))
