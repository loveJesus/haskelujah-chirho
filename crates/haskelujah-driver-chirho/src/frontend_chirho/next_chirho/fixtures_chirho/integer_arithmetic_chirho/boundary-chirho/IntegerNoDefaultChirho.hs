-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module IntegerNoDefaultChirho where
import GHC.Internal.Num (Num(..))
import GHC.Internal.Bignum.Integer (Integer)
data TChirho = AChirho | BChirho
class HomeChirho aChirho where tagChirho :: aChirho -> TChirho
instance HomeChirho Integer where tagChirho _ = AChirho
charChirho AChirho = 'a'
charChirho BChirho = 'b'
resultChirho = charChirho (tagChirho (1 + 2))
