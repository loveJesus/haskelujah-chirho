-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module IntegerSharingChirho where
import GHC.Internal.Num (Num(..))
import GHC.Internal.Bignum.Integer (Integer)
pairChirho :: (Integer, Integer)
pairChirho = let valueChirho = 340282366920938463463374607431768211457 + 1 in (valueChirho, valueChirho)
resultChirho :: Integer
resultChirho = case pairChirho of (leftChirho, rightChirho) -> leftChirho + rightChirho
