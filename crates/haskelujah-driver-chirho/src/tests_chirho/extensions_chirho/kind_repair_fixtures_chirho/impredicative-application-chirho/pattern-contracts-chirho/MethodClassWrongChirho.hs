-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE RankNTypes, TypeApplications #-}
module MethodClassWrongChirho where
class CChirho aChirho where
  methodChirho :: forall bChirho. aChirho -> bChirho -> aChirho
instance CChirho Bool where
  methodChirho xChirho _ = xChirho
valueChirho :: Bool
valueChirho = methodChirho @Int True (3 :: Int)
