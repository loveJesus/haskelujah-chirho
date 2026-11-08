-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, ConstraintKinds, FlexibleContexts #-}
module Main where
type CurriedChirho = Int -> (forall aChirho. Eq aChirho => aChirho -> Bool)
curriedChirho :: CurriedChirho
curriedChirho _ xChirho = xChirho == xChirho
main :: IO ()
main = pure ()
