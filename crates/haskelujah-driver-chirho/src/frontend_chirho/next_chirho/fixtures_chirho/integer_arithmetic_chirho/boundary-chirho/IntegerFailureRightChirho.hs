-- For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
{-# LANGUAGE NoImplicitPrelude #-}
module IntegerFailureRightChirho where
import GHC.Internal.Num (Num(..))
import GHC.Internal.Bignum.Integer (Integer)
data FailureChirho = FirstChirho | SecondChirho
boomChirho :: Integer
boomChirho = case FirstChirho of SecondChirho -> 1
resultChirho :: Integer
resultChirho = 0 * boomChirho
