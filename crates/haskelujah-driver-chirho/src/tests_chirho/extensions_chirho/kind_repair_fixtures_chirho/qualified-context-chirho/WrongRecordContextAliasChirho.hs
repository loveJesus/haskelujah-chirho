-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes #-}
module Main where
type CheckChirho = forall aChirho. Eq aChirho => aChirho -> Bool
data BoxChirho = BoxChirho { runChirho :: CheckChirho }
boxChirho :: BoxChirho
boxChirho = BoxChirho { runChirho = \valueChirho -> valueChirho == True }
main :: IO ()
main = pure ()
