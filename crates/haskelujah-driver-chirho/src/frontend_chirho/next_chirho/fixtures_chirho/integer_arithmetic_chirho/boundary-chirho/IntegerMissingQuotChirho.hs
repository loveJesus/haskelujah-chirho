-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module IntegerMissingQuotChirho where
import GHC.Internal.Num (Num(..))
import GHC.Internal.Bignum.Integer (Integer, integerQuot)
resultChirho :: Integer
resultChirho = integerQuot 9 2
