-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, ConstraintKinds, FlexibleContexts #-}
module Main where
type RankChirho = forall aChirho. Eq aChirho => aChirho -> Bool
rankChirho :: RankChirho
rankChirho xChirho = xChirho == True
main :: IO ()
main = pure ()
