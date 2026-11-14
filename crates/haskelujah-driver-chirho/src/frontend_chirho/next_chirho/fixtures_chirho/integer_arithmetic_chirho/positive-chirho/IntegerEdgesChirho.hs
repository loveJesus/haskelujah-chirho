-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module IntegerEdgesChirho where
import GHC.Internal.Num (Num(..))
import GHC.Internal.Bignum.Integer (Integer)
overflowChirho :: Integer
overflowChirho = 9223372036854775807 + 1
negativeEdgeChirho :: Integer
negativeEdgeChirho = negate (-9223372036854775808)
absoluteEdgeChirho :: Integer
absoluteEdgeChirho = abs (-9223372036854775808)
cancelChirho :: Integer
cancelChirho = 9223372036854775808 + (-9223372036854775808)
productChirho :: Integer
productChirho = 18446744073709551616 * 18446744073709551616
negativeProductChirho :: Integer
negativeProductChirho = (-18446744073709551616) * 18446744073709551616
negativeSignChirho :: Integer
negativeSignChirho = signum (-12345678901234567890)
zeroSignChirho :: Integer
zeroSignChirho = signum 0
positiveSignChirho :: Integer
positiveSignChirho = signum 12345678901234567890
subtractChirho :: Integer
subtractChirho = 5 - 7
