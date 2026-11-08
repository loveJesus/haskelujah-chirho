-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
module Main where

import qualified Prelude as P

-- No program class shares a spelling here: the qualified given names the Prelude's Num,
-- and the unqualified `+` in the body asks for that same class.
twiceChirho :: P.Num a => a -> a
twiceChirho valueChirho = valueChirho + valueChirho

main :: IO ()
main = print (twiceChirho (21 :: Int))
