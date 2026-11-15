-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. - John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module HomeShowDefaultChirho where
import GHC.Internal.Num (Num(..))
import GHC.Internal.Bignum.Integer (Integer)
class Show aChirho where tagChirho :: aChirho -> Integer
instance Show Integer where tagChirho xChirho = xChirho
resultChirho = tagChirho (1 + 2)
