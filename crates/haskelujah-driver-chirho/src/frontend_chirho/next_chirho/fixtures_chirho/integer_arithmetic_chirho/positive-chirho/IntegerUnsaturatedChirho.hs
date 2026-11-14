-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module IntegerUnsaturatedChirho where
import GHC.Internal.Num (Num(..))
import GHC.Internal.Bignum.Integer (Integer, integerAdd)
applyChirho :: (Integer -> Integer) -> Integer
applyChirho functionChirho = functionChirho 41
resultChirho :: Integer
resultChirho = applyChirho (integerAdd 1)
negativeChirho :: Integer
negativeChirho = applyChirho negate
