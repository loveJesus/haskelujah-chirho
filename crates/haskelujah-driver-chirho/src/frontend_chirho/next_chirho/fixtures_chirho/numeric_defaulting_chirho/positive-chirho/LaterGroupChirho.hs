-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module LaterGroupChirho where
import GHC.Internal.Num (Num(..))
data TChirho = TChirho
instance Num TChirho where
  (+) _ _ = TChirho
  (*) _ _ = TChirho
  negate xChirho = xChirho
  abs xChirho = xChirho
  signum xChirho = xChirho
  fromInteger _ = TChirho
valueChirho = 1
pickChirho TChirho = 'b'
resultChirho = pickChirho valueChirho
