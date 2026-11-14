-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module IntegerStrictLeftChirho where
import GHC.Internal.Num (Num(..))
import GHC.Internal.Bignum.Integer (Integer)
loopChirho :: Integer
loopChirho = loopChirho
resultChirho :: Integer
resultChirho = loopChirho * 0
